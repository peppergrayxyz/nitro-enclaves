// SPDX-License-Identifier: Apache-2.0

use nitro_enclaves::{
    device::Device,
    launch::{ImageType, Launcher, MemoryInfo, PollTimeout, StartFlags},
};
use nix::{
    poll::{poll, PollFd, PollFlags},
    sys::{
        socket::{connect, socket, AddressFamily, SockFlag, SockType, VsockAddr as NixVsockAddr},
        time::{TimeVal, TimeValLike},
    },
    unistd::read,
};
use std::{
    fs::File,
    io::{Read, Write},
    os::fd::{AsRawFd, RawFd},
};
use vsock::{VsockAddr, VsockListener};

const ENCLAVE_READY_VSOCK_PORT: u32 = 9000;
const CID_TO_CONSOLE_PORT_OFFSET: u32 = 10000;

const VMADDR_CID_PARENT: u32 = 3;
const VMADDR_CID_HYPERVISOR: u32 = 0;

const SO_VM_SOCKETS_CONNECT_TIMEOUT: i32 = 6;

const HEART_BEAT: u8 = 0xb7;

#[test]
fn launch() {
    let device = Device::open().unwrap();

    let mut launcher = Launcher::new(&device).unwrap();

    let mut eif = File::open("tests/test_data/hello.eif").unwrap();
    let mem = MemoryInfo::new(ImageType::Eif(&mut eif), 512);

    launcher.mem_set(mem).unwrap();
    launcher.vcpu_add(None).unwrap();

    let sockaddr = VsockAddr::new(VMADDR_CID_PARENT, ENCLAVE_READY_VSOCK_PORT);
    let listener = VsockListener::bind(&sockaddr).unwrap();

    let cid: u32 = launcher
        .start(StartFlags::DEBUG, None)
        .unwrap()
        .try_into()
        .unwrap();

    let poll_timeout = PollTimeout::try_from((&eif, 512 << 20)).unwrap();

    enclave_check(listener, poll_timeout.into(), cid);

    listen(VMADDR_CID_HYPERVISOR, cid + CID_TO_CONSOLE_PORT_OFFSET);
}

pub fn enclave_check(listener: VsockListener, poll_timeout_ms: libc::c_int, cid: u32) {
    let mut poll_fds = [PollFd::new(listener.as_raw_fd(), PollFlags::POLLIN)];
    let result = poll(&mut poll_fds, poll_timeout_ms);
    if result == Ok(0) {
        panic!("no pollfds have selected events");
    } else if result != Ok(1) {
        panic!("more than one pollfd has selected events");
    }

    let mut stream = listener.accept().unwrap();

    // Wait until the other end is closed
    let mut buf = [0u8];
    let bytes = stream.0.read(&mut buf).unwrap();

    if bytes != 1 || buf[0] != HEART_BEAT {
        panic!("enclave check produced wrong output");
    }

    stream.0.write_all(&buf).unwrap();

    if stream.1.cid() != cid {
        panic!("CID mismatch");
    }
}

fn listen(cid: u32, port: u32) {
    let socket_fd = socket(
        AddressFamily::Vsock,
        SockType::Stream,
        SockFlag::empty(),
        None,
    )
    .unwrap();

    let sockaddr = NixVsockAddr::new(cid, port);

    vsock_timeout(socket_fd);
    connect(socket_fd, &sockaddr).unwrap();

    let mut boot_msg_found = false;

    let mut buf = [0u8; 512];
    loop {
        let ret = read(socket_fd, &mut buf);
        let Ok(sz) = ret else {
            break;
        };
        if sz != 0 {
            let msg = String::from_utf8(buf[..sz].to_vec()).unwrap();
            if msg.contains("Booting Linux") {
                boot_msg_found = true;
            }
            print!("{}", msg);
        } else {
            break;
        }
    }

    if !boot_msg_found {
        panic!("Linux boot message not found from vsock output");
    }
}

fn vsock_timeout(socket_fd: RawFd) {
    let timeval = TimeVal::milliseconds(20000);
    let ret = unsafe {
        libc::setsockopt(
            socket_fd,
            libc::AF_VSOCK,
            SO_VM_SOCKETS_CONNECT_TIMEOUT,
            &timeval as *const _ as *const libc::c_void,
            size_of::<TimeVal>() as u32,
        )
    };

    if ret != 0 {
        panic!("error setting vsock timeout");
    }
}

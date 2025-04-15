// SPDX-License-Identifier: Apache-2.0

use nitro_enclaves::{
    device::Device,
    launch::{ImageType, Launcher, MemoryInfo},
};
use std::fs::File;

#[test]
fn launch() {
    let device = Device::open().unwrap();

    let mut launcher = Launcher::new(&device).unwrap();

    let eif = File::open("tests/test_data/hello.eif").unwrap();
    let mem = MemoryInfo::new(ImageType::Eif(eif), 512);

    launcher.mem_set(mem).unwrap();
    launcher.vcpu_add(None).unwrap();
}

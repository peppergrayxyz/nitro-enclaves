// SPDX-License-Identifier: Apache-2.0

use nitro_enclaves::{
    device::Device,
    launch::{ImageType, Launcher, MemoryInfo},
};

#[test]
fn launch() {
    let device = Device::open().unwrap();

    let mut launcher = Launcher::new(&device).unwrap();

    let mem = MemoryInfo::new(ImageType::Eif, 256);

    launcher.mem_set(mem).unwrap();
    launcher.vcpu_add(None).unwrap();
}

// SPDX-License-Identifier: Apache-2.0

use nitro_enclaves::{
    device::Device,
    launch::{ImageType, Launcher},
};

#[test]
fn launch() {
    let device = Device::open().unwrap();

    let mut launcher = Launcher::new(&device, ImageType::Eif).unwrap();
    launcher.vcpu_add(None).unwrap();
}

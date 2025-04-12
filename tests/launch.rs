// SPDX-License-Identifier: Apache-2.0

use nitro_enclaves::{device::Device, launch::Launcher};

#[test]
fn launch() {
    let device = Device::open().unwrap();

    let _launcher = Launcher::new(device);
}

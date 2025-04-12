// SPDX-License-Identifier: Apache-2.0

use nitro_enclaves::device::Device;

#[test]
fn launch() {
    let _device = Device::open().unwrap();
}

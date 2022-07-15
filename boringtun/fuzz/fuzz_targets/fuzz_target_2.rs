#![no_main]
use libfuzzer_sys::fuzz_target;
use boringtun::device;
use std::str;

fuzz_target!(|data: &[u8]| {
    device::DeviceHandle::new(str::from_utf8(&data).unwrap(), device::DeviceConfig::default()).unwrap();
});

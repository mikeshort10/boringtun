#![no_main]
use libfuzzer_sys::fuzz_target;
use boringtun::device::tun::TunSocket;
use std::str;

fuzz_target!(|data: &[u8]| {
    TunSocket::new(str::from_utf8(&data).unwrap());
});

#![no_main]
#[macro_use] extern crate libfuzzer_sys;
extern crate boringtun;

fuzz_target!(|data: &[u8]| {
    // if let Ok(s) = std::str::from_utf8(data) {
        let _ = boringtun::device::poll::block_signal(i32::from(data[0]));
    // }
});

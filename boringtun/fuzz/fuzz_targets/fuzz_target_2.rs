#![no_main]
use libfuzzer_sys::fuzz_target;
use boringtun::crypto::Blake2s;

fuzz_target!(|data: &[u8]| {
    Blake2s::new_mac(&data);
});

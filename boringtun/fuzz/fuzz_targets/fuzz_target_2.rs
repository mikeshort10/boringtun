#![no_main]
use libfuzzer_sys::fuzz_target;
use boringtun::crypto::Blake2s;

fuzz_target!(|data: &[u8]| {
    // let outlen: usize = 0;
    // let mac: bool = true;
    // let black = Blake2s::new(data, outlen, mac);

    Blake2s::new_mac(data);
});

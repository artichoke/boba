#![no_main]
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let encoded = boba::encode(data);
    let roundtripped = boba::decode(encoded).unwrap();
    assert_eq!(roundtripped, data);
});

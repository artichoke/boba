#![no_main]
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let encoded = bubblebabble::encode(data);
    let roundtripped = bubblebabble::decode(encoded).unwrap();
    assert_eq!(roundtripped, data);
});

use alloc::string::String;

const VOWELS: [u8; 6] = *b"aeiouy";
const CONSONANTS: [u8; 16] = *b"bcdfghklmnprstvz";
const HEADER: &str = "x";
const TRAILER: &str = "x";
const SEPARATOR: &str = "-";
const MID: &str = "x";

#[must_use]
pub fn inner(data: &[u8]) -> String {
    if data.is_empty() {
        return String::from("xexax");
    }

    let mut encoded = String::with_capacity(6 * (data.len() / 2) + 3 + 2);
    encoded.push_str(HEADER);
    let mut checksum = 1_u8;
    let mut chunks = data.chunks_exact(2);
    while let Some(&[left, right]) = chunks.next() {
        odd_partial(left, checksum, &mut encoded);
        let d = (right >> 4) & 15;
        let e = right & 15;
        // Panic safety:
        //
        // - `d` is constructed with a mask of `0b1111`.
        // - `CONSONANTS` is a fixed size array with 16 elements.
        // - Maximum value of `d` is 15.
        encoded.push(CONSONANTS[d as usize].into());
        encoded.push_str(SEPARATOR);
        // Panic safety:
        //
        // - `e` is constructed with a mask of `0b1111`.
        // - `CONSONANTS` is a fixed size array with 16 elements.
        // - Maximum value of `e` is 15.
        encoded.push(CONSONANTS[e as usize].into());
        checksum = ((u16::from(checksum * 5) + u16::from(left) * 7 + u16::from(right)) % 36) as u8;
    }
    if let [byte] = chunks.remainder() {
        odd_partial(*byte, checksum, &mut encoded);
    } else {
        even_partial(checksum, &mut encoded);
    }
    encoded.push_str(TRAILER);
    encoded
}

#[inline]
fn odd_partial(raw_byte: u8, checksum: u8, buf: &mut String) {
    let a = (((raw_byte >> 6) & 3) + checksum) % 6;
    let b = (raw_byte >> 2) & 15;
    let c = ((raw_byte & 3) + checksum / 6) % 6;
    // Panic safety:
    //
    // - `a` is constructed with mod 6.
    // - `VOWELS` is a fixed size array with 6 elements.
    // - Maximum value of `a` is 5.
    buf.push(VOWELS[a as usize].into());
    // Panic safety:
    //
    // - `b` is constructed with a mask of `0b1111`.
    // - `CONSONANTS` is a fixed size array with 16 elements.
    // - Maximum value of `e` is 15.
    buf.push(CONSONANTS[b as usize].into());
    // Panic safety:
    //
    // - `c` is constructed with mod 6.
    // - `VOWELS` is a fixed size array with 6 elements.
    // - Maximum value of `c` is 5.
    buf.push(VOWELS[c as usize].into());
}

#[inline]
fn even_partial(checksum: u8, buf: &mut String) {
    let a = checksum % 6;
    // let b = 16;
    let c = checksum / 6;
    // Panic safety:
    //
    // - `a` is constructed with mod 6.
    // - `VOWELS` is a fixed size array with 6 elements.
    // - Maximum value of `a` is 5.
    buf.push(VOWELS[a as usize].into());
    buf.push_str(MID);
    // Panic safety:
    //
    // - `c` is constructed with divide by 6.
    // - Maximum value of `checksum` is 36 -- see `encode` loop.
    // - `VOWELS` is a fixed size array with 6 elements.
    // - Maximum value of `c` is 5.
    buf.push(VOWELS[c as usize].into());
}

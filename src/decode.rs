use alloc::vec::Vec;

use crate::DecodeError;

const HEADER: u8 = b'x';
const TRAILER: u8 = b'x';

// one `bool` for every byte. The positions that are set to `true` are the byte
// values for characters in the alphabet:
//
// ```
// const ALPHABET: [u8; 24] = *b"aeiouybcdfghklmnprstvzx-";
// ```
//
// This table is generated with the following Ruby script:
//
// ```ruby
// a = Array.new(256, 0)
// bytes = "aeiouybcdfghklmnprstvzx-".split("").map(&:ord)
// bytes.each {|b| a[b] = 1}
// puts a.inspect
// ```
const ALPHABET_TABLE: [u8; 256] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 0, 1, 1, 1, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
];

pub fn inner(encoded: &[u8]) -> Result<Vec<u8>, DecodeError> {
    // `xexax` is the encoded representation of an empty bytestring. Test for it
    // directly to short circuit.
    if encoded == b"xexax" {
        return Ok(Vec::new());
    }
    let enc = match encoded {
        [HEADER, enc @ .., TRAILER] => enc,
        [HEADER, ..] => return Err(DecodeError::MalformedTrailer),
        [.., TRAILER] => return Err(DecodeError::MalformedHeader),
        _ => return Err(DecodeError::Corrupted),
    };
    // This validation step ensures that the encoded bytestring only contains
    // ASCII bytes in the 24 character encoding alphabet.
    //
    // Code below must still handle None results from find_byte because bytes
    // may not be from the right subset of the alphabet, e.g. a vowel present
    // when a consonant is expected.
    if let Some((_, pos)) = enc
        .iter()
        .zip(1_usize..) // start pos at 1 because we stripped off a leading 'x'
        .find(|(&byte, _)| ALPHABET_TABLE[usize::from(byte)] == 0)
    {
        return Err(DecodeError::InvalidByte(pos));
    }
    let mut decoded = {
        let len = encoded.len();
        Vec::with_capacity(if len == 5 { 1 } else { 2 * ((len + 1) / 6) })
    };
    let mut checksum = 1_u8;
    let mut chunks = enc.chunks_exact(6);
    while let Some(&[left, mid, right, up, b'-', down]) = chunks.next() {
        let byte1 = decode_3_tuple(
            index_from_vowel(left).ok_or(DecodeError::ExpectedVowel)?,
            index_from_consonant(mid).ok_or(DecodeError::ExpectedConsonant)?,
            index_from_vowel(right).ok_or(DecodeError::ExpectedVowel)?,
            checksum,
        )?;
        let byte2 = decode_2_tuple(
            index_from_consonant(up).ok_or(DecodeError::ExpectedConsonant)?,
            index_from_consonant(down).ok_or(DecodeError::ExpectedConsonant)?,
        );
        checksum =
            ((u16::from(checksum * 5) + (u16::from(byte1) * 7) + u16::from(byte2)) % 36) as u8;
        decoded.push(byte1);
        decoded.push(byte2);
    }
    if let [left, mid, right] = *chunks.remainder() {
        let a = index_from_vowel(left).ok_or(DecodeError::ExpectedVowel)?;
        let c = index_from_vowel(right).ok_or(DecodeError::ExpectedVowel)?;

        match mid {
            b'x' if a != checksum % 6 || c != checksum / 6 => Err(DecodeError::ChecksumMismatch),
            b'x' => Ok(decoded),
            _ => {
                let b = index_from_consonant(mid).ok_or(DecodeError::ExpectedConsonant)?;
                let byte = decode_3_tuple(a, b, c, checksum)?;
                decoded.push(byte);
                Ok(decoded)
            }
        }
    } else {
        Err(DecodeError::Corrupted)
    }
}

#[inline]
fn index_from_consonant(consonant: u8) -> Option<u8> {
    let index = match consonant {
        b'b' => 0,
        b'c' => 1,
        b'd' => 2,
        b'f' => 3,
        b'g' => 4,
        b'h' => 5,
        b'k' => 6,
        b'l' => 7,
        b'm' => 8,
        b'n' => 9,
        b'p' => 10,
        b'r' => 11,
        b's' => 12,
        b't' => 13,
        b'v' => 14,
        b'z' => 15,
        _ => return None,
    };
    Some(index)
}

#[inline]
fn index_from_vowel(vowel: u8) -> Option<u8> {
    let index = match vowel {
        b'a' => 0,
        b'e' => 1,
        b'i' => 2,
        b'o' => 3,
        b'u' => 4,
        b'y' => 5,
        _ => return None,
    };
    Some(index)
}

#[inline]
fn decode_3_tuple(byte1: u8, byte2: u8, byte3: u8, checksum: u8) -> Result<u8, DecodeError> {
    // Will not overflow since:
    // - byte1 is guaranteed to be ASCII or < 128.
    // Will not underflow since:
    // - 6 - (checksum % 6) > 0
    let high = (byte1 + 6 - (checksum % 6)) % 6;
    let mid = byte2;
    // Will not overflow since:
    // - byte3 is guaranteed to be ASCII or < 128.
    // Will not underflow since:
    // - 6 - ((checksum / 6) % 6) > 0
    let low = (byte3 + 6 - ((checksum / 6) % 6)) % 6;
    if high >= 4 || low >= 4 {
        Err(DecodeError::Corrupted)
    } else {
        Ok((high << 6) | (mid << 2) | low)
    }
}

#[inline]
fn decode_2_tuple(byte1: u8, byte2: u8) -> u8 {
    (byte1 << 4) | byte2
}

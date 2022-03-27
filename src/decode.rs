#[cfg(feature = "alloc")]
use alloc::vec::Vec;

use bstr::ByteSlice;

use crate::{DecodeError, ALPHABET, CONSONANTS, HEADER, TRAILER, VOWELS};

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
    if let Some(pos) = enc.find_not_byteset(ALPHABET) {
        // Return `pos + 1` because the subslicing above removes the initial 'x'
        // header byte.
        return Err(DecodeError::InvalidByte(pos + 1));
    }
    let mut decoded = {
        let len = encoded.len();
        Vec::with_capacity(if len == 5 { 1 } else { 2 * ((len + 1) / 6) })
    };
    let mut checksum = 1_u8;
    let mut chunks = enc.chunks_exact(6);
    while let Some(&[left, mid, right, up, b'-', down]) = chunks.next() {
        let byte1 = decode_3_tuple(
            VOWELS.find_byte(left).ok_or(DecodeError::ExpectedVowel)? as u8,
            CONSONANTS
                .find_byte(mid)
                .ok_or(DecodeError::ExpectedConsonant)? as u8,
            VOWELS.find_byte(right).ok_or(DecodeError::ExpectedVowel)? as u8,
            checksum,
        )?;
        let byte2 = decode_2_tuple(
            CONSONANTS
                .find_byte(up)
                .ok_or(DecodeError::ExpectedConsonant)? as u8,
            CONSONANTS
                .find_byte(down)
                .ok_or(DecodeError::ExpectedConsonant)? as u8,
        );
        checksum =
            ((u16::from(checksum * 5) + (u16::from(byte1) * 7) + u16::from(byte2)) % 36) as u8;
        decoded.push(byte1);
        decoded.push(byte2);
    }
    if let [left, mid, right] = *chunks.remainder() {
        let a = VOWELS.find_byte(left).ok_or(DecodeError::ExpectedVowel)? as u8;
        let c = VOWELS.find_byte(right).ok_or(DecodeError::ExpectedVowel)? as u8;

        match mid {
            b'x' if a != checksum % 6 || c != checksum / 6 => Err(DecodeError::ChecksumMismatch),
            b'x' => Ok(decoded),
            _ => {
                let b = CONSONANTS
                    .find_byte(mid)
                    .ok_or(DecodeError::ExpectedConsonant)? as u8;
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

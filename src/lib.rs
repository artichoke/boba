#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![allow(clippy::cast_possible_truncation)]
#![warn(clippy::cargo)]
#![warn(missing_docs, intra_doc_link_resolution_failure)]
#![warn(missing_debug_implementations)]
#![warn(rust_2018_idioms)]
#![forbid(unsafe_code)]

//! The Bubble Babble binary data encoding.
//!
//! This is a native Rust implementation of a Bubble Babble encoder and decoder.
//!
//! # Usage
//!
//! You can encode binary data by calling [`encode`]:
//!
//! ```
//! let enc = boba::encode("Pineapple");
//! assert_eq!(enc, "xigak-nyryk-humil-bosek-sonax");
//! ```
//!
//! Decoding binary data is done by calling [`decode`]:
//!
//! ```
//! # fn main() -> Result<(), boba::DecodeError> {
//! let dec = boba::decode("xexax")?;
//! assert_eq!(dec, vec![]);
//! # Ok(())
//! # }
//! ```
//!
//! Decode is fallible and can return [`DecodeError`]. For example, all Bubble
//! Babble-encoded data has an ASCII alphabet, so attempting to decode an emoji
//! will fail.
//!
//! ```
//! # use boba::DecodeError;
//! let dec = boba::decode("x🦀x");
//! // The `DecodeError` contains the offset of the first invalid byte.
//! assert_eq!(Err(DecodeError::InvalidByte(1)), dec);
//! ```

#![doc(html_root_url = "https://docs.rs/boba/3.0.0")]
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(doctest)]
doc_comment::doctest!("../README.md");

use bstr::ByteSlice;
use core::fmt;

const VOWELS: [u8; 6] = *b"aeiouy";
const CONSONANTS: [u8; 16] = *b"bcdfghklmnprstvz";
const ALPHABET: [u8; 24] = *b"aeiouybcdfghklmnprstvzx-";

const HEADER: u8 = b'x';
const TRAILER: u8 = b'x';

/// Decoding errors from [`boba::decode`](decode).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DecodeError {
    /// Checksum mismatch when decoding input.
    ChecksumMismatch,
    /// Corrupted input caused a decoding failure.
    Corrupted,
    /// Expected to process a consonant from the encoding alphabet, but got
    /// something else.
    ExpectedConsonant,
    /// Expected to process a vowel from the encoding alphabet, but got
    /// something else.
    ExpectedVowel,
    /// Input contained a byte not in the encoding alphabet at this position.
    InvalidByte(usize),
    /// Input was missing a leading `x` header.
    MalformedHeader,
    /// Input was missing a final `x` trailer.
    MalformedTrailer,
}

#[cfg(feature = "std")]
impl std::error::Error for DecodeError {}

impl fmt::Display for DecodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ChecksumMismatch => write!(f, "Checksum mismatch"),
            Self::Corrupted => write!(f, "Corrupted input"),
            Self::ExpectedConsonant => write!(f, "Expected consonant, got something else"),
            Self::ExpectedVowel => write!(f, "Expected vowel, got something else"),
            Self::InvalidByte(pos) => write!(
                f,
                "Encountered byte outside of encoding alphabet at position {}",
                pos
            ),
            Self::MalformedHeader => write!(f, "Missing required 'x' header"),
            Self::MalformedTrailer => write!(f, "Missing required 'x' trailer"),
        }
    }
}

/// Encode a byte slice with the Bubble Babble encoding to a [`String`].
///
/// # Examples
///
/// ```
/// assert_eq!(boba::encode([]), "xexax");
/// assert_eq!(boba::encode("1234567890"), "xesef-disof-gytuf-katof-movif-baxux");
/// assert_eq!(boba::encode("Pineapple"), "xigak-nyryk-humil-bosek-sonax");
/// ```
#[must_use]
pub fn encode<T: AsRef<[u8]>>(data: T) -> String {
    let data = data.as_ref();
    let mut encoded = String::with_capacity(6 * (data.len() / 2) + 3 + 2);
    encoded.push(HEADER.into());
    let mut checksum = 1_u8;
    let mut chunks = data.chunks_exact(2);
    while let Some([left, right]) = chunks.next() {
        odd_partial(*left, checksum, &mut encoded);
        let d = (*right >> 4) & 15;
        let e = *right & 15;
        // Panic safety:
        //
        // - `d` is constructed with a mask of `0b1111`.
        // - `CONSONANTS` is a fixed size array with 17 elements.
        // - Maximum value of `d` is 16.
        encoded.push(CONSONANTS[d as usize].into());
        encoded.push('-');
        // Panic safety:
        //
        // - `e` is constructed with a mask of `0b1111`.
        // - `CONSONANTS` is a fixed size array with 17 elements.
        // - Maximum value of `e` is 15.
        encoded.push(CONSONANTS[e as usize].into());
        checksum =
            ((u16::from(checksum * 5) + u16::from(*left) * 7 + u16::from(*right)) % 36) as u8;
    }
    if let [byte] = chunks.remainder() {
        odd_partial(*byte, checksum, &mut encoded);
    } else {
        even_partial(checksum, &mut encoded);
    }
    encoded.push(TRAILER.into());
    encoded
}

/// Decode Bubble Babble-encoded byte slice to a `Vec<u8>`.
///
/// # Examples
///
/// ```
/// assert_eq!(boba::decode("xexax"), Ok(vec![]));
/// assert_eq!(boba::decode("xesef-disof-gytuf-katof-movif-baxux"), Ok(b"1234567890".to_vec()));
/// assert_eq!(boba::decode("xigak-nyryk-humil-bosek-sonax"), Ok(b"Pineapple".to_vec()));
/// ```
///
/// # Errors
///
/// Decoding is fallible and might return [`DecodeError`]:
///
/// - If the input is not an ASCII string, an error is returned.
/// - If the input contains an ASCII character outside of the Bubble Babble
///   encoding alphabet, an error is returned.
/// - If the input does not start with a leading 'x', an error is returned.
/// - If the input does not end with a trailing 'x', an error is returned.
/// - If the decoded result does not checksum properly, an error is returned.
///
/// ```
/// # use boba::DecodeError;
/// assert_eq!(boba::decode("x💎🦀x"), Err(DecodeError::InvalidByte(1)));
/// assert_eq!(boba::decode("x789x"), Err(DecodeError::InvalidByte(1)));
/// assert_eq!(boba::decode("yx"), Err(DecodeError::MalformedHeader));
/// assert_eq!(boba::decode("xy"), Err(DecodeError::MalformedTrailer));
/// assert_eq!(boba::decode(""), Err(DecodeError::Corrupted));
/// assert_eq!(boba::decode("z"), Err(DecodeError::Corrupted));
/// assert_eq!(boba::decode("xx"), Err(DecodeError::Corrupted));
/// ```
pub fn decode<T: AsRef<[u8]>>(encoded: T) -> Result<Vec<u8>, DecodeError> {
    let encoded = encoded.as_ref();
    if encoded == b"xexax" {
        return Ok(Vec::new());
    }
    let enc = match encoded {
        [b'x', enc @ .., b'x'] => enc,
        [b'x', ..] => return Err(DecodeError::MalformedTrailer),
        [.., b'x'] => return Err(DecodeError::MalformedHeader),
        _ => return Err(DecodeError::Corrupted),
    };
    // This validation step ensures that the encoded bytestring only contains
    // ASCII bytes in the 24 character encoding alphabet.
    //
    // Code below must still handle None results from find_byte because bytes
    // may not be from the right subset of the alphabet, e.g. a vowel present
    // when a consonant is expected.
    if let Some(pos) = enc.find_not_byteset(ALPHABET) {
        return Err(DecodeError::InvalidByte(pos + 1));
    }
    let len = encoded.len();
    let mut decoded = Vec::with_capacity(if len == 5 { 1 } else { 2 * ((len + 1) / 6) });
    let mut checksum = 1_u8;
    let mut chunks = enc.chunks_exact(6);
    while let Some([left, mid, right, up, b'-', down]) = chunks.next() {
        let byte1 = decode_3_tuple(
            VOWELS.find_byte(*left).ok_or(DecodeError::ExpectedVowel)? as u8,
            CONSONANTS
                .find_byte(*mid)
                .ok_or(DecodeError::ExpectedConsonant)? as u8,
            VOWELS.find_byte(*right).ok_or(DecodeError::ExpectedVowel)? as u8,
            checksum,
        )?;
        let byte2 = decode_2_tuple(
            CONSONANTS
                .find_byte(*up)
                .ok_or(DecodeError::ExpectedConsonant)? as u8,
            CONSONANTS
                .find_byte(*down)
                .ok_or(DecodeError::ExpectedConsonant)? as u8,
        );
        checksum =
            ((u16::from(checksum * 5) + (u16::from(byte1) * 7) + u16::from(byte2)) % 36) as u8;
        decoded.push(byte1);
        decoded.push(byte2);
    }
    if let [left, mid, right] = chunks.remainder() {
        let a = VOWELS.find_byte(*left).ok_or(DecodeError::ExpectedVowel)? as u8;
        let c = VOWELS.find_byte(*right).ok_or(DecodeError::ExpectedVowel)? as u8;

        if *mid == b'x' {
            if a != checksum % 6 || c != checksum / 6 {
                return Err(DecodeError::ChecksumMismatch);
            }
        } else {
            let b = CONSONANTS.find_byte(*mid).unwrap() as u8;
            decoded.push(decode_3_tuple(a, b, c, checksum)?);
        }
        Ok(decoded)
    } else {
        Err(DecodeError::Corrupted)
    }
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
    // - `CONSONANTS` is a fixed size array with 17 elements.
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
    buf.push('x');
    // Panic safety:
    //
    // - `c` is constructed with divide by 6.
    // - Maximum value of `checksum` is 36 -- see `encode` loop.
    // - `VOWELS` is a fixed size array with 6 elements.
    // - Maximum value of `c` is 5.
    buf.push(VOWELS[c as usize].into());
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

#[cfg(test)]
#[allow(clippy::non_ascii_literal)]
mod tests {
    use crate::DecodeError;

    #[test]
    fn encode() {
        assert_eq!(crate::encode([]), "xexax");
        assert_eq!(
            crate::encode("1234567890"),
            "xesef-disof-gytuf-katof-movif-baxux"
        );
        assert_eq!(crate::encode("Pineapple"), "xigak-nyryk-humil-bosek-sonax");

        assert_eq!(
            crate::encode("💎🦀❤️✨💪"),
            "xusan-zugom-vesin-zenom-bumun-tanav-zyvam-zomon-sapaz-bulin-dypux"
        );
    }

    #[test]
    fn decode() {
        assert_eq!(crate::decode("xexax"), Ok(vec![]));
        assert_eq!(
            crate::decode("xesef-disof-gytuf-katof-movif-baxux"),
            Ok(b"1234567890".to_vec())
        );
        assert_eq!(
            crate::decode("xigak-nyryk-humil-bosek-sonax"),
            Ok(b"Pineapple".to_vec())
        );

        assert_eq!(
            crate::decode("xusan-zugom-vesin-zenom-bumun-tanav-zyvam-zomon-sapaz-bulin-dypux"),
            Ok("💎🦀❤️✨💪".to_string().into_bytes())
        );
    }

    #[test]
    fn decode_error_sub_dash() {
        assert_eq!(
            crate::decode("xesefxdisofxgytufxkatofxmovifxbaxux"),
            Err(DecodeError::ChecksumMismatch)
        );
    }

    #[test]
    fn decode_sub_vowel_to_consonant() {
        assert_eq!(
            crate::decode("xssef-disof-gytuf-katof-movif-baxux"),
            Err(DecodeError::ExpectedVowel),
        );
    }

    #[test]
    fn decode_sub_consonant_to_vowel() {
        assert_eq!(
            crate::decode("xeeef-disof-gytuf-katof-movif-baxux"),
            Err(DecodeError::ExpectedConsonant)
        );
    }

    #[test]
    fn decode_error() {
        assert_eq!(crate::decode(""), Err(DecodeError::Corrupted));
        assert_eq!(crate::decode("z"), Err(DecodeError::Corrupted));
        assert_eq!(crate::decode("xy"), Err(DecodeError::MalformedTrailer));
        assert_eq!(crate::decode("yx"), Err(DecodeError::MalformedHeader));
        assert_eq!(crate::decode("xx"), Err(DecodeError::Corrupted));
        assert_eq!(crate::decode("x💎🦀x"), Err(DecodeError::InvalidByte(1)));
        assert_eq!(crate::decode("x789x"), Err(DecodeError::InvalidByte(1)));
    }
}

#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![allow(clippy::cast_possible_truncation)]
#![warn(clippy::cargo)]
#![allow(unknown_lints)]
#![warn(missing_docs, broken_intra_doc_links)]
#![warn(missing_debug_implementations)]
#![warn(missing_copy_implementations)]
#![warn(rust_2018_idioms)]
#![warn(trivial_casts, trivial_numeric_casts)]
#![warn(unused_qualifications)]
#![warn(variant_size_differences)]
#![forbid(unsafe_code)]

//! This crate provides an implementation of a Bubble Babble encoder and
//! decoder.
//!
//! The Bubble Babble encoding uses alternation of consonants and vowels to
//! encode binary data to pseudowords that can be pronounced more easily than
//! arbitrary lists of hexadecimal digits.
//!
//! Bubble Babble is part of the Digest libraries in Perl and Ruby.
//!
//! # Usage
//!
//! You can encode binary data by calling [`encode`]:
//!
//! ```
//! let encoded = boba::encode("Pineapple");
//! assert_eq!(encoded, "xigak-nyryk-humil-bosek-sonax");
//! ```
//!
//! Decoding binary data is done by calling [`decode`]:
//!
//! ```
//! # use boba::DecodeError;
//! # fn example() -> Result<(), DecodeError> {
//! let decoded = boba::decode("xexax")?;
//! assert_eq!(decoded, vec![]);
//! # Ok(())
//! # }
//! # example().unwrap();
//! ```
//!
//! Decoding data is fallible and can return [`DecodeError`]. For example, all
//! Bubble Babble–encoded data has an ASCII alphabet, so attempting to decode an
//! emoji will fail.
//!
//! ```
//! # use boba::DecodeError;
//! let decoded = boba::decode("x🦀x");
//! // The `DecodeError` contains the offset of the first invalid byte.
//! assert_eq!(decoded, Err(DecodeError::InvalidByte(1)));
//! ```
//!
//! # Crate Features
//!
//! Boba is `no_std` compatible with a required dependency on the [`alloc`]
//! crate.
//!
//! Boba has several Cargo features, all of which are enabled by default:
//!
//! - **std** - Adds a dependency on [`std`], the Rust Standard Library. This
//!   feature enables [`std::error::Error`] implementations on error types in
//!   this crate. Enabling the **std** feature also enables the **alloc**
//!   feature.
//! - **alloc** - Adds a dependency on [`alloc`], the Rust allocation and
//!   collections library. Currently, Boba requires this feature to build, but
//!   may relax this requirement in the future.
//!
//! [`std`]: https://doc.rust-lang.org/stable/std/index.html
//! [`std::error::Error`]: https://doc.rust-lang.org/stable/std/error/trait.Error.html

#![no_std]
#![doc(html_root_url = "https://docs.rs/boba/4.2.0")]

// Without the `alloc` feature, build `boba` without alloc.
// This configuration is unsupported and will result in a compile error.
#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "std")]
extern crate std;

#[cfg(feature = "alloc")]
use alloc::string::String;
#[cfg(feature = "alloc")]
use alloc::vec::Vec;

// Ensure code blocks in README.md compile
#[cfg(doctest)]
macro_rules! readme {
    ($x:expr) => {
        #[doc = $x]
        mod readme {}
    };
    () => {
        readme!(include_str!("../README.md"));
    };
}
#[cfg(all(feature = "alloc", doctest))]
readme!();

use bstr::ByteSlice;
use core::fmt;

const VOWELS: [u8; 6] = *b"aeiouy";
const CONSONANTS: [u8; 16] = *b"bcdfghklmnprstvz";
const ALPHABET: [u8; 24] = *b"aeiouybcdfghklmnprstvzx-";

const HEADER: u8 = b'x';
const TRAILER: u8 = b'x';

/// Decoding errors from [`boba::decode`](decode).
///
/// `decode` will return a `DecodeError` if:
///
/// - The input is not an ASCII string, an error is returned.
/// - The input contains an ASCII character outside of the Bubble Babble
///   encoding alphabet, an error is returned.
/// - The input does not start with a leading 'x', an error is returned.
/// - The input does not end with a trailing 'x', an error is returned.
/// - The decoded result does not checksum properly, an error is returned.
///
/// # Examples
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
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
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
            Self::ChecksumMismatch => f.write_str("Checksum mismatch"),
            Self::Corrupted => f.write_str("Corrupted input"),
            Self::ExpectedConsonant => f.write_str("Expected consonant, got something else"),
            Self::ExpectedVowel => f.write_str("Expected vowel, got something else"),
            Self::InvalidByte(pos) => write!(
                f,
                "Encountered byte outside of encoding alphabet at position {}",
                pos
            ),
            Self::MalformedHeader => f.write_str("Missing required 'x' header"),
            Self::MalformedTrailer => f.write_str("Missing required 'x' trailer"),
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
        encoded.push('-');
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
    encoded.push(TRAILER.into());
    encoded
}

/// Decode Bubble Babble-encoded byte slice to a [`Vec<u8>`](Vec).
///
/// # Examples
///
/// ```
/// # use boba::DecodeError;
/// # fn example() -> Result<(), DecodeError> {
/// assert_eq!(boba::decode("xexax")?, vec![]);
/// assert_eq!(boba::decode("xesef-disof-gytuf-katof-movif-baxux")?, b"1234567890");
/// assert_eq!(boba::decode("xigak-nyryk-humil-bosek-sonax")?, b"Pineapple");
/// # Ok(())
/// # }
/// # example().unwrap();
/// ```
///
/// # Errors
///
/// Decoding is fallible and might return [`DecodeError`] if:
///
/// - The input is not an ASCII string, an error is returned.
/// - The input contains an ASCII character outside of the Bubble Babble
///   encoding alphabet, an error is returned.
/// - The input does not start with a leading 'x', an error is returned.
/// - The input does not end with a trailing 'x', an error is returned.
/// - The decoded result does not checksum properly, an error is returned.
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
    let len = encoded.len();
    let mut decoded = Vec::with_capacity(if len == 5 { 1 } else { 2 * ((len + 1) / 6) });
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
    use alloc::string::String;
    use alloc::vec;

    use crate::{decode, encode, DecodeError};

    #[test]
    fn encoder() {
        assert_eq!(encode([]), "xexax");
        assert_eq!(encode("1234567890"), "xesef-disof-gytuf-katof-movif-baxux");
        assert_eq!(encode("Pineapple"), "xigak-nyryk-humil-bosek-sonax");

        assert_eq!(
            encode("💎🦀❤️✨💪"),
            "xusan-zugom-vesin-zenom-bumun-tanav-zyvam-zomon-sapaz-bulin-dypux"
        );
    }

    #[test]
    fn decoder() {
        assert_eq!(decode("xexax"), Ok(vec![]));
        assert_eq!(
            decode("xesef-disof-gytuf-katof-movif-baxux"),
            Ok(b"1234567890".to_vec())
        );
        assert_eq!(
            decode("xigak-nyryk-humil-bosek-sonax"),
            Ok(b"Pineapple".to_vec())
        );

        assert_eq!(
            decode("xusan-zugom-vesin-zenom-bumun-tanav-zyvam-zomon-sapaz-bulin-dypux"),
            Ok(String::from("💎🦀❤️✨💪").into_bytes())
        );
    }

    #[test]
    fn decode_error_sub_dash() {
        assert_eq!(
            decode("xesefxdisofxgytufxkatofxmovifxbaxux"),
            Err(DecodeError::ChecksumMismatch)
        );
    }

    #[test]
    fn decode_sub_vowel_to_consonant() {
        assert_eq!(
            decode("xssef-disof-gytuf-katof-movif-baxux"),
            Err(DecodeError::ExpectedVowel),
        );
    }

    #[test]
    fn decode_sub_consonant_to_vowel() {
        assert_eq!(
            decode("xeeef-disof-gytuf-katof-movif-baxux"),
            Err(DecodeError::ExpectedConsonant)
        );
    }

    #[test]
    fn decode_error() {
        assert_eq!(decode(""), Err(DecodeError::Corrupted));
        assert_eq!(decode("z"), Err(DecodeError::Corrupted));
        assert_eq!(decode("xy"), Err(DecodeError::MalformedTrailer));
        assert_eq!(decode("yx"), Err(DecodeError::MalformedHeader));
        assert_eq!(decode("xx"), Err(DecodeError::Corrupted));
        assert_eq!(decode("x💎🦀x"), Err(DecodeError::InvalidByte(1)));
        assert_eq!(decode("x789x"), Err(DecodeError::InvalidByte(1)));
    }

    #[test]
    fn decode_error_bad_alphabet() {
        assert_eq!(
            decode("xigak-nyryk-/umil-bosek-sonax"),
            Err(DecodeError::InvalidByte(12))
        );
        assert_eq!(decode(b"x\xFFx"), Err(DecodeError::InvalidByte(1)));
        assert_eq!(
            decode("xigak-nyryk-Humil-bosek-sonax"),
            Err(DecodeError::InvalidByte(12))
        );
        assert_eq!(
            decode("XIGAK-NYRYK-HUMIL-BOSEK-SONAX"),
            Err(DecodeError::Corrupted)
        );
        assert_eq!(
            decode("xIGAK-NYRYK-HUMIL-BOSEK-SONAX"),
            Err(DecodeError::MalformedTrailer)
        );
        assert_eq!(
            decode("xIGAK-NYRYK-HUMIL-BOSEK-SONAx"),
            Err(DecodeError::InvalidByte(1))
        );
    }
}

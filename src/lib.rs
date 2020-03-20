#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![allow(clippy::cast_possible_truncation)]
#![deny(clippy::cargo)]
#![deny(missing_docs, intra_doc_link_resolution_failure)]
#![warn(rust_2018_idioms)]

//! # bubblebabble
//!
//! `bubblebabble` is a Rust implementation of the
//! [Bubble Babble binary data encoding](https://github.com/artichoke/bubblebabble/blob/master/spec/Bubble_Babble_Encoding.txt).
//!
//! The spec defines the following test vectors:
//!
//! ```rust
//! assert_eq!(
//!     bubblebabble::encode(&[]),
//!     String::from("xexax")
//! );
//! assert_eq!(
//!     bubblebabble::encode(&b"1234567890"[..]),
//!     String::from("xesef-disof-gytuf-katof-movif-baxux")
//! );
//! assert_eq!(
//!     bubblebabble::encode(&b"Pineapple"[..]),
//!     String::from("xigak-nyryk-humil-bosek-sonax")
//! );
//! ```
//!
//! `bubblebabble` supports decoding to a byte vector:
//!
//! ```rust
//! assert_eq!(
//!     bubblebabble::decode("xexax"),
//!     Ok(vec![])
//! );
//! assert_eq!(
//!     bubblebabble::decode("xesef-disof-gytuf-katof-movif-baxux"),
//!     Ok(Vec::from(&b"1234567890"[..]))
//! );
//! assert_eq!(
//!     bubblebabble::decode("xigak-nyryk-humil-bosek-sonax"),
//!     Ok(Vec::from(&b"Pineapple"[..]))
//! );
//! ```
//!
//! ## License
//!
//! `bubblebabble` is licensed under the
//! [MIT License](https://github.com/artichoke/bubblebabble/blob/master/LICENSE)
//! (c) Ryan Lopopolo.
//!
//! `bubblebabble` is derived from `bubble-babble-ts` @
//! [v1.0.1](https://github.com/JonathanWilbur/bubble-babble-ts/tree/v1.0.1).
//! `bubble-babble-ts` is licensed under the
//! [MIT License](https://github.com/JonathanWilbur/bubble-babble-ts/blob/v1.0.1/LICENSE.txt)
//! Copyright (c) 2018 Jonathan M. Wilbur \<jonathan@wilbur.space\>.

use bstr::ByteSlice;
use std::error;
use std::fmt;

const VOWELS: [u8; 6] = *b"aeiouy";
const CONSONANTS: [u8; 17] = *b"bcdfghklmnprstvzx";

const HEADER: u8 = b'x';
const TRAILER: u8 = b'x';

/// Decoding errors from [`bubblebabble::decode`](decode).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DecodeError {
    /// Checksum mismatch when decoding input.
    ChecksumMismatch,
    /// Corrupted input caused a decoding failure.
    Corrupted,
    /// Input contained a byte not in the encoding alphabet at this position.
    InvalidByte(usize),
    /// Input was missing a leading `x` header.
    MalformedHeader,
    /// Input was missing a final `x` trailer.
    MalformedTrailer,
}

impl error::Error for DecodeError {}

impl fmt::Display for DecodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ChecksumMismatch => write!(f, "Checksum mismatch"),
            Self::Corrupted => write!(f, "Corrupted input"),
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

/// Encode a vector of bytes with the Bubble Babble encoding to a `String`.
///
/// ```rust
/// assert_eq!(bubblebabble::encode(&[]), String::from("xexax"));
/// assert_eq!(bubblebabble::encode(&b"1234567890"[..]), String::from("xesef-disof-gytuf-katof-movif-baxux"));
/// assert_eq!(bubblebabble::encode(&b"Pineapple"[..]), String::from("xigak-nyryk-humil-bosek-sonax"));
/// ```
#[must_use]
pub fn encode<T: AsRef<[u8]>>(data: T) -> String {
    let data = data.as_ref();
    let mut encoded = Vec::with_capacity(6 * (data.len() / 2) + 3 + 2);
    encoded.push(HEADER);
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
        encoded.push(CONSONANTS[d as usize]);
        encoded.push(b'-');
        // Panic safety:
        //
        // - `e` is constructed with a mask of `0b1111`.
        // - `CONSONANTS` is a fixed size array with 17 elements.
        // - Maximum value of `e` is 15.
        encoded.push(CONSONANTS[e as usize]);
        checksum =
            ((u16::from(checksum * 5) + u16::from(*left) * 7 + u16::from(*right)) % 36) as u8;
    }
    if let [byte] = chunks.remainder() {
        odd_partial(*byte, checksum, &mut encoded);
    } else {
        even_partial(checksum, &mut encoded);
    }
    encoded.push(TRAILER);
    // Safety:
    //
    // - `encoded` is pushed to by indexing into the `VOWELS` and `CONSONANTS`
    //   arrays.
    // - `VOWELS` only contains bytes that are valid ASCII.
    // - `CONSONANTS` only contains bytes that are valid ASCII.
    unsafe { String::from_utf8_unchecked(encoded) }
}

/// Decode a string slice to a vector of bytes.
///
/// ```rust
/// assert_eq!(bubblebabble::decode("xexax"), Ok(vec![]));
/// assert_eq!(bubblebabble::decode("xesef-disof-gytuf-katof-movif-baxux"), Ok(Vec::from(&b"1234567890"[..])));
/// assert_eq!(bubblebabble::decode("xigak-nyryk-humil-bosek-sonax"), Ok(Vec::from(&b"Pineapple"[..])));
/// ```
///
/// # Errors
///
/// Decoding is fallible and might return [`DecodeError`].
///
/// ```rust
/// # use bubblebabble::DecodeError;
/// assert_eq!(bubblebabble::decode(""), Err(DecodeError::Corrupted));
/// assert_eq!(bubblebabble::decode("z"), Err(DecodeError::Corrupted));
/// assert_eq!(bubblebabble::decode("xy"), Err(DecodeError::MalformedTrailer));
/// assert_eq!(bubblebabble::decode("yx"), Err(DecodeError::MalformedHeader));
/// assert_eq!(bubblebabble::decode("xx"), Err(DecodeError::Corrupted));
/// assert_eq!(bubblebabble::decode("x💎🦀x"), Err(DecodeError::InvalidByte(1)));
/// assert_eq!(bubblebabble::decode("x999x"), Err(DecodeError::InvalidByte(1)));
/// ```
pub fn decode<T: AsRef<[u8]>>(encoded: T) -> Result<Vec<u8>, DecodeError> {
    let encoded = encoded.as_ref();
    if encoded == b"xexax" {
        return Ok(Vec::new());
    }
    let enc = match encoded {
        [b'x', enc @ ..] => {
            if let [enc @ .., b'x'] = enc {
                enc
            } else {
                return Err(DecodeError::MalformedTrailer);
            }
        }
        [.., b'x'] => return Err(DecodeError::MalformedHeader),
        _ => return Err(DecodeError::Corrupted),
    };
    if let Some(pos) = encoded.find_non_ascii_byte() {
        return Err(DecodeError::InvalidByte(pos));
    }
    let len = encoded.len();
    let mut decoded = Vec::with_capacity(if len == 5 { 1 } else { 2 * ((len + 1) / 6) });
    let mut checksum = 1_u8;
    let mut chunks = enc.chunks_exact(6);
    let mut pos = 1;
    while let Some([left, mid, right, up, _, down]) = chunks.next() {
        let byte1 = decode_3_tuple(
            VOWELS
                .find_byte(*left)
                .ok_or_else(|| DecodeError::InvalidByte(pos))? as u8,
            CONSONANTS
                .find_byte(*mid)
                .ok_or_else(|| DecodeError::InvalidByte(pos + 1))? as u8,
            VOWELS
                .find_byte(*right)
                .ok_or_else(|| DecodeError::InvalidByte(pos + 2))? as u8,
            checksum,
        )?;
        let byte2 = decode_2_tuple(
            CONSONANTS
                .find_byte(*up)
                .ok_or_else(|| DecodeError::InvalidByte(pos + 3))? as u8,
            CONSONANTS
                .find_byte(*down)
                .ok_or_else(|| DecodeError::InvalidByte(pos + 5))? as u8,
        );
        pos += 6;
        checksum =
            ((u16::from(checksum * 5) + (u16::from(byte1) * 7) + u16::from(byte2)) % 36) as u8;
        decoded.push(byte1);
        decoded.push(byte2);
    }
    if let [left, mid, right] = chunks.remainder() {
        let a = VOWELS
            .find_byte(*left)
            .ok_or_else(|| DecodeError::InvalidByte(pos))? as u8;
        let b = CONSONANTS
            .find_byte(*mid)
            .ok_or_else(|| DecodeError::InvalidByte(pos + 1))? as u8;
        let c = VOWELS
            .find_byte(*right)
            .ok_or_else(|| DecodeError::InvalidByte(pos + 2))? as u8;

        if b == 16 {
            if a != checksum % 6 || c != checksum / 6 {
                return Err(DecodeError::ChecksumMismatch);
            }
        } else {
            decoded.push(decode_3_tuple(a, b, c, checksum)?);
        }
        Ok(decoded)
    } else {
        Err(DecodeError::Corrupted)
    }
}

#[inline]
fn odd_partial(raw_byte: u8, checksum: u8, buf: &mut Vec<u8>) {
    let a = (((raw_byte >> 6) & 3) + checksum) % 6;
    let b = (raw_byte >> 2) & 15;
    let c = ((raw_byte & 3) + checksum / 6) % 6;
    // Panic safety:
    //
    // - `a` is constructed with mod 6.
    // - `VOWELS` is a fixed size array with 6 elements.
    // - Maximum value of `a` is 5.
    buf.push(VOWELS[a as usize]);
    // Panic safety:
    //
    // - `b` is constructed with a mask of `0b1111`.
    // - `CONSONANTS` is a fixed size array with 17 elements.
    // - Maximum value of `e` is 15.
    buf.push(CONSONANTS[b as usize]);
    // Panic safety:
    //
    // - `c` is constructed with mod 6.
    // - `VOWELS` is a fixed size array with 6 elements.
    // - Maximum value of `c` is 5.
    buf.push(VOWELS[c as usize]);
}

#[inline]
fn even_partial(checksum: u8, buf: &mut Vec<u8>) {
    let a = checksum % 6;
    // let b = 16;
    let c = checksum / 6;
    // Panic safety:
    //
    // - `a` is constructed with mod 6.
    // - `VOWELS` is a fixed size array with 6 elements.
    // - Maximum value of `a` is 5.
    buf.push(VOWELS[a as usize]);
    buf.push(b'x');
    // Panic safety:
    //
    // - `c` is constructed with divide by 6.
    // - Maximum value of `checksum` is 36 -- see `encode` loop.
    // - `VOWELS` is a fixed size array with 6 elements.
    // - Maximum value of `c` is 5.
    buf.push(VOWELS[c as usize]);
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
    #[test]
    fn encode() {
        assert_eq!(super::encode(&[]), "xexax");
        assert_eq!(
            super::encode(&b"1234567890"[..]),
            "xesef-disof-gytuf-katof-movif-baxux"
        );
        assert_eq!(
            super::encode(&b"Pineapple"[..]),
            "xigak-nyryk-humil-bosek-sonax"
        );
    }

    #[test]
    fn decode() {
        assert_eq!(super::decode("xexax"), Ok(vec![]));
        assert_eq!(
            super::decode("xesef-disof-gytuf-katof-movif-baxux"),
            Ok(String::from("1234567890").into_bytes())
        );
        assert_eq!(
            super::decode("xigak-nyryk-humil-bosek-sonax"),
            Ok(String::from("Pineapple").into_bytes())
        );
    }

    #[test]
    fn decode_error() {
        assert_eq!(super::decode(""), Err(super::DecodeError::Corrupted));
        assert_eq!(super::decode("z"), Err(super::DecodeError::Corrupted));
        assert_eq!(
            super::decode("xy"),
            Err(super::DecodeError::MalformedTrailer)
        );
        assert_eq!(
            super::decode("yx"),
            Err(super::DecodeError::MalformedHeader)
        );
        assert_eq!(super::decode("xx"), Err(super::DecodeError::Corrupted));
        assert_eq!(
            super::decode("x💎🦀x"),
            Err(super::DecodeError::InvalidByte(1))
        );
        assert_eq!(
            super::decode("x999x"),
            Err(super::DecodeError::InvalidByte(1))
        );
    }
}

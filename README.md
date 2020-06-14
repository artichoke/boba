# boba

[![GitHub Actions](https://github.com/artichoke/boba/workflows/CI/badge.svg)](https://github.com/artichoke/boba/actions)
[![Discord](https://img.shields.io/discord/607683947496734760)](https://discord.gg/QCe2tp2)
[![Twitter](https://img.shields.io/twitter/follow/artichokeruby?label=Follow&style=social)](https://twitter.com/artichokeruby)
<br>
[![Crate](https://img.shields.io/crates/v/boba.svg)](https://crates.io/crates/boba)
[![API](https://docs.rs/boba/badge.svg)](https://docs.rs/boba)
[![API master](https://img.shields.io/badge/docs-master-blue.svg)](https://artichoke.github.io/boba/boba/)

Implements the the
[Bubble Babble binary data encoding](/spec/Bubble_Babble_Encoding.txt).

> The Bubble Babble Encoding encodes arbitrary binary data into pseudowords that
> are more natural to humans and that can be pronounced relatively easily.

Bubble Babble encodes 6 characters in 16 bits and includes a checksum embedded
in the encoded data. See the
[Bubble Babble spec](spec/Bubble_Babble_Encoding.txt).

This crate depends on [bstr](https://crates.io/crates/bstr).

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
boba = "4"
```

Then encode and decode data like:

```rust
assert_eq!(boba::encode("Pineapple"), "xigak-nyryk-humil-bosek-sonax");
assert_eq!(boba::decode(b"xexax"), Ok(vec![]));
```

## Crate Features

`boba` has a `std` feature which is enabled by default that adds `Vec` and
`String` support as well as `std::error::Error` impls. `boba` does not compile
if this feature is disabled, but exists so this crate can add `no_std` support
backwards compatibly.

`boba` is [fuzzed](fuzz/fuzz_targets) with
[cargo-fuzz](https://crates.io/crates/cargo-fuzz).

## Minimum Rust Version Policy

This crate's minimum supported `rustc` version (MSRV) is `1.42.0`.

MSRV may be bumped in minor version releases.

## License

`boba` is licensed under the [MIT License](LICENSE) (c) Ryan Lopopolo.

`boba` is derived from `bubble-babble-ts` @
[v1.0.1](https://github.com/JonathanWilbur/bubble-babble-ts/tree/v1.0.1).
`bubble-babble-ts` is licensed under the
[MIT License](https://github.com/JonathanWilbur/bubble-babble-ts/blob/v1.0.1/LICENSE.txt)
Copyright (c) 2018 Jonathan M. Wilbur \<jonathan@wilbur.space\>.

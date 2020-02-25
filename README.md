# bubblebabble

[![GitHub Actions](https://github.com/artichoke/bubblebabble/workflows/CI/badge.svg)](https://github.com/artichoke/bubblebabble/actions)
[![Discord](https://img.shields.io/discord/607683947496734760)](https://discord.gg/QCe2tp2)
[![Twitter](https://img.shields.io/twitter/follow/artichokeruby?label=Follow&style=social)](https://twitter.com/artichokeruby)
<br>
[![Documentation](https://img.shields.io/badge/docs-bubblebabble-blue.svg)](https://artichoke.github.io/bubblebabble/bubblebabble)

`bubblebabble` is a Rust implementation of the
[Bubble Babble binary data encoding](https://github.com/artichoke/bubblebabble/blob/master/spec/Bubble_Babble_Encoding.txt).

The spec defines the following test vectors:

```rust
assert_eq!(
    bubblebabble::encode(&[]),
    String::from("xexax")
);
assert_eq!(
    bubblebabble::encode(&b"1234567890"[..]),
    String::from("xesef-disof-gytuf-katof-movif-baxux")
);
assert_eq!(
    bubblebabble::encode(&b"Pineapple"[..]),
    String::from("xigak-nyryk-humil-bosek-sonax")
);
```

`bubblebabble` supports decoding to a byte vector:

```rust
assert_eq!(
    bubblebabble::decode("xexax"),
    Ok(vec![])
);
assert_eq!(
    bubblebabble::decode("xesef-disof-gytuf-katof-movif-baxux"),
    Ok(Vec::from(&b"1234567890"[..]))
);
assert_eq!(
    bubblebabble::decode("xigak-nyryk-humil-bosek-sonax"),
    Ok(Vec::from(&b"Pineapple"[..]))
);
```

## License

`bubblebabble` is licensed under the [MIT License](/LICENSE) (c) Ryan Lopopolo.

`bubblebabble` is derived from `bubble-babble-ts` @
[v1.0.1](https://github.com/JonathanWilbur/bubble-babble-ts/tree/v1.0.1).
`bubble-babble-ts` is licensed under the
[MIT License](https://github.com/JonathanWilbur/bubble-babble-ts/blob/v1.0.1/LICENSE.txt)
Copyright (c) 2018 Jonathan M. Wilbur \<jonathan@wilbur.space\>.

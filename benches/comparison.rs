#![feature(test)]

extern crate test;

mod encode {
    #[bench]
    fn benchmark_encode_empty(b: &mut ::test::Bencher) {
        b.iter(|| bubblebabble::bubblebabble(&[]))
    }

    #[bench]
    fn benchmark_encode_vector_1234567890(b: &mut ::test::Bencher) {
        b.iter(|| bubblebabble::bubblebabble(b"1234567890"))
    }

    #[bench]
    fn benchmark_encode_vector_pineapple(b: &mut ::test::Bencher) {
        b.iter(|| bubblebabble::bubblebabble(b"Pineapple"))
    }

    #[bench]
    fn benchmark_encode_emoji(b: &mut ::test::Bencher) {
        b.iter(|| bubblebabble::bubblebabble("💎🦀❤️✨💪".as_bytes()))
    }
}

#![feature(test)]
extern crate test;

mod encode {
    #[bench]
    fn benchmark_encode_empty(b: &mut ::test::Bencher) {
        b.iter(|| bubblebabble::encode([]))
    }

    #[bench]
    fn benchmark_encode_vector_1234567890(b: &mut ::test::Bencher) {
        b.iter(|| bubblebabble::encode("1234567890"))
    }

    #[bench]
    fn benchmark_encode_vector_pineapple(b: &mut ::test::Bencher) {
        b.iter(|| bubblebabble::encode("Pineapple"))
    }
}

mod decode {
    #[bench]
    fn benchmark_decode_empty(b: &mut ::test::Bencher) {
        b.iter(|| bubblebabble::decode("xexax"))
    }

    #[bench]
    fn benchmark_decode_vector_1234567890(b: &mut ::test::Bencher) {
        b.iter(|| bubblebabble::decode("xesef-disof-gytuf-katof-movif-baxux"))
    }

    #[bench]
    fn benchmark_decode_vector_pineapple(b: &mut ::test::Bencher) {
        b.iter(|| bubblebabble::decode("xigak-nyryk-humil-bosek-sonax"))
    }
}

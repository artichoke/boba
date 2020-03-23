#![feature(test)]

extern crate test;

mod encode {
    #[bench]
    fn benchmark_encode_empty(b: &mut ::test::Bencher) {
        b.iter(|| boba::encode([]))
    }

    #[bench]
    fn benchmark_encode_vector_1234567890(b: &mut ::test::Bencher) {
        b.iter(|| boba::encode("1234567890"))
    }

    #[bench]
    fn benchmark_encode_vector_pineapple(b: &mut ::test::Bencher) {
        b.iter(|| boba::encode("Pineapple"))
    }

    #[bench]
    fn benchmark_encode_emoji(b: &mut ::test::Bencher) {
        b.iter(|| boba::encode("💎🦀❤️✨💪"))
    }
}

mod decode {
    #[bench]
    fn benchmark_decode_empty(b: &mut ::test::Bencher) {
        b.iter(|| boba::decode("xexax"))
    }

    #[bench]
    fn benchmark_decode_vector_1234567890(b: &mut ::test::Bencher) {
        b.iter(|| boba::decode("xesef-disof-gytuf-katof-movif-baxux"))
    }

    #[bench]
    fn benchmark_decode_vector_pineapple(b: &mut ::test::Bencher) {
        b.iter(|| boba::decode("xigak-nyryk-humil-bosek-sonax"))
    }

    #[bench]
    fn benchmark_decode_emoji(b: &mut ::test::Bencher) {
        b.iter(|| boba::decode("xusan-zugom-vesin-zenom-bumun-tanav-zyvam-zomon-sapaz-bulin-dypux"))
    }
}

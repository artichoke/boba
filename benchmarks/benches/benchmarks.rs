use criterion::{criterion_group, criterion_main};

mod encode {
    use criterion::Criterion;

    pub fn bench(c: &mut Criterion) {
        let mut group = c.benchmark_group("boba::encode");
        group.bench_function("empty", |b| b.iter(|| boba::encode([])));
        group.bench_function("1234567890", |b| b.iter(|| boba::encode("1234567890")));
        group.bench_function("Pineapple", |b| b.iter(|| boba::encode("Pineapple")));
        group.bench_function("emoji", |b| b.iter(|| boba::encode("💎🦀❤️✨💪")));
    }
}

mod decode {
    use criterion::Criterion;

    pub fn bench(c: &mut Criterion) {
        let mut group = c.benchmark_group("boba::decode");
        group.bench_function("empty", |b| b.iter(|| boba::decode("xexax")));
        group.bench_function("1234567890", |b| {
            b.iter(|| boba::decode("xesef-disof-gytuf-katof-movif-baxux"))
        });
        group.bench_function("Pineapple", |b| {
            b.iter(|| boba::decode("xigak-nyryk-humil-bosek-sonax"))
        });
        group.bench_function("emoji", |b| {
            b.iter(|| {
                boba::decode("xusan-zugom-vesin-zenom-bumun-tanav-zyvam-zomon-sapaz-bulin-dypux")
            })
        });
    }
}

criterion_group!(encode, encode::bench);
criterion_group!(decode, decode::bench);
criterion_main!(encode, decode);

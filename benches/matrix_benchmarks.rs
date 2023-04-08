use criterion::{black_box, criterion_group, criterion_main, Criterion};
use matrix_rs::rain::{
    digital_rain, draw, rain_drop,
    rain_options::{DigitalRainOptions, DigitalRainOptionsBuilder},
};
use rand;
use std::time::Duration;

fn get_sane_options() -> DigitalRainOptions {
    DigitalRainOptionsBuilder::new((100, 100))
        .drops_range((20, 30))
        .speed_range((10, 20))
        .build()
}

fn run_loop_benchmark(_c: &mut Criterion) {
    let mut cc = Criterion::default()
        .warm_up_time(std::time::Duration::from_secs(3)) // 3 seconds warm-up time
        .measurement_time(std::time::Duration::from_secs(10)) // 10 seconds measurement time
        .sample_size(100);

    cc.bench_function("benchmark_run_loop", |b| {
        b.iter(|| {
            let mut stdout = Vec::new();
            let _ = draw::run_loop(black_box(&mut stdout), Some(3));
        })
    });
}

fn vertical_worm_benchmark(c: &mut Criterion) {
    let options = get_sane_options();
    c.bench_function("benchmark_worm_new_1000", |b| {
        b.iter(|| {
            let mut rng = rand::thread_rng();
            for index in 1..=1000 {
                rain_drop::RainDrop::new(&options, index, &mut rng);
            }
        })
    });

    c.bench_function("benchmark_worm_update_1000", |b| {
        let mut rng = rand::thread_rng();
        let options = get_sane_options();
        let mut drops: Vec<rain_drop::RainDrop> = vec![];
        for index in 1..=1000 {
            drops.push(rain_drop::RainDrop::new(&options, index, &mut rng));
        }
        b.iter(|| {
            for drop in drops.iter_mut() {
                drop.update(&options, Duration::from_millis(50), &mut rng);
            }
        })
    });
}

fn digital_rain_benchmark(c: &mut Criterion) {
    c.bench_function("benchmark_rain_new", |b| {
        b.iter(|| {
            let options = get_sane_options();
            let _ = digital_rain::DigitalRain::new(options);
        })
    });

    c.bench_function("benchmark_rain_update", |b| {
        b.iter(|| {
            let options = get_sane_options();
            let mut rain = digital_rain::DigitalRain::new(options);
            for _ in 1..=10 {
                rain.update();
            }
        })
    });
}

criterion_group!(
    benches,
    run_loop_benchmark,
    vertical_worm_benchmark,
    digital_rain_benchmark
);
criterion_main!(benches);

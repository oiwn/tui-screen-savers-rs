use criterion::{black_box, criterion_group, criterion_main, Criterion};
use matrix_rs::rain::{digital_rain, draw, rain_drop};
use rand;
use std::time::Duration;

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
    c.bench_function("benchmark_worm_new_1000", |b| {
        b.iter(|| {
            let mut rng = rand::thread_rng();
            for index in 1..=1000 {
                rain_drop::RainDrop::new(100, 100, index, &mut rng);
            }
        })
    });

    c.bench_function("benchmark_worm_update_1000", |b| {
        let mut rng = rand::thread_rng();
        let mut worms: Vec<rain_drop::RainDrop> = vec![];
        for index in 1..=1000 {
            worms.push(rain_drop::RainDrop::new(100, 100, index, &mut rng));
        }
        b.iter(|| {
            for worm in worms.iter_mut() {
                worm.update(100, 100, Duration::from_millis(50), &mut rng);
            }
        })
    });
}

fn digital_rain_benchmark(c: &mut Criterion) {
    c.bench_function("benchmark_rain_new_200", |b| {
        b.iter(|| {
            let _ = digital_rain::DigitalRain::new(100, 100, 200);
        })
    });

    c.bench_function("benchmark_rain_update_10", |b| {
        b.iter(|| {
            let mut rain = digital_rain::DigitalRain::new(100, 100, 200);
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

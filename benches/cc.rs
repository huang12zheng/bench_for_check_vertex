// Gnuplot not found, using plotters backend
// Benchmarking is_vertex: Warming up for 3.0000 s
// Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 38.7s, or reduce sample count to 10.
// Benchmarking is_vertex: Collecting 100 samples in estimated 38.659 s (100 iterati
// is_vertex               time:   [386.84 ms 387.07 ms 387.33 ms]
// Found 9 outliers among 100 measurements (9.00%)
//   8 (8.00%) high mild
//   1 (1.00%) high severe

// Benchmarking get_edge_direction: Warming up for 3.0000 s
// Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 38.7s, or reduce sample count to 10.
// Benchmarking get_edge_direction: Collecting 100 samples in estimated 38.659 s (10
// get_edge_direction      time:   [387.66 ms 388.92 ms 390.73 ms]
// Found 7 outliers among 100 measurements (7.00%)
//   4 (4.00%) high mild
//   3 (3.00%) high severe

use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn is_vertex(graph_info: u64) -> bool {
    (graph_info >> 63) == 1
}

pub const IN_EDGE_MASK: u64 = 1 << 62;

pub fn get_edge_direction(graph_info: u64) -> u64 {
    graph_info & IN_EDGE_MASK
}

fn bench_is_vertex(c: &mut Criterion) {
    c.bench_function("is_vertex", |b| {
        b.iter(|| {
            for i in 0..1000000000 {
                is_vertex(black_box(i));
            }
        })
    });
}

fn bench_get_edge_direction(c: &mut Criterion) {
    c.bench_function("get_edge_direction", |b| {
        b.iter(|| {
            for i in 0..1000000000 {
                get_edge_direction(black_box(i));
            }
        })
    });
}

criterion_group!(benches, bench_is_vertex, bench_get_edge_direction);
criterion_main!(benches);

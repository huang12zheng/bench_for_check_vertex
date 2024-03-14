// enchmarking is_vertex: Collecting 100 samples in estimated 77.606 s (100 iterati
//     is_vertex               time:   [776.87 ms 778.05 ms 779.50 ms]
//                             change: [+9894.0% +9913.7% +9935.2%] (p = 0.00 < 0.05)
//                             Performance has regressed.
//     Found 11 outliers among 100 measurements (11.00%)
//       4 (4.00%) high mild
//       7 (7.00%) high severe

//     Benchmarking get_edge_direction: Warming up for 3.0000 s
//     Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 77.6s, or reduce sample count to 10.
//     Benchmarking get_edge_direction: Collecting 100 samples in estimated 77.558 s (10
//     get_edge_direction      time:   [777.18 ms 778.42 ms 780.01 ms]
//                             change: [+9648.2% +9763.3% +9857.6%] (p = 0.00 < 0.05)
//                             Performance has regressed.
//     Found 6 outliers among 100 measurements (6.00%)
//       3 (3.00%) high mild
//       3 (3.00%) high severe

//          Running benches/cc.rs (target/release/deps/cc-5065d0e3466db582)

//     running 0 tests

//     test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

// Gnuplot not found, using plotters backend
// Benchmarking is_vertex: Collecting 100 samples in estimated 5.4458 s (700 iterati
// is_vertex               time:   [7.7613 ms 7.7699 ms 7.7797 ms]
//                         change: [-98.019% -98.011% -98.004%] (p = 0.00 < 0.05)
//                         Performance has improved.
// Found 16 outliers among 100 measurements (16.00%)
//   6 (6.00%) high mild
//   10 (10.00%) high severe

// Benchmarking get_edge_direction: Collecting 100 samples in estimated 5.5665 s (70
// get_edge_direction      time:   [7.8202 ms 7.8921 ms 7.9845 ms]
//                         change: [-97.998% -97.979% -97.956%] (p = 0.00 < 0.05)
//                         Performance has improved.
// Found 7 outliers among 100 measurements (7.00%)
//   1 (1.00%) high mild
//   6 (6.00%) high severe

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
                let _ = is_vertex(black_box(i)).cmp(&false);
            }
            for i in 1 << 62..(1 << 62) + 1000000000 {
                let _ = is_vertex(black_box(i)).cmp(&false);
            }
        })
    });
}

fn bench_get_edge_direction(c: &mut Criterion) {
    c.bench_function("get_edge_direction", |b| {
        b.iter(|| {
            for i in 0..1000000000 {
                let _ = get_edge_direction(black_box(i)).cmp(&0);
            }
            for i in 1 << 62..(1 << 62) + 1000000000 {
                let _ = get_edge_direction(black_box(i)).cmp(&0);
            }
        })
    });
}

criterion_group!(benches, bench_is_vertex, bench_get_edge_direction);
criterion_main!(benches);

// use criterion::{black_box, criterion_group, criterion_main, Criterion};

// pub fn is_vertex(graph_info: u64) -> bool {
//     (graph_info >> 63) == 1
// }

// pub const IN_EDGE_MASK: u64 = 1 << 62;

// pub fn get_edge_direction(graph_info: u64) -> u64 {
//     graph_info & IN_EDGE_MASK
// }

// fn bench_is_vertex(c: &mut Criterion) {
//     c.bench_function("is_vertex", |b| {
//         b.iter(|| is_vertex(black_box(0xFFFFFFFFFFFFFFFF)))
//     });
// }

// fn bench_get_edge_direction(c: &mut Criterion) {
//     c.bench_function("get_edge_direction", |b| {
//         b.iter(|| get_edge_direction(black_box(0xFFFFFFFFFFFFFFFF)))
//     });
// }

// criterion_group!(benches, bench_is_vertex, bench_get_edge_direction);
// criterion_main!(benches);

// // fn main() {
// //     let (a, b) = (true, false);
// //     println!("{:?}", b.cmp(&a));
// // }

// // pub fn is_vertex(graph_info: u64) -> bool {
// //     (graph_info >> 63) == 1
// // }
// // pub const IN_EDGE_MASK: u64 = 1 << 62;
// // pub fn get_edge_direction(graph_info: u64) -> u64 {
// //     graph_info & IN_EDGE_MASK
// // }

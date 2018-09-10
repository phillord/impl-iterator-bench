#[macro_use]
extern crate bencher;
extern crate impl_iterator_bench;
use impl_iterator_bench::*;

use std::collections::HashSet;
use std::iter::FromIterator;
use bencher::Bencher;

fn big_hs<'a>() -> HashSet<i64> {
    HashSet::from_iter(1..1000)
}

fn iter_flatmap_call(bench:&mut Bencher){
    let hs = HashSet::new();
    let ohs:Option<&HashSet<i64>> = Some(&hs);
    bench.iter(|| {
        get_iter_flat_map(&ohs);
    })
}

fn iter_flatmap_call_and_use(bench:&mut Bencher){
    let big_hs = big_hs();
    let ohs = Some(&big_hs);
    bench.iter(|| {
        for i in get_iter_flat_map(&ohs) {
            let _ = i+1;
        }
    })
}

fn iter_either_call(bench:&mut Bencher){
    let hs = HashSet::new();
    let ohs:Option<&HashSet<i64>> = Some(&hs);
    bench.iter(|| {
        get_iter_either(&ohs);
    })
}

fn iter_either_call_and_use(bench:&mut Bencher){
    let big_hs = big_hs();
    let ohs = Some(&big_hs);
    bench.iter(|| {
        for i in get_iter_either(&ohs) {
            let _ = i+1;
        }
    })
}

fn iter_enum_call(bench:&mut Bencher){
    let hs = HashSet::new();
    let ohs:Option<&HashSet<i64>> = Some(&hs);
    bench.iter(|| {
        get_iter_enum(&ohs);
    })
}

fn iter_enum_call_and_use(bench:&mut Bencher){
    let big_hs = big_hs();
    let ohs = Some(&big_hs);
    bench.iter(|| {
        for i in get_iter_enum(&ohs) {
            let _ = i+1;
        }
    })
}

fn iter_dyn_call(bench:&mut Bencher){
    let hs = HashSet::new();
    let ohs:Option<&HashSet<i64>> = Some(&hs);
    bench.iter(|| {
        get_iter_dyn(&ohs);
    })
}

fn iter_dyn_call_and_use(bench:&mut Bencher){
    let big_hs = big_hs();
    let ohs = Some(&big_hs);
    bench.iter(|| {
        for i in get_iter_dyn(&ohs) {
            let _ = i + 1;
        }
    })
}



benchmark_group!(benches,
                 iter_either_call,iter_either_call_and_use,
                 iter_flatmap_call,iter_flatmap_call_and_use,
                 iter_enum_call,iter_enum_call_and_use,
                 iter_dyn_call,iter_dyn_call_and_use,


);
benchmark_main!(benches);

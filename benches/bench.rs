#[macro_use]
extern crate criterion;
extern crate rand;
extern crate rgep;

use std::iter;
use std::iter::*;

use criterion::Criterion;

use rand::prelude::*;

use rgep::*;


fn bench_crossover(c: &mut Criterion) {
    c.bench_function("naive crossover", |b| b.iter(|| {
        let n = 10000;
        let ind1 = Ind(iter::repeat(0x0).take(n).collect());
        let ind2 = Ind(iter::repeat(0xF).take(n).collect());

        let pair = &mut [ind1, ind2];

        let cross_points = [1, n / 4, n / 2, 3 * (n / 4)];

        cross_at_points_naive(pair, 4, &cross_points);
    }));

    c.bench_function("crossover", |b| b.iter(|| {
        let n = 10000;
        let ind1 = Ind(iter::repeat(0x0).take(n).collect());
        let ind2 = Ind(iter::repeat(0xF).take(n).collect());

        let pair = &mut [ind1, ind2];

        let cross_points = [1, n / 4, n / 2, 3 * (n / 4)];

        cross_at_points(pair, 4, &cross_points);
    }));
}

fn bench_rotation_offsets(c: &mut Criterion) {
    c.bench_function("rotation_one", |b| b.iter(|| {
        let n = 10000;
        let mut ind = Ind(iter::repeat(0x0).take(n).collect());
        rotate_naive(&mut ind, 1);
    }));

    c.bench_function("rotation_half", |b| b.iter(|| {
        let n = 10000;
        let mut ind = Ind(iter::repeat(0x0).take(n).collect());
        rotate_naive(&mut ind, n/2);
    }));

    c.bench_function("rotation_small", |b| b.iter(|| {
        let n = 10000;
        let mut ind = Ind(iter::repeat(0x0).take(n).collect());
        rotate_naive(&mut ind, n/128);
    }));
}

fn bench_rotation_sizes(c: &mut Criterion) {
    c.bench_function("rotation_100", |b| b.iter(|| {
        let n = 100;
        let mut ind = Ind(iter::repeat(0x0).take(n).collect());
        rotate_naive(&mut ind, n/2);
    }));

    c.bench_function("rotation_1000", |b| b.iter(|| {
        let n = 1000;
        let mut ind = Ind(iter::repeat(0x0).take(n).collect());
        rotate_naive(&mut ind, n/2);
    }));

    c.bench_function("rotation_10000", |b| b.iter(|| {
        let n = 10000;
        let mut ind = Ind(iter::repeat(0x0).take(n).collect());
        rotate_naive(&mut ind, n/2);
    }));
}

fn bench_rotation_types(c: &mut Criterion) {
    c.bench_function("rotation", |b| b.iter(|| {
        let n = 10000;
        let mut ind = Ind(iter::repeat(0x0).take(n).collect());
        rotate_naive(&mut ind, 1);
    }));

    c.bench_function("rotation_copy", |b| b.iter(|| {
        let n = 10000;
        let mut ind = Ind(iter::repeat(0x0).take(n).collect());
        let mut scratch = Vec::with_capacity(n);
        rotate_copy(&mut ind, &mut scratch, 1);
    }));
}

fn bench_point_mutation(c: &mut Criterion) {
    c.bench_function("naive_point_mutation", |b| b.iter(|| {
        let n = 10000;
        let mut ind = Ind(iter::repeat(0x0).take(n).collect());
        let pm = 0.01;
        let mut rng = thread_rng();
        point_mutate_naive(&mut ind, 4, pm, &mut rng);
    }));

    c.bench_function("point_mutation", |b| b.iter(|| {
        let n = 10000;
        let mut ind = Ind(iter::repeat(0x0).take(n).collect());
        let pm = 0.01;
        let mut rng = thread_rng();
        point_mutate(&mut ind, 4, pm, &mut rng);
    }));
}

fn bench_point_mutation_geometric(c: &mut Criterion) {
    c.bench_function("point_mutation_0.1", |b| b.iter(|| {
        let n = 10000;
        let mut ind = Ind(iter::repeat(0x0).take(n).collect());
        let pm = 0.1;
        let mut rng = thread_rng();
        point_mutate(&mut ind, 4, pm, &mut rng);
    }));

    c.bench_function("point_mutation_0.01", |b| b.iter(|| {
        let n = 10000;
        let mut ind = Ind(iter::repeat(0x0).take(n).collect());
        let pm = 0.01;
        let mut rng = thread_rng();
        point_mutate(&mut ind, 4, pm, &mut rng);
    }));

    c.bench_function("point_mutation_0.001", |b| b.iter(|| {
        let n = 10000;
        let mut ind = Ind(iter::repeat(0x0).take(n).collect());
        let pm = 0.001;
        let mut rng = thread_rng();
        point_mutate(&mut ind, 4, pm, &mut rng);
    }));
}

fn bench_rgep_operators(c: &mut Criterion) {
    c.bench_function("point_mutation_operator", |b| b.iter(|| {
        let n = 10000;
        let mut ind = Ind(iter::repeat(0x0).take(n).collect());
        let pm = 0.01;
        let mut rng = thread_rng();
        point_mutate(&mut ind, 4, pm, &mut rng);
    }));

    c.bench_function("rotation_operator", |b| b.iter(|| {
        let n = 10000;
        let mut ind = Ind(iter::repeat(0x0).take(n).collect());
        let mut scratch = Vec::new();
        rotate_copy(&mut ind, &mut scratch, n/2);
    }));

    c.bench_function("crossover_one_point_operator", |b| b.iter(|| {
        let n = 10000;
        let ind1 = Ind(iter::repeat(0x0).take(n).collect());
        let ind2 = Ind(iter::repeat(0xF).take(n).collect());

        let pair = &mut [ind1, ind2];

        let cross_points = [n / 2];

        cross_at_points(pair, 4, &cross_points);
    }));

    c.bench_function("crossover_two_point_operator", |b| b.iter(|| {
        let n = 10000;
        let ind1 = Ind(iter::repeat(0x0).take(n).collect());
        let ind2 = Ind(iter::repeat(0xF).take(n).collect());

        let pair = &mut [ind1, ind2];

        let cross_points = [n / 4, 3 * (n / 4)];

        cross_at_points(pair, 4, &cross_points);
    }));

    c.bench_function("select_stochastic_universal", |b| b.iter(|| {
        let ind_len = 100;
        let n = 1000;
        let ind = Ind(iter::repeat(0x0).take(ind_len).collect());
        let pop = Pop(iter::repeat(ind).take(n).collect());

        let alt_ind = Ind(iter::repeat(0x0).take(ind_len).collect());
        let mut alt_pop = Pop(iter::repeat(alt_ind).take(n).collect());

        let fitnesses = (0..n).map(|f| f as f64).collect();

        select_stochastic_universal(&pop, &mut alt_pop, fitnesses, 1, 0.5);
    }));
}

fn bench_selection(c: &mut Criterion) {
    let ind_len = 1000;
    let n = 1000;

    let ind = Ind(iter::repeat(0x0).take(ind_len).collect());
    let pop = Pop(iter::repeat(ind).take(n).collect());

    c.bench_function("select_stochastic_universal_naive", move |b| b.iter(|| {
        let fitnesses = (0..n).map(|f| f as f64).collect();
        select_stochastic_universal_naive(&pop, fitnesses, 1, 0.5);
    }));

    let ind = Ind(iter::repeat(0x0).take(ind_len).collect());
    let pop = Pop(iter::repeat(ind).take(n).collect());
    let empty_ind = Ind(Vec::with_capacity(n));
    let mut new_pop = Pop(iter::repeat(empty_ind).take(n).collect());

    c.bench_function("select_stochastic_universal", move |b| b.iter(|| {
        let fitnesses = (0..n).map(|f| f as f64).collect();
        select_stochastic_universal(&pop, &mut new_pop, fitnesses, 1, 0.5);
    }));
}

criterion_group!(point_mutation, bench_point_mutation, bench_point_mutation_geometric);
criterion_group!(crossover, bench_crossover);
criterion_group!(rotation, bench_rotation_offsets, bench_rotation_sizes, bench_rotation_types);
criterion_group!(rgep, bench_rgep_operators);
criterion_group!(selection, bench_selection);
criterion_main!(selection, rgep, crossover, rotation, point_mutation);

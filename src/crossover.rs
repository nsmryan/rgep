use rand::prelude::*;
use rand::distributions::Distribution;

use statrs::distribution::Uniform;

use im::vector::Vector;

use types::*;


pub fn crossover_one_point<R: Rng>(pop: &mut Pop, words_per_ind: usize, bits_per_sym: usize, pc1: f64, rng: &mut R) {
    let pc1_sampler = Uniform::new(0.0, 1.0).unwrap();
    let cross_point_sampler = Uniform::new(0.0, (words_per_ind * bits_per_sym) as f64).unwrap();

    for mut pair in pop.0.chunks_mut(2) {
        if pair.len() != 2 {
            break;
        }

        if pc1_sampler.sample(rng) < pc1 {
            let cross_point = cross_point_sampler.sample(rng) as usize;

            cross_at_point(&mut pair, bits_per_sym, cross_point);
        }
    }
}

pub fn cross_at_point(pair: &mut [Ind<u8>], bits_per_sym: usize, cross_point: usize) {
    let cross_word_index = cross_point / bits_per_sym;
    for word_index in 0..cross_word_index {
        let tmp = pair[0].0[word_index];
        pair[0].0[word_index] = pair[1].0[word_index];
        pair[1].0[word_index] = tmp;
    }
    // cross the word that the cross point is within
    let l1 = pair[0].0[cross_word_index];
    let l2 = pair[1].0[cross_word_index];
    let bit_mask = (2_u32.pow((cross_point % bits_per_sym) as u32) - 1) as u8;
    pair[0].0[cross_word_index] = (l1 & bit_mask) | (l2 & !bit_mask);
    pair[1].0[cross_word_index] = (l2 & bit_mask) | (l1 & !bit_mask);
}

#[test]
fn test_cross_at_point() {
    let ind1 = Ind(vec!(0xF, 0xF, 0xF, 0xF, 0xF));
    let ind2 = Ind(vec!(0x0, 0x0, 0x0, 0x0, 0x0));
    let mut pair = [ind1, ind2];

    cross_at_point(&mut pair, 4, 10);
    assert!(pair[0] == Ind(vec!(0, 0, 3, 0xF, 0xF)));
    assert!(pair[1] == Ind(vec!(0xF, 0xF, 0xC, 0x0, 0x0)));
}

pub fn crossover_two_point<R: Rng>(pop: &mut Pop, words_per_ind: usize, bits_per_sym: usize, pc2: f64, rng: &mut R) {
    let pc2_sampler = Uniform::new(0.0, 1.0).unwrap();
    let cross_point_sampler = Uniform::new(0.0, (words_per_ind * bits_per_sym) as f64).unwrap();

    for mut pair in pop.0.chunks_mut(2) {
        if pair.len() != 2 {
            break;
        }

        if pc2_sampler.sample(rng) < pc2 {
            let cross_point_one = cross_point_sampler.sample(rng) as usize;
            let cross_point_two = cross_point_sampler.sample(rng) as usize;

            let mut locs = [cross_point_one, cross_point_two];
            locs.sort();
            cross_at_points(&mut pair, bits_per_sym, &locs);
        }
    }
}

pub fn cross_at_points_im<T>(pair: (Vector<T>, Vector<T>), bits_per_sym: usize, cross_points: &[usize]) -> (Vector<T>, Vector<T>) 
    where T: Clone {
    let (mut left, mut right) = (Vector::new(), Vector::new());

    let (mut first, mut second) = pair;

    let mut running_sum = 0;

    for cross_point in cross_points {
        //println!("point = {}, len = {}, sum = {}", cross_point, first.len(), running_sum);

        let (head_first, tail_first) = first.split_at(*cross_point - running_sum);
        let (head_second, tail_second) = second.split_at(*cross_point - running_sum);

        running_sum += cross_point - 1;

        first = tail_first;
        second = tail_second;

        left.append(head_first);
        right.append(head_second);
    }

    (left, right)
}

// Generic multipoint crossover. This version skips indices that will not be effected,
// making it somewhat more complex then necessary.
pub fn cross_at_points(pair: &mut [Ind<u8>], bits_per_sym: usize, cross_points: &[usize]) {
    let ind_len = pair[0].0.len();

    let mut bounded_cross_points = Vec::new();

    // add boundary indices to cross points
    bounded_cross_points.push(0);
    bounded_cross_points.extend_from_slice(cross_points);
    bounded_cross_points.push(ind_len * bits_per_sym - 1);

    // this is used to flip between where we want to do bitwise
    // crossover- the start or end index of the cross points
    let mut flip_flop = true;

    // for each pair of indices
    for point_pair in bounded_cross_points.chunks(2) {
        if point_pair.len() != 2 {
            break;
        }
        // get the word indices to start and end the crossover
        let cross_start = point_pair[0] / bits_per_sym;
        let mut cross_end   = point_pair[1] / bits_per_sym;

        // set up our alternating positions
        let mut first_side = 0;
        let mut other_side = 1;
        let mut cross_index = cross_start;
        if flip_flop {
            first_side = 1;
            other_side = 0;
            cross_index = cross_end;
        }

        // if crossing the end of a word, we may need to go
        // off by 1 to get the right crossed indices
        if point_pair[1] % bits_per_sym != 0 {
            cross_end += other_side;
        }

        // for each index, swap words
        for index in cross_start..cross_end {
            let tmp = pair[0].0[index];
            pair[0].0[index] = pair[1].0[index];
            pair[1].0[index] = tmp;
        }

        // cross the word that the cross point is within
        let cross_bit_index = point_pair[first_side] % bits_per_sym;
        if cross_bit_index != (bits_per_sym - 1) {
            let (first, second) = cross_word(pair[first_side].0[cross_index],
                                             pair[other_side].0[cross_index],
                                             cross_bit_index as u8);

            pair[0].0[cross_index] = second;
            pair[1].0[cross_index] = first;
        }

        flip_flop = !flip_flop;
    }
}

#[test]
fn test_cross_at_points() {
    let ind1 = Ind(vec!(0x00, 0x00, 0x00, 0x00, 0x00));
    let ind2 = Ind(vec!(0x0F, 0x0F, 0x0F, 0x0F, 0x0F));

    let pair = &mut [ind1, ind2];

    cross_at_points(pair, 4, &[1, 6]);
    assert!(pair[0] == Ind(vec!(0x01, 0x03, 0x0F, 0x0F, 0x0F)));
    assert!(pair[1] == Ind(vec!(0x0E, 0x0C, 0x00, 0x00, 0x00)));
}

pub fn cross_word(first: u8, second: u8, bit_index: u8) -> (u8, u8) {
    let bit_mask = (2_u32.pow(bit_index as u32) - 1) as u8;

    let first_result  = (first  & !bit_mask) | (second & bit_mask);
    let second_result = (second & !bit_mask) | (first  & bit_mask);

    (first_result, second_result)
}

#[test]
fn test_cross_word() {
    let (first, second) = cross_word(0xff, 0x00, 4);
    assert!(first  == 0xF0, format!("was {:b}, expected {:b}", first,  0xF0));
    assert!(second == 0x0F, format!("was {:b}, expected {:b}", second, 0x0F));
}

pub fn cross_at_points_naive(pair: &mut [Ind<u8>], bits_per_sym: usize, cross_points: &[usize]) {
    let ind_len = pair[0].0.len();

    let mut cross_point_index = 0;

    for index in 0..ind_len {
        let tmp = pair[0].0[index];
        pair[0].0[index] = pair[1].0[index];
        pair[1].0[index] = tmp;

        // are there more cross points?
        if cross_points.len() > cross_point_index {
            // time to move to next cross point?
            let cross_index = cross_points[cross_point_index] / bits_per_sym;
            if index == cross_index {
                let cross_bit_index = cross_points[cross_point_index] % bits_per_sym;
                if cross_bit_index != (bits_per_sym - 1) {
                    let (first, second) = cross_word(pair[0].0[index],
                                                     pair[1].0[index],
                                                     cross_bit_index as u8);

                    pair[0].0[index] = second;
                    pair[1].0[index] = first;
                }
                pair.swap(0, 1);
                cross_point_index += 1;
            }
        }
    }
}

#[test]
fn test_cross_at_points_naive() {
    let ind1 = Ind(vec!(0x00, 0x00, 0x00, 0x00, 0x00));
    let ind2 = Ind(vec!(0x0F, 0x0F, 0x0F, 0x0F, 0x0F));

    let pair = &mut [ind1, ind2];

    cross_at_points_naive(pair, 4, &[1, 6]);
    assert!(pair[0] == Ind(vec!(0x01, 0x03, 0x0F, 0x0F, 0x0F)));
    assert!(pair[1] == Ind(vec!(0x0E, 0x0C, 0x00, 0x00, 0x00)));
}


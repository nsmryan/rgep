use rand::prelude::*;

use statrs::distribution::Uniform;

use types::*;


pub fn rotation<T, R>(pop: &mut Pop<T>, pr: f64, rng: &mut R) 
    where T: Copy ,
          R: Rng {
    let ind_len = pop.0[0].0.len();

    let rotation_sampler = Uniform::new(0.0, 1.0).unwrap();
    let rotation_point_sampler = Uniform::new(0.0, ind_len as f64).unwrap();

    let mut scratch = Vec::new();

    for ind in pop.0.iter_mut() {
        if rotation_sampler.sample(rng) < pr {
            let rotation_point = rotation_point_sampler.sample(rng) as usize;
            rotate_copy(ind, &mut scratch, rotation_point);
        }
    }
}

pub fn rotate_naive<T>(ind: &mut Ind<T>, rotation_point: usize) 
    where T: Copy {
    let ind_len = ind.0.len();

    let mut index = 0;
    let mut tmp = ind.0[0];

    for _ in 0..ind.0.len() {
        let other_index = (index + rotation_point) % ind_len; 

        let tmp2 = ind.0[other_index];
        ind.0[other_index] = tmp;
        tmp = tmp2;

        index = other_index;
    }
}

#[test]
fn test_rotate() {
    let mut ind = Ind(vec!(0, 1, 2, 3, 4));

    let rotation_point = 3;

    rotate_naive(&mut ind, rotation_point);

    let expected = Ind(vec!(2, 3, 4, 0, 1));

    assert!(ind == expected, format!("{:?} != {:?}", ind, expected));
}

pub fn rotate_copy<T>(ind: &mut Ind<T>, scratch: &mut Vec<T>, rotation_point: usize) 
    where T: Copy {
    let ind_len = ind.0.len();

    scratch.clear();

    {
        let slice = ind.0.as_slice();

        scratch.extend_from_slice(&slice[rotation_point..ind_len]);
        scratch.extend_from_slice(&slice[0..rotation_point]);
    }

    ind.0.clear();
    ind.0.extend_from_slice(&scratch[0..ind_len]);
}

#[test]
fn test_rotate_copy() {
    let mut ind = Ind(vec!(0, 1, 2, 3, 4));
    let mut scratch = Vec::new();

    let rotation_point = 3;

    rotate_copy(&mut ind, &mut scratch, rotation_point);

    let expected = Ind(vec!(3, 4, 0, 1, 2));

    assert!(ind == expected, format!("{:?} != {:?}", ind, expected));
}


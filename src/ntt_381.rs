use scalar::{Scalar};
use ff::{PrimeField};

pub fn ntt_381(elements: &Vec<Scalar>) -> Vec<Scalar> {
    let n = elements.len() as u64;
    let mut matrix: Vec<Vec<u64>> = vec![vec![0; n as usize]; n as usize];
    (0..n)
        .for_each(|x| (0..n)
            .for_each(
                |y| matrix[x as usize][y as usize] = x * y));
    // println!("{:#?}", matrix);
    matrix
        .iter()
        .map(|row| {
            elements
                .iter()
                .zip(row)
                .map(|(elem, ij)|
                    elem * Scalar::ROOT_OF_UNITY
                        .pow(&[*ij * ((u32::MAX as u64 + 1) / n), 0, 0, 0])
                )
                .sum::<Scalar>()
        }).collect::<Vec<Scalar>>()
}

pub fn i_ntt_381(elements: &Vec<Scalar>) -> Vec<Scalar> {
    let n = elements.len() as u64;
    let mut matrix: Vec<Vec<u64>> = vec![vec![0; n as usize]; n as usize];
    (0..n)
        .for_each(|x| (0..n)
            .for_each(
                |y| matrix[x as usize][y as usize] = x * y));
    matrix
        .iter()
        .map(|row| {
            elements
                .iter()
                .zip(row)
                .map(|(elem, ij)|
                    elem * Scalar::ROOT_OF_UNITY_INV
                        .pow(&[*ij * ((u32::MAX as u64 + 1) / n), 0, 0, 0])
                )
                .sum::<Scalar>() * Scalar::from(n).invert().unwrap()
        }).collect::<Vec<Scalar>>()
}

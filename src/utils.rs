extern crate bellman_ce;
extern crate byteorder;
extern crate num_bigint;
extern crate num_traits;
extern crate rand;

use bellman_ce::pairing::{
    bn256::{Bn256, Fq12, G1Affine, G2Affine},
    ff::PrimeField,
    CurveAffine,
};
use itertools::Itertools;
use num_bigint::BigUint;
use num_traits::Num;
use std::fmt::Display;

pub fn repr_to_big<T: Display>(r: T) -> String {
    BigUint::from_str_radix(&format!("{}", r)[2..], 16).unwrap().to_str_radix(10)
}

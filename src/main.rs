pub mod constants;
pub mod utils;
pub mod eth_root;
pub mod tonelli_shanks;

use std::ops::Mul;
use ark_bn254::{Bn254, Fq12, Fr, G1Affine, G2Affine};
use ark_ec::{pairing::{MillerLoopOutput, Pairing}, AffineRepr};
use constants::{H, U};
use utils::exp;
use ark_ff::{Field, Zero, One};
use ark_std::{rand::{rngs::StdRng, SeedableRng}, UniformRand};

use crate::{constants::{E, RESIDUE}, eth_root::eth_root, tonelli_shanks::TS, utils::sample_27th_root_of_unity};

pub fn compute_wi(mlo: MillerLoopOutput<Bn254>) -> Fq12 {
    exp(mlo.0, &H)
}

pub fn compute_c(mlo: MillerLoopOutput<Bn254>) -> Fq12 {
    exp(mlo.0, &U)
}

fn main() {
    let rng = &mut StdRng::seed_from_u64(333324431231312u64);

    let a = Fr::rand(rng);
    let b = Fr::rand(rng);

    let c = b.clone();
    let d = a.clone();

    let g1 = G1Affine::generator();
    let g2 = G2Affine::generator();

    let a = g1.mul(&a);
    let b = g2.mul(&b);
    let c = g1.mul(&c);
    let d = g2.mul(&d);

    let x = Bn254::multi_miller_loop(&[a, -c], &[b, d]);
    let e = Bn254::final_exponentiation(x).unwrap();
    assert_eq!(e.0, Fq12::one());

    // this roots can be hardcoded instead of sampling each time
    let w27 = sample_27th_root_of_unity(rng);

    let mut eth_residue = Fq12::zero();

    for i in 0..3 {
        let tmp_shift = w27.pow(&[i as u64, 0, 0, 0]);
        let tmp_eth = x.0 * tmp_shift;

        if exp(tmp_eth, &RESIDUE) == Fq12::one() {
            println!("found at {}", i);
            // shift = tmp_shift;
            eth_residue = tmp_eth;

            break;
        }
    }

    // this roots can be hardcoded instead of sampling each time
    let w27 = sample_27th_root_of_unity(rng);
    let ts = TS { w: w27 };

    let root = eth_root(eth_residue, ts);
    assert_eq!(exp(root, &E), eth_residue);
}

#![feature(trait_alias)] // Add this line

use halo2_base::{halo2_proofs::halo2curves::ff::PrimeField, utils::BigPrimeField};

pub mod sha256;
pub mod util;

pub trait Field = BigPrimeField + PrimeField<Repr = [u8; 32]>;

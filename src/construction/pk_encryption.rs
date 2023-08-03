// Copyright © 2023 Niklas Siemer
//
// This file is part of qFALL-crypto.
//
// qFALL-crypto is free software: you can redistribute it and/or modify it under
// the terms of the Mozilla Public License Version 2.0 as published by the
// Mozilla Foundation. See <https://mozilla.org/en-US/MPL/2.0/>.

//! This module provides the trait a struct should implement if it is an
//! instance of a public key encryption scheme. Furthermore, it contains
//! cryptographic schemes implementing the `PKEncryption` trait.
//!
//! The main references are listed in the following:
//! - \[2\] Gentry, Craig and Peikert, Chris and Vaikuntanathan, Vinod (2008).
//! Trapdoors for hard lattices and new cryptographic constructions.
//! In: Proceedings of the fortieth annual ACM symposium on Theory of computing.
//! <https://dl.acm.org/doi/pdf/10.1145/1374376.1374407>

mod dual_regev_discrete_gauss;
mod regev;
mod regev_discrete_gauss;
pub use dual_regev_discrete_gauss::DualRegevWithDiscreteGaussianRegularity;
pub use regev::Regev;
pub use regev_discrete_gauss::RegevWithDiscreteGaussianRegularity;

use qfall_math::integer::Z;

pub trait PKEncryption {
    type PublicKey;
    type SecretKey;
    type Cipher;

    fn gen(&self) -> (Self::PublicKey, Self::SecretKey);
    fn enc(&self, pk: &Self::PublicKey, message: impl Into<Z>) -> Self::Cipher;
    fn dec(&self, sk: &Self::SecretKey, cipher: &Self::Cipher) -> Z;
}

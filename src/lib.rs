//! A library to compute Bessel functions
//!
//! # Introduction
//!
//! Rust library that acts as a wrapper for the Fortran subroutines developed by D.E. Amos.
//! The library provides functionality to compute the Bessel, Hankel and Airy functions of complex argument and real order.
//! Negative orders are implemented via the standard formulae.
//!
//! # Dependencies
//!
//! To compile this library is necessary to install gfortran-13
//!
//! Heavily inspired by [this](https://github.com/joeydumont/complex_bessel) project

pub mod bessel_j;
pub mod bessel_k;
pub mod bessel_y;

pub(crate) mod bindings;
pub(crate) mod derivative;

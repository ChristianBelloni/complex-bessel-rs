![Maintenance](https://img.shields.io/badge/maintenance-actively--developed-brightgreen.svg)
[![crates-io](https://img.shields.io/crates/v/complex-bessel-rs.svg)](https://crates.io/crates/complex-bessel-rs)
[![api-docs](https://docs.rs/complex-bessel-rs/badge.svg)](https://docs.rs/complex-bessel-rs)

A library to compute Bessel functions

## Introduction

Rust library that acts as a wrapper for the Fortran subroutines developed by D.E. Amos.
The library provides functionality to compute the Bessel, Hankel and Airy functions of complex argument and real order.
Negative orders are implemented via the standard formulae.

## Dependencies

To compile this library is necessary to install gfortran-13

Heavily inspired by [this](https://github.com/joeydumont/complex_bessel) project

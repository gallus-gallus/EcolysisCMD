//! This module contains functions to simulate populations demographics (including genetics) using forward-direction individual-based simulation methods. Populations are represented by a list of individuals with defined behaviors and attributes.

struct Individual {
    id: usize,
    age: u16,
    lifestage: u8,
    parents: Vec<usize>,
    genotype: Vec<Vec<u8>>,
}

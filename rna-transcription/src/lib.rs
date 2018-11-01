#![feature(try_from)]
use std::convert::{TryFrom};

#[derive(Debug, PartialEq)]
pub struct DNA {
    nucleotides: Vec<DNANucleotide>
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum DNANucleotide {
    G,
    C,
    T,
    A
}

impl TryFrom<char> for DNANucleotide {
    type Error = ();

    fn try_from(c: char) -> Result<DNANucleotide, ()> {
        match c.to_uppercase().next() {
            Some('A') => Ok(DNANucleotide::A),
            Some('C') => Ok(DNANucleotide::C),
            Some('T') => Ok(DNANucleotide::T),
            Some('G') => Ok(DNANucleotide::G),
            _ => Err(())
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct RNA {
    nucleotides: Vec<RNANucleotide>
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum RNANucleotide {
    G,
    C,
    U,
    A
}

impl TryFrom<char> for RNANucleotide {
    type Error = ();

    fn try_from(c: char) -> Result<RNANucleotide, ()> {
        match c.to_uppercase().next() {
            Some('A') => Ok(RNANucleotide::A),
            Some('C') => Ok(RNANucleotide::C),
            Some('U') => Ok(RNANucleotide::U),
            Some('G') => Ok(RNANucleotide::G),
            _ => Err(())
        }
    }
}

impl From<DNANucleotide> for RNANucleotide {

    fn from(dna: DNANucleotide) -> RNANucleotide {
        match dna {
            DNANucleotide::G => RNANucleotide::C,
            DNANucleotide::C => RNANucleotide::G,
            DNANucleotide::T => RNANucleotide::A,
            DNANucleotide::A => RNANucleotide::U
        }
    }
}

impl DNA {
    pub fn new(dna: &str) -> Result<DNA, usize> {
       match dna
            .chars()
            .enumerate()
            .try_fold(vec![], |mut acc, (i, c)| {
                match DNANucleotide::try_from(c) {
                    Ok(n) => {
                        acc.push(n);
                        Ok(acc)
                    },
                    _ => Err(i)
                }
            }) {
            Ok(nucleotides) => Ok(DNA {
                nucleotides,
            }),
            Err(i) => Err(i)
        }
    }

    pub fn to_rna(&self) -> RNA {
        RNA {
            nucleotides: self.nucleotides.iter().map(|&n| {
                n.into()
            }).collect::<Vec<RNANucleotide>>()
        }
    }
}

impl RNA {
    pub fn new(rna: &str) -> Result<RNA, usize> {
        match rna
            .chars()
            .enumerate()
            .try_fold(vec![], |mut acc, (i, c)| {
                match RNANucleotide::try_from(c) {
                    Ok(n) => {
                        acc.push(n);
                        Ok(acc)
                    },
                    _ => Err(i)
                }
            }) {
            Ok(nucleotides) => Ok(RNA {
                nucleotides
            }),
            Err(i) => Err(i)
        }
    }
}

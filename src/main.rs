use std::{path::Path};

mod sequences;
mod matrix;

use matrix::Matrix;

pub fn jk(observed_distance: f64) -> f64 {
    -0.75*(1.0-4.0/3.0*observed_distance).ln()
}

#[derive(Debug, Clone)]
pub struct Sequence{
    label: String,
    content: String,
}

impl Sequence {
    pub fn length(&self) -> usize{
        self.content.len()
    }

    pub fn hamming_distance(&self, other: &Sequence) -> Option<u64> {
        if self.length() == other.length() {
            let iter = self.content.chars().into_iter().zip(other.content.chars().into_iter());
            let result = iter.map(|(l, r)| if l == r { 0 } else { 1 }).sum();
            Some(result)
        } else {
            None
        }
    }

    pub fn hamming_distance_relative(&self, other: &Sequence) -> Option<f64>{
        if let Some(distance) = self.hamming_distance(other) {
            Some(distance as f64/self.length() as f64)
        } else {
            None
        }
    }

    pub fn jukes_cantor_distance(&self, other: &Sequence, observed_substitution_frequency: f64) -> Option<f64> {
        let jukes_cantor = jk(observed_substitution_frequency);
        if self.length() == other.length() {
            let iter = self.content.chars().into_iter().zip(other.content.chars().into_iter());
            let result = iter.map(|(l, r)| if l == r { 0.0 } else { jukes_cantor }).sum();
            Some(result)
        } else {
            None
        }
    }
}

fn main() {
    let mut matrix = Matrix::<f64>::new(11, 10, 8184.0);
    matrix.set(5, 0, 218334.0);
    matrix.set(9, 4, 21833423.0);
    matrix.set(9, 9, 0.0);
    
	// Pretty printing and testing a matrix.
    println!("\n\nHere should be your matrix:\n{}", matrix);
    println!("Max Val: {}", matrix.max());
    println!("Min Val: {}", matrix.min());

	println!("\n");
    let sequences_a = sequences::from_file(Path::new("data/align3.fasta"));
	println!("Printing out sequences from align3.fasta:\n{:?}\n\n", sequences_a);
    let sequences_b = sequences::from_file(Path::new("data/align1.fasta"));
    println!("Printing out sequences from align1.fasta:\n{:?}\n\n", sequences_b);

    let result = sequences::join(&sequences_a, &sequences_b);
    println!("Joined sequences written to output.fasta:\n{:?}\n\n", result);
    sequences::to_file(Path::new("output.fasta"), &result);

    println!("Calculating hamming distances:");
    println!("{}", sequences::distance_matrix_hamming(&sequences_b));

    println!("Calculating cantor distances with substitution factor 0.15:");
    println!("{}", sequences::distance_matrix_cantor(&sequences_b, 0.15));

}

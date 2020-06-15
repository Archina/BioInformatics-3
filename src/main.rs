use std::{path::Path};

mod sequences;
mod matrix;

use matrix::Matrix;

fn main() {
    

	println!("\n");
    let sequences_a = sequences::from_file(Path::new("data/align3.fasta"));
	println!("Printing out sequences from align3.fasta:\n{:#?}\n\n", sequences_a);
    let sequences_b = sequences::from_file(Path::new("data/align1.fasta"));
    println!("Printing out sequences from align1.fasta:\n{:#?}\n\n", sequences_b);

    let result = sequences::join(&sequences_a, &sequences_b);
    println!("Joined sequences written to output.fasta:\n{:#?}\n\n", result);
    sequences::to_file(Path::new("output/output.fasta"), &result);

    println!("Calculating hamming distances:");
    let matrix = sequences::distance_matrix_hamming(&sequences_b); 
    println!("{}\nMin: {} | Max: {}", matrix, matrix.min(), matrix.max());

    println!("Calculating cantor distances with substitution factor 0.15:");
    println!("{}", sequences::distance_matrix_cantor(&sequences_b, 0.15));
}

use std::{io::{BufRead, BufReader, Write}, fs::File, path::Path, fmt};

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

impl fmt::Display for Sequence {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, ">{}\n{}", self.label, self.content)
    }
}

pub fn from_file(path: &Path) -> Vec<Sequence> {
    let file = File::open(path).expect("Not a file path...");
    let mut sequences: Vec<Sequence> = vec!();
    let mut next_sequence: Option<Sequence> = None;
    for next_line in BufReader::new(file).lines() {
        if let Ok(line) = next_line{
            if line.starts_with(">") {
                if let Some(moop) = next_sequence.take() {
                    sequences.push(moop);
                }
                next_sequence = Some(Sequence{
                    label: line.replace("\n", "").replace(">", ""),
                    content: String::from("")
                })
            } else {
                for entry in next_sequence.iter_mut() {
                    entry.content = format!("{}{}", entry.content, line.replace("\n", ""));
                }
            }
        }
    }
    if let Some(moop) = next_sequence.take() {
        sequences.push(moop);
    }
    return sequences
}

pub fn to_file(path: &Path, seqs: &Vec<Sequence>) {
    let mut file = File::create(path).expect("Cannot create file...");
    for seq in seqs {
        file.write(format!("> {}\n", seq.label).as_bytes()).expect("Couldn't write line to file.");
        file.write(format!("{}\n", seq.content).as_bytes()).expect("Couldn't write line to file.");
    }
}

pub fn join(a: &Vec<Sequence>, b: &Vec<Sequence>) -> Vec<Sequence>{
    let mut output = [a.as_slice(), b.as_slice()].concat();
    let length = output.iter().map(|x| x.length()).max();
    if let Some(length) = length{
        for seq in output.iter_mut() {
            while seq.length() < length {
                seq.content.push('-');
            }
        }
    }
    output
}

pub fn distance_matrix_hamming(seqs: &Vec<Sequence>) -> crate::matrix::Matrix<f64> {
    let mut matrix = crate::matrix::Matrix::<f64>::new(seqs.len(),seqs.len(), 0.0.into());

    for l_idx in 0..seqs.len(){
        for t_idx in 0..seqs.len(){
            let left = &seqs[l_idx];
            let top = &seqs[t_idx];
            matrix.set(l_idx, t_idx, left.hamming_distance_relative(top).unwrap_or_default().into());
        }
    }
    matrix
}

pub fn distance_matrix_cantor(seqs: &Vec<Sequence>, osf: f64) -> crate::matrix::Matrix<f64> {
    let mut matrix = crate::matrix::Matrix::<f64>::new(seqs.len(),seqs.len(), 0.0.into());

    for l_idx in 0..seqs.len(){
        for t_idx in 0..seqs.len(){
            let left = &seqs[l_idx];
            let top = &seqs[t_idx];
            matrix.set(l_idx, t_idx, left.jukes_cantor_distance(top, osf).unwrap_or_default().into());
        }
    }
    matrix
}

fn jk(observed_distance: f64) -> f64 {
    -0.75*(1.0-4.0/3.0*observed_distance).ln()
}

mod test{
    use crate::sequences::{Sequence, from_file};
    use std::path::Path;

    #[test]
    fn test_read_sequence() {
        let sequences = from_file(Path::new("data/align4.fasta"));
        assert_eq!(sequences.len(), 1);
        assert_eq!(sequences[0].content, "ATATTCG");
        assert_eq!(sequences[0].label, "Taxon_1");
        assert_eq!(format!("{}", sequences[0]), ">Taxon_1\nATATTCG\n");
    }

    #[test]
    fn test_distance_success(){
        let seq_a = Sequence{
            label: String::from("Seq_A"),
            content: String::from("AAAAAAAAAA")
        };
        let seq_b = Sequence{
            label: String::from("Seq_B"),
            content: String::from("AAACCAAGGA")
        };
        assert_eq!(
            seq_a.hamming_distance(&seq_a),
            Some(0)
        );
        assert_eq!(
            seq_a.hamming_distance(&seq_b),
            Some(4)
        );
        assert_eq!(
            seq_a.hamming_distance_relative(&seq_a),
            Some(0.0)
        );
        assert_eq!(
            seq_a.hamming_distance_relative(&seq_b),
            Some(0.4)
        );
    }

    #[test]
    fn test_distance_fail(){
        let seq_a = Sequence{
            label: String::from("Seq_A"),
            content: String::from("AAAAAAAAAA")
        };
        let seq_b = Sequence{
            label: String::from("Seq_B"),
            content: String::from("AAACCAA")
        };
        assert_eq!(
            seq_a.hamming_distance(&seq_b),
            None
        );
        assert_eq!(
            seq_a.hamming_distance_relative(&seq_b),
            None
        );
    }
}
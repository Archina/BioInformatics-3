use crate::Sequence;
use std::{io::{BufRead, BufReader, Write}, fs::File, path::Path};

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
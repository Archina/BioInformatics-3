use std::fmt;

pub struct Matrix<T>{
    content: Vec<Vec<T>>,
    width: usize,
    height: usize,
}

impl<T: Clone> Matrix<T>{
    pub fn new(x: usize, y: usize, default_val: T) -> Matrix<T>{
        let mut result = vec!();
        for _ in 0..y {
            result.push(vec![default_val.clone(); x])
        }
        Matrix{
            content: result,
            width: x,
            height: y
        }
    }

    pub fn set(&mut self, x: usize, y: usize, value: T){
        self.content[x][y] = value;
    }

    pub fn get(&self, x: usize, y: usize) -> &T {
        &self.content[x][y]
    }

    pub fn width(&self) -> &usize {
        &self.width
    }

    pub fn height(&self) -> &usize {
        &self.height
    }
}

impl<T: Ord + Default + Copy> Matrix<T>{
    pub fn max(&self) -> Option<&T>{
        self.content.iter().map(|row| row.iter().max().unwrap()).max()
    }

    // fn min(&self) -> Option<&T>{
    //     self.content.iter().map(|row| row.iter().min().unwrap()).min()
    // }
}

impl Matrix<ordered_float::OrderedFloat<f64>>{
    pub fn min(&self) -> Option<&ordered_float::OrderedFloat<f64>>{
        self.content.iter().map(|row| row.iter().filter(|&x| x != &0f64.into()).min().unwrap()).min()
    }
}

impl<T: Ord + Clone + fmt::Display> fmt::Display for Matrix<T>{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let lengths_column: Vec<usize> = vec![0; self.width].iter().enumerate()
            .map(|(idx, _)| {
                format!("{}", vec![0; self.height].iter().enumerate().map(|(idx_x, _)| self.get(idx_x, idx)).max().unwrap()).len()
            })
            .collect();
        for row in &self.content {
            let length = row.len();
            for (idx, entry) in row.iter().enumerate(){
                let max_length = lengths_column[idx];
                let mut to_string = format!("{}{}", vec![' '; max_length].iter().collect::<String>(), entry);
                to_string = to_string.chars().rev().collect();
                to_string.truncate(max_length);
                to_string = to_string.chars().rev().collect();
                to_string = if length == idx +1 {
                    format!("{}", to_string)
                } else {
                    format!("{}, ", to_string)
                };
                write!(f, "{}", to_string);
            }
            write!(f, "\n");
        }
        write!(f, "")
    }
}
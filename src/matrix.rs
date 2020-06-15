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

impl Matrix<f64>{
    pub fn max(&self) -> f64{
        let mut max = std::f64::NAN;
        for x_idx in 0..*self.width() {
            for y_idx in 0..*self.height() {
                if x_idx != y_idx {
                    max = max.max(*self.get(y_idx, x_idx));
                }
            }
        }
        max
    }

    pub fn min(&self) -> f64{
        let mut min = std::f64::NAN;
        for x_idx in 0..*self.width() {
            for y_idx in 0..*self.height() {
                if x_idx != y_idx {
                    min = min.min(*self.get(y_idx, x_idx));
                }
            }
        }
        min
    }
}

impl<T: Clone + fmt::Display> fmt::Display for Matrix<T>{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result = fmt::Result::Ok(());
        let lengths_column: Vec<usize> = vec![0; self.width].iter().enumerate()
            .map(|(idx, _)| {
                vec![0; self.height].iter().enumerate().map(|(idx_x, _)| format!("{}", self.get(idx_x, idx)).len()).max().unwrap()
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
                result = result.and(write!(f, "{}", to_string));
            }
            result = result.and(write!(f, "\n"));
        }
        write!(f, "")
    }
}

mod test{

    use crate::matrix::Matrix;

    #[test]
    fn test_matrix() {
        let mut matrix = Matrix::<f64>::new(11, 10, 8184.0);
        matrix.set(5, 0, 218334.0);
        matrix.set(9, 4, 21833423.0);
        matrix.set(9, 9, 0.0); // Zero on the diagonal

        assert_eq!(matrix.get(9,9), &0.0);
        assert_eq!(matrix.min(), 8184.0);
        assert_eq!(matrix.max(), 21833423.0);
        
        matrix.set(0, 3, -0.1);
        assert_eq!(matrix.min(), -0.1);
    }

}
extern crate rand;
use rand::{thread_rng, Rng};
pub mod sudoku;

#[cfg(not(test))]
fn main() {

    let mut rng = thread_rng();
    let choices: [usize; 9] = [0, 1, 2, 3, 4, 5, 6, 7, 8];
    let (&i, &j) = (rng.choose(&choices).unwrap(), rng.choose(&choices).unwrap());
    let before_swap = sudoku::Sudoku::new([8, 1, 6, 3, 5, 7, 4, 9, 2]);
    let after_swap  = before_swap.swap(i, j);
    println!("{}", before_swap);
    println!("{}", before_swap.complete());
    println!("{}", after_swap);
    println!("{}", after_swap.complete());
}

#[cfg(test)]
mod test {
    fn true_test() {
        let example = sudoku::Sudoku::new([1, 2, 3, 4, 5, 6, 7, 8, 9]);
        println!("{}", example);
        println!("{}", example.complete());
    }

    fn false_test() {
        let example  = sudoku::Sudoku::new([8, 1, 6, 3, 5, 7, 4, 9, 2]);
        println!("{}", example);
        println!("{}", example.complete());
    }
}

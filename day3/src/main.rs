#![feature(let_chains)]
use itertools::Itertools;
mod consts;

fn main() {
    let inputs :Vec<Vec<char>> = consts::BIG_INPUT.iter().map(|line| line.chars().collect_vec()).collect();
    for i in 0..inputs.len() {
        for j in 0..inputs[0].len() {
            if (is_symbol(inputs[i][j])) {
                println!("{i}, {j}: {}", inputs[i][j]);
            }

            
        }
    }

}

fn is_symbol(c: char) -> bool {
    c != '.' && (c < '0'|| c > '9')
}


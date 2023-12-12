mod consts;
fn main() {
    let inputs: Vec<(Vec<usize>, Vec<usize>)> = consts::INPUT
        .into_iter()
        .map(|s| {
            let l_r: Vec<&str> = s.split("|").collect();
            let [a,b ]: [&str] = l_r[0..2] else {panic!()};
            let winning: Vec<usize> = unsafe_parse(a, 2);
            let numbers: Vec<usize> = unsafe_parse(b, 0);
            (winning, numbers)
        })
        .collect();
    println!("{:?}", inputs);
}

fn unsafe_parse(line:  &str, skip: usize) -> Vec<usize> {
    line.split_whitespace().skip(skip).map(|str| str.parse().expect(&format!("could not parse: {str}"))).collect()
}

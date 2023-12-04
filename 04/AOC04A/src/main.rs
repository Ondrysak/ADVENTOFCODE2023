use regex::Regex;
use std::collections::HashSet;

fn main() {
    let mut total = 0;
    let input = include_str!("../input_real.txt");
    // Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53

    for line in input.split("\n") {
        println!("{}", line);
        let mut l_total = 0;
        let mut winners: HashSet<_> = HashSet::new();
        let parts: Vec<_>= line.split(":").collect();
        let both = parts[1];
        let b_parts: Vec<_>  = both.split("|").collect();
        let re = Regex::new(r"\d+").unwrap();
        for m in re.find_iter(b_parts[0])
        {
            winners.insert(m.as_str());
        }
        for m in re.find_iter(b_parts[1])
        {
            if winners.contains(m.as_str()){
                if l_total > 0 {
                    l_total = l_total*2;
                }
                else{
                    l_total = 1;
                }
            }
        }
        total += l_total;

    }
    println!("Final total {}", total);
}
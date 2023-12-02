use regex::Regex;

fn main() {
    let mut total = 0;
    let input = include_str!("../input_real.txt");
    println!("{}", input);
    let re = Regex::new(r"[0-9]").unwrap();
    for line in input.split("\n") {
        let matches: Vec<_> = re.find_iter(line).map(|m| m.as_str()).collect();
        let first = matches[0];
        let last = matches[matches.len()-1];
        println!("{} {}",first, last);
        let mut firstlast: String = first.to_owned();
        firstlast.push_str(last);
        let num: i32 = firstlast.parse().unwrap();
 
        total += num;
        println!("{}",total);
    }
    println!("done");
    println!("{}",total);
}

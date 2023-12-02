use regex::Regex;

fn main() {
    let mut total = 0;
    let mut second_total = 0;
    let input = include_str!("../input_real.txt");
    println!("{}", input);
    let re = Regex::new(r"[1-9]|one|two|three|four|five|six|seven|eight|nine").unwrap();
    let re_greed = Regex::new(r".*([1-9]|one|two|three|four|five|six|seven|eight|nine)").unwrap();

    for line in input.split("\n") {
        let matches: Vec<_> = re.find_iter(line).map(|m| m.as_str()).collect();
        let caps = re_greed.captures(line).unwrap();
        let substr1 = caps.get(1).map_or("", |m| m.as_str());
        let first = match matches[0] {
            "one" => "1",
            "two" => "2",
            "three" => "3",
            "four" => "4",
            "five" => "5",
            "six" => "6",
            "seven" => "7",
            "eight" => "8",
            "nine" => "9",
            _ => matches[0],
        };
        let last = match  substr1 {
            "one" => "1",
            "two" => "2",
            "three" => "3",
            "four" => "4",
            "five" => "5",
            "six" => "6",
            "seven" => "7",
            "eight" => "8",
            "nine" => "9",
            _ =>  substr1,
        };

        println!("{}", line);
        println!("{} {}",first, last);
        let mut firstlast: String = first.to_owned();
        firstlast.push_str(last);
        let num: i32 = firstlast.parse().unwrap();
        println!("Adding {}", num);
        total += num;
        
        println!("Total end of loop {}",total);
    }
    println!("done");
    println!("{}",total);
}

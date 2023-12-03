use regex::Regex;


fn line_possible(text: &str, r: i32, g: i32, b: i32) -> i32 {
    //Game (\d+): (.*)
    let re_id = Regex::new(r"(\d+): (.*)").unwrap();
    //(\d+) [a-z]+
    let re_rgb = Regex::new(r"(\d+) ([a-z]+)").unwrap();
    let first_match = re_id.captures(text).unwrap();
    let id = first_match.get(1).map_or("", |m| m.as_str());
    let draws = first_match.get(2).map_or("", |m| m.as_str());

    println!("{}",id);
    println!("{}",draws);
    for line in draws.split(";") {
        let mut lred = 0;
        let mut lgreen = 0;
        let mut lblue = 0;
        for (_, [count, color]) in re_rgb.captures_iter(line).map(|c| c.extract()) {
            let count_int = count.parse::<i32>().unwrap();

            match color {
                "red" => lred = count_int,
                "green" => lgreen = count_int,
                "blue" => lblue = count_int,
                _ => (),
            };
            println!("{}",color);
            println!("{}",count);
        }
        if lred > r || lblue > b || lgreen > g {
            println!("Impossibru yo! Because of {}", line);
            return 0;
        }
        println!("Found in {}",line);
    }

    let id_int: i32 = id.parse::<i32>().unwrap();
    return id_int;
}


fn main() {
    let input = include_str!("../input_real.txt");
    // 12 red cubes, 13 green cubes, and 14 blue cubes
    let red_limit: i32 = 12;
    let green_limit: i32 = 13;
    let blue_limit: i32 = 14;
    let mut sum = 0;
    for line in input.split("\n") {
        println!("{}",line);
        sum += line_possible(line,red_limit, green_limit, blue_limit);
    }
    println!("Final answer {}", sum);
}

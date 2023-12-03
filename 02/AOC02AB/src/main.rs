use regex::Regex;


fn line_possible(text: &str, r: i32, g: i32, b: i32) -> (i32, i32) {
    //Game (\d+): (.*)
    let re_id = Regex::new(r"(\d+): (.*)").unwrap();
    //(\d+) [a-z]+
    let re_rgb = Regex::new(r"(\d+) ([a-z]+)").unwrap();
    let first_match = re_id.captures(text).unwrap();
    let id = first_match.get(1).map_or("", |m| m.as_str());
    let draws = first_match.get(2).map_or("", |m| m.as_str());

    println!("{}",id);
    println!("{}",draws);
    let mut minred = 0;
    let mut mingreen = 0;
    let mut minblue = 0;
    let mut impossible = false;

    for line in draws.split(";") {
        let mut lred = 0;
        let mut lgreen = 0;
        let mut lblue = 0;
        for (_, [count, color]) in re_rgb.captures_iter(line).map(|c| c.extract()) {
            let count_int = count.parse::<i32>().unwrap();

            match color {
                "red" => {
                    lred = count_int;
                    if minred < lred {
                        minred = lred;
                    }
                },
                "green" => {
                    lgreen = count_int;
                    if mingreen < lgreen {
                        mingreen = lgreen;
                    }
                },
                "blue" => {
                    lblue = count_int;
                    if minblue < lblue {
                        minblue = lblue;
                    }
                }
                _ => (),
            };

            println!("{}",color);
            println!("{}",count);
        }
        if lred > r || lblue > b || lgreen > g {
            println!("Impossibru yo! Because of {}", line);
            impossible = true;
        }
        println!("Found in {}",line);
    }
    if !impossible {
        let id_int: i32 = id.parse::<i32>().unwrap();
        return (id_int,minred*mingreen*minblue);
    }
    else{
        return (0,minred*mingreen*minblue);
    }

}


fn main() {
    let input = include_str!("../input_real.txt");
    // 12 red cubes, 13 green cubes, and 14 blue cubes
    let red_limit: i32 = 12;
    let green_limit: i32 = 13;
    let blue_limit: i32 = 14;
    let mut sum = 0;
    let mut powersum = 0;
    for line in input.split("\n") {
        println!("{}",line);
        let (possible, power) = line_possible(line,red_limit, green_limit, blue_limit);
        sum += possible;
        powersum += power;
    }
    println!("Final answer(A) {}", sum);
    println!("Final answer(B) {}", powersum);
}

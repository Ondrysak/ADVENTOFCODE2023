use regex::Regex;
use std::collections::HashSet;

fn main() {
    let input = include_str!("../input_real.txt");
    let mut total = 0;
    let mut line_vec: Vec<Vec<char>> = Vec::new();
    // Type inference lets us omit an explicit type signature (which
    // would be `HashSet<String>` in this example).
    let mut symbols = HashSet::new();
    for line in input.split("\n") {
        println!("{}", line);
        let char_vec: Vec<char> = line.chars().collect();
        line_vec.push(char_vec);
    }
    for (posl, l) in line_vec.iter().enumerate() {
        for (posc, c) in l.iter().enumerate() {
            if !c.is_digit(10) && *c != '.' {
                symbols.insert((posl, posc));
                print!("-{} ", c);
            } else {
                print!(" {} ", c);
            }
        }
        println!("");
    }
    for x in &symbols {
        println!("{x:?}");
    }
    let re = Regex::new(r"\d+").unwrap();
    for (posl, l) in line_vec.iter().enumerate() {
        let linestring: String = l.into_iter().collect();
        for m in re.find_iter(&linestring)
        {
            let mut ok = false;
            let number: i32 = m.as_str().parse().unwrap();
            let offset = m.range();

            println!("Number {}",number);
            for n in offset{
                println!("Offset {}", n);
                if symbols.contains(&(posl+1,n)){
                    println!("Found symboln in line below mid");          
                    ok = true;
                }
                if n>0 && symbols.contains(&(posl+1,n-1)){
                    println!("Found symboln in line below left");          
                    ok = true;
                }
                if symbols.contains(&(posl+1,n+1)){
                    println!("Found symboln in line below right");          
                    ok = true;
                }
                if n>0 && symbols.contains(&(posl,n-1)) {
                    println!("Found symboln in current line left");          
                    ok = true;
                }
                if symbols.contains(&(posl,n+1)){
                    println!("Found symboln in current line right");          
                    ok = true;
                }
 
                if posl>0 && symbols.contains(&(posl-1,n+1)){
                    println!("Found symboln in line above right");          
                    ok = true;
                }
                if posl>0 && n>0 && symbols.contains(&(posl-1,n-1)) {
                    println!("Found symboln in line above left");          
                    ok = true;
                }
                if posl>0 && symbols.contains(&(posl-1,n)){
                    println!("Found symboln in line above mid");          
                    ok = true;
                }
            }
            if ok{
                total += number;
            }
            else{
                println!("Skipping {}", number);
            }
        }

        println!("{}", linestring);

    }
    println!("Final total {}", total);
}

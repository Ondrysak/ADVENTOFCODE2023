use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
    let input = include_str!("../input_real.txt");
    let mut total = 0;
    let mut line_vec: Vec<Vec<char>> = Vec::new();
    // Type inference lets us omit an explicit type signature (which
    // would be `HashSet<String>` in this example).
    let mut symbols = HashSet::new();
    let mut gears = HashMap::new();
    for line in input.split("\n") {
        println!("{}", line);
        let char_vec: Vec<char> = line.chars().collect();
        line_vec.push(char_vec);
    }
    for (posl, l) in line_vec.iter().enumerate() {
        for (posc, c) in l.iter().enumerate() {
            if *c == '*' {
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
                    if !gears.contains_key(&(posl+1,n)){
                        gears.insert((posl+1,n), Vec::new());
                    }
                    
                    let mutvec = gears.get_mut(&(posl+1,n)).unwrap();          
                    mutvec.push(number);
                    break
                }
                if n>0 && symbols.contains(&(posl+1,n-1)){
                    if !gears.contains_key(&(posl+1,n-1)){
                        gears.insert((posl+1,n-1), Vec::new());
                    }
                    
                    let mutvec = gears.get_mut(&(posl+1,n-1)).unwrap();          
                    mutvec.push(number);
                    break
                }
                if symbols.contains(&(posl+1,n+1)){
                    if !gears.contains_key(&(posl+1,n+1)){
                        gears.insert((posl+1,n+1), Vec::new());
                    }
                    
                    let mutvec = gears.get_mut(&(posl+1,n+1)).unwrap();          
                    mutvec.push(number);
                    break
                }
                if n>0 && symbols.contains(&(posl,n-1)) {
                    if !gears.contains_key(&(posl,n-1)){
                        gears.insert((posl,n-1), Vec::new());
                    }
                    
                    let mutvec = gears.get_mut(&(posl,n-1)).unwrap();          
                    mutvec.push(number);
                    break
                }
                if symbols.contains(&(posl,n+1)){
                    if !gears.contains_key(&(posl,n+1)){
                        gears.insert((posl,n+1), Vec::new());
                    }
                    
                    let mutvec = gears.get_mut(&(posl,n+1)).unwrap();          
                    mutvec.push(number);
                    break
                }
 
                if posl>0 && symbols.contains(&(posl-1,n+1)){
                    if !gears.contains_key(&(posl-1,n+1)){
                        gears.insert((posl-1,n+1), Vec::new());
                    }
                    
                    let mutvec = gears.get_mut(&(posl-1,n+1)).unwrap();          
                    mutvec.push(number);
                    break
                }
                if posl>0 && n>0 && symbols.contains(&(posl-1,n-1)) {
                    if !gears.contains_key(&(posl-1,n-1)){
                        gears.insert((posl-1,n-1), Vec::new());
                    }
                    
                    let mutvec = gears.get_mut(&(posl-1,n-1)).unwrap();          
                    mutvec.push(number);
                    break
                }
                if posl>0 && symbols.contains(&(posl-1,n)){
                    if !gears.contains_key(&(posl-1,n)){
                        gears.insert((posl-1,n), Vec::new());
                    }
                    let mutvec = gears.get_mut(&(posl-1,n)).unwrap();          
                    mutvec.push(number);
                    break
                }
            }
           
        }

        println!("{}", linestring);

    }
    println!("Gears size {}", gears.len());
    for (key, val) in gears.iter() {
        println!("len: {}", val.len());
        if val.len() == 2{
            total += val[0]*val[1]; 
        }

    }
    println!("Final total {}", total);

}

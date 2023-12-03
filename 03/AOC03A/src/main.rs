

fn main() {
    let input = include_str!("../input.txt");
    let mut line_vec: Vec<Vec<char>> = Vec::new();
    for line in input.split("\n") {
        println!("{}",line);
        let char_vec: Vec<char> = line.chars().collect();
        line_vec.push(char_vec);

    }
    for l in line_vec{
        
        for c in l{
            if !c.is_digit(10) && c != '.'{
                print!("-{} ",c);
            }
            else{
                print!(" {} ",c);
            }
            
        
        }
        println!("");
    }
}

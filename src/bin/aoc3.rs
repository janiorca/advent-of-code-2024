use std::fs;

fn get_number( string: &str ) -> Option<(i64,usize)> {
    let cc = string.chars().position( |c|!c.is_ascii_digit()).unwrap();
    if cc > 0 {
        let number = &string[ 0..cc].parse::<i64>().unwrap();
        return Some((*number,cc));
    }
    None
}


fn run_program( input: String, enablers: bool ) -> i64 {
    let mut total = 0;
    let mut pos = 0;
    let mut enable = true;

    while pos +8 < input.len() {
        let cmd = input.get( pos..).unwrap();
        if &cmd[ 0..4] == "do()" {
            enable = true;
        } else if &cmd[ 0..7] == "don't()" && enablers == true {
            enable = false;
        }
        else if &cmd[0..4] == "mul(" {
            let mut sub_pos = pos + 4;
            let mut sub_cmd = &input[sub_pos..];
            if let Some(left) = get_number(sub_cmd) {
                if let Some( nxt ) = sub_cmd.get( left.1.. ){
                    sub_cmd = nxt;
                } else { break; }

                if &sub_cmd[ 0..1 ] == "," {
                    if let Some( nxt ) = sub_cmd.get( 1.. ){
                        sub_cmd = nxt;
                    } else { break; }
                    if let Some(right) = get_number(sub_cmd) {
                        if let Some( nxt ) = sub_cmd.get( right.1.. ){
                            sub_cmd = nxt;
                        } else { break; }

                        if &sub_cmd[ 0..1 ] == ")" && enable {
                            total += left.0*right.0;
                        }
                    }
                }
            }
        }
        pos += 1;
    }
    return total;
}

fn main() {
    let input = fs::read_to_string("inputs/aoc3").unwrap();

    let total = run_program( input.clone(), false );
    println!( "part1: {}", total);

    let total2 = run_program( input, true );
    println!( "part1: {}", total2);

}

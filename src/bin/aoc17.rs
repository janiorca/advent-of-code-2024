use std::fs;

fn show_disasm( program: &Vec<u64>) {
    let mut pos = 0;
    loop {
        if pos >= program.len() {
            break
        }
        let opcode = program[pos];
        let operand = program[pos+1];

        let combo = match operand {
            0..=3 => operand.to_string(),
            4 => "A".to_string(),
            5 => "B".to_string(),
            6 => "C".to_string(),
            _ => "C".to_string(),
        };

        let line = match opcode {
            0 => { "adv     A = A >> ".to_owned() + combo.as_str() },
            1 => { "bxl     B = B XOR ".to_owned() + operand.to_string().as_str() },
            2 => { "bst     B = ".to_owned() + combo.as_str() + " MOD 8 ".to_owned().as_str() } ,
            3 => { "jnz     A,".to_owned() + operand.to_string().as_str() },
            4 => { "bxc     B = B XOR C".to_owned() },
            5 => { "out    ".to_owned() + combo.as_str() },
            6 => { "bdv     B = A >> ".to_owned() + combo.as_str() },
            7 => { "cdv     C = A >> ".to_owned() + combo.as_str() },
            _ => panic!( "bad code")
        };
        println!( "{pos}: {line}" );
        pos += 2;
    }
}

fn run_program( program: &Vec<u64>, mut a: u64, mut b: u64, mut c: u64) -> Vec<u64> {
    let mut ip = 0;
    let mut out: Vec<u64> = Vec::new();

    while ip >= 0 && ip < program.len() {
        let opcode = program[ip];
        let operand = program[ip+1];
        ip += 2;

        let combo_operand = match operand {
            0..=3 => { operand },
            4 => a,
            5 => b,
            6 => c,
            _ => 999 //panic!("bad operand")
        };

        match opcode {
            0 => { a = a / (2u32.pow( combo_operand as u32) ) as u64  },
            1 => { b = b ^ operand }, //??
            2 => { b = combo_operand & 0x0007 },
            3 => {
                if a != 0 {
                    ip = operand as usize;
                }
            },
            4 => { b = b ^ c; },
            //            5 => { println!("{}", combo_operand & 0x0007) },
            5 => { out.push(  combo_operand & 0x0007); },
            6 => { b = a / (2i32.pow( combo_operand as u32) ) as u64  },
            7 => { c = a / (2i32.pow( combo_operand as u32) ) as u64  },
            _ => panic!("bad operand")
        }
        //        println!( "registers {}  {}  {} ", a, b,c);
    }
    out
}

fn get_regiater( line: &str) -> u64 {
    let parts: Vec<&str> = line.split_whitespace().collect();
    parts[2].parse::<u64>().unwrap()
}

// Constructs the list of tokens backwards. At each point there could be more than one token that
// satisfies the current need but may not be compatible with tokens  further up. Search all potentials at each step
fn find_slution( curr_a: u64, program_tokens: &Vec<u64>, required: &[u64] ) -> Option<u64> {
    if required.len() == 0 {
        return Some(curr_a);
    } else {
        for value in 0..=7 {
            let new_a = (curr_a << 3) + value;
            let out  = run_program( &program_tokens, new_a, 0, 0);
            if out[0] == required[0] {
                if let Some( res ) =  find_slution( new_a, program_tokens, &required[1..]) {
                    return Some( res );
                }
            }
        }
        return None;
    }
}

fn main() {
    let input = fs::read_to_string("inputs/aoc17").unwrap();
    let mut lines = input.lines();
    let mut a = get_regiater(lines.next().unwrap());
    let mut b = get_regiater(lines.next().unwrap());
    let mut c = get_regiater(lines.next().unwrap());
    lines.next();

    let program_line = lines.next().unwrap();
    let program_parts: Vec<&str> = program_line.split_whitespace().collect();
    let program_tokens: Vec<u64> = program_parts[1].split(',').map(|x| x.parse::<u64>().unwrap()).collect();

    show_disasm(&program_tokens);

    let result = run_program( &program_tokens, a, b, c);
    let tt: Vec<String> = result.iter().map( |x| x.to_string()  ).collect();
    println!( "Part1: {}", tt.join( ","));

    let mut rev_numbers = program_tokens.clone();
    rev_numbers.reverse();
    let p2 = find_slution( 0, &program_tokens, rev_numbers.as_slice() ).unwrap();
    println!( "Part2: {}", p2);
}

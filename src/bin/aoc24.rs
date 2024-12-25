use std::collections::HashMap;
use std::fs;

#[derive(Clone)]
struct Operation{
    src1: String,
    src2: String,
    dest: String,
    op: String,
}

fn execute( mut states: HashMap<String, u8>, mut code_ops: Vec<Operation> ) -> HashMap<String,u8> {
    loop{
        if code_ops.len() == 0 { break; }
        let mut remove = 99999usize;
        for (idx, attempt)  in code_ops.iter().enumerate() {
            if states.contains_key( &attempt.src1) && states.contains_key( &attempt.src2) {
                let src1 = states[ &attempt.src1 ];
                let src2 = states[ &attempt.src2 ];
                let result = match attempt.op.as_str() {
                    "XOR" => src1 ^ src2,
                    "AND" => src1 & src2,
                    "OR" => src1 | src2,
                    _ => panic!("unexpected op")
                };
                states.insert( attempt.dest.clone(), result);
                remove = idx;
                break;
            }
        }
        code_ops.remove( remove );
    }
    states
}

fn analyze_wires( code_ops: &Vec<Operation>) -> bool {
    for bit in 0..45 {
        for (operands, truth) in [ ((0,0), (0,0)), ((0,1),(1,0)), ((1,0),(1,0)), ((1,1),(0,1)) ] {
            let mut states: HashMap<String,u8> = HashMap::new();
            for ibit in 0..45 {
                let xname = "x".to_string() + &format!( "{:0>2}", ibit);
                let yname = "y".to_string() + &format!( "{:0>2}", ibit);
                if ibit == bit {
                    states.insert( xname, operands.0);
                    states.insert( yname, operands.1);
                } else {
                    states.insert( xname, 0);
                    states.insert( yname, 0);
                }
            }
            states = execute( states, code_ops.clone() );
            let zname = "z".to_string() + &format!( "{:0>2}", bit);
            let zname_c = "z".to_string() + &format!( "{:0>2}", bit+1);
            if states[ &zname ] != truth.0 || states[ &zname_c] != truth.1 {
                // all others should be zero ...
                // Fail at basic adder level
                println!( "Error found at bit {bit}  -- {:?}", operands);
                for ibit in 0..46 {
                    let iname = "z".to_string() + &format!( "{:0>2}", ibit);
                    if states[ &iname ] == 1 {
                        println!( "Bad bit found at {ibit}");
                    }
                }
                return false;
            }
        }
    }
    true
}

fn find_operation_tgt( code_ops: &Vec<Operation>, src1: &str, src2: &str, op: &str) -> Option<String> {
    for co in code_ops {
        if co.op != op {
            continue;
        }
        if ( co.src1 == src1 && co.src2 == src2 ) || ( co.src1 == src2 && co.src2 == src1 ) {
            return Some(co.dest.clone());
        }
    }
    None
}
fn identify_logic_errors( code_ops: &Vec<Operation> ) -> Option<(String,String)>{
    // Find the carry ( AND x and y 001)
    let mut carry_name = find_operation_tgt(code_ops, "x00", "y00", "AND").unwrap();

    for bit in 1..45 {
        let src_x = "x".to_string() + &format!("{:0>2}", bit);
        let src_y = "y".to_string() + &format!("{:0>2}", bit);
        let dst_z = "z".to_string() + &format!("{:0>2}", bit);

        let half_adder_lo_carry = find_operation_tgt(code_ops, &src_x, &src_y, "AND").unwrap();
        let half_adder_lo_sum = find_operation_tgt(code_ops, &src_x, &src_y, "XOR").unwrap();
        let half_adder_hi_carry_option = find_operation_tgt(code_ops, &carry_name, &half_adder_lo_sum, "AND");
        let half_adder_hi_sum_option = find_operation_tgt(code_ops, &carry_name, &half_adder_lo_sum, "XOR");
        if half_adder_hi_carry_option.is_none() && half_adder_hi_sum_option.is_none() {
            // Mixed destinations on lo
            println!( "Mixed destinations on {half_adder_lo_carry} and {half_adder_lo_sum} ");
            return Some(( half_adder_lo_carry, half_adder_lo_sum));
        }

        let half_adder_hi_carry = half_adder_hi_carry_option.unwrap();
        let half_adder_hi_sum = half_adder_hi_sum_option.unwrap();

        if half_adder_lo_carry.contains( "z") {
            println!( "Mixed destinations on {half_adder_lo_carry} and {half_adder_hi_sum} ");
            return Some(( half_adder_lo_carry, half_adder_hi_sum));
        }

        let new_carry_name_option = find_operation_tgt(code_ops, &half_adder_lo_carry, &half_adder_hi_carry, "OR");
        if new_carry_name_option.is_none() {
            println!("Missing carry operands {half_adder_lo_carry} and {half_adder_hi_carry}");
            return Some(( half_adder_hi_carry, half_adder_hi_sum));
        }
        let new_carry_name = new_carry_name_option.unwrap();

        if half_adder_hi_sum != dst_z && new_carry_name.contains( "z") {
            println!("wrong sum bit destination {half_adder_hi_sum} and bad carry bit destination {new_carry_name}");
            return Some(( half_adder_hi_sum, new_carry_name));
        }
        carry_name = new_carry_name;
    }
    None
}
fn main() {
    let input = fs::read_to_string( "inputs/aoc24").unwrap();

    let mut states: HashMap<String, u8> = HashMap::new();
    let mut lines = input.lines();
    loop{
        let line = lines.next().unwrap();
        if line.len() == 0 { break; }
        let to = &line[0..3];
        let state = line.as_bytes()[5] as u8 -'0' as u8;
        states.insert( to.to_string(), state);
    }

    let mut code_ops: Vec<Operation> = Vec::new();
    while let Some(line) = lines.next() {
        let mut parts = line.split_whitespace();
        let src1 = parts.next().unwrap().to_string();
        let op =  parts.next().unwrap().to_string();
        let src2 = parts.next().unwrap().to_string();
        parts.next();
        let dest = parts.next().unwrap().to_string();
        code_ops.push( Operation{ src1, src2, dest, op});
    }

    states = execute( states, code_ops.clone());
    let mut result = 0u64;
    for idx in 0..100 {
        let name = "z".to_string() + &format!( "{:0>2}", idx);
        if let Some(bit) = states.get( &name ) {
            result = result  | (( *bit as u64 )<<idx);
        } else {
            break;
        }
    }

    let mut swaps: Vec<String> = Vec::new();
    loop{
        if analyze_wires( &code_ops ) {
            println!( "Clean wires");
            break;
        };
        let pair = identify_logic_errors( &code_ops ).unwrap();
        swaps.push( pair.0.clone());
        swaps.push( pair.1.clone());

        for op in &mut code_ops {
            if op.dest == pair.0 {
                println!( "Setting {} to {}", op.dest, pair.1);
                op.dest = pair.1.clone();
            } else if op.dest == pair.1 {
                println!( "Setting {} to {}", op.dest, pair.0);
                op.dest = pair.0.clone();
            }
        }
    }

    println!( "Part1: {result}" );
    swaps.sort();
    println!( "Part2: {}", swaps.join(",") );
}
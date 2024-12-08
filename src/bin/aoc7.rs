use std::fs;

fn compute1( numbers: &[u64], current_value: u64, target: u64) -> bool {
    if numbers.len() == 0 {
        return current_value == target;
    } else {
        return compute1( &numbers[1..], current_value*numbers[0], target) ||
            compute1( &numbers[1..], current_value+numbers[0], target);
    }
}

fn compute2( numbers: &[u64], current_value: u64, target: u64) -> bool {
    if numbers.len() == 0 {
        return current_value == target;
    } else {
        let concat_value = (current_value.to_string() + &numbers[0].to_string()).parse::<u64>().unwrap();
        return compute2(&numbers[1..], current_value*numbers[0], target) ||
            compute2(&numbers[1..], concat_value, target) ||
            compute2(&numbers[1..], current_value+numbers[0], target);
    }
}

fn main() {
    let input = fs::read_to_string("inputs/aoc7").unwrap();

    let mut answer1 = 0;
    let mut answer2 = 0;
    for line in input.lines() {
        let mut tokens  = line.split_whitespace();
        let target_str = tokens.next().unwrap();
        let target = (&target_str[0..target_str.len()-1]).parse::<u64>().unwrap();
        let inputs: Vec<u64> = tokens.map( |x|x.parse::<u64>().unwrap()).collect();
        if compute1( &inputs[1..], inputs[0], target ) {
            answer1 += target;
        }
        if compute2( &inputs[1..], inputs[0], target ) {
            answer2 += target;
        }
    }
    print!("Part1: {}", answer1);
    print!("Part2: {}", answer2);
}
use std::fs;

#[derive(Clone)]
struct Machine{
    a: (i64,i64),
    b: (i64,i64),
    price: (i64,i64)
}

fn parse_line( line: &str) -> (i64,i64) {
    let parts: Vec<&str> = line.split(['+', ',', '=' ] ).collect();
    (parts[1].parse::<i64>().unwrap(), parts[3].parse::<i64>().unwrap())
}

fn cost( mut machines: Vec<Machine>, offset: i64 ) -> i64 {
    let mut total_tokens = 0;
    for mut machine in machines {
        machine.price.0 += offset;
        machine.price.1 += offset;
        let c = machine.price.0 as f64 / machine.price.1 as f64;
        let r = (machine.b.1 as f64 * c - machine.b.0 as f64) / (machine.a.0 as f64 - machine.a.1 as f64 * c);

        let x_steps = machine.a.0 as f64 * r + machine.b.0 as f64;
        let num_b_steps = (machine.price.0 as f64 / x_steps).round() as i64;
        let num_a_steps = (machine.price.0 as f64 / x_steps * r).round() as i64;

        let step_pos = (machine.a.0 * num_a_steps + machine.b.0 * num_b_steps, machine.a.1 * num_a_steps + machine.b.1 * num_b_steps);
        if step_pos.0 == machine.price.0 && step_pos.1 == machine.price.1 {
            total_tokens += num_a_steps * 3 + num_b_steps;
        }
    }
    total_tokens
}

fn main() {
    let input = fs::read_to_string("inputs/aoc13").unwrap();
    let mut lines = input.lines();

    let mut machines: Vec<Machine> = Vec::new();
    loop {
        let a = parse_line(lines.next().unwrap());
        let b = parse_line(lines.next().unwrap());
        let price = parse_line(lines.next().unwrap());
        machines.push( Machine{ a, b, price } );
        if lines.next().is_none() {
            break
        }
    }

    println!( "Part1 {:?}", cost( machines.clone(), 0 ) ) ;
    println!( "Part2 {:?}", cost( machines.clone(), 10000000000000 ) ) ;
}
use std::collections::HashMap;
use std::fs;

fn count_changes( v: &Vec<char>) -> u64 {
    if v.len() < 2 {
        return 0;
    }
    let mut diff_count = 0;
    let mut iter = v.iter();
    let mut prev = iter.next().unwrap();
    while let Some(n) = iter.next() {
        if *n != *prev { diff_count += 1 }
        prev = n;
    }
    diff_count
}

fn find_best_path( from: (i32,i32), to: (i32,i32), layout: &Vec<Vec<char>>, path_so_far: Vec<char> ) -> Option<(Vec<char>)> {
    if from == to {
        return Some(path_so_far);
    }
    if path_so_far.len() > 6 {
        return None;
    }
    let mut best_solution: Option<Vec<char>> = None;
    // The order matters for getting she shortest paths
    for dir in [ '<', '^', 'v', '>' ] {
        let new_pos = match dir {
            '<' => ( from.0, from.1-1),
            '>' => ( from.0, from.1+1),
            '^' => ( from.0-1, from.1),
            'v' => ( from.0+1, from.1),
            _ => panic!( "bad dir")
        };
        if new_pos.0 < 0 || new_pos.1 < 0 || new_pos.0 >= layout.len() as i32|| new_pos.1 >= layout[0].len() as i32 {
            continue;
        }
        if layout[new_pos.0 as usize][new_pos.1 as usize] == 'X'  {
            continue;
        }
        let mut new_path = path_so_far.clone();
        new_path.push( dir );
        if let Some( solution ) = find_best_path( new_pos, to, layout, new_path) {
            if let Some( best) = best_solution {
                if solution.len() < best.len() {
                    best_solution = Some(solution);
                } else if solution.len() == best.len() {
                    // same len. favor the oe with fewer turns
                    if count_changes( &solution) < count_changes( &best ) {
                        best_solution = Some(solution);
                    } else {
                        best_solution = Some(best);
                    }
                } else {
                    best_solution = Some(best);
                }
            } else {
                best_solution = Some(solution);
            }
        }
    }
    best_solution
}

struct Keypad{
    mappings: HashMap<(char,char), Vec<char>>
}

impl Keypad {
    fn new( layout: &Vec<Vec<char>> ) -> Keypad{
        // find hte directional mappings from each to all other keys
        let mut mappings: HashMap<(char,char), Vec<char>> = HashMap::new();

        for sy in 0..layout.len() {
            for sx in 0..layout[1].len() {
                let from= layout[sy][sx];
                if from == 'X' { continue;}
                for ey in 0..layout.len() {
                    for ex in 0..layout[1].len() {
                        let to = layout[ey][ex];
                        if to == 'X' { continue;}
                        mappings.insert( (from, to), find_best_path((sy as i32,sx as i32),(ey as i32,ex as i32), layout, vec![] ).unwrap());
                    }
                }
            }
        }

        Keypad{mappings}
    }

    fn get_sequence( &self, out: &Vec<char> ) -> Vec<char> {
        let mut current = 'A';
        let mut seq: Vec<char> = Vec::new();
        for c in out {
            let mut subs_seq = self.mappings.get( &(current, *c)).unwrap().clone();
            seq.append( &mut subs_seq );
            seq.push( 'A');
            current = *c;
        }
        seq
    }

    fn get_sequence_len( &self, out: &Vec<char>, depth: usize, memo: &mut HashMap<(Vec<char>,usize), u64> ) -> u64 {
        if let Some( len ) = memo.get(&(out.clone(),depth)) {
            return *len;
        }
        if depth == 0 {
            return 1;
        }
        let mut current = 'A';
        let mut total_len= 0u64;
        for c in out {
            let mut subs_seq = self.mappings.get( &(current, *c)).unwrap().clone();
            let mut seq: Vec<char> = Vec::new();
            seq.append( &mut subs_seq );
            seq.push( 'A');
            current = *c;

            let len = self.get_sequence_len( &seq, depth-1, memo );
            total_len += len;
        }
        memo.insert( (out.clone(),depth), total_len );
        total_len
    }

}

fn main() {
    let input = fs::read_to_string( "inputs/aoc21").unwrap();
    let mut rows: Vec<Vec<char>> = Vec::new();
    for line in input.lines() {
        rows.push( Vec::from_iter( line.chars()));
    }

    let nums = vec![vec![ '7','8','9' ], vec!['4', '5', '6'],vec! [ '1', '2', '3'],vec! [ 'X', '0', 'A']];
    let numeric_pad = Keypad::new(  &nums );

    let dirs = vec![vec![ 'X','^','A' ], vec!['<', 'v', '>']];
    let dir_pad = Keypad::new(  &dirs );

    let mut total = 0;
    for row in &rows {
        let num_value_str = String::from_iter(row.iter());
        let num_value = num_value_str.get(0..3).unwrap().parse::<i64>().unwrap();
        let seq = numeric_pad.get_sequence(&row);
        let seq = dir_pad.get_sequence(&seq);
        let final_seq = dir_pad.get_sequence(&seq);
        total += num_value*final_seq.len() as i64;
    }
    println!( "Part1 {total}" );

    total = 0;
    let mut memo: HashMap<(Vec<char>,usize), u64> = HashMap::new();
    for row in rows {
        let num_value_str = String::from_iter(row.iter());
        let num_value = num_value_str.get(0..3).unwrap().parse::<i64>().unwrap();
        let mut seq = numeric_pad.get_sequence(&row);
        let len = dir_pad.get_sequence_len( &seq, 25+1, &mut memo );
        println!("{}   -   {}", num_value, len);
        total += num_value*len as i64;
    }
    println!( "Part2 {total}" );
}

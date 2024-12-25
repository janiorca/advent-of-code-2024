use std::fs;

fn main() {
    let input = fs::read_to_string( "inputs/aoc25").unwrap();

    let mut keys: Vec<[u8;7]> = Vec::new();
    let mut locks: Vec<[u8;7]> = Vec::new();
    let mut lines = input.lines();
    loop {
        let mut rows: Vec<Vec<char>> = Vec::new();
        for _t in 0..7 {
            let row: Vec<char> = Vec::from_iter( lines.next().unwrap().chars() );
            rows.push( row );
        }
        let is_lock = rows[0][0] == '#' && rows[0][1] == '#' && rows[0][2] == '#' && rows[0][3] == '#' && rows[0][4 ] == '#';
        let mut bit_shape = [0u8;7];
        for (idx,row) in rows.iter().enumerate() {
            let mut mask = 0;
            for pin in 0..5 {
                mask |= if row[pin] == '#' { 1 } else { 0 };
                mask = mask << 1;
            }
            bit_shape[ idx ] = mask>>1;
        }
        if is_lock {
            locks.push( bit_shape );
        } else {
            keys.push( bit_shape );
        }

        if lines.next().is_none()  {
            break;
        }
    }

    let mut fits = 0;
    for lock in &locks {
        for key in &keys{
            let m: u8 = lock.iter().zip( key.iter()).map( |(a,b)| *a & *b ).sum();

            if m == 0  {
                fits += 1
            }
        }
    }
    println!( "Part1 {} fits, ", fits);
}
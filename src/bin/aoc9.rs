use std::fs;

fn check_sum( disk: Vec<i64> ) -> i64 {
    let mut checksum = 0;
    for pos in 0..disk.len() as i64 {
        if disk[pos as usize ] != -1 {
            checksum += pos*disk[pos as usize];
        }
    }
    return checksum;
}

fn compact1( mut disk: Vec<i64>) -> Vec<i64> {
    let mut front = 0;
    let mut back = disk.len()-1;
    loop{
        while disk[ front ] != -1 {
            front +=1;
        }
        while disk[ back ] == -1 {
            back -=1;
        }
        if front > back {
            break;
        }
        disk.swap(front,back);
        front += 1;
        back -= 1;
    }
    return disk;
}

fn compact2( mut disk: Vec<i64>, mut file_blocks: Vec<(i64,i64)> ) -> Vec<i64> {
    file_blocks.reverse();
'outer: for block in file_blocks {
        let mut front = 0;
        // find empty space thats big enough to fit next block
        loop{
            if front + block.1 > block.0 {
                continue 'outer;
            }
            let empty = disk.as_slice()[ front as usize..].iter().position( |x|*x!=-1).unwrap();
            if empty as i64>= block.1 {
                break;
            } else {
                front+= 1;
            }
        }
        // Move the block
        for count in 0..block.1 {
            disk.swap( front as usize +count as usize, block.0 as usize +count as usize );
        }
    }
    return disk;
}

fn main() {
    let input = fs::read_to_string( "inputs/aoc9").unwrap();

    let mut file_mode = true;
    let mut disk: Vec<i64> = vec![0;0];
    let mut file_id = 0;
    let mut source_pos = 0;
    let mut file_blocks: Vec<(i64,i64)> = Vec::new();
    for c in input.chars() {
        let len = c as usize - '0' as usize;
        for _x in 0..len {
            disk.push( if file_mode { file_id } else { -1 } );
        }
        if file_mode == true {
            file_blocks.push( (source_pos, len as i64 ));
            file_id += 1;
        }
        source_pos += len as i64;
        file_mode = !file_mode;
    }
   let compacted = compact1( disk.clone());
    println!( "Part1: {}", check_sum( compacted  ));

    let compacted = compact2( disk.clone(), file_blocks);
    println!( "Part2: {}", check_sum( compacted ));
}
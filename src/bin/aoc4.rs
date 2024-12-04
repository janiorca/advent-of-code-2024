use std::fs;

fn get_word( x:i32, y:i32,map: &Vec<Vec<u8>>, offsets: &[(i32,i32)]) -> Vec<u8> {
    let width = map[0].len() as i32;
    let height = map.len() as i32 ;

    let mut word: Vec<u8> = Vec::new();
    for offset in offsets {
        let get_x = x as i32+ offset.0;
        let get_y = y as i32 + offset.1;
        if get_x >= 0 && get_x < width && get_y >= 0 && get_y < height {
            word.push(map[get_y as usize ][get_x as usize]);
        } else {
            word.push(0);
        }
    }
    return word;
}

fn part1( map: &Vec<Vec<u8>>) -> i32{
    let width = map[0].len() as i32;
    let height = map.len() as i32 ;

    let mut count = 0;
    for y in 0..height {
        for x in 0..width {
            let offset_ves = [
                [ (0i32,0i32), (1,0), (2,0), (3,0) ],
                [ (0,0), (-1,0), (-2,0), (-3,0) ],
                [ (0,0), (0,1), (0,2), (0,3) ],
                [ (0,0), (0,-1), (0,-2), (0,-3) ],
                [ (0,0), (1,1), (2,2), (3,3) ],
                [ (0,0), (-1,-1), (-2,-2), (-3,-3) ],
                [ (0,0), (1,-1), (2,-2), (3,-3) ],
                [ (0,0), (-1,1), (-2,2), (-3,3) ],
            ];
            for offsets in offset_ves {
                let word = get_word( x,y, &map, &offsets);
                if word[0] == 'X' as u8 && word[1] == 'M' as u8 && word[2] == 'A' as u8 && word[3] == 'S' as u8 {
                    count += 1;
                }
            }
        }
    }
    return count;
}

fn part2( map: &Vec<Vec<u8>>) -> i32{
    let width = map[0].len() as i32;
    let height = map.len() as i32 ;

    let mut count = 0;
    for y in 0..height {
        for x in 0..width {
            let offsets = [ (-1,-1), (0,0), (1,1), (-1,1), (0,0),( 1,-1)  ];
            let word = get_word( x,y, &map, &offsets);
            if ( ( word[0] == 'M' as u8 && word[1] == 'A' as u8 && word[2] == 'S' as u8 )|| ( word[0] == 'S' as u8 && word[1] == 'A' as u8 && word[2]  == 'M' as u8 ) ) &&
                ( ( word[3] == 'M' as u8 && word[4] == 'A' as u8 && word[5] == 'S' as u8 )|| ( word[3] == 'S' as u8 && word[4] == 'A' as u8 && word[5]  == 'M' as u8 ) )  {
                count += 1;
            }
        }
    }
    return count;
}

fn main() {
    let mut map: Vec<Vec<u8>> = Vec::new();
    let input = fs::read_to_string("inputs/aoc4").unwrap();
    for line in input.lines() {
        map.push( line.as_bytes().to_vec() );
    }

    println!( "Part1 {}", part1( &map));
    println!( "Part2 {}", part2( &map));
}
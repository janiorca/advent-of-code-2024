use std::fs;

fn find_starting_pos( map: &mut Vec<Vec<char>>) -> (i32,i32){
    // Starting pos
    let mut robo_pos = (0i32,0i32);
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if map[y][x] == '@' {
                robo_pos = (y as i32, x as i32);
                map[y][x] = '.';
            }
        }
    }
    robo_pos
}

fn make_wide_map( in_map: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut out_map = Vec::new();
    for row in in_map {
        let mut new_row: Vec<char> = Vec::new();
        for c in row {
            let mut data = match c {
                '#' => [ '#','#'],
                'O' => [ '[',']'],
                '.' => [ '.','.'],
                '@' => [ '@','.'],
                _ => panic!("Unexpected map input")
            };
            new_row.append( &mut data.to_vec() );
        }
        out_map.push( new_row );
    }
    out_map
}

fn gps_sum( map: &Vec<Vec<char>> ) -> i64 {
    let mut gps_sum = 0;
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if map[y][x] == '[' || map[y][x] == 'O' {
                gps_sum += y*100+x;
            }
        }
    }
    gps_sum as i64
}

fn move_vertical( map: &mut Vec<Vec<char>>, from: (i32,i32), dir: (i32,i32)) -> bool {
    let to = (from.0 + dir.0, from.1 + dir.1);
    let block = map[ to.0 as usize][ to.1 as usize ];
    match block {
        '#' => return false,
        '.' => {
            map[ to.0 as usize][ to.1 as usize] = map[ from.0 as usize][ from.1 as usize];
            map[ from.0 as usize][ from.1 as usize] = '.';
            return true;
        }
        'O' => {
            if move_vertical( map, to, dir ) {
                map[to.0 as usize][to.1 as usize] = map[from.0 as usize][from.1 as usize];
                map[from.0 as usize][from.1 as usize] = '.';
                return true;
            } else { return false; }

        }
        '[' | ']' => {
            if move_vertical( map, to, dir ) {
                if !move_vertical( map, (to.0, to.1+ if block == ']' { -1 } else { 1 }), dir )  {
                    return false
                }
                map[ to.0 as usize][ to.1 as usize] = map[ from.0 as usize][ from.1 as usize];
                map[ from.0 as usize][ from.1 as usize] = '.';
                return true;
            }
            return false;
        }
        _ => panic!("bad map data")
    }
}

fn process_orders( mut robo_pos: (i32,i32), orders: Vec<char>, mut map: Vec<Vec<char>>) -> i64 {
    for order in orders {
        map[ robo_pos.0 as usize][ robo_pos.1  as usize] = '@';
        map[ robo_pos.0  as usize][ robo_pos.1  as usize] = '.';

        let dir = match order {
            '<' => (0, -1),
            '>' => (0, 1),
            '^' => (-1, 0),
            'v' => (1, 0),
            _ => panic!("Unknown order")
        };

        let mut new_pos = (robo_pos.0 + dir.0, robo_pos.1 + dir.1);
        let block = map[new_pos.0 as usize][new_pos.1 as usize];
        match block {
            '#' => {},
            '.' => robo_pos = new_pos,
            '[' | ']' | 'O' => {
                if order == '^' || order == 'v' {
                    let mut changed_map = map.clone();
                    if block == 'O' {
                        if move_vertical(&mut changed_map, new_pos, dir.clone()) {
                            robo_pos = new_pos;
                            map = changed_map;
                        }
                    } else {
                        let (left, right) = if block == ']' { ((new_pos.0, new_pos.1 - 1), new_pos) } else { (new_pos, (new_pos.0, new_pos.1 + 1)) };
                        if move_vertical(&mut changed_map, left.clone(), dir.clone()) && move_vertical(&mut changed_map, right.clone(), dir.clone()) {
                            robo_pos = new_pos;
                            map = changed_map;
                        }
                    }
                } else {
                    // check forward for any gaps
                    let mut push_pos = (new_pos.0 + dir.0, new_pos.1 + dir.1);
                    let mut can_push = false;
                    loop {
                        match map[push_pos.0 as usize][push_pos.1 as usize] {
                            '.' => {
                                can_push = true;
                                break
                            }
                            '#' => break,
                            _ => { push_pos = (push_pos.0 + dir.0, push_pos.1 + dir.1) }
                        }
                    }

                    if can_push {
                        // horizontal push
                        if order == '<'  {
                            for dest_x in push_pos.1..new_pos.1 {
                                map[ push_pos.0 as usize][ dest_x  as usize] = map[ push_pos.0 as usize][(dest_x + 1) as usize]
                            }
                        } else if order == '>' {
                            for dest_x in (new_pos.1+1..=push_pos.1).rev() {
                                map[ push_pos.0 as usize][ dest_x  as usize] = map[ push_pos.0 as usize][(dest_x - 1) as usize]
                            }
                        }
                        map[new_pos.0 as usize][new_pos.1 as usize] = '.';
                        robo_pos = new_pos;
                    }
                }
            },
            _ => panic!("bad terrain")
        }
    }
    gps_sum( &map )
}

fn main() {
    let input = fs::read_to_string("inputs/aoc15").unwrap();

    let mut lines = input.lines();
    let mut map: Vec<Vec<char>> = Vec::new();
    loop {
        let line = lines.next().unwrap();
        if line == "" {
            break;
        } else {
            let row = line.chars().collect();
            map.push(row);
        }
    }
    let mut wide_map = make_wide_map(&map);

    let mut orders: Vec<char> = Vec::new();
    while let Some(line) = lines.next() {
        line.chars().for_each( |x|orders.push(x));
    }

    let robo_pos = find_starting_pos( &mut map );
    let part1 = process_orders( robo_pos.clone(), orders.clone(), map.clone() );
    println!( "Part1 {part1}");
    let robo_pos = find_starting_pos( &mut wide_map );
    let part2 = process_orders( robo_pos.clone(), orders, wide_map.clone() );
    println!( "Part2 {part2}");

}
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

fn gps_sum( map: &Vec<Vec<char>>) -> i64 {
    let mut gps_sum = 0;
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if map[y][x] == 'O' {
                gps_sum += y*100+x;
            }
        }
    }
    gps_sum as i64
}

fn process_orders( mut robo_pos: (i32,i32), orders: Vec<char>, mut map: Vec<Vec<char>>) -> i64 {
    // process orders
    for order in orders {
        let dir = match order {
            '<' => (0, -1),
            '>' => (0, 1),
            '^' => (-1, 0),
            'v' => (1, 0),
            _ => panic!("Unknown order")
        };

        let mut new_pos = (robo_pos.0 + dir.0, robo_pos.1 + dir.1);
        match map[new_pos.0 as usize][new_pos.1 as usize] {
            '#' => {},
            '.' => robo_pos = new_pos,
            'O' => {
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
                        _   => { push_pos = (push_pos.0 + dir.0, push_pos.1 + dir.1) }
                    }
                }

                if can_push {
                    map[push_pos.0 as usize][push_pos.1 as usize] = 'O';
                    map[new_pos.0 as usize][new_pos.1 as usize] = '.';
                    robo_pos = new_pos;
                }
            },
            _ => panic!("bad terrain")
        }
    }
    gps_sum( &map )
}

fn process_orders2( mut robo_pos: (i32,i32), orders: Vec<char>, mut map: Vec<Vec<char>>) -> i64 {
    // process orders
    for order in orders {
        map[ robo_pos.0 as usize][ robo_pos.1  as usize] = '@';
        for row in &map {
            let rr = row.iter().fold( "".to_string(), |mut b, x1| {b.push( *x1 ); b} );
            println!( "{rr}");
        }
        map[ robo_pos.0  as usize][ robo_pos.1  as usize] = '.';


        let dir = match order {
            '<' => (0, -1),
            '>' => (0, 1),
            '^' => (-1, 0),
            'v' => (1, 0),
            _ => panic!("Unknown order")
        };
        println!( "{order}  -  {:?}", dir);


        let mut new_pos = (robo_pos.0 + dir.0, robo_pos.1 + dir.1);
        match map[new_pos.0 as usize][new_pos.1 as usize] {
            '#' => {},
            '.' => robo_pos = new_pos,
            '[' | ']' => {
                if order == '^' || order == 'v' {
                    println!( "Vertical");
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
                            println!( "{}       {}", push_pos.1, new_pos.1 );
                            for dest_x in (push_pos.1..new_pos.1).rev() {
                                map[ push_pos.0 as usize][ dest_x  as usize] = map[ push_pos.0 as usize][(dest_x - 1) as usize]
                            }
                        }
                        else {

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
//    let input = fs::read_to_string("inputs/aoc15").unwrap();
    let input = fs::read_to_string("inputs/test").unwrap();

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


    for row in &wide_map {
        let rr = row.iter().fold( "".to_string(), |mut b, x1| {b.push( *x1 ); b} );
        println!( "{rr}");
    }

    let robo_pos = find_starting_pos( &mut wide_map );

    let part2 = process_orders2( robo_pos.clone(), orders, wide_map.clone() );
    println!( "Part2 {part2}");


}
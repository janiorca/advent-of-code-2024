use std::collections::HashSet;
use std::fs;

fn get_next_pos( map: &Vec<Vec<char>>, pos:&(i32,i32,i32) ) -> Option<(i32,i32,i32)> {
    let directions = [ (-1i32,0i32), (0,1), (1,0), (0,-1) ];
    let (height, width) = (map.len() as i32, map[0].len() as i32);
    let mut next_pos = (pos.0+directions[pos.2 as usize].0, pos.1+directions[pos.2 as usize].1, pos.2);
    if next_pos.0 >= height || next_pos.0 < 0 || next_pos.1 >= width || next_pos.1 < 0{
        return None;
    }
    if map[next_pos.0 as usize][next_pos.1 as usize] == '#' {
        next_pos = (pos.0, pos.1, (pos.2+1)%4 );
    }
    Some( next_pos )
}
fn main(){
    let mut map: Vec<Vec<char>> = Vec::new();
    let input = fs::read_to_string("inputs/aoc6").unwrap();

    for line in input.lines(){
        map.push(line.chars().collect());
    }
    let (height, width) = (map.len() as i32, map[0].len() as i32);
    let mut pos = (0i32, 0i32, 0i32);       // y, x, direction
'ut:for y in 0..height{
        for x in 0..width{
            if map[y as usize][x as usize] == '^' {
                map[y as usize][x as usize] = 'X';
                pos = (y, x, 0);
                break 'ut;
            }
        }
    }
    let start_pos = pos;
    let mut positions: HashSet<(i32,i32,i32)> = HashSet::new();
    positions.insert( pos );
    while let Some( new_pos ) = get_next_pos( &map, &pos) {
        positions.insert( new_pos );
        pos = new_pos;
    }
    let mut locs: HashSet<(i32,i32)> = HashSet::from_iter( positions.iter().map(|x|(x.0,x.1)) );
    println!("Part1: {}", locs.len());

    let mut loops = 0;
    locs.remove(&(start_pos.0,start_pos.1));
    for loc in locs {
        let mut visited_positions: HashSet<(i32,i32,i32)> = HashSet::new();
        map[loc.0 as usize][loc.1 as usize] = '#';
        pos = start_pos;
        while let Some( new_pos ) = get_next_pos( &map, &pos) {
            if visited_positions.contains( &new_pos ) {
                loops += 1;
                break;
            }
            visited_positions.insert( new_pos );
            pos = new_pos;
        }
        map[loc.0 as usize][loc.1 as usize] = '.';
    }
    println!("Part2: {}", loops);
}
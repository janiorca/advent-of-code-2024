use std::collections::{HashMap, HashSet};
use std::fs;
use crate::Direction::{East, North, South, West};

fn find( map: &Vec<Vec<char>>, c: char ) -> (i32,i32) {
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if map[y][x] == c {
                return (y as i32,x as i32);
            }
        }
    }
    panic!("Not found");
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
enum Direction{
    East,
    South,
    West,
    North
}

impl Direction{
    fn turn_cw( &self ) -> Direction {
        match self {
            East => South,
            South => West,
            West => North,
            North => East
        }
   }
    fn turn_ccw( & self ) -> Direction {
        return self.turn_cw().turn_cw().turn_cw();
    }
    fn step_vec(&self) -> (i32,i32) {
        match self {
            East => (0,1),
            South => (1,0),
            West => (0,-1),
            North => (-1,0)
        }
    }
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
struct SearchLocation {
    pos: (i32,i32),
    direction: Direction
}

fn grow_search( cost: i64, curr_loc: SearchLocation, search_path: HashSet<(i32,i32)>, map: &Vec<Vec<char>>, visited: &mut HashMap<(i32,i32),i64>,
                searches: &mut Vec<(SearchLocation,i64,HashSet<(i32,i32)>)>  ) {
    let mut dir = curr_loc.direction.turn_ccw();
    for turns in -1..=1 {
        let mut new_search_path = search_path.clone();
        if turns != 0 || cost == 0  {
            for step in 1..100{
                let new_pos = (curr_loc.pos.0 + dir.step_vec().0*step, curr_loc.pos.1 + dir.step_vec().1*step );
                new_search_path.insert( new_pos );
                let new_sl = SearchLocation{ pos: new_pos, direction: dir.clone() };
                if map[ new_pos.0 as usize][ new_pos.1 as usize] =='#' {
                    break;
                } else {
                    let new_cost = cost + (turns*1000i64).abs() + step as i64;
                    if let Some( prev ) = visited.get(&new_sl.pos) {
                        if *prev < new_cost { continue; }
                    }
                    visited.insert( new_sl.pos.clone(), new_cost );
                    searches.push( (new_sl.clone(), new_cost, new_search_path.clone()) );
                    //                shortest_path[new_pos.0 as usize][new_pos.1 as usize] = shortest_path[new_pos.0 as usize][new_pos.1 as usize].min(new_cost);
                }
            }
        }
        dir  = dir.turn_cw();
    }
}
fn  main() {
    let input = fs::read_to_string( "inputs/aoc16").unwrap();
    let mut map: Vec<Vec<char>> = Vec::new();
    for line in input.lines() {
        let row: Vec<char> = line.chars().collect();
        map.push( row );
    }

    let start = find( &map, 'S');
    let stop = find( &map, 'E');

    let mut visited: HashMap<(i32,i32),i64> = HashMap::new();
    let mut searches: Vec<(SearchLocation,i64, HashSet<(i32,i32)>)> = Vec::new();

    let mut initial_set = HashSet::new();
    initial_set.insert( start );
    searches.push( (SearchLocation{ pos: start, direction: East}, 0, initial_set));

    let mut cheapest = 1000_000_0000_000i64;
  //  shortest_path[start.0][start.1 ] = 0;

    let mut winning_set: HashSet<(i32,i32)> = HashSet::new();
    while let Some(search) = searches.pop() {
        if search.0.pos.0 == stop.0 && search.0.pos.1 == stop.1 {
            if search.1 < cheapest {
                cheapest = search.1;
                println!( "==> Picking s> Found it   cost {}   {:?}", search.1, search.0);
                winning_set = search.2;
            } else if search.1 == cheapest {
                for n in search.2 {
                    winning_set.insert(n);
                }
            }
        } else if search.1 < cheapest {
            grow_search( search.1, search.0, search.2, &map, &mut visited, &mut searches);
        }
    }
    println!( "Part1  {}", winning_set.len());

    for w in winning_set {
        map[w.0 as usize][w.1 as usize] = 'O';

    }
    for r in map{
        for c in r{
            print!("{c}")
        }
        print!("\n");
    }
}
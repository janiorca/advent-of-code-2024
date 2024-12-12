use std::collections::{HashMap};
use std::fs;

struct Region{
    fencing: i64,
    area: i64
}

fn fill( pos: (i32,i32), dest_map: &mut Vec<Vec<u32>>, src_map: &Vec<Vec<char>>, plant: char, region_id: u32, map_size: (i32,i32)) {
    if pos.0 < 0 || pos.1 < 0 || pos.0 >= map_size.0 || pos.1 >= map_size.1 {
        return;
    }
    if src_map[pos.0 as usize][pos.1 as usize] != plant {
        return;
    }
    if dest_map[pos.0 as usize][pos.1 as usize] == region_id {
        return;
    }
    dest_map[pos.0 as usize][pos.1 as usize] = region_id;
    for dir in [(0,1),(0,-1),(1,0),(-1,0)] {
        fill((pos.0 + dir.0, pos.1 + dir.1), dest_map, src_map, plant, region_id, map_size);
    }
}

fn flood( map: Vec<Vec<char>>, map_size: (i32,i32)) -> Vec<Vec<u32>> {
    let mut result = vec![ vec![0;map[0].len()];map.len()];

    let mut region_id = 0;
    for y in 0..map_size.0{
        for x in 0..map_size.1 {
            if result[y as usize][x as usize] != 0 {
                continue;
            }
            fill( (y,x), &mut result, &map, map[y as usize][x as usize], region_id, map_size );
            region_id += 1;
        }
    }
    result
}

fn main() {
    let mut map: Vec<Vec<char>> = Vec::new();
    let input = fs::read_to_string( "inputs/aoc12").unwrap();
    for line in input.lines() {
        let row: Vec<char> = line.chars().collect();
        map.push(row);
    }
    let (width,height) = (map[0].len() as i32, map.len() as i32);

    let region_map = flood( map, (height,width));
    let mut regions: HashMap<u32,Region> = HashMap::new();
    let mut region_fence_depths: HashMap<u32, Vec<HashMap<i32,Vec<i32>>>> = HashMap::new();
    for y in 0..height{
        for x in 0..width {
            let plant = region_map[y as usize][x as usize];
            let mut fences = 0;
            let fence_depths = region_fence_depths.entry(plant).or_insert(vec![HashMap::new();4]);
            for (index,dir) in [(0,1),(0,-1),(1,0),(-1,0)].iter().enumerate() {
                let nbor_pos = (y+dir.0, x+dir.1);
                if ( nbor_pos.0 >= height || nbor_pos.1 >= width || nbor_pos.0 < 0 || nbor_pos.1 < 0 ) ||
                    ( region_map[nbor_pos.0 as usize][nbor_pos.1 as usize] != plant )
                {
                    fences += 1;
                    if index < 2 {
                        fence_depths[ index ].entry( x ).or_insert(Vec::new()).push( y );
                    } else {
                        fence_depths[ index ].entry( y ).or_insert(Vec::new()).push( x );
                    }
                }
            }
            let region = regions.entry(plant).or_insert( Region{fencing:0, area:0});
            region.area += 1;
            region.fencing += fences;
        }
    }

    let mut total_price = 0;
    for region in regions.values() {
        total_price += region.fencing*region.area;
    }
    println!( "Part 1 {:?}", total_price);

    let mut total_price = 0;
    for (plant, region) in regions {
        let side_info = region_fence_depths.get(&plant).unwrap();
        let mut sides = 0;
        for side in side_info {
            for (_depth, positions) in side {
                // each group of coniguous blocks is one side
                let mut ordered = positions.clone();
                ordered.sort();
                let mut jumps = 0;
                for p in 1..ordered.len() {
                    if ordered[p] - ordered[p-1] > 1 {
                        jumps += 1;
                    }
                }
                sides += jumps+1;
            }
        }
        total_price += sides as i64 *region.area;
    }
    println!( "Part 2 {:?}", total_price);
}

use std::fs;
use lodepng::ColorType;

#[derive(Clone,Eq,PartialEq,Hash)]
struct Robot{
    pos: (i64,i64),
    velocity: (i64,i64)
}

fn analyze( robots: &Vec<Robot>, width: i64, height: i64, second: u64 ) -> bool {
    let mut image = vec![ 0u8; width as usize * height as usize ];
    for robot in robots {
        image[ (robot.pos.1 * width + robot.pos.0 ) as usize ] = 255;
    }

    // A christmas tree will have a lot of pixels grouped together
    let mut max_clustering = 0;
    for y in 0..height-5 {
        for x in 0..width-5 {
            let mut clustering = 0;
            for cy in 0..5 {
                for cx in 0..5 {
                    if image[ ((cy+y)*width+(cx+x)) as usize ] == 255 {
                        clustering += 1;
                    }
                }
            }
            max_clustering = max_clustering.max(clustering);
        }
    }
    if max_clustering >= 25 {
        let name = "image".to_string() + &second.to_string() + &".png";
        lodepng::encode_file( name, image.as_slice(), width as usize, height as usize, ColorType::GREY, 8).unwrap();
        true
    } else { false }
}
fn main() {
    let (width, height) = (101,103);
    let input = fs::read_to_string("inputs/aoc14").unwrap();

    let mut robots = Vec::new();
    for line in input.lines() {
        let parts: Vec<&str> = line.split([' ', ',', '=' ] ).collect();
        let (px,py) = ( parts[1].parse::<i64>().unwrap(), parts[2].parse::<i64>().unwrap() );
        let (vx,vy) = ( parts[4].parse::<i64>().unwrap(), parts[5].parse::<i64>().unwrap() );
        robots.push(Robot{pos: (px,py), velocity: (vx,vy)});
    }
    let original_bots = robots.clone();

    for _steps in 0..100 {
        for mut robot in &mut robots {
            robot.pos.0 = (robot.pos.0 + robot.velocity.0 + width )%width;
            robot.pos.1 = (robot.pos.1 + robot.velocity.1 + height )%height;
        }
    }

    let mut bots_in_quadrants = [ 0;4];
    for robot in &robots {
        if robot.pos.0 == width/2 || robot.pos.1 == height/2 {
            continue;
        }
        let mut q_idx = if robot.pos.0 < width/2 { 0 } else { 2 };
        q_idx += if robot.pos.1 < height/2 { 1 } else { 0 };
        bots_in_quadrants[ q_idx ] += 1;
    }

    println!( "Part 1 {}", bots_in_quadrants.iter().product::<i64>() );

    robots = original_bots.clone();
    for steps  in 1..(width*height) as u64 {
        for mut robot in &mut robots {
            robot.pos.0 = (robot.pos.0 + robot.velocity.0 + width )%width;
            robot.pos.1 = (robot.pos.1 + robot.velocity.1 + height )%height;
        }
        if analyze( &robots, width, height, steps ) {
            println!( "Part 2 {}", steps );
        };
    }
}

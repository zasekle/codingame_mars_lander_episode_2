#![allow(unused_doc_comments)]

use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: u32,
    y: u32,
}

const DIVISOR: u32 = 50;

#[derive(Debug, Clone)]
struct PathInfo {
    path: Vec<Point>,
    distance: u32,
}

#[derive(Debug, Copy, Clone)]
struct PointPair {
    start: Point,
    end: Point,
}

//TODO: may want to add the starting and stopping points
// more than one could exist tho
// they could change directions while inside the node
#[derive(Debug, Copy, Clone)]
struct MapNode {
    crossing_lines: [Option<PointPair>; 5],
    crossing_lines_idx: usize,
    contains_landing_line: bool,
    has_been_used: bool,
}

impl MapNode {
    fn new() -> MapNode {
        MapNode {
            crossing_lines: [None; 5],
            crossing_lines_idx: 0,
            contains_landing_line: false,
            has_been_used: false,
        }
    }
}

fn main() {
    //TODO:
    // need to get all the x and y coordinates
    // need to find the flat ground
    // need to find the highest y in my way
    // need to make sure I get over it
    // need to find the next highest when I get over it

    //TODO:
    // Not 100% sure how I want to set this up yet.
    // How do I get over the peaks and stuff, I know I have to calculate several moves in advance. I
    //  also know(?) that I can get the entire solution on the first move.
    // Can I calculate it backwards like a pathfinding thing? This type of breadth first search probably
    //  won't work just like the racing one didn't work.

    //TODO:
    // From an intuitive pov I can draw a line between the craft and the nearest 'safe' spot, then
    //  modify it to fit.
    // So draw the line (figure out the specific safe point last).
    //  1) Need to move the end point left or right.
    //  2) Need to take the straight line and curve it.
    //   -So a curve might be able to be laid overtop of it. But in reality it is a lot of lines.
    //   -Curve will only need to move up. Never down. For advanced cases, may need to move around a little more.
    //  3) This has a 3rd dimension which is velocity.


    //TODO:
    // Maybe I make the best case line, then follow it backwards using some kind of pathfinding?.
    //  -It seems that final velocity is still unknown, so maybe I can just use <40 instead?
    // Maybe do a depth first search following the line?

    //TODO:
    // 1) Draw the best possible line from the flat ground to the ship.
    //  -In the cave one the side of the flat ground that is `closest` is the opposite side.
    //  -Maybe if the line passes 90 degrees. move the `closest` to the other side.
    // 2) Do a depth first search on it while first moving towards the line, then moving away from it.
    //  -Need to have both thrust and rotation as parameters here, probably want some restrictions to make
    //   choosing easier.

    //#1
    // 0,100 1000,500 1500,1500 3000,1000 4000,150 5500,150 6999,800
    // 2500,2700
    // let ground_points = [
    //     Point { x: 0, y: 100 },
    //     Point { x: 1000, y: 500 },
    //     Point { x: 1500, y: 1500 },
    //     Point { x: 3000, y: 1000 },
    //     Point { x: 4000, y: 150 },
    //     Point { x: 5500, y: 150 },
    //     Point { x: 6999, y: 800 },
    // ];
    // let shuttle_point = Point {
    //     x: 2500,
    //     y: 2700,
    // };
    // let first_flat_index = 4;
    // let second_flat_index = 5;

    //#2
    // 0,100 1000,500 1500,100 3000,100 3500,500 3700,200 5000,1500 5800,300 6000,1000 6999,2000
    // 6500 2800

    //#3
    // 0,100 1000,500 1500,1500 3000,1000 4000,150 5500,150 6999,800
    // 6500 2800

    //#4
    // 0,1000 300,1500 350,1400 500,2000 800,1800 1000,2500 1200,2100 1500,2400 2000,1000 2200,500 2500,100 2900,800 3000,500 3200,1000 3500,2000 3800,800 4000,200 5000,200 5500,1500 6999,2800
    // 500 2700
    let ground_points = [
        Point { x:0, y:1000 },
        Point { x:300, y:1500 },
        Point { x:350, y:1400 },
        Point { x:500, y:2000 },
        Point { x:800, y:1800 },
        Point { x:1000,y:2500 },
        Point { x:1200,y:2100 },
        Point { x:1500,y:2400 },
        Point { x:2000,y:1000 },
        Point { x:2200,y:500 },
        Point { x:2500,y:100 },
        Point { x:2900,y:800 },
        Point { x:3000,y:500 },
        Point { x:3200,y:1000 },
        Point { x:3500,y:2000 },
        Point { x:3800,y:800 },
        Point { x:4000,y:200 },
        Point { x:5000,y:200 },
        Point { x:5500,y:1500 },
        Point { x:6999,y:2800 },
    ];
    let shuttle_point = Point {
        x: 500,
        y: 2700,
    };
    let first_flat_index = 16;
    let second_flat_index = 17;

    //#5
    // 0,1000 300,1500 350,1400 500,2100 1500,2100 2000,200 2500,500 2900,300 3000,200 3200,1000 3500,500 3800,800 4000,200 4200,800 4800,600 5000,1200 5500,900 6000,500 6500,300 6999,500
    // 6500 2700

    //Dummy points
    // 0,1000 3000,2000 4000,300 6999,300
    // let ground_points = [
    //     Point { x: 0, y: 1000 },
    //     Point { x: 3000, y: 2000 },
    //     Point { x: 4000, y: 300 },
    //     Point { x: 6999, y: 300 },
    // ];
    // let shuttle_point = Point {
    //     x: 2000,
    //     y: 2500,
    // };
    // let first_flat_index = 2;
    // let second_flat_index = 3;

    //TODO: Finishing up the line
    // Maybe 'move' the ground out a little bit (or modify something to make it work)
    // Find the lines that connect my 'path'
    // Still need to choose the shortest distance line.

    //TODO: There are at least 2 things wrong with this
    // 1) The diagonal lines are not catching the intersections
    // 2) Still the problem that it isn't working on setting of 200
    // 3) It is filling the entire map with 'used' even though it seems to be getting the correct
    //  answer.
    //  -Probably make sure below tDO is not relevant anywhere else (it could be causing this)
    //  -Probably refactor function below a little, might be easier to work with
    let mut final_paths = calculate_line(
        &ground_points,
        first_flat_index,
        second_flat_index,
        &shuttle_point,
    );

    final_paths.sort_by(|a, b| a.distance.partial_cmp(&b.distance).unwrap());

    for path_info in final_paths.iter() {
        let mut s = String::new();
        for point in path_info.path.iter() {
            if !s.is_empty() {
                s.push(' ');
            }
            s += point.x.to_string().as_str();
            s.push(',');
            s += point.y.to_string().as_str();
        }

        println!("{s}");
        // println!("distance: {}", path_info.distance);
    }
    println!("finalPaths.len: {}", final_paths.len());
    // println!("finalPaths: {:?}", final_paths);
}

//TODO: probably clean this up a TAD bit
// calculating the equations for the ground points (m & b) before hand would be nice
fn calculate_line(
    ground_points: &[Point],
    flat_surface_first_index: usize,
    flat_surface_second_index: usize,
    shuttle_point: &Point,
) -> Vec<PathInfo> {
    //Breadth first search, then find the shortest path

    let mut map = [[MapNode::new(); (7000 / DIVISOR) as usize]; (3000 / DIVISOR) as usize];

    //Iterate through all lines and save them to their respective map nodes.
    for i in 1..ground_points.len() {
        let start_point = &ground_points[i - 1];
        let end_point = &ground_points[i];

        //y1 = m * x1 + b
        //y2 = m * x2 + b
        //m = (y2-y1)/(x2-x1)
        //b = y1-m*x1

        let start_point_x_f = start_point.x as f32;
        let start_point_y_f = start_point.y as f32;
        let end_point_x_f = end_point.x as f32;
        let end_point_y_f = end_point.y as f32;

        let m = (end_point_y_f - start_point_y_f) / (end_point_x_f - start_point_x_f);
        let b = start_point_y_f - m * start_point_x_f;

        let start_x = start_point.x / DIVISOR;

        //TODO: Because the end point can hit the VERY EDGE of the line, an extra entire
        // square can be added.
        // Is this the best way to handle it?
        // Is this problem relevant anywhere else?
        let end_x = if end_point.x % DIVISOR == 0 {
            (end_point.x - 1) / DIVISOR
        } else {
            end_point.x / DIVISOR
        };

        println!("x range {start_x}..={end_x}");
        //The starting point here is a mirror of the end point of the last
        // loop. This has to be done in order to make sure both lines are added to crossing_lines
        // member.
        for x in start_x..=end_x {
            let mut y_begin =
                if x == start_point.x {
                    start_point.y
                } else {
                    (m * ((x * DIVISOR) as f32) + b) as u32
                };

            let mut y_end =
                if x == end_point.x {
                    end_point.y
                } else {
                    (m * (((x + 1) * DIVISOR) as f32) + b) as u32
                };

            println!("start_point {:?} end_point {:?}", start_point, end_point);
            println!("m {m} b {b} x {x}");
            y_begin /= DIVISOR;
            y_end /= DIVISOR;

            let range = if y_begin <= y_end {
                println!("y range {y_begin}..={y_end}");
                (y_begin..=y_end).collect::<Vec<_>>()
            } else {
                println!("y range {y_end}..={y_begin}");
                (y_end..=y_begin).rev().collect::<Vec<_>>()
            };

            for y in range {
                let mut map_ele = &mut map[y as usize][x as usize];

                map_ele.crossing_lines[map_ele.crossing_lines_idx] =
                    Some(
                        PointPair {
                            start: start_point.clone(),
                            end: end_point.clone(),
                        }
                    );

                map_ele.crossing_lines_idx += 1;

                if i - 1 == flat_surface_first_index && i == flat_surface_second_index {
                    map_ele.contains_landing_line = true;
                }
            }
        }
    }

    for row in map.iter().rev() {
        let mut string = String::new();
        for ele in row.iter() {
            if ele.contains_landing_line {
                string.push('#');
            } else if ele.crossing_lines_idx > 0 {
                string.push('0');
            } else {
                string.push('.');
            }
        }
        println!("{string}");
    }

    let mut paths = Vec::<PathInfo>::new();

    /** For now assuming first points cannot have any lines to intersect with. **/

    let normalized_shuttle_point = Point {
        x: shuttle_point.x - shuttle_point.x % DIVISOR,
        y: shuttle_point.y - shuttle_point.y % DIVISOR,
    };

    let first_distance = calculate_dist_for_two_points(
        normalized_shuttle_point.y,
        shuttle_point.y,
        normalized_shuttle_point.x,
        shuttle_point.x,
    );

    map[(normalized_shuttle_point.y / DIVISOR) as usize][(normalized_shuttle_point.x / DIVISOR) as usize].has_been_used = true;

    paths.push(
        PathInfo {
            path: Vec::from([shuttle_point.clone(), normalized_shuttle_point]),
            distance: first_distance,
        },
    );

    if shuttle_point.x + DIVISOR <= 6999 {
        let second_point = Point {
            x: normalized_shuttle_point.x + DIVISOR,
            y: normalized_shuttle_point.y,
        };

        let second_distance = calculate_dist_for_two_points(
            second_point.y,
            shuttle_point.y,
            second_point.x,
            shuttle_point.x,
        );

        map[(second_point.y / DIVISOR) as usize][(second_point.x / DIVISOR) as usize].has_been_used = true;

        paths.push(
            PathInfo {
                path: Vec::from([shuttle_point.clone(), second_point]),
                distance: second_distance,
            }
        );
    }

    let mut final_paths = Vec::<PathInfo>::new();
    /**
      could probably use diagonals, but there are some issues
      would probably need to find the shortest path to a point
      ./|.
      .\|.
      for example the above two paths are not equal, its b/c the lines are different lengths
    **/
    while !paths.is_empty() {
        // println!("\n\n\n{:?}", paths);
        // for row in map.iter().rev() {
        //     let mut string = String::new();
        //     for ele in row.iter() {
        //         if ele.has_been_used {
        //             string.push('L');
        //         } else {
        //             string.push('.');
        //         }
        //     }
        //     println!("{string}");
        // }
        let paths_copy = paths;
        let mut temp_paths = HashMap::<Point, PathInfo>::new();

        paths = Vec::<PathInfo>::new();

        for path in paths_copy {
            let final_x = path.path.last().expect("path empty").x;
            let final_y = path.path.last().expect("path empty").y;
            // println!("x {final_x} y {final_y}");
            if final_y >= DIVISOR { //down
                // println!("down");

                let next_y = final_y - DIVISOR;
                let next_element = &mut map[(next_y / DIVISOR) as usize][(final_x / DIVISOR) as usize];

                if !next_element.has_been_used
                    // || next_element.contains_landing_line
                {
                    let mut path_ended = false;
                    for i in 0..next_element.crossing_lines_idx {
                        let point_pair = next_element.crossing_lines[i].expect("invalid crossing idx {i}");

                        let start_point_x_f = point_pair.start.x as f32;
                        let start_point_y_f = point_pair.start.y as f32;
                        let end_point_x_f = point_pair.end.x as f32;
                        let end_point_y_f = point_pair.end.y as f32;

                        let m = (end_point_y_f - start_point_y_f) / (end_point_x_f - start_point_x_f);
                        let b = start_point_y_f - m * start_point_x_f;
                        let x_line = final_x as f32;

                        let y_intersection = (m * x_line + b) as u32;

                        if (y_intersection / DIVISOR) == (next_y / DIVISOR) {

                            path_ended = true;

                            if point_pair.start == ground_points[flat_surface_first_index]
                                && point_pair.end == ground_points[flat_surface_second_index] {
                                let mut path_clone = path.clone();
                                let path_last_val = path_clone.path.last().expect("path empty");

                                path_clone.distance += calculate_dist_for_two_points(
                                    path_last_val.y,
                                    y_intersection,
                                    path_last_val.x,
                                    final_x,
                                );

                                path_clone.path.push(
                                    Point {
                                        x: final_x,
                                        y: y_intersection,
                                    }
                                );

                                final_paths.push(
                                    path_clone
                                );
                            }
                        }
                    }

                    if !path_ended {
                        let mut path_clone = path.clone();
                        let path_last_val = path_clone.path.last().expect("path empty");

                        path_clone.distance += calculate_dist_for_two_points(
                            path_last_val.y,
                            next_y,
                            path_last_val.x,
                            final_x,
                        );

                        let next_point = Point {
                            x: final_x,
                            y: next_y,
                        };

                        let prev_value = temp_paths.get(&next_point);

                        if prev_value.is_none() || prev_value.unwrap().distance > path_clone.distance {
                            // println!("storing x {final_x} y {next_y}");
                            path_clone.path.push(next_point);
                            temp_paths.insert(next_point, path_clone);
                        }
                    }
                }
            }
/*
            if final_x >= DIVISOR && final_y >= DIVISOR { //down-left
                println!("down-left");

                let next_x = final_x - DIVISOR;
                let next_y = final_y - DIVISOR;
                let next_element = &mut map[(next_y / DIVISOR) as usize][(next_x / DIVISOR) as usize];
                if !next_element.has_been_used
                    // || next_element.contains_landing_line
                {
                    let mut path_ended = false;
                    for i in 0..next_element.crossing_lines_idx {
                        let point_pair = next_element.crossing_lines[i].expect("invalid crossing idx {i}");

                        let start_point_x_f = point_pair.start.x as f32;
                        let start_point_y_f = point_pair.start.y as f32;
                        let end_point_x_f = point_pair.end.x as f32;
                        let end_point_y_f = point_pair.end.y as f32;

                        let m = (end_point_y_f - start_point_y_f) / (end_point_x_f - start_point_x_f);
                        let b = start_point_y_f - m * start_point_x_f;

                        let diagonal_m = 1 as f32;
                        let diagonal_b = (next_y as f32) - diagonal_m * (next_x as f32);

                        // y = m1*x + b1
                        // y = m2*x + b2
                        // 0 = x(m1-m2) + b1 - b2
                        // (b2 - b1)/(m1-m2) = x

                        let x_intersection = (diagonal_b - b) / (m - diagonal_m);
                        let y_intersection = (m * x_intersection + b) as u32;

                        let x_intersection = x_intersection as u32;

                        // println!("m {m} diagonal_m {diagonal_m} diagonal_b {diagonal_b} b {b} next_x {next_x} x_intersection {x_intersection} next_y {next_y} y_intersection {y_intersection}");
                        if (x_intersection / DIVISOR) == (next_x / DIVISOR)
                            && (y_intersection / DIVISOR) == (next_y / DIVISOR)
                        {
                            path_ended = true;

                            if point_pair.start == ground_points[flat_surface_first_index]
                                && point_pair.end == ground_points[flat_surface_second_index] {
                                let mut path_clone = path.clone();
                                let path_last_val = path_clone.path.last().expect("path empty");

                                path_clone.distance += calculate_dist_for_two_points(
                                    path_last_val.y,
                                    y_intersection,
                                    path_last_val.x,
                                    x_intersection,
                                );

                                path_clone.path.push(
                                    Point {
                                        x: x_intersection,
                                        y: y_intersection,
                                    }
                                );

                                final_paths.push(
                                    path_clone
                                );
                            }
                        }
                    }

                    println!("path_ended {path_ended}");
                    if !path_ended {
                        let mut path_clone = path.clone();
                        let path_last_val = path_clone.path.last().expect("path empty");

                        path_clone.distance += calculate_dist_for_two_points(
                            path_last_val.y,
                            next_y,
                            path_last_val.x,
                            next_x,
                        );

                        let next_point = Point {
                            x: next_x,
                            y: next_y,
                        };

                        let prev_value = temp_paths.get(&next_point);

                        if prev_value.is_none() || prev_value.unwrap().distance > path_clone.distance {
                            path_clone.path.push(next_point);
                            temp_paths.insert(next_point, path_clone);
                        }
                    }
                }
            }

            if final_x + DIVISOR <= 6999 && final_y >= DIVISOR { //down-right

                println!("down-right");
                let next_x = final_x + DIVISOR;
                let next_y = final_y - DIVISOR;
                // println!("down-right {next_x},{next_y}");
                let next_element = &mut map[(next_y / DIVISOR) as usize][(next_x / DIVISOR) as usize];
                if !next_element.has_been_used
                    // || next_element.contains_landing_line
                {
                    let mut path_ended = false;
                    for i in 0..next_element.crossing_lines_idx {
                        let point_pair = next_element.crossing_lines[i].expect("invalid crossing idx {i}");

                        let start_point_x_f = point_pair.start.x as f32;
                        let start_point_y_f = point_pair.start.y as f32;
                        let end_point_x_f = point_pair.end.x as f32;
                        let end_point_y_f = point_pair.end.y as f32;

                        let m = (end_point_y_f - start_point_y_f) / (end_point_x_f - start_point_x_f);
                        let b = start_point_y_f - m * start_point_x_f;

                        let diagonal_m = -1 as f32;
                        let diagonal_b = (next_y as f32) - diagonal_m * (next_x as f32);

                        // y = m1*x + b1
                        // y = m2*x + b2
                        // 0 = x(m1-m2) + b1 - b2
                        // (b2 - b1)/(m1-m2) = x

                        let x_intersection = (diagonal_b - b) / (m - diagonal_m);
                        let y_intersection = (m * x_intersection + b) as u32;

                        //2600,2100 -> 2800,1900
                        println!("inter down-right {next_x},{next_y}");
                        if final_x == 2600 && final_y == 2100 {
                            println!("found next_x {next_x} next_y {next_y}");
                            println!("found x_intersection {x_intersection} y_intersection {y_intersection}");
                        }

                        let x_intersection = x_intersection as u32;

                        // println!("m {m} diagonal_m {diagonal_m} diagonal_b {diagonal_b} b {b} next_x {next_x} x_intersection {x_intersection} next_y {next_y} y_intersection {y_intersection}");
                        if (x_intersection / DIVISOR) == (next_x / DIVISOR)
                            && (y_intersection / DIVISOR) == (next_y / DIVISOR)
                        {
                            path_ended = true;

                            if point_pair.start == ground_points[flat_surface_first_index]
                                && point_pair.end == ground_points[flat_surface_second_index] {
                                let mut path_clone = path.clone();
                                let path_last_val = path_clone.path.last().expect("path empty");

                                path_clone.distance += calculate_dist_for_two_points(
                                    path_last_val.y,
                                    y_intersection,
                                    path_last_val.x,
                                    x_intersection,
                                );

                                path_clone.path.push(
                                    Point {
                                        x: x_intersection,
                                        y: y_intersection,
                                    }
                                );

                                final_paths.push(
                                    path_clone
                                );
                            }
                        }
                    }

                    println!("path_ended {path_ended}");
                    if !path_ended {
                        let mut path_clone = path.clone();
                        let path_last_val = path_clone.path.last().expect("path empty");

                        path_clone.distance += calculate_dist_for_two_points(
                            path_last_val.y,
                            next_y,
                            path_last_val.x,
                            next_x,
                        );

                        let next_point = Point {
                            x: next_x,
                            y: next_y,
                        };

                        let prev_value = temp_paths.get(&next_point);

                        if prev_value.is_none() || prev_value.unwrap().distance > path_clone.distance {
                            path_clone.path.push(next_point);
                            temp_paths.insert(next_point, path_clone);
                        }
                    }
                }
            }
*/

            if final_x >= DIVISOR { //left
                // println!("left");

                let next_x = final_x - DIVISOR;
                let next_element = &mut map[(final_y / DIVISOR) as usize][(next_x / DIVISOR) as usize];
                if !next_element.has_been_used
                    // || next_element.contains_landing_line
                {
                    let mut path_ended = false;
                    for i in 0..next_element.crossing_lines_idx {
                        let point_pair = next_element.crossing_lines[i].expect("invalid crossing idx {i}");

                        let start_point_x_f = point_pair.start.x as f32;
                        let start_point_y_f = point_pair.start.y as f32;
                        let end_point_x_f = point_pair.end.x as f32;
                        let end_point_y_f = point_pair.end.y as f32;

                        let m = (end_point_y_f - start_point_y_f) / (end_point_x_f - start_point_x_f);
                        let b = start_point_y_f - m * start_point_x_f;
                        let y_line = final_y as f32;

                        let x_intersection = ((y_line - b) / m) as u32;

                        if (x_intersection / DIVISOR) == (next_x / DIVISOR) {
                            path_ended = true;

                            if point_pair.start == ground_points[flat_surface_first_index]
                                && point_pair.end == ground_points[flat_surface_second_index] {
                                let mut path_clone = path.clone();
                                let path_last_val = path_clone.path.last().expect("path empty");

                                path_clone.distance += calculate_dist_for_two_points(
                                    path_last_val.y,
                                    final_y,
                                    path_last_val.x,
                                    x_intersection,
                                );

                                path_clone.path.push(
                                    Point {
                                        x: x_intersection,
                                        y: final_y,
                                    }
                                );

                                final_paths.push(
                                    path_clone
                                );
                            }
                        }
                    }

                    if !path_ended {
                        let mut path_clone = path.clone();
                        let path_last_val = path_clone.path.last().expect("path empty");

                        path_clone.distance += calculate_dist_for_two_points(
                            path_last_val.y,
                            final_y,
                            path_last_val.x,
                            next_x,
                        );

                        let next_point = Point {
                            x: next_x,
                            y: final_y,
                        };

                        let prev_value = temp_paths.get(&next_point);

                        if prev_value.is_none() || prev_value.unwrap().distance > path_clone.distance {
                            // println!("storing x {next_x} y {final_y}");
                            path_clone.path.push(next_point);
                            temp_paths.insert(next_point, path_clone);
                        }
                    }
                }
            }

            if final_x + DIVISOR <= 6999 { //right
                // println!("right");
                let next_x = final_x + DIVISOR;
                let next_element = &mut map[(final_y / DIVISOR) as usize][(next_x / DIVISOR) as usize];
                if !next_element.has_been_used
                    // || next_element.contains_landing_line
                {
                    let mut path_ended = false;
                    for i in 0..next_element.crossing_lines_idx {
                        let point_pair = next_element.crossing_lines[i].expect("invalid crossing idx {i}");

                        let start_point_x_f = point_pair.start.x as f32;
                        let start_point_y_f = point_pair.start.y as f32;
                        let end_point_x_f = point_pair.end.x as f32;
                        let end_point_y_f = point_pair.end.y as f32;

                        let m = (end_point_y_f - start_point_y_f) / (end_point_x_f - start_point_x_f);
                        let b = start_point_y_f - m * start_point_x_f;
                        let y_line = final_y as f32;

                        let x_intersection = ((y_line - b) / m) as u32;

                        if (x_intersection / DIVISOR) == (next_x / DIVISOR) {
                            path_ended = true;

                            if point_pair.start == ground_points[flat_surface_first_index]
                                && point_pair.end == ground_points[flat_surface_second_index] {
                                let mut path_clone = path.clone();
                                let path_last_val = path_clone.path.last().expect("path empty");

                                path_clone.distance += calculate_dist_for_two_points(
                                    path_last_val.y,
                                    final_y,
                                    path_last_val.x,
                                    x_intersection,
                                );

                                path_clone.path.push(
                                    Point {
                                        x: x_intersection,
                                        y: final_y,
                                    }
                                );

                                final_paths.push(
                                    path_clone
                                );
                            }
                        }
                    }

                    if !path_ended {
                        let mut path_clone = path.clone();
                        let path_last_val = path_clone.path.last().expect("path empty");

                        path_clone.distance += calculate_dist_for_two_points(
                            path_last_val.y,
                            final_y,
                            path_last_val.x,
                            next_x,
                        );

                        let next_point = Point {
                            x: next_x,
                            y: final_y,
                        };

                        let prev_value = temp_paths.get(&next_point);

                        if prev_value.is_none() || prev_value.unwrap().distance > path_clone.distance {
                            // println!("storing x {next_x} y {final_y}");
                            path_clone.path.push(next_point);
                            temp_paths.insert(next_point, path_clone);
                        }
                    }
                }
            }
        }

        for path in temp_paths {
            let next_element = &mut map[(path.0.y / DIVISOR) as usize][(path.0.x / DIVISOR) as usize];
            next_element.has_been_used = true;
            paths.push(path.1);
        }

    }

    for row in map.iter().rev() {
        let mut string = String::new();
        for ele in row.iter() {
            if ele.has_been_used {
                string.push('U');
            } else {
                string.push('.');
            }
        }
        println!("{string}");
    }

    final_paths
}

fn calculate_dist_for_two_points(
    y1: u32,
    y2: u32,
    x1: u32,
    x2: u32,
) -> u32 {
    ((y2 as i32 - y1 as i32).pow(2) + (x2 as i32 - x1 as i32).pow(2)) as u32
}
#![allow(unused_doc_comments)]

use std::collections::HashMap;

//The size of the squares the the map is divided into when finding the shortest path.
const SIZE_OF_SQUARES: u32 = 50;

//Amount of distance to take into consideration between shuttle and ground for shortest path.
const AMOUNT_OF_LEEWAY: f64 = 100.0;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: u32,
    y: u32,
}

//TODO: delete me
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Pointz {
    x: i32,
    y: i32,
}

impl Pointz {
    fn new(&point: &Point) -> Pointz {
        Pointz {
            x: point.x as i32,
            y: point.y as i32,
        }
    }
}

//TODO: delete me
#[derive(Debug, Copy, Clone)]
struct PointPairz {
    start: Pointz,
    end: Pointz,
}

#[derive(Debug, Copy, Clone)]
struct LineEquation {
    m: f64,
    b: f64,
}

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

    //#1
    // 0,100 1000,500 1500,1500 3000,1000 4000,150 5500,150 6999,800
    // 2500,2700
    // let ground_points = [
    //     Some(Point { x: 0, y: 100 }),
    //     Some(Point { x: 1000, y: 500 }),
    //     Some(Point { x: 1500, y: 1500 }),
    //     Some(Point { x: 3000, y: 1000 }),
    //     Some(Point { x: 4000, y: 150 }),
    //     Some(Point { x: 5500, y: 150 }),
    //     Some(Point { x: 6999, y: 800 }),
    // ];
    // let shuttle_point = Point {
    //     x: 2500,
    //     y: 2700,
    // };
    // let first_flat_index = 4;
    // let second_flat_index = 5;
    // let ground_points_size = 7;

    //#2
    // 0,100 1000,500 1500,100 3000,100 3500,500 3700,200 5000,1500 5800,300 6000,1000 6999,2000
    // 6500 2800

    //#3
    // 0,100 1000,500 1500,1500 3000,1000 4000,150 5500,150 6999,800
    // 6500 2800

    //#4
    // 0,1000 300,1500 350,1400 500,2000 800,1800 1000,2500 1200,2100 1500,2400 2000,1000 2200,500 2500,100 2900,800 3000,500 3200,1000 3500,2000 3800,800 4000,200 5000,200 5500,1500 6999,2800
    // 500 2700
    // let ground_points = [
    //     Some(Point { x: 0, y: 1000 }),
    //     Some(Point { x: 300, y: 1500 }),
    //     Some(Point { x: 350, y: 1400 }),
    //     Some(Point { x: 500, y: 2000 }),
    //     Some(Point { x: 800, y: 1800 }),
    //     Some(Point { x: 1000, y: 2500 }),
    //     Some(Point { x: 1200, y: 2100 }),
    //     Some(Point { x: 1500, y: 2400 }),
    //     Some(Point { x: 2000, y: 1000 }),
    //     Some(Point { x: 2200, y: 500 }),
    //     Some(Point { x: 2500, y: 100 }),
    //     Some(Point { x: 2900, y: 800 }),
    //     Some(Point { x: 3000, y: 500 }),
    //     Some(Point { x: 3200, y: 1000 }),
    //     Some(Point { x: 3500, y: 2000 }),
    //     Some(Point { x: 3800, y: 800 }),
    //     Some(Point { x: 4000, y: 200 }),
    //     Some(Point { x: 5000, y: 200 }),
    //     Some(Point { x: 5500, y: 1500 }),
    //     Some(Point { x: 6999, y: 2800 }),
    // ];
    // let shuttle_point = Point {
    //     x: 500,
    //     y: 2700,
    // };
    // let first_flat_index = 16;
    // let second_flat_index = 17;
    // let ground_points_size = 20;

    //#5
    // 0,1000 300,1500 350,1400 500,2100 1500,2100 2000,200 2500,500 2900,300 3000,200 3200,1000 3500,500 3800,800 4000,200 4200,800 4800,600 5000,1200 5500,900 6000,500 6500,300 6999,500
    // 6500 2700

    //#1 Episode 3
    // 0,450 300,750 1000,450 1500,650 1800,850 2000,1950 2200,1850 2400,2000 3100,1800 3150,1550 2500,1600 2200,1550 2100,750 2200,150 3200,150 3500,450 4000,950 4500,1450 5000,1550 5500,1500 6000,950 6999,1750
    // 6500 2600
    // let ground_points = [
    //     Some(Point { x: 0, y: 450 }),
    //     Some(Point { x: 300, y: 750 }),
    //     Some(Point { x: 1000, y: 450 }),
    //     Some(Point { x: 1500, y: 650 }),
    //     Some(Point { x: 1800, y: 850 }),
    //     Some(Point { x: 2000, y: 1950 }),
    //     Some(Point { x: 2200, y: 1850 }),
    //     Some(Point { x: 2400, y: 2000 }),
    //     Some(Point { x: 3100, y: 1800 }),
    //     Some(Point { x: 3150, y: 1550 }),
    //     Some(Point { x: 2500, y: 1600 }),
    //     Some(Point { x: 2200, y: 1550 }),
    //     Some(Point { x: 2100, y: 750 }),
    //     Some(Point { x: 2200, y: 150 }),
    //     Some(Point { x: 3200, y: 150 }),
    //     Some(Point { x: 3500, y: 450 }),
    //     Some(Point { x: 4000, y: 950 }),
    //     Some(Point { x: 4500, y: 1450 }),
    //     Some(Point { x: 5000, y: 1550 }),
    //     Some(Point { x: 5500, y: 1500 }),
    //     Some(Point { x: 6000, y: 950 }),
    //     Some(Point { x: 6999, y: 1750 }),
    // ];
    // let shuttle_point = Point {
    //     x: 6500,
    //     y: 2600,
    // };
    // let first_flat_index = 13;
    // let second_flat_index = 14;
    // let ground_points_size = 22;

    //#2 Episode 3
    // 0,1800 300,1200 1000,1550 2000,1200 2500,1650 3700,220 4700,220 4750,1000 4700,1650 4000,1700 3700,1600 3750,1900 4000,2100 4900,2050 5100,1000 5500,500 6200,800 6999,600
    // 6500 2000
    let ground_points = [
        Some(Point { x: 0, y: 1800 }),
        Some(Point { x: 300, y: 1200 }),
        Some(Point { x: 1000, y: 1550 }),
        Some(Point { x: 2000, y: 1200 }),
        Some(Point { x: 2500, y: 1650 }),
        Some(Point { x: 3700, y: 220 }),
        Some(Point { x: 4700, y: 220 }),
        Some(Point { x: 4750, y: 1000 }),
        Some(Point { x: 4700, y: 1650 }),
        Some(Point { x: 4000, y: 1700 }),
        Some(Point { x: 3700, y: 1600 }),
        Some(Point { x: 3750, y: 1900 }),
        Some(Point { x: 4000, y: 2100 }),
        Some(Point { x: 4900, y: 2050 }),
        Some(Point { x: 5100, y: 1000 }),
        Some(Point { x: 5500, y: 500 }),
        Some(Point { x: 6200, y: 800 }),
        Some(Point { x: 6999, y: 600 }),
    ];
    let shuttle_point = Point {
        x: 6500,
        y: 2000,
    };
    let first_flat_index = 5;
    let second_flat_index = 6;
    let ground_points_size = 18;

    //Dummy points
    // 0,1000 3000,2000 4000,300 6999,300
    // 2000 2500
    //  let ground_points = [
    //     Some(Point { x: 0, y: 1000 }),
    //     Some(Point { x: 3000, y: 2000 }),
    //     Some(Point { x: 4000, y: 300 }),
    //     Some(Point { x: 6999, y: 300 }),
    // ];
    // let shuttle_point = Point {
    //     x: 2000,
    //     y: 2500,
    // };
    // let first_flat_index = 2;
    // let second_flat_index = 3;
    // let ground_points_size = 4;

    //TODO:
    // 1) Draw the best possible line from the flat ground to the ship.
    //  -In the cave one the side of the flat ground that is `closest` is the opposite side.
    //  -Maybe if the line passes 90 degrees. move the `closest` to the other side.
    // 2) Do a depth first search on it while first moving towards the line, then moving away from it.
    //  -Need to have both thrust and rotation as parameters here, probably want some restrictions to make
    //   choosing easier.

    //TODO: Want to iterate backwards through it and check if any of them intercept with leeway ground_points, make
    // the final lines as straight as possible. Less lines is less intercepts to check for although it doesn't matter
    // that much I don't think.
    //TODO: Clean up my structs above, don't need 2 types of 'Point' structs (or do I?) certainly don't need
    // PointPairz.
    //TODO: Clean up the functions I made so I can read them later.
    //TODO: Clean up the TODO stuff.
    let leeway_return_values = give_leeway_for_ground(
        &ground_points,
        ground_points_size,
        first_flat_index,
        second_flat_index,
    );

    //TODO: need to make sure that the index variables have not changed (maybe return them from above)
    println!("adjusted_ground_points");
    for i in 0..leeway_return_values.adjusted_ground_points_size {
        print!("{},{} ", leeway_return_values.adjusted_ground_points[i].unwrap().x, leeway_return_values.adjusted_ground_points[i].unwrap().y);
    }
    println!();

    let mut final_paths = calculate_line(
        &leeway_return_values.adjusted_ground_points,
        leeway_return_values.adjusted_ground_points_size,
        leeway_return_values.flat_surface_first_index,
        leeway_return_values.flat_surface_second_index,
        &shuttle_point,
    );

    final_paths.sort_by(|a, b| a.distance.partial_cmp(&b.distance).unwrap());

    //TODO: Finishing up the line
    // Choose the shortest distance line.

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

//TODO: move this out of the way
fn get_equation_of_line(
    start: &Pointz,
    end: &Pointz,
) -> LineEquation {
    let start_point_x_f = start.x as f64;
    let start_point_y_f = start.y as f64;
    let end_point_x_f = end.x as f64;
    let end_point_y_f = end.y as f64;

    let m = (end_point_y_f - start_point_y_f) / (end_point_x_f - start_point_x_f);
    let b = start_point_y_f - m * start_point_x_f;

    LineEquation { m, b }
}

struct LeewayReturnValues {
    flat_surface_first_index: usize,
    flat_surface_second_index: usize,
    adjusted_ground_points: [Option<Point>; 30],
    adjusted_ground_points_size: usize,
}

fn give_leeway_for_ground(
    ground_points: &[Option<Point>],
    ground_points_size: usize,
    flat_surface_first_index: usize,
    flat_surface_second_index: usize,
) -> LeewayReturnValues {
    let mut adjusted_ground_points: [Option<Point>; 30] = [None; 30];
    let mut current_index = 0;
    let mut new_first_flat_surface_index = flat_surface_first_index;
    let mut new_second_flat_surface_index = flat_surface_second_index;

    let mut previous_line = LineEquation {
        m: 0.0,
        b: 0.0,
    };
    for i in 1..ground_points_size {
        let start_point = &ground_points[i - 1].unwrap();
        let end_point = &ground_points[i].unwrap();

        //y1 = m * x1 + b
        //y2 = m * x2 + b
        //m = (y2-y1)/(x2-x1)
        //b = y1-m*x1
        let original_line_equation = get_equation_of_line(
            &Pointz::new(start_point),
            &Pointz::new(end_point),
        );

        let perpendicular_m = -1.0 / original_line_equation.m;

        // next_m = Dy/Dx;
        // d = sqrt((Dy)^2 + (Dx)^2);
        // d = sqrt(Dx^2 + Dx^2 * next_m^2)
        // d = sqrt(Dx^2 * (1 + next_m^2))
        // d^2/(1 + next_m^2) = Dx^2
        // +/- sqrt(d^2/(1 + next_m^2)) = Dx
        // +/- d/sqrt((1 + next_m^2)) = Dx

        let (delta_x, delta_y) =
            if i - 1 == flat_surface_first_index
                && i == flat_surface_second_index {
                new_first_flat_surface_index = i - 1;
                new_second_flat_surface_index = i;
                (0.0, 0.0)
            } else {
                let delta_x = AMOUNT_OF_LEEWAY / (perpendicular_m * perpendicular_m + 1.0).sqrt();
                let delta_y = perpendicular_m * delta_x;
                (delta_x, delta_y)
            };

        let mut multiplier =
            if original_line_equation.m > 0.0 {
                -1.0
            } else {
                1.0
            };

        if start_point.x > end_point.x {
            multiplier *= -1.0;
        }

        let start_x = start_point.x as f64 + (multiplier * delta_x);
        let start_y = start_point.y as f64 + (multiplier * delta_y);
        let end_x = end_point.x as f64 + (multiplier * delta_x);
        let end_y = end_point.y as f64 + (multiplier * delta_y);

        //Original line slope has not changed.
        let new_b = start_y - original_line_equation.m * start_x;

        if i == 1 { //at start
            adjusted_ground_points[current_index] = Some(
                Point {
                    x: 0,
                    y: new_b as u32,
                }
            );
            current_index += 1;
        } else {
            let new_line_equation = get_equation_of_line(
                &Pointz {
                    x: start_x as i32,
                    y: start_y as i32,
                },
                &Pointz {
                    x: end_x as i32,
                    y: end_y as i32,
                },
            );

            //The next point can just use the previous point, if the slopes are equal there will
            // be no difference.
            if previous_line.m == new_line_equation.m {
                continue;
            }

            let x_interception = (new_line_equation.b - previous_line.b) / (previous_line.m - new_line_equation.m);
            let y_interception = new_line_equation.m * x_interception + new_line_equation.b;

            adjusted_ground_points[current_index] = Some(
                Point {
                    x: x_interception as u32,
                    y: y_interception as u32,
                }
            );
            current_index += 1;

            previous_line = new_line_equation;

            if i == ground_points_size - 1 { // last point
                let final_y = new_line_equation.m * 6999.0 + new_line_equation.b;
                adjusted_ground_points[current_index] = Some(
                    Point {
                        x: 6999,
                        y: final_y as u32,
                    }
                );
                current_index += 1;
            }
        }

        previous_line = LineEquation {
            m: original_line_equation.m,
            b: new_b,
        };
    }

    LeewayReturnValues {
        flat_surface_first_index: new_first_flat_surface_index,
        flat_surface_second_index: new_second_flat_surface_index,
        adjusted_ground_points,
        adjusted_ground_points_size: current_index,
    }
}

//TODO: probably clean this up a TAD bit
// calculating the equations for the ground points (m & b) before hand would be nice
// extracting out all the times I calculate distance would be nice (f64 must be used to avoid float->int conversion errors)
fn calculate_line(
    ground_points: &[Option<Point>],
    ground_points_size: usize,
    flat_surface_first_index: usize,
    flat_surface_second_index: usize,
    shuttle_point: &Point,
) -> Vec<PathInfo> {
    //Breadth first search, then find the shortest path

    let mut map = [[MapNode::new(); (7000 / SIZE_OF_SQUARES) as usize]; (3000 / SIZE_OF_SQUARES) as usize];

    //Iterate through all lines and save them to their respective map nodes.
    for i in 1..ground_points_size {
        let curr_ele = &ground_points[i].unwrap();
        let prev_ele = &ground_points[i - 1].unwrap();
        //If the point with the smaller x does NOT come first, there will be a problem with inclusion.
        // This happens because if a point starts on the very starting point of a square, the initial point
        // will not be included in the calculation.
        let (start_point, end_point) =
            if prev_ele.x <= curr_ele.x {
                (prev_ele, curr_ele)
            } else {
                (curr_ele, prev_ele)
            };

        //y1 = m * x1 + b
        //y2 = m * x2 + b
        //m = (y2-y1)/(x2-x1)
        //b = y1-m*x1

        let start_point_x_f = start_point.x as f64;
        let start_point_y_f = start_point.y as f64;
        let end_point_x_f = end_point.x as f64;
        let end_point_y_f = end_point.y as f64;

        let m = (end_point_y_f - start_point_y_f) / (end_point_x_f - start_point_x_f);
        let b = start_point_y_f - m * start_point_x_f;

        let start_x = start_point.x / SIZE_OF_SQUARES;
        let end_x = end_point.x / SIZE_OF_SQUARES;

        //The starting point here is a mirror of the end point of the last
        // loop. This has to be done in order to make sure both lines are added to crossing_lines
        // member.
        for x in start_x..=end_x {
            let mut y_begin =
                if x == start_x {
                    start_point.y
                } else {
                    (m * ((x * SIZE_OF_SQUARES) as f64) + b) as u32
                };

            let mut y_end =
                if x == end_x {
                    end_point.y
                } else {
                    (m * (((x + 1) as u32 * SIZE_OF_SQUARES) as f64) + b) as u32
                };

            println!("start_point {:?} end_point {:?}", start_point, end_point);
            println!("m {m} b {b} x {x}");
            println!("y_begin {y_begin} y_end {y_end}");
            y_begin /= SIZE_OF_SQUARES;
            y_end /= SIZE_OF_SQUARES;

            let y_range = if y_begin <= y_end {
                println!("y range increasing {y_begin}..={y_end}");
                (y_begin..=y_end).collect::<Vec<_>>()
            } else {
                println!("y range decreasing {y_end}..={y_begin}");
                (y_end..=y_begin).rev().collect::<Vec<_>>()
            };

            for y in y_range {
                println!("x {x} y {y}");
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
    println!();

    // println!("node (10,6) {:?}", map[12][13]);

    let mut paths = Vec::<PathInfo>::new();

    /** For now assuming first points cannot have any lines to intersect with. **/

    let normalized_shuttle_point = Point {
        x: shuttle_point.x - shuttle_point.x % SIZE_OF_SQUARES,
        y: shuttle_point.y - shuttle_point.y % SIZE_OF_SQUARES,
    };

    let first_distance = calculate_dist_for_two_points(
        normalized_shuttle_point.y,
        shuttle_point.y,
        normalized_shuttle_point.x,
        shuttle_point.x,
    );

    map[(normalized_shuttle_point.y / SIZE_OF_SQUARES) as usize][(normalized_shuttle_point.x / SIZE_OF_SQUARES) as usize].has_been_used = true;

    paths.push(
        PathInfo {
            path: Vec::from([shuttle_point.clone(), normalized_shuttle_point]),
            distance: first_distance,
        },
    );

    if shuttle_point.x + SIZE_OF_SQUARES <= 6999 {
        let second_point = Point {
            x: normalized_shuttle_point.x + SIZE_OF_SQUARES,
            y: normalized_shuttle_point.y,
        };

        let second_distance = calculate_dist_for_two_points(
            second_point.y,
            shuttle_point.y,
            second_point.x,
            shuttle_point.x,
        );

        map[(second_point.y / SIZE_OF_SQUARES) as usize][(second_point.x / SIZE_OF_SQUARES) as usize].has_been_used = true;

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
        let paths_copy = paths;
        let mut temp_paths = HashMap::<Point, PathInfo>::new();

        paths = Vec::<PathInfo>::new();

        for path in paths_copy {
            let final_x = path.path.last().expect("path empty").x;
            let final_y = path.path.last().expect("path empty").y;
            // println!("x {final_x} y {final_y}");
            if final_y >= SIZE_OF_SQUARES { //down
                // println!("down");
                let next_y = final_y - SIZE_OF_SQUARES;
                let mut next_element = &mut map[(next_y / SIZE_OF_SQUARES) as usize][(final_x / SIZE_OF_SQUARES) as usize];
                if !next_element.has_been_used {
                    check_if_path_valid(
                        ground_points,
                        flat_surface_first_index,
                        flat_surface_second_index,
                        &mut next_element,
                        &mut final_paths,
                        &mut temp_paths,
                        &path,
                        final_x,
                        next_y,
                        MoveDirection::DOWN,
                    );
                }
            }

            if final_x >= SIZE_OF_SQUARES { //left
                // println!("left");
                let next_x = final_x - SIZE_OF_SQUARES;
                let mut next_element = &mut map[(final_y / SIZE_OF_SQUARES) as usize][(next_x / SIZE_OF_SQUARES) as usize];
                if !next_element.has_been_used {
                    check_if_path_valid(
                        ground_points,
                        flat_surface_first_index,
                        flat_surface_second_index,
                        &mut next_element,
                        &mut final_paths,
                        &mut temp_paths,
                        &path,
                        next_x,
                        final_y,
                        MoveDirection::LEFT,
                    );
                }
            }

            if final_x + SIZE_OF_SQUARES <= 6999 { //right
                // println!("right");
                let next_x = final_x + SIZE_OF_SQUARES;
                let next_element = map[(final_y / SIZE_OF_SQUARES) as usize][(next_x / SIZE_OF_SQUARES) as usize];
                if !next_element.has_been_used {

                    //TODO: Would like a cleaner solution here.
                    //Checking right is a bit special because.
                    // 1) It needs to check the CURRENT block not the next block (handled inside check_if_path_valid).
                    // 2) It needs to also check the single point in the next block because it will be moving there.
                    let mut run_func = true;
                    for i in 0..next_element.crossing_lines_idx {
                        let point_pair = next_element.crossing_lines[i].unwrap();

                        let start_point_x_f = point_pair.start.x as f64;
                        let start_point_y_f = point_pair.start.y as f64;
                        let end_point_x_f = point_pair.end.x as f64;
                        let end_point_y_f = point_pair.end.y as f64;

                        let m = (end_point_y_f - start_point_y_f) / (end_point_x_f - start_point_x_f);
                        let b = start_point_y_f - m * start_point_x_f;

                        if (m * (next_x as f64) + b) as u32 == final_y {
                            run_func = false;
                            break;
                        }
                    }

                    if run_func {
                        let mut next_element = &mut map[(final_y / SIZE_OF_SQUARES) as usize][(final_x / SIZE_OF_SQUARES) as usize];

                        check_if_path_valid(
                            ground_points,
                            flat_surface_first_index,
                            flat_surface_second_index,
                            &mut next_element,
                            &mut final_paths,
                            &mut temp_paths,
                            &path,
                            next_x,
                            final_y,
                            MoveDirection::RIGHT,
                        );
                    }
                }
            }

            if final_y + SIZE_OF_SQUARES <= 2999 { //up
                // println!("up");
                let next_y = final_y + SIZE_OF_SQUARES;
                //TODO: need to check the next spot in up to make sure its clear
                let next_element = &map[(next_y / SIZE_OF_SQUARES) as usize][(final_x / SIZE_OF_SQUARES) as usize];

                if !next_element.has_been_used {

                    //Checking right is a bit special because.
                    // 1) It needs to check the CURRENT block not the next block (handled inside check_if_path_valid).
                    // 2) It needs to also check the single point in the next block because it will be moving there.
                    let mut run_func = true;
                    for i in 0..next_element.crossing_lines_idx {
                        let point_pair = next_element.crossing_lines[i].unwrap();

                        let start_point_x_f = point_pair.start.x as f64;
                        let start_point_y_f = point_pair.start.y as f64;
                        let end_point_x_f = point_pair.end.x as f64;
                        let end_point_y_f = point_pair.end.y as f64;

                        let m = (end_point_y_f - start_point_y_f) / (end_point_x_f - start_point_x_f);
                        let b = start_point_y_f - m * start_point_x_f;

                        if (((next_y as f64) - b) / m) as u32 == final_x {
                            run_func = false;
                            break;
                        }
                    }

                    if run_func {
                        let mut next_element = &mut map[(final_y / SIZE_OF_SQUARES) as usize][(final_x / SIZE_OF_SQUARES) as usize];
                        check_if_path_valid(
                            ground_points,
                            flat_surface_first_index,
                            flat_surface_second_index,
                            &mut next_element,
                            &mut final_paths,
                            &mut temp_paths,
                            &path,
                            final_x,
                            next_y,
                            MoveDirection::UP,
                        );
                    }
                }
            }
        }

        for path in temp_paths {
            let next_element = &mut map[(path.0.y / SIZE_OF_SQUARES) as usize][(path.0.x / SIZE_OF_SQUARES) as usize];
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

#[derive(Debug, PartialEq)]
enum MoveDirection {
    LEFT,
    RIGHT,
    DOWN,
    UP,
}

fn check_if_path_valid(
    ground_points: &[Option<Point>],
    flat_surface_first_index: usize,
    flat_surface_second_index: usize,
    next_element: &mut MapNode,
    final_paths: &mut Vec<PathInfo>,
    temp_paths: &mut HashMap<Point, PathInfo>,
    path: &PathInfo,
    passed_x: u32,
    passed_y: u32,
    move_direction: MoveDirection,
) {
    let x_div = 13;
    let y_div = 12;
    if passed_x / SIZE_OF_SQUARES == x_div && passed_y / SIZE_OF_SQUARES == y_div {
        println!("move_direction {:?}", move_direction);
    }
    let mut path_ended = false;
    for i in 0..next_element.crossing_lines_idx {
        let point_pair = next_element.crossing_lines[i].expect("invalid crossing idx {i}");

        let start_point_x_f = point_pair.start.x as f64;
        let start_point_y_f = point_pair.start.y as f64;
        let end_point_x_f = point_pair.end.x as f64;
        let end_point_y_f = point_pair.end.y as f64;

        let m = (end_point_y_f - start_point_y_f) / (end_point_x_f - start_point_x_f);
        let b = start_point_y_f - m * start_point_x_f;

        let (comparator, intersection, x_val, y_val) =
            if move_direction == MoveDirection::LEFT
                || move_direction == MoveDirection::RIGHT {
                //TODO: this is pretty convoluted, having to subtract x from it, might want to add
                // a passed value for x comparator or something a bit more sensible
                let y_line = passed_y as f64;
                let comparator =
                    if move_direction == MoveDirection::RIGHT {
                        (passed_x / SIZE_OF_SQUARES) - 1
                    } else {
                        passed_x / SIZE_OF_SQUARES
                    };
                let x_intersection = ((y_line - b) / m) as u32;
                (comparator, x_intersection / SIZE_OF_SQUARES, x_intersection, passed_y)
            } else { // UP || DOWN
                let x_line = passed_x as f64;
                let comparator =
                    if move_direction == MoveDirection::UP {
                        (passed_y / SIZE_OF_SQUARES) - 1
                    } else {
                        passed_y / SIZE_OF_SQUARES
                    };
                let y_intersection = (m * x_line + b) as u32;
                (comparator, y_intersection / SIZE_OF_SQUARES, passed_x, y_intersection)
            };

        //4,10 -> 5,10 goes through a line
        if passed_x / SIZE_OF_SQUARES == x_div && passed_y / SIZE_OF_SQUARES == y_div {
            println!("m {m} b {b}");
            println!("passed_x {passed_x} passed_y {passed_y}");
            println!("comparator {comparator} intersection {intersection} move_direction {:?}", move_direction);
            println!("next_element {:?}", next_element.crossing_lines);
            println!("point_pair {:?}", point_pair);
        }

        if comparator == intersection {
            path_ended = true;

            if point_pair.start == ground_points[flat_surface_first_index].unwrap()
                && point_pair.end == ground_points[flat_surface_second_index].unwrap() {
                let mut path_clone = path.clone();
                let path_last_val = path_clone.path.last().expect("path empty");

                path_clone.distance += calculate_dist_for_two_points(
                    path_last_val.y,
                    y_val,
                    path_last_val.x,
                    x_val,
                );

                path_clone.path.push(
                    Point {
                        x: x_val,
                        y: y_val,
                    }
                );

                final_paths.push(
                    path_clone
                );
            }
        }
    }

    if passed_x / SIZE_OF_SQUARES == x_div && passed_y / SIZE_OF_SQUARES == y_div {
        println!("passed_x {passed_x} passed_y {passed_y} path_ended {path_ended} move_direction {:?}", move_direction);
    }

    if !path_ended {
        let mut path_clone = path.clone();
        let path_last_val = path_clone.path.last().expect("path empty");

        path_clone.distance += calculate_dist_for_two_points(
            path_last_val.y,
            passed_y,
            path_last_val.x,
            passed_x,
        );

        let next_point = Point {
            x: passed_x,
            y: passed_y,
        };

        let prev_value = temp_paths.get(&next_point);

        if prev_value.is_none() || prev_value.unwrap().distance > path_clone.distance {
            // println!("storing x {next_x} y {final_y}");
            path_clone.path.push(next_point);
            temp_paths.insert(next_point, path_clone);
        }
    }
}

fn calculate_dist_for_two_points(
    y1: u32,
    y2: u32,
    x1: u32,
    x2: u32,
) -> u32 {
    ((y2 as i32 - y1 as i32).pow(2) + (x2 as i32 - x1 as i32).pow(2)) as u32
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

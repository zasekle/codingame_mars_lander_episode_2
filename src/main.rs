#![allow(unused_doc_comments)]

use std::collections::HashMap;
use std::time::Instant;

//The size of the squares the the map is divided into when finding the shortest path.
const SIZE_OF_SQUARES: i32 = 50;

//Amount of distance to take into consideration between shuttle and ground for shortest path.
const AMOUNT_OF_LEEWAY: f64 = 50.0;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, Copy, Clone)]
struct LineEquation {
    m: f64,
    b: f64,
}

#[derive(Debug, PartialEq, Clone)]
enum MoveDirection {
    LEFT,
    RIGHT,
    DOWN,
    UP,
}

#[derive(Debug, Clone)]
struct PathInfo {
    path: Vec<Point>,
    distance: i32,
    most_recent_move_direction: MoveDirection,
}

#[derive(Debug, Copy, Clone)]
struct LineSegment {
    start: Point,
    end: Point,
    line_equation: LineEquation,
}

impl LineSegment {
    fn new(start_x: i32, start_y: i32, end_x: i32, end_y: i32) -> LineSegment {
        let start = Point {
            x: start_x,
            y: start_y,
        };
        let end = Point {
            x: end_x,
            y: end_y,
        };
        let line_equation = get_equation_of_line(
            &start,
            &end,
        );

        LineSegment {
            start,
            end,
            line_equation,
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct MapNode {
    crossing_lines: [Option<LineSegment>; 5],
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

#[derive(Debug)]
struct LeewayReturnValues {
    flat_line_index: usize,
    adjusted_ground_points: [Option<LineSegment>; 30],
    adjusted_ground_points_size: usize,
}

fn main() {

    let start = Instant::now();

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
        Some(LineSegment::new(0, 1800, 300, 1200)),
        Some(LineSegment::new(300, 1200, 1000, 1550)),
        Some(LineSegment::new(1000, 1550, 2000, 1200)),
        Some(LineSegment::new(2000, 1200, 2500, 1650)),
        Some(LineSegment::new(2500, 1650, 3700, 220)),
        Some(LineSegment::new(3700, 220, 4700, 220)),
        Some(LineSegment::new(4700, 220, 4750, 1000)),
        Some(LineSegment::new(4750, 1000, 4700, 1650)),
        Some(LineSegment::new(4700, 1650, 4000, 1700)),
        Some(LineSegment::new(4000, 1700, 3700, 1600)),
        Some(LineSegment::new(3700, 1600, 3750, 1900)),
        Some(LineSegment::new(3750, 1900, 4000, 2100)),
        Some(LineSegment::new(4000, 2100, 4900, 2050)),
        Some(LineSegment::new(4900, 2050, 5100, 1000)),
        Some(LineSegment::new(5100, 1000, 5500, 500)),
        Some(LineSegment::new(5500, 500, 6200, 800)),
        Some(LineSegment::new(6200, 800, 6999, 600)),
    ];
    let shuttle_point = Point {
        x: 6500,
        y: 2000,
    };
    let flat_line_index = 5;
    let ground_points_size = 17;

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

    //TODO: Clean up the warnings maybe.
    //TODO: After it is cleaned up, test all the examples I have out.
    let leeway_return_values = give_leeway_for_ground(
        &ground_points,
        ground_points_size,
        flat_line_index,
    );

    for i in 0..leeway_return_values.adjusted_ground_points_size {
        print!("{},{} ", leeway_return_values.adjusted_ground_points[i].unwrap().start.x, leeway_return_values.adjusted_ground_points[i].unwrap().start.y);
    }
    print!("{},{} ", leeway_return_values.adjusted_ground_points[leeway_return_values.adjusted_ground_points_size - 1].unwrap().end.x, leeway_return_values.adjusted_ground_points[leeway_return_values.adjusted_ground_points_size - 1].unwrap().end.y);
    println!();

    let shuttle_path = calculate_line(
        &leeway_return_values.adjusted_ground_points,
        leeway_return_values.adjusted_ground_points_size,
        leeway_return_values.flat_line_index,
        &shuttle_point,
    );

    //Shuttle path should be trimmed to the smallest length it can be. The end of the path
    // will always be 'as vertical as possible'. This is done by prioritizing 'DOWN' for
    // movements. Then when cleaning up the final line, longest vertical line that can be
    // taken will be used.
    let mut s = String::new();
    let mut prev_end_point = Point { x: 0, y: 0 };
    for point in shuttle_path.iter() {
        if point.is_none() {
            s.push(' ');
            s += prev_end_point.x.to_string().as_str();
            s.push(',');
            s += prev_end_point.y.to_string().as_str();
            break;
        }
        let point = point.unwrap();
        if !s.is_empty() {
            s.push(' ');
        }
        s += point.start.x.to_string().as_str();
        s.push(',');
        s += point.start.y.to_string().as_str();
        prev_end_point = point.end;
    }
    println!("{s}");
    let duration = start.elapsed();

    println!("Time: {:?}", duration);
}

fn give_leeway_for_ground(
    ground_points: &[Option<LineSegment>],
    ground_points_size: usize,
    flat_line_index: usize,
) -> LeewayReturnValues {
    let mut adjusted_ground_points: [Option<LineSegment>; 30] = [None; 30];
    let mut current_index = 0;
    let mut new_flat_line_index = flat_line_index;

    let mut previous_point = Point {
        x: 0,
        y: 0,
    };

    let mut previous_line = LineEquation {
        m: 0.0,
        b: 0.0,
    };

    println!("size {ground_points_size}");
    for i in 0..ground_points_size {
        let line_segment = &ground_points[i].unwrap();

        let start_point = &line_segment.start;
        let end_point = &line_segment.end;
        let original_line_equation = &line_segment.line_equation;

        let perpendicular_m = -1.0 / original_line_equation.m;

        // next_m = Dy/Dx;
        // d = sqrt((Dy)^2 + (Dx)^2);
        // d = sqrt(Dx^2 + Dx^2 * next_m^2)
        // d = sqrt(Dx^2 * (1 + next_m^2))
        // d^2/(1 + next_m^2) = Dx^2
        // +/- sqrt(d^2/(1 + next_m^2)) = Dx
        // +/- d/sqrt((1 + next_m^2)) = Dx

        let (delta_x, delta_y) =
            if i == flat_line_index {
                new_flat_line_index = i;
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

        if i == 0 { //at start
            previous_point =
                Point {
                    x: 0,
                    y: new_b as i32,
                };

            // current_index += 1;
        } else {
            let new_line_equation = get_equation_of_line(
                &Point {
                    x: start_x as i32,
                    y: start_y as i32,
                },
                &Point {
                    x: end_x as i32,
                    y: end_y as i32,
                },
            );

            //The next point can just use the previous point, if the slopes are equal there will
            // be no difference. This is because the previous point must intersect the current
            // point as they are ground points. Therefore, there can be no intersection unless the
            // y-intercepts are the same as well.
            if previous_line.m == new_line_equation.m {
                continue;
            }

            let x_interception = (new_line_equation.b - previous_line.b) / (previous_line.m - new_line_equation.m);
            let y_interception = new_line_equation.m * x_interception + new_line_equation.b;

            let current_point =
                Point {
                    x: x_interception as i32,
                    y: y_interception as i32,
                };

            adjusted_ground_points[current_index] = Some(
                LineSegment {
                    start: previous_point,
                    end: current_point,
                    line_equation: previous_line,
                }
            );

            previous_point = current_point;

            current_index += 1;
        }

        previous_line = LineEquation {
            m: original_line_equation.m,
            b: new_b,
        };
    }

    let final_y = previous_line.m * 6999.0 + previous_line.b;
    let final_point =
        Point {
            x: 6999,
            y: final_y as i32,
        };

    adjusted_ground_points[current_index] = Some(
        LineSegment {
            start: previous_point,
            end: final_point,
            line_equation: LineEquation {
                m: previous_line.m,
                b: previous_line.b,
            },
        }
    );

    //Used to denote size below.
    current_index += 1;

    LeewayReturnValues {
        flat_line_index: new_flat_line_index,
        adjusted_ground_points,
        adjusted_ground_points_size: current_index,
    }
}

fn calculate_line(
    ground_points: &[Option<LineSegment>],
    ground_points_size: usize,
    flat_line_index: usize,
    shuttle_point: &Point,
) -> [Option<LineSegment>; 20] {
    let mut map = setup_map_for_ground_points(
        &ground_points,
        ground_points_size,
        flat_line_index,
    );

    let paths = build_paths_with_starting_points(
        shuttle_point,
        &mut map,
    );

    let shortest_path = find_shortest_path(
        ground_points,
        flat_line_index,
        &mut map,
        paths,
    );

    let cleaned_path = clean_path(
        &ground_points,
        ground_points_size,
        &shortest_path,
    );

    cleaned_path
}

fn clean_path(
    ground_points: &[Option<LineSegment>],
    ground_points_size: usize,
    shortest_path: &PathInfo,
) -> [Option<LineSegment>; 20] {
    let mut cleaned_path: [Option<LineSegment>; 20] = [None; 20];
    let mut shortest_path_current_idx: i32 = shortest_path.path.len() as i32 - 1;
    let mut cleaned_path_current_idx = 0;
    let mut previous_point = Point { x: 0, y: 0 };
    while shortest_path_current_idx >= 0 {
        if shortest_path_current_idx == (shortest_path.path.len() - 1) as i32 { //Last element (first element looked at).
            let final_point = &shortest_path.path[shortest_path_current_idx as usize];
            //Want the final part of the line to be vertical for as long as possible.
            while shortest_path_current_idx > 0 && shortest_path.path[(shortest_path_current_idx - 1) as usize].x == final_point.x {
                shortest_path_current_idx -= 1;
            }
            let next_point = &shortest_path.path[shortest_path_current_idx as usize];
            let line_equation = get_equation_of_line(
                &final_point,
                next_point,
            );

            cleaned_path[cleaned_path_current_idx] =
                Some(
                    LineSegment {
                        start: next_point.clone(),
                        end: final_point.clone(),
                        line_equation,
                    }
                );
            cleaned_path_current_idx += 1;

            previous_point = final_point.clone();
        } else { //Not last element.

            //Find the line that does not intersect with anything.
            let working_point = shortest_path.path[shortest_path_current_idx as usize];
            shortest_path_current_idx -= 1;

            'outer: while shortest_path_current_idx >= 0 {
                let current_point = shortest_path.path[shortest_path_current_idx as usize];
                let current_line = get_equation_of_line(
                    &current_point,
                    &working_point,
                );

                let (current_min_x, current_max_x) = min_max(current_point.x, working_point.x);
                let (current_min_y, current_max_y) = min_max(current_point.y, working_point.y);

                for i in 0..ground_points_size {
                    let line_segment = &ground_points[i].unwrap();
                    let start_point = &line_segment.start;
                    let end_point = &line_segment.end;

                    let (ground_min_x, ground_max_x) = min_max(start_point.x, end_point.x);
                    let (ground_min_y, ground_max_y) = min_max(start_point.y, end_point.y);

                    //This condition is not needed, it just avoids some calculations.
                    if ground_min_x >= current_min_x
                        || current_max_x >= ground_max_x
                        || ground_min_y >= current_min_y
                        || current_max_y >= ground_max_y
                    { //If there is overlap between lines.

                        let working_line = get_equation_of_line(
                            &start_point,
                            &end_point,
                        );

                        if current_line.m == working_line.m {
                            if current_line.b == working_line.b {
                                //These are the same line, they are intersecting at every point.
                                let next_point = &shortest_path.path[(shortest_path_current_idx + 1) as usize];
                                let line_equation = get_equation_of_line(
                                    &previous_point,
                                    next_point,
                                );

                                cleaned_path[cleaned_path_current_idx] =
                                    Some(
                                        LineSegment {
                                            start: next_point.clone(),
                                            end: previous_point,
                                            line_equation,
                                        }
                                    );
                                cleaned_path_current_idx += 1;

                                previous_point = next_point.clone();
                                break 'outer;
                            }
                            {
                                //These lines are parallel, they will never intersect.
                                continue;
                            }
                        }

                        let x_intersection = (working_line.b - current_line.b) / (current_line.m - working_line.m);
                        let y_intersection = (current_line.m * x_intersection + current_line.b) as i32;

                        let x_intersection = x_intersection as i32;

                        //If intersection is at line segment.
                        if current_min_x <= x_intersection && x_intersection <= current_max_x
                            && current_min_y <= y_intersection && y_intersection <= current_max_y
                            && ground_min_x <= x_intersection && x_intersection <= ground_max_x
                            && ground_min_y <= y_intersection && y_intersection <= ground_max_y
                        {
                            //Found the stopping point. Add the previous point and end.
                            let next_point = &shortest_path.path[(shortest_path_current_idx + 1) as usize];
                            let line_equation = get_equation_of_line(
                                &previous_point,
                                next_point,
                            );

                            cleaned_path[cleaned_path_current_idx] =
                                Some(
                                    LineSegment {
                                        start: next_point.clone(),
                                        end: previous_point,
                                        line_equation,
                                    }
                                );
                            cleaned_path_current_idx += 1;

                            previous_point = next_point.clone();
                            break 'outer;
                        }
                    }
                }

                if shortest_path_current_idx == 0 {
                    let next_point = &shortest_path.path[(shortest_path_current_idx) as usize];
                    let line_equation = get_equation_of_line(
                        &previous_point,
                        next_point,
                    );

                    cleaned_path[cleaned_path_current_idx] =
                        Some(
                            LineSegment {
                                start: next_point.clone(),
                                end: previous_point,
                                line_equation,
                            }
                        );
                    cleaned_path_current_idx += 1;
                }
                shortest_path_current_idx -= 1;
            }
        }
    }

    let mut final_path_idx = 0;
    let mut final_path: [Option<LineSegment>; 20] = [None; 20];
    for i in (0..20).rev() {
        let segment = cleaned_path[i];
        if segment.is_some() {
            final_path[final_path_idx] = segment;
            final_path_idx += 1;
        }
    }
    final_path
}

fn find_shortest_path(
    ground_points: &[Option<LineSegment>],
    flat_line_index: usize,
    map: &mut [[MapNode; (7000 / SIZE_OF_SQUARES) as usize]; (3000 / SIZE_OF_SQUARES) as usize],
    passed_paths: Vec<PathInfo>,
) -> PathInfo {
    let mut paths = passed_paths;
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
                        flat_line_index,
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
                        flat_line_index,
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

                    //Checking right is a bit special because.
                    // 1) It needs to check the CURRENT block not the next block (handled inside check_if_path_valid).
                    // 2) It needs to also check the single point in the next block because it will be moving there.
                    let mut run_func = true;
                    for i in 0..next_element.crossing_lines_idx {
                        let line_equation = &next_element.crossing_lines[i].unwrap().line_equation;

                        if (line_equation.m * (next_x as f64) + line_equation.b) as i32 == final_y {
                            run_func = false;
                            break;
                        }
                    }

                    if run_func {
                        let mut next_element = &mut map[(final_y / SIZE_OF_SQUARES) as usize][(final_x / SIZE_OF_SQUARES) as usize];

                        check_if_path_valid(
                            ground_points,
                            flat_line_index,
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
                let next_element = &map[(next_y / SIZE_OF_SQUARES) as usize][(final_x / SIZE_OF_SQUARES) as usize];

                if !next_element.has_been_used {

                    //Checking up is a bit special because.
                    // 1) It needs to check the CURRENT block not the next block (handled inside check_if_path_valid).
                    // 2) It needs to also check the single point in the next block because it will be moving there.
                    let mut run_func = true;
                    for i in 0..next_element.crossing_lines_idx {
                        let line_equation = &next_element.crossing_lines[i].unwrap().line_equation;

                        if (((next_y as f64) - line_equation.b) / line_equation.m) as i32 == final_x {
                            run_func = false;
                            break;
                        }
                    }

                    if run_func {
                        let mut next_element = &mut map[(final_y / SIZE_OF_SQUARES) as usize][(final_x / SIZE_OF_SQUARES) as usize];
                        check_if_path_valid(
                            ground_points,
                            flat_line_index,
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
    println!();

    final_paths.sort_by(|a, b| b.distance.partial_cmp(&a.distance).unwrap());

    let shortest_path = final_paths.pop().expect("final_paths was empty");

    let mut s = String::new();
    for point in shortest_path.path.iter() {
        if !s.is_empty() {
            s.push(' ');
        }
        s += point.x.to_string().as_str();
        s.push(',');
        s += point.y.to_string().as_str();
    }
    println!("{s}");
    shortest_path
}

fn build_paths_with_starting_points(
    shuttle_point: &Point,
    mut map: &mut [[MapNode; (7000 / SIZE_OF_SQUARES) as usize]; (3000 / SIZE_OF_SQUARES) as usize],
) -> Vec<PathInfo> {
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

    //Distance of zero means the shuttle is starting in the corner. No need to add the same point
    // twice.
    if first_distance == 0 {
        paths.push(
            PathInfo {
                path: Vec::from([shuttle_point.clone()]),
                distance: first_distance,
                most_recent_move_direction: MoveDirection::DOWN,
            },
        );
    } else {
        paths.push(
            PathInfo {
                path: Vec::from([shuttle_point.clone(), normalized_shuttle_point]),
                distance: first_distance,
                most_recent_move_direction: MoveDirection::DOWN,
            },
        );
    }

    if shuttle_point.x + SIZE_OF_SQUARES <= 6999 {
        let second_point = Point {
            x: normalized_shuttle_point.x + SIZE_OF_SQUARES,
            y: normalized_shuttle_point.y,
        };

        paths.push(
            generate_starting_path_for_point(
                second_point,
                shuttle_point,
                &mut map,
            )
        );
    }

    if shuttle_point.y + SIZE_OF_SQUARES <= 6999 {
        let third_point = Point {
            x: normalized_shuttle_point.x,
            y: normalized_shuttle_point.y + SIZE_OF_SQUARES,
        };

        paths.push(
            generate_starting_path_for_point(
                third_point,
                shuttle_point,
                &mut map,
            )
        );
    }

    if shuttle_point.x + SIZE_OF_SQUARES <= 6999
        && shuttle_point.y + SIZE_OF_SQUARES <= 6999 {
        let fourth_point = Point {
            x: normalized_shuttle_point.x + SIZE_OF_SQUARES,
            y: normalized_shuttle_point.y + SIZE_OF_SQUARES,
        };

        paths.push(
            generate_starting_path_for_point(
                fourth_point,
                shuttle_point,
                &mut map,
            )
        );
    }
    paths
}

fn generate_starting_path_for_point(
    new_point: Point,
    shuttle_point: &Point,
    map: &mut [[MapNode; (7000 / SIZE_OF_SQUARES) as usize]; (3000 / SIZE_OF_SQUARES) as usize],
) -> PathInfo {
    let distance = calculate_dist_for_two_points(
        new_point.y,
        shuttle_point.y,
        new_point.x,
        shuttle_point.x,
    );

    map[(new_point.y / SIZE_OF_SQUARES) as usize][(new_point.x / SIZE_OF_SQUARES) as usize].has_been_used = true;

    //DOWN is always returned here even though it is technically correct. However, it is irrelevant
    // on the first move.
    PathInfo {
        path: Vec::from([shuttle_point.clone(), new_point]),
        distance,
        most_recent_move_direction: MoveDirection::DOWN,
    }
}

fn setup_map_for_ground_points(
    ground_points: &&[Option<LineSegment>],
    ground_points_size: usize,
    flat_line_index: usize,
) -> [[MapNode; (7000 / SIZE_OF_SQUARES) as usize]; (3000 / SIZE_OF_SQUARES) as usize] {
    let mut map = [[MapNode::new(); (7000 / SIZE_OF_SQUARES) as usize]; (3000 / SIZE_OF_SQUARES) as usize];

    //Iterate through all lines and save them to their respective map nodes.
    for i in 0..ground_points_size {
        let current_line_segment = &ground_points[i].unwrap();
        let start_point_raw = &current_line_segment.start;
        let end_point_raw = &current_line_segment.end;
        let line_equation = &current_line_segment.line_equation;

        //If the point with the smaller x does NOT come first, there will be a problem with inclusion.
        // This happens because if a point starts on the very starting point of a square, the initial point
        // will not be included in the calculation.
        let (start_point, end_point) =
            if end_point_raw.x <= start_point_raw.x {
                (end_point_raw, start_point_raw)
            } else {
                (start_point_raw, end_point_raw)
            };

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
                    (line_equation.m * ((x * SIZE_OF_SQUARES) as f64) + line_equation.b) as i32
                };

            let mut y_end =
                if x == end_x {
                    end_point.y
                } else {
                    (line_equation.m * (((x + 1) as i32 * SIZE_OF_SQUARES) as f64) + line_equation.b) as i32
                };

            println!("start_point {:?} end_point {:?}", start_point, end_point);
            println!("m {} b {} x {x}", line_equation.m, line_equation.b);
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
                        LineSegment {
                            start: start_point.clone(),
                            end: end_point.clone(),
                            line_equation: *line_equation,
                        }
                    );

                map_ele.crossing_lines_idx += 1;

                if i == flat_line_index {
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

    map
}

fn check_if_path_valid(
    ground_points: &[Option<LineSegment>],
    flat_line_index: usize,
    next_element: &mut MapNode,
    final_paths: &mut Vec<PathInfo>,
    temp_paths: &mut HashMap<Point, PathInfo>,
    path: &PathInfo,
    passed_x: i32,
    passed_y: i32,
    move_direction: MoveDirection,
) {
    let x_div = 13;
    let y_div = 12;
    if passed_x / SIZE_OF_SQUARES == x_div && passed_y / SIZE_OF_SQUARES == y_div {
        println!("move_direction {:?}", move_direction);
    }
    let mut path_ended = false;
    for i in 0..next_element.crossing_lines_idx {
        let crossing_line_segment = &next_element.crossing_lines[i].expect("invalid crossing idx {i}");
        let crossing_line_equation = &crossing_line_segment.line_equation;

        let (comparator, intersection, x_val, y_val) =
            if move_direction == MoveDirection::LEFT
                || move_direction == MoveDirection::RIGHT {
                let y_line = passed_y as f64;
                let comparator =
                    if move_direction == MoveDirection::RIGHT {
                        (passed_x / SIZE_OF_SQUARES) - 1
                    } else {
                        passed_x / SIZE_OF_SQUARES
                    };
                let x_intersection = ((y_line - crossing_line_equation.b) / crossing_line_equation.m) as i32;
                (comparator, x_intersection / SIZE_OF_SQUARES, x_intersection, passed_y)
            } else { // UP || DOWN
                let x_line = passed_x as f64;
                let comparator =
                    if move_direction == MoveDirection::UP {
                        (passed_y / SIZE_OF_SQUARES) - 1
                    } else {
                        passed_y / SIZE_OF_SQUARES
                    };
                let y_intersection = (crossing_line_equation.m * x_line + crossing_line_equation.b) as i32;
                (comparator, y_intersection / SIZE_OF_SQUARES, passed_x, y_intersection)
            };

        if passed_x / SIZE_OF_SQUARES == x_div && passed_y / SIZE_OF_SQUARES == y_div {
            println!("crossing_line_equation {:?}", crossing_line_equation);
            println!("passed_x {passed_x} passed_y {passed_y}");
            println!("comparator {comparator} intersection {intersection} move_direction {:?}", move_direction);
            println!("next_element {:?}", next_element.crossing_lines);
        }

        if comparator == intersection {
            path_ended = true;
            let ground_line_segment = ground_points[flat_line_index].unwrap();
            if crossing_line_segment.start == ground_line_segment.start
                && crossing_line_segment.end == ground_line_segment.end {
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

        if prev_value.is_none()
            || (prev_value.unwrap().most_recent_move_direction != MoveDirection::DOWN
            && move_direction == MoveDirection::DOWN)
        {
            path_clone.most_recent_move_direction = move_direction;
            path_clone.path.push(next_point);
            temp_paths.insert(next_point, path_clone);
        }
    }
}

fn get_equation_of_line(
    start: &Point,
    end: &Point,
) -> LineEquation {
    let start_point_x_f = start.x as f64;
    let start_point_y_f = start.y as f64;
    let end_point_x_f = end.x as f64;
    let end_point_y_f = end.y as f64;

    let m = (end_point_y_f - start_point_y_f) / (end_point_x_f - start_point_x_f);
    let b = start_point_y_f - m * start_point_x_f;

    LineEquation { m, b }
}

fn calculate_dist_for_two_points(
    y1: i32,
    y2: i32,
    x1: i32,
    x2: i32,
) -> i32 {
    (y2 as i32 - y1 as i32).pow(2) + (x2 as i32 - x1 as i32).pow(2)
}

fn min_max<T: Ord>(a: T, b: T) -> (T, T) {
    if a < b {
        (a, b)
    } else {
        (b, a)
    }
}
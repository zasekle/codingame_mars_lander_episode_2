#[derive(Debug, Clone, Copy, PartialEq)]
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

    //TODO: Still need to choose the shortest distance line.
    // calculate_line(
    //
    // )
    //#1 0,100 1000,500 1500,1500 3000,1000 4000,150 5500,150 6999,800
}

//TODO: probably clean this up a TAD bit
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
        let end_x = end_point.x / DIVISOR;

        for x in start_x..=end_x {
            let mut y_begin =
                if x == start_point.x {
                    start_point.y
                } else {
                    (m * (x as f32) + b) as u32
                };

            let mut y_end =
                if x == end_point.x {
                    end_point.y
                } else {
                    (m * ((x + 1) as f32) + b) as u32
                };

            y_begin /= DIVISOR;
            y_end /= DIVISOR;

            for y in y_begin..=y_end {
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

    let mut paths = Vec::<PathInfo>::new();

    /** For now assuming first points cannot have any lines to intersect with. **/

    let first_point = Point {
        x: shuttle_point.x,
        y: shuttle_point.y,
    };

    let first_distance = (first_point.y - shuttle_point.y).pow(2) + (first_point.x - shuttle_point.x).pow(2);

    map[(first_point.y / DIVISOR) as usize][(first_point.x / DIVISOR) as usize].has_been_used = true;

    paths.push(
        PathInfo {
            path: Vec::from([shuttle_point.clone(), first_point]),
            distance: first_distance,
        },
    );

    if shuttle_point.x + DIVISOR <= 6999 {
        let second_point = Point {
            x: shuttle_point.x + DIVISOR,
            y: shuttle_point.y,
        };

        let second_distance = (second_point.y - shuttle_point.y).pow(2) + (second_point.x - shuttle_point.x).pow(2);

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
    while paths.is_empty() {
        let paths_copy = paths;
        paths = Vec::<PathInfo>::new();

        for path in paths_copy {
            let final_x = path.path.last().expect("path empty").x;
            let final_y = path.path.last().expect("path empty").y;

            if final_x >= DIVISOR { //left
                let next_x = final_x - DIVISOR;
                let next_element = &mut map[(final_y / DIVISOR) as usize][(next_x / DIVISOR) as usize];
                if !next_element.has_been_used || next_element.contains_landing_line {
                    next_element.has_been_used = true;
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

                                path_clone.distance += (path_last_val.y - final_y).pow(2) + (path_last_val.x - x_intersection).pow(2);

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

                        path_clone.distance += (path_last_val.y - final_y).pow(2) + (path_last_val.x - next_x).pow(2);

                        path_clone.path.push(
                            Point {
                                x: next_x,
                                y: final_y,
                            }
                        );

                        paths.push(
                            path_clone
                        );
                    }
                }
            }

            if final_x + DIVISOR <= 6999 { //right
                let next_x = final_x + DIVISOR;
                let next_element = &mut map[(final_y / DIVISOR) as usize][(next_x / DIVISOR) as usize];
                if !next_element.has_been_used || next_element.contains_landing_line {
                    next_element.has_been_used = true;
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

                                path_clone.distance += (path_last_val.y - final_y).pow(2) + (path_last_val.x - x_intersection).pow(2);

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

                        path_clone.distance += (path_last_val.y - final_y).pow(2) + (path_last_val.x - next_x).pow(2);

                        path_clone.path.push(
                            Point {
                                x: next_x,
                                y: final_y,
                            }
                        );

                        paths.push(
                            path_clone
                        );
                    }
                }
            }

            if final_y >= DIVISOR { //down
                let next_y = final_y + DIVISOR;
                let next_element = &mut map[(next_y / DIVISOR) as usize][(final_x / DIVISOR) as usize];
                if !next_element.has_been_used || next_element.contains_landing_line {
                    next_element.has_been_used = true;
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

                                path_clone.distance += (path_last_val.y - y_intersection).pow(2) + (path_last_val.x - final_x).pow(2);

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

                        path_clone.distance += (path_last_val.y - next_y).pow(2) + (path_last_val.x - final_x).pow(2);

                        path_clone.path.push(
                            Point {
                                x: final_x,
                                y: next_y,
                            }
                        );

                        paths.push(
                            path_clone
                        );
                    }
                }
            }
        }
    }

    final_paths
}
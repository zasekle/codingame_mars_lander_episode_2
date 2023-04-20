#[derive(Debug)]
struct Point {
    x: u32,
    y: u32,
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
}

const DIVISOR: usize = 50;

struct PathInfo {
    path: Vec<Point>,
    distance: u32,
}

//TODO: may want to add the starting and stopping points
// more than one could exist tho
// they could change directions while inside the node
struct MapNode {
    contains_line: bool,
    contains_landing_line: bool,
    has_been_used: bool,
}

fn calculate_line(
    ground_points: &[Point],
    flat_surface_first_index: usize,
    flat_surface_second_index: usize,
    shuttle_point: &Point,
) {
    //TODO: I can draw the entire thing as a grid, then draw a straight path between the starting point and each point on the line
    // but I can't have 90 degree turns or anything like that
    //TODO: probably something like a shortest path is more appropriate
    // so I can divide it up into squares that are size 10(?), makes it 700x300 or 210000 points
    // OR I can divide it up into say size 100(?), which makes it 70x30 or 2100 points
    // this would be fairly easy to calculate and I could probably 'iron' it out if I needed to

    //TODO: Will divide them into boxes of size 100
    // any point inside of the box will need to be taken into account
    // bring in the list of ground points, the shuttle points, the flat service indexes

    //TODO: So my first thought is just to divide them all by 50, but I don't think this actually works, in fact I probably just want to use
    // the standard coords, need to find if it intersects with anything
    // only need to go down left and right, the shuttle will never need to go up

    //Breadth first search, then find the shortest path

    let mut map = [[false; 7000 / DIVISOR]; 3000 / DIVISOR];

    //TODO: is this the best way to do it?
    // the alternative is to iterate through every ground point every move though
    for i in 1..ground_points.len() {
        let start_point = &ground_points[i-1];
        let end_point = &ground_points[i];

        //TODO: there are straight lines and there are curved lines

        //TODO: properties
        // it seems to always 'step'
        // maybe do intersections of straight lines and see which ones it crosses?

        //TODO: maybe I can
        // iterate through each x coordinate of a line segment
        // intersect it with each y coordinate line
        // use that intersection to see which y boxes were passed through during it (the intersection is the last one but there could be only one)

        //y1 = m * x1 + b
        //y2 = m * x2 + b
        //m = (y2-y1)/(x2-x1)
        //b = y1-m*x1

        let start_x = start_point.x as f32;
        let start_y = start_point.y as f32;
        let end_x = end_point.x as f32;
        let end_y = end_point.y as f32;

        let m = (end_y - start_y)/(end_x - start_x);
        let b = start_y - m * start_x;

    }

    //TODO: the first move and last move will be difficult because they aern't at even numbers
    // I suppose I need to make sure that the square I am in does not intersect with anything
    // then I can just start from all points around it,
    // maybe keep track of the distance to each corner and path as a whole
    // then I can find the shortest path by the path total distance

    //TODO: need to send in all 2 points 'around' the ship

    let mut first_point = Point {
        x: shuttle_point.x / DIVISOR,
        y: shuttle_point.y / DIVISOR,
    };

    //mark the point on the map
    map[first_point.y][first_point.x] = true;

    //TODO: would conceptualizing this point in the 'center' work out better for me?
    // it might be, but all I need to do is keep the 'boxes' the same and change where the intersection is?
    // I think it is more overhead in terms of programming, there would be less points because the
    // right and top will actually lose a row, actually no it won't, all points will just move up and right

    //TODO: need to make sure first_point does not intersect
    // make sure no lines go through this box maybe? If it does will need to look 'closer'
    // maybe make the map a struct with the starting and stopping points of each intersecting line and if the square has already been passed

    //TODO: probably iterate through the 'ground' itself and check for intersections
    let first_distance = (first_point.y * DIVISOR - shuttle_point.y).pow(2) + (first_point.x * DIVISOR - shuttle_point.x).pow(2);

    let mut paths = Vec::<PathInfo>::new();

    paths.push(
        PathInfo {
            path: Vec::from([first_point]),
            distance: first_distance,
        }
    );






    let mut final_paths = Vec::<PathInfo>::new();

    let mut distance_modifier = 1;
    while paths.is_empty() {
        let mut paths_copy = paths;
        paths = Vec::<PathInfo>::new();

        for path in paths.iter_mut() {

        }
    }
}
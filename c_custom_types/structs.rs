// An attribute to hide warnings for unused code.
#![allow(dead_code)]

#[derive(Debug)]
struct Person {
    name: &'static str,
    age: u8,
}

// A unit struct
struct Unit;

// A tuple struct
struct Pair(i32, f32);

// A struct with two fields
struct Point {
    x: f32,
    y: f32,
}

struct Rectangle {
    // A rectangle can be specified by where the top left and bottom right
    // corners are in space.
    top_left: Point,
    bottom_right: Point,
}

fn rect_area(points: &Rectangle) -> f32 {
    let Rectangle {
        top_left: Point { x: x1, y: y1 },
        bottom_right: Point { x: x2, y: y2 },
    } = points;

    let width = (x2 - x1).abs();
    let height = (y2 - y1).abs();

    width * height
}

fn square(top_left: Point, size: f32) -> Rectangle {
    let Point { x, y } = top_left;

    Rectangle {
        top_left: Point { x, y },
        bottom_right: Point {
            x: top_left.x + size,
            y: top_left.y - size,
        },
    }
}

fn main() {
    let name: &str = "Peter";
    let age = 27;
    let peter = Person { name, age };

    // Print debug struct
    println!("{:?}", peter);

    // Instantiate a `Point`
    let point: Point = Point { x: 5.2, y: 0.4 };
    let another_point: Point = Point { x: 10.2, y: 5.4 };

    // Access the fields of the point
    println!("Point coordinates: ({}, {})", point.x, point.y);
    println!(
        "Point coordinates: ({}, {})",
        another_point.x, another_point.y
    );

    // Make a new point by using struct update syntax to use the fields of our
    // other one
    let bottom_right = Point {
        x: 15.4,
        ..another_point
    };

    // `bottom_right.y` will be the same as `another_point.y` because we used that field
    // from `another_point`
    println!("Second point: ({}, {})", bottom_right.x, bottom_right.y);

    // Destructure the point using a `let` binding
    let Point {
        x: left_edge,
        y: top_edge,
    } = point;

    let _rectangle = Rectangle {
        // struct instantiation is an expression too
        top_left: Point {
            x: left_edge,
            y: top_edge,
        },
        bottom_right: bottom_right,
    };

    // Instantiate a unit struct
    let _unit = Unit;

    // Instantiate a tuple struct
    let pair = Pair(1, 0.1);

    // Access the fields of a tuple struct
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    // Destructure a tuple struct
    let Pair(integer, decimal) = pair;

    println!("pair contains {:?} and {:?}", integer, decimal);
    /*
     * 1. Activity
     * Add a function rect_area which calculates the area of a Rectangle (try using nested destructuring).
     */
    println!("Rectangle area: {}", rect_area(&_rectangle));

    /*
     * 2. Activity
     * Add a function square which takes a Point and a f32 as arguments, and returns a      Rectangle with its top left corner on the point, and a width and height corresponding to the f32.
     */

    let square_rect = square(Point { x: 1.0, y: 4.0 }, 3.0);
    println!(
        "Square: top_left ({}, {}), bottom_right ({}, {})",
        square_rect.top_left.x,
        square_rect.top_left.y,
        square_rect.bottom_right.x,
        square_rect.bottom_right.y
    );
    println!("Square area: {}", rect_area(&square_rect));
}

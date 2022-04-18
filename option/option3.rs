// option3.rs
// Make me compile! Execute `rustlings hint option3` for hints

// I AM DONE

struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let y: Option<Point> = Some(Point { x: 100, y: 200 });

    // `&` indicates that you want reference to an object.
    // `ref` indicates that you want reference to an unpacked value.
    match y {
        Some(ref p) => println!("Co-ordinates are {},{} ", p.x, p.y),
        _ => println!("no match"),
    }
    y; // Fix without deleting this line.
}

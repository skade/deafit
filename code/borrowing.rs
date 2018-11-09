struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let mut p = Point { x: 1, y: 2};
    print_point(&p);
    p.x = 2;

    // <<< Hier wird Point aus dem Speicher entfernt
}

fn print_point(p: &Point) {
    println!("x: {}, y: {}", p.x, p.y);
}
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let mut p = Point { x: 1, y: 2};

    move_point(&mut p, 2, 4);

    // <<< Hier wird Point aus dem Speicher entfernt
}

fn move_point(p: &mut Point, x: i32, y: i32) {
    p.x = x;
    p.y = y;
}
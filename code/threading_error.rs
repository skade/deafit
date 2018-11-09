struct Counter {
    count: u32
}

fn main() {
    let mut counter = Counter { count: 0 };

    for _ in 1..=3 {
        std::thread::spawn(move || {
            counter.count += 1
        });
    }
}
use std::fs::File;
use std::io::Read;

fn main() -> Result<(), std::io::Error> {
    let mut contents = String::new();
    let mut file = File::open("hello_world.rs")?;

    file.read_to_string(&mut contents)?;

    println!("{}", contents);
    Ok(())
}
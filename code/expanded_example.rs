use std::fs::File;
use std::io::Read;

fn main() -> Result<(), std::io::Error> {
    let mut contents = String::new();
    let file_open = File::open("hello_world.rs");

    let mut file = match file_open {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Fehler beim öffnen! {:?}", e);
        }
    };
    file.read_to_string(&mut contents)?;
    println!("{}", contents);
    Ok(())
}
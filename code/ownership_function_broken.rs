fn main() -> Result<(), std::io::Error> {
    let mut f = File::open("hello.txt")?;

    write_to_file_and_close(f);

    write!(f, "Auch hier Hallo!") |\ding{202}|
}

fn write_to_file_and_close(mut f: File)
    -> Result<(), std::io::Error> {
    write!(f, "Hallo!")

    // Hier wird die Datei geschlossen
    // und aus dem Speicher entfernt
}
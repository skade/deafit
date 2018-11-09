fn main() {
    fn main() -> Result<(), std::io::Error> {
    let f = File::open("hello.txt")?;

    write_to_file_and_close(f);
}

fn write_to_file_and_close(mut f: File) {
    write!(f, "Hallo!");

    // Hier wird die Datei geschlossen
    // und aus dem Speicher entfernt
}
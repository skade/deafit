fn main() -> Result<(), std::io::Error> {
    let f = File::open("hello.txt")?;

    // Hier wird die Datei geschlossen
    // und aus dem Speicher entfernt
}
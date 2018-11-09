struct FileList {
    list: Vec<File> |\ding{202}|
}

enum Result<T,E> {
    Ok(T), |\ding{203}|
    Err(E) |\ding{204}|
}
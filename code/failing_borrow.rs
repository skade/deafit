fn main() {
    let mut vector = vec![1,2,3];|\ding{202}|
    let element = &vector[2]; |\ding{203}|
    let last = vector.pop(); |\ding{204}|
    println!("{}", element);
    // was steht jetzt in `element`? 
}
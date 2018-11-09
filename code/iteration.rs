fn main() {
    let mut vec = vec![1,2,3];

    for i in vec.iter_mut() {
       *i += 1; |\ding{202}|
    }

    println!("{:?}", vec);

    for (i,j) in vec.iter().zip(vec.iter()) {
        println!("{:?}", (i,j)); |\ding{203}|
    }

    for i in vec.into_iter() {
        println!("{:?}", i); |\ding{204}|
    }
}
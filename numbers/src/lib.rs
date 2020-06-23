pub fn say_hello() {
    println!("Hello, World!");
}


pub fn numbers() {

    let numbers = [1, 2, 3, 4, 5];
    for n in numbers.iter() {
        println!("{}", n);
    }
}

pub fn numbers_vec() {
    let numbers = vec![1, 2, 3, 4, 5];
    for n in numbers {
        println!("{}", n);
    }
}

// pub fn say_hello() {
//     println!("Hello, World!");
// }
// 
// 
// pub fn numbers() {
// 
//     let numbers = [1, 2, 3, 4, 5];
//     for n in numbers.iter() {
//         println!("{}", n);
//     }
// }
// 
// pub fn numbers_vec() {
//     let numbers = vec![1, 2, 3, 4, 5];
//     for n in numbers {
//         println!("{}", n);
//     }
// }
// 
// pub fn print() {
//     let numbers = [1, 2, 3, 4, 5];
//     output_sequence(numbers);
// }
// 
// fn output_sequence(numbers: [u8; 5]) {
//     for n in numbers.iter() {
//         println!("{}", n);
//     }
// }

pub fn print(limit: u8) {
    let numbers = generate_sequence(limit);
    output_sequence(&numbers);
}

// fn generate_sequence(limit: u8) -> Vec<u8> {
//     let mut numbers = Vec::new();
//     for n in 1..=limit {
//         numbers.push(n);
//     }
//     numbers
// }

// Improved generate_sequence() with .collect()
fn generate_sequence(limit: u8) -> Vec<u8> { 
    (1..=limit).collect()

}

fn output_sequence(numbers: &[u8]) {
    for n in numbers {
        println!("{}", n);
    }
}


#[test]
fn gnerate_sequence_should_work() {
    let result = generate_sequence(3);
    assert_eq!(result, &[1, 2, 3]);
}

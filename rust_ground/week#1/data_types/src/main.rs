// // fn main() {
    
// //     let tup = (500, 6.4, 1);

// //     let (x, y, z) = tup;

// //     println!("The value of y is: {}", y);

// //     println!("Hello, world!");
// // }

// use std::io;

// fn main() {
//     let a = [1, 2, 3, 4, 5];

//     println!("Please enter an array index.");

//     let mut index = String::new();

//     io::stdin()
//         .read_line(&mut index)
//         .expect("Failed to read line");

//     let index: usize = index
//         .trim()
//         .parse()
//         .expect("Index entered was not a number");

//     let element = a[index];

//     println!(
//         "The value of the element at index {} is: {}",
//         index, element
//     );
// }

fn main() {
    let x = 5;

    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);
}
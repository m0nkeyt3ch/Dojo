fn main() {
    println!("{}", remove_char("oops"));
}

pub fn remove_char(s: &str) -> String {

    //optimize
    s[1..s.len() - 1].to_string()
    // let mut new_str = s.to_string();
    // new_str.remove(0);
    // new_str.pop();
    // String::from(new_str.as_str())
}
// fn main() {
//     let mut a: String = "Rat".to_string();
//     println!("{}", a); // Rat

//     a.remove(0);
//     println!("{}", a); // at

//     a.pop();
//     println!("{}", a); // Ca

// }
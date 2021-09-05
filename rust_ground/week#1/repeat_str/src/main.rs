mod rep;

fn main() {
    //println!("Hello, world!");
    println!("{}",rep::repeat_str("hello ", 3));
    assert_eq!(rep::repeat_str("a", 4), "aaaa");
    assert_eq!(rep::repeat_str("hello ", 3), "hello hello hello ");
    assert_eq!(rep::repeat_str("abc", 2), "abcabc");
}

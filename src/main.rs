fn main() {
    let count : i8 = 12;
    let plural : &str;
    if count > 1 { plural = "s" } else { plural = "" };
    println!("I have {} apple{}", count, plural);
}

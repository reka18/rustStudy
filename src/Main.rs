use std::io;

// const DAYS_IN_WEEK: u8 = 7;

fn main() {
    let mut arr = vec![];
    arr.push("one");
    arr.push( "two");
    arr.push( "three");
    arr.push( "four");
    arr.push( "five");

    // let mut i = 0;
    // while i < 5 {
    //     print_arr(&arr);
    //     i += 1;
    // }

    for n in 1 .. 5 {
        print_arr(&arr);
    }
}

fn print_arr(arr: &[&str]) {
    for s in arr {
        println!("{}", s);
    }
}
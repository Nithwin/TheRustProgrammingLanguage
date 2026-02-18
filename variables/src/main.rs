// use std::io;

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}


fn main() {
    let s= String::from("hello world");

    let word = first_word(&s);

    //s.clear(); // error!

    println!("the first word is: {word}");
}

// fn main() {
//     let mut cur: u32 = 1;
//     let mut prev: u32 = 1;
//     let mut n: String = String::new();
//     io::stdin().read_line(&mut n).expect("Failed");
//     let n: u32 = n.trim().parse().expect("Failed");
//     for _ in 1..n {
//         let tmp = cur+prev;
//         prev = cur;
//         cur = tmp;
//         //print!("{cur} ");
//     }   
//     print!("{prev}");
// }


    // let mut x = 5;
    // println!("The value of x is: {x}");
    // x = 6;
    // println!("The value of x is: {x}");
    // let guess: u32 = "42".parse().expect("Not a number!");
    // println!("{guess}");
    // let arr = [1,2,3,4,5];
    // for i in arr {
    //     print!("{i}");
    //}
    // for i in (1..5).rev() {
    //     print!("{i} ");
    // }
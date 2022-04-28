// variables3.rs
// Make me compile! Execute the command `rustlings hint variables3` if you want a hint :)

fn main() {
    let mut x = 3;
    // let x = 3;
    println!("Number {}", x);
    x = 5; // don't change this line
    // let x = 5; // declaring a variable with the same name as another one is a SHADOWING, where the first named variable isnt overwritten
    println!("Number {}", x);
}

fn main() {
    let a_number = 10; // notice the `mut` keyword the solution for this is type "let mut a_number" 
    println!("the number is {}.", a_number);
    a_number = 15;
    println!("and now the number is {}.", a_number);
}
//this code cannot compile
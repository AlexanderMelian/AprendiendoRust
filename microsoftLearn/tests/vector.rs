fn main(){
    let mut vec = Vec::new();
    vec.push(1);
    println!("{}",vec[0]);
    vec.push(5);
    vec.push(6);
    vec.push(7);
    for x in &vec{
        println!("{}",x)
    }
}
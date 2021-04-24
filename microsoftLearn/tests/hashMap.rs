use std::collections::HashMap;


fn main(){
    let mut first_hm: HashMap<char,usize> = HashMap::new();
    
    first_hm.insert('c', 2);
    println!("Hola");
    println!("{:?}",first_hm.get(&'c'));

}
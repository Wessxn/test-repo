
use std::collections::HashMap;
   pub fn print_a_hashmap() {

    let field_name = String::from("Favorite color");
   let field_value = String::from("Blue");
    
    let mut map = HashMap::new();
    map.insert(field_name, field_value);
        
    map.entry(String::from("orange")).or_insert(String::from("Worse colour"));
    map.entry(String::from("Favorite color")).or_insert("Blue".to_string());
    println!("{:?}", map)
   } 
        
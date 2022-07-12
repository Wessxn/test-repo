pub fn third_element(v: &Vec<i32>) -> () {
    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }
}
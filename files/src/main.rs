#[derive(Debug)]



//C-like Struct 
struct Person {
  name: String,
  age: u8,
  race: String,
  place_of_birth: String,
}

//Tuple Struct
struct FavouriteColor(u32, u32, u32); 

 fn create_person(age: u8, place_of_birth: String, ) -> Person {
  Person {
    name: String::from("John Doe"),
    age, 
    race: "Unknown".to_string(),
    place_of_birth,
  }
 }



//Enums  
enum CharacterMovement{
    Forward,
    Backward,
    Left,
    Right
  }
  
  fn movement(x: CharacterMovement){
    match x{
      CharacterMovement::Forward => {
        println!("Moved Forward")
      },
      CharacterMovement::Backward => {
        println!("Moved Forwards ")
      }
      CharacterMovement::Left => {
        println!("Moved Left")
      },
      CharacterMovement::Right => {
        println!("Moved Right")
      }
    }
  }

  fn main() {
    let mut character = CharacterMovement::Forward; 
    movement(character);
  
    character = CharacterMovement::Backward;
    movement(character);
  
    character = CharacterMovement::Left;
    movement(character);
  
    character = CharacterMovement::Right;
    movement(character);

    let age = 18;
    let place_of_birth = "Japan".to_string();
    let person1 = create_person(age, place_of_birth); 
    let color: FavouriteColor = FavouriteColor(255, 0, 0);

    println!("{:?}", person1);
    println!("{}'s favourite color is {:?}", person1.name, color.1);
  
  }
  

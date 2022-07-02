//C-like Struct 
struct Person {
    name: String,
    age: u8,
    race: String,
    place_of_birth: String,
  }
  
  struct person {
    age: u8,
  }
  
//Person Method
  impl Person{
    fn double_area(&self) -> u8{
    self.age * self.age
    }
  
  fn is_adult(&self) -> bool{
    self.age > 18 
    }
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
  
//Enums practice 
enum CharacterMovement{
    Forward,
    Backward,
    Left,
    Right
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
  
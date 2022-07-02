mod::data_structures; 
mod::primitive_types;

fn main(){
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

  println!("{}", person1.name);
  println!("{0}'s favourite color is {:?}", person1.name, color);
  
}
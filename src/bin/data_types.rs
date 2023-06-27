//enum
    enum Direction {
        Up,
        Down,
        Left,
        Right, //comma on last value is optional
    }
//here the up down .. are called variants in an enum
    //lets say there is a fn that is acceseing these values from enum 

fn main() {
   let Go = Direction::Left;
   match Go{
     Direction::Left => println!("Go left"),
     Direction::Up => println!("Go U"),
     Direction::Down => println!("Go D"),
     Direction::Right => println!("Go Right"),
     
   } 

    
    
}

use std::fmt::Display;

pub trait Shape {
    fn cmp_display(&self) -> f32;
}
#[derive(Debug)]
struct Pair<T> {
    x: T,
    y: T,
    z: T
}
impl <T> Pair<T> {
    fn new(x: T,y: T,z: T) -> Self {
        Self{x,y,z}
    }
}
impl<T:Display + PartialOrd > Pair<T> {
    fn cmp_display(&self) {
        if (self.x > self.y) && (self.x > self.z) {
            println!("The largest member is x = {}", self.x);
        } else if (self.x < self.y) && (self.y> self.z) {
            println!("The largest member is y = {}", self.y);
        } else {
            println!("The largest member is z = {}",self.z); 
        }           
   }
}
fn main() {
    let pair = Pair {
        x: 34,
        y:67,
        z:7
    };
    println!("{:?}",pair);
        
    
}
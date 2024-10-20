fn main(){
 let mut x = 5;
//  println!("xp {}",*x);
 let y = &mut x;
 *y +=1;
 
*y +=1;

 println!("y {}",y);
 

 // can have one mutable reference or n immutable reference

}

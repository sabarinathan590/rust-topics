//1. each value has an owner
//2. There can be one owner at any time
//3. when the owner goes out of scope, the value will be dropped

fn main(){
    let a:String = String::from("RUST");
    let mut b = a;
    b.push_str(" WORLD");
    let c = &mut b;
    c.push_str("string");
    println!("Hi, {} and {}",b,c);
    

    // 1. here a is the owner of the string
    // let a:String = String::from("RUST");
    
    // 2. here value of a is borrowed to b 
    // b is the owner and if we try to access value in println it will throw compile time error
    // let a:String = String::from("RUST");
    // let b = a;
    // println!("sss{}",a);

    //3. if we access the value out of main method we will get error


    //additional point to note
    // string borrowed as reference, the borrower cannot change the value
    // if the ownership is transferred, and the new owner is defined as mutable, the value can be modified
    // once the value is referred through & , the value cannot be modified (unless it is a mutable reference)
    
    // immutable field cannot be shared as a mutable reference
    // mutable field can be shared as mutable reference so that the borrower can mutate the value



    let mut x = 5;
//  println!("xp {}",*x);
 let y = &mut x;
 *y +=1;
 
*y +=1;

 println!("y {}",y);
 

 // can have one mutable reference or n immutable reference
}

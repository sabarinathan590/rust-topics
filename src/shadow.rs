fn main(){

    // shadowing is not same as mutable.

    let x = 5; //x=5
    println!("x addr {:p}",&x);

    let x = x +1; //x=6
    println!("x addr {:p}",&x);
    {
        let x = x*2;
        println!("value of x in inner block is: {x}");
    }
    println!("value of x in outer block is: {x}");


    //mut
    let mut a:u16 = 20;
    println!("address of a is {:p}",&a);
    a=25;
    println!("address of a is {:p}",&a);
    

}
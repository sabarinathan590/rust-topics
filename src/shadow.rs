fn main(){
    let x = 5; //x=5
    let x = x +1; //x=6
    {
        let x = x*2;
        println!("value of x in inner block is: {x}");
    }
    println!("value of x in outer block is: {x}");
}
fn main() {
    //integers i and u
    // let _x:i8 = -44;
    // let _y:u64 = 44;
    // println!("Signed integer {}",_x);
    // println!("Unsigned integer {}",_y);


    //floating points
    // let pi:f64 = 3.14;
    // println!("floating point {}",pi);

    //boolean
    // let is_snow:bool=true;
    // println!(" boolean value {}",is_snow);


    //char
    // let first_char:char = 'A';
    // println!("first character {}",first_char);

    // let ss:String = String("sss");
    "".to_string();

    //Compound Datatype
    //Array
    // let nims: [i32; 3] = [1,3,4];
    // println!("Numbers {:?}",nims);

    //Tuples
    // let fruit :(&str,i32,bool) = ("Apple",20,true);



    // let human:(&str,i8,bool) = ("ssn",33,true);
    // println!("Hi {:?}",human);

    let mut string:String = String::from("Sabari");
    string.push_str(" Nathan");
    // println!("Hi {}",string);
    let name:&str = &string;
    println!("name :: {}",name);


    
}

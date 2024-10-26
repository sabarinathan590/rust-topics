fn main(){
    // let mut counter:i8 = 0;
    // loop {
    //     println!("Hello World !");
    //     counter+=1;
    //     if counter==10 {
    //         break;
    //     }
    // }

    // let mut counter:i32 = 0;

    // let loop_result = loop {
    //     counter+=1;

    //     if counter==10{
    //         break counter*2;
    //     }
    // };

    // println!("Loop result {loop_result}")


    //Loop Label
    /*
    When you have multiple loops, we can use loop label to break a specific loop
    using loop label
     */

    let mut counter:i32 = 0;

    let loop_result = 'count_up : loop {
        counter+=1;

        if counter==10{
            break 'count_up counter*2;
        }
    };

    println!("Loop result {loop_result}");


let mut while_counter = 0;
//While loop
while while_counter<3  {
    println!("While loop counter {while_counter}");
    while_counter+=1;
}

//Loop through a collection using for loop
let array = [2,32,56,22,67,23];

for element in array{
    println!("array value {element}");
}

}

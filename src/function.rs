fn main(){
    print_result(add(8, 8));
    let res:i8 = add(5, 6);
    print_result(res);
    println!("result :: {:.2}",bmi_calculator(73.0, 1.82));
}

fn bmi_calculator(weight_kg:f64,height_m:f64) -> f64 {
    return weight_kg/(height_m*height_m);
}

fn add(a:i8,b:i8) -> i8{
    a+b
}

fn print_result(result:i8){
    println!("Result is {}",result);
}
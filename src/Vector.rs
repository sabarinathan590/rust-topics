fn main(){
    let mut _v:Vec<i32> = Vec::new();
    let mut _v = vec![1,2,3,4];
    _v.push(5);
    _v.push(6);

    println!("values {:?}",_v);
    let third_value = _v[2];
    println!("third value {:?}",third_value);
    let val = &_v[2];
    println!("updated values {:?}",val);
    let val1 = _v.get(2);
    check_val(val1);
    let val2 = _v.get(20);
    check_val(val2);
    

}
fn check_val(val:Option<&i32>){
    match val {
        Some(s) => println!("value is {:?}",s),
        None => println!("invalid index")
    }
}
fn main() {
    let some_number = Some(5);
    let some_string = Some("a string");

    let absent_number: Option<i32> = None;  //널값처럼 이용하는 법
}


// fn main0(){
//     let x: i8 = 5;
//     let y: Option<i8> = Some(5);

//     let sum = x+y;
// } i8타입과 Option<i8>타입은 다름
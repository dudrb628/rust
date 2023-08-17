fn main() {
    println!("안녕하세요!");
    another_function(5, -6);

    let z = minus_seven();
    println!("z의 값: {}", z);

    let zz: u32 = zzis();
    println!("zz의 값: {}", zz.to_string());
}
fn minus_seven() -> i32 {
    -7 // return 명시구문이 없을 경우 마지막 표현식의 값이 return됨
}
fn zzis() -> u32 {
    return 49;
}
fn another_function(x: u32, y: i32) {
    println!("x의 값: {}", x);
    println!("y의 값: {}", y);
}

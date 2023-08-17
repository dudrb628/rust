fn main() {
    let number = 6;

    if number % 4 == 0 {
        println!("변수가 4로 나누어떨어집니다.");
    } else if number % 3 == 0 {
        println!("변수가 3으로 나누어떨어집니다.");
    } else if number % 2 == 0 {
        println!("변수가 2로 나누어떨어집니다.");
    } else {
        println!("변수가 4, 3, 또는 2로 나누어떨어지지 않습니다.");
    }

    let condition = true;
    let number2 = if condition { 5 } else { 6 };

    println!("number2의 값: {}", number2);
}

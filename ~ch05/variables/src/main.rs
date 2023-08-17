fn main() {
    let mut x = 5; // 변수      mut은 가변
    println!("x의 값: {}", x);
    x = 6;
    println!("x의 값: {}", x); // 얘는 변수
    println!("최고 점수: {}", MAX_POINTS); // 얘는 상수. 무조건 불변
    println!("최저 점수: {}", MIN_POINTS); // 얘는 상수. 무조건 불변
    shadowed();
}

const MAX_POINTS: u32 = 100_10000; //상수    u는 부호 없는 정수(자연수)
const MIN_POINTS: i32 = -100_1000; //상수   i는 부호 있는 정수(정수)

fn shadowed() {
    let x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("x의 값: {}", x);
    mutlet()
}

fn mutlet() {
    let spaces = "       ";
    let spaces = spaces.len(); // length 함수
    println!("공백길이 = {}", spaces);
    introduceArray();
}

fn introduceArray() {
    let months = [
        //크기는 불변. 가변으로 사용하고 싶다면 Vector를 써야 함
        "January",
        "Februry",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    println!("{}", months[0]);

    let a: [i32; 5] = [1, 2, 3, 4, 5]; // a = [1,2,3,4,5]
    let b = [3; 5]; // b = [3,3,3,3,3]
}

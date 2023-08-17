// 자리지정자: 대다수의 경우를 처리하고 싶지 않을 때.
fn main() { 
    let mut some_u8_value = 0u8;
    some_u8_value=118;  
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => println!("What the"),  // _ 패턴 : 모든 값에 일치. match 표현식의 가장 마지막에 추가하면 앞에 나열했던 패턴에 일치하지 않는 나머지 모든 패턴에 일치하게 된다.
    }   
    // 마치 Switch case문 같네
}


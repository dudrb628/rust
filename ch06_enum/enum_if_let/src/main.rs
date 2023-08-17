fn main() {
    println!("Hello, world!");
    //let some_u8_value = Some(0u8);
    let some_u8_value = Some(3); // 이 값일 때만 아래 three 출력 가능
    match some_u8_value{
        Some(3) => println!("three"),   
        _=> (),
    }

    // 위 보다 더 간결하게 가능
    if let Some(3) = some_u8_value{
        println!("three");
    }
    
}

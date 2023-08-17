enum Coin {
    Penny,
    Nickle,
    Dime,
    Quarter,
}
fn value_in_cents(coin: Coin) -> u32{
    match coin {
        Coin::Penny => 1,   //패턴 가지에 연관된 코드가 짧은 경우 중괄호를 사용하지 않지만
        Coin::Nickle => 5,
        Coin::Dime => {
            println!("다임");
            10},    //여러 줄의 코드를 실행하고자 한다면 중괄호를 사용해야함
        Coin::Quarter => 25,
    }
}

fn main() {
    let someCoin = Coin::Dime;
        println!("{}",value_in_cents(someCoin));
}


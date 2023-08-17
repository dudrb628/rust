#[derive(Debug)]    // 저장된 주 이름을 쉽게 디버깅&할 수 있다.
enum UsState {
    Alabama, Alaska,
    // 등등 미국의 주. 더 추가하면 좋음. 25센트 동전만 50개 주 별로 달랐던 경우의 예시를 위함
}
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u32{
    match coin{
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        },
    }
}

// 25센트 동전을 출시한 주를 출력하는 동시에 나머지 동전의 개수를 세기
fn value_in_cents0(coin: Coin){ 
    let mut count = 0;
    match coin {
        Coin::Quarter(state) => println!("{:?}주의 25센트 동전!", state),
        _ => count += 1,
    }
    //혹은
    let mut count0 = 0;
    if let Coin::Quarter(state) = coin{     // 여기서 나는 state 에러는 바로 위에서 데이터를 소모했기 때문
        println!("{:?}주의 25센트 동전!", state);
    }   else {
        count0 += 1;
    }
}


    fn main() {
        let sexy_coin = Coin::Quarter(UsState::Alabama);
        value_in_cents(sexy_coin);
    }
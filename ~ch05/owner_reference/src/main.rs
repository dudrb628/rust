fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);
    // 참조표시 &.  본래 소유권을 잃으면서 s1을 더이상 못 쓰지만 참조&가 있기 때문에 계속해서 쓰기 가능. 단 그 함수의 매개변수 자료형 앞에도 &가 있어야 함

    println!("'{}'의 길이는 {}입니다.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    // 매개변수 s가 String에 대한 참조
    //이 함수에서 참조를 함.
    //여기에도 s의 자료형 String에 &가 붙어있음

    s.len()
} //이 시점에서 변수 s가 범위를 벗어난다.
  //하지만 이 변수는 자신이 가리키는 값에 대한 소유권이 없으므로 아무 일도 일어나지 않는다.

//이렇게 함수 매개변수로 참조를 전달하는 것을 대여(borrowing)라고 함

fn main() {
    // 죽은 포인터: 이미 해제되어 다른 정보를 저장하도록 변경된 메모리를 계속해서 참조하는 포인터. 러스트의 경우 컴파일러가 방지함

    // let reference_to_nothing = dangle(); //이 함수의 리턴 타입은 대여한 값을 리턴하고자 하지만 실제로 대여해 올 값이 존재하지 않음
    let reference_to_nothing0 = dangle0(); //이 함수의 리턴 타입은 대여한 값을 리턴하고자 하지만 실제로 대여해 올 값이 존재하지 않음
    println!("{}", reference_to_nothing0);
}
// fn dangle() -> &String { // dangle 함수는 String에 대한 참조를 리턴한다.
//     let s = String::from("hello");   // String 타입의 새로운 변수 s를 생성한다.

//     &s   // String 타입의 변수 s에 대한 참조를 리턴한다.
// }    // 하지만 이 지점에서 변수 s가 범위를 벗어나므로 drop 함수가 호출되고 메모리가 해제된다.
// 따라서 이 함수는 에러의 위험을 내포하고 있다.
fn dangle0() -> String {
    let s = String::from("hello");

    s
} // 이 코드는 소유권이 함수를 호출한 코드로 이동하기 때문에 메모리가 해제되지 않기 때문에 아무 문제 없음

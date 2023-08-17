// use std::any::Any;

fn main() {
    // 변수 s를 아직 선언하지는 않았으므로 변수가 유효하지 않다
    let s = "hi"; // 변수는 이 지점부터 유효하다.
    println!("{}", s);
    // 변수 s를 이용해 필요한 동작을 수행한다.
    sub();
} // 여기서 범위를 벗어나므로 변수 s는 이제 유효하지 않다.

fn sub() {
    let mut t = String::from("hello");
    t.push_str(", world!");
    println!("{}", t);
    
    Move();
    Clone();
    Copy();
}

fn Move() {
    let x = 5;
    let y = x;

    let s1 = String::from("hello");
    let s2 = s1;

    // println!("{}, world!", s1);  얘는 안됨. 메모리 소유권이 s2로 이동했기 때문에. 정수형처럼 스택에 저장되는 것과 달리 스트링은 힙 영역에 저장되므로 소유권의 중복을 방지하기 위해. 자세한 사항은 p.78 참조
    println!("{}, world!!", s2);
}
fn Clone() {
    //힙 메모리에 저장된 데이터를 복사하기 원할 때 쓰는 메서드 clone();
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);
}

fn Copy() {
    //
    let x = 5;
    let y = x;

    println!("x = {}, y = {}", x, y);
}

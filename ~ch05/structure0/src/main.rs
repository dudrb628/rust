//튜플 구조체
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}

//구조체 데이터의 소유권은 구조체가 가진다.
//구조체에 다른 변수가 소유한 데이터의 참조를 저장할 수는 있지만 수명(lifetimes)을 사용해야함. 그렇지 않으면 아래 코드의 에러가 뜸. p.106참조
// struct User {
//     username: &str,
//     email: &str,
//     sign_in_count: u64,
//     active: bool,
// }

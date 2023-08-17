

struct QuitMessage; //유닛 구조체
struct MoveMessage{
    x: i32,
    y: i32,
}
struct WriteMessage(String);    // 튜플 구조체
struct ChangeColorMessage(i32, i32, i32);   // 튜플 구조체


// 위와 같은 구조체를 열거형으로 쓰면
enum Message {
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {//열거자도 메서드를 정의할 수 있다
    fn call(&self){
        //메서드 본문
    }
}
fn main()
{
    let m = Message::Write(String::from("hello"));
    m.call();
}
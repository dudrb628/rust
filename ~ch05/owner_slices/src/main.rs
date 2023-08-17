// 문자열에서 첫 번째 단어만 리턴하는 함수 작성하기
fn main() {
    let mut s = String::from("Sex With Clare");
    let word = first_word(&s); // 변수 word에는 3이 할당된다.
    s.clear(); // 문자열을 비워 빈 문자열("")을 만든다.
               // 변수 word에는 여전히 3이 할당되었지만, 이 값을 적용할 문자열이 더는 없다.
               // 그래서 변수 word는 아무 쓸모가 없게 된다.

    main0();

    let mut s0 = String::from("hello world");
    let word0 = first_word0(&s0);
    println!("{}", word0);

    main1();
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        //enumerate() 메서드는 튜플을 리턴함. 그러므로 튜플을 해제하는 패턴(i, &item)을 사용함.
        //이때 메서드가 리턴하는 요소에 대한 참조를 얻어와야 하므로 패턴에서 & 기호를 사용함
        if item == b' ' {
            //마치 contain같네 b' '
            return i;
        }
    }
    s.len()
}
fn first_word0(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
fn first_word1(s: &str) -> &str {
    // 문자열 슬라이스를 매개변수로 사용하는 함수
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

fn main0() {
    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];

    let slice0 = &s[0..2];
    let slice0 = &s[..2]; // 같은 말임. 시작을 생략할 경우 default값 = 0

    let slice1 = &s[3..s.len()];
    let slice1 = &s[3..]; // 같은 말임. 끝을 생략할 경우 default값 = 길이

    let slice2 = &s[0..s.len()];
    let slice2 = &s[..]; // 같은 말임. 위와 같은 맥락으로, 시작과 끝을 생략할 경우 전체를 슬라이스로 참조
}

fn main1() {
    let my_string = String::from("hello world");

    // first_word 함수에 String 타입으로부터 생성한 문자열 슬라이스를 전달한다.
    let word = first_word1(&my_string[..]);

    let my_string_literal = "hello world";

    // first_word 함수에 문자열 리터럴의 슬라이스를 전달한다.
    let word = first_word1(&my_string_literal[..]);

    // 문자열 리터럴은 이미 문자열 슬라이스이기 때문에 아래의 코드는 슬라이스 문법 없이도 정상적으로 동작한다.
    let word = first_word1(my_string_literal);
}

fn other_type_slice() {
    //컬렉션에도 활용 가능
    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];
}

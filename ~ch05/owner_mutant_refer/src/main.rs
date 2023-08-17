fn main() {
    let s = String::from("hello");
    change(&s);

    main0(); // 가변 참조
             // main1();
    main2();
    // main3();
}
fn change(some_string: &String) {
    // some_string.push_str(", world!");   // 에러남. 참조하는 값을 변경할 수 없음
}

fn main0() {
    //가변 참조
    let mut s = String::from("hello");
    println!("main0_0 {}", s);
    change0(&mut s);
    println!("main0_1 {}", s);
}
fn change0(some_string: &mut String) {
    some_string.push_str(", world");
}
// fn main1() {
//     // 가변 참조는 동시에 한 개의 데이터만 가능. 데이터 경합을 방지하기 위한 러스트 제약(p.89 참조)
//     let mut s = String::from("hello");

//     let r1 = &mut s;
//     let r2 = &mut s; // 컴파일 실행하면 에러(E0499])가 나옴.
//     println!("{}, {}", r1, r2);
// }
fn main2() {
    let mut s = String::from("hello");
    {
        let r1 = &mut s;
    } //이 지점에서 변수 r1이 범위를 벗어나므로 이 다음부터는 새로운 참조를 또 생성할 수 있다.
    let r2 = &mut s;
}
// fn main3() {
//     let mut s = String::from("hello");

//     let r1 = &s; //아무 문제 없음
//     let r2 = &s; //아무 문제 없음
//     let r3 = &mut s; //문제가 발생함
//     println!("{}, {}, and {}", r1, r2, r3); // 불변참조된 값을 변경해서는 안 되기 때문에
// }

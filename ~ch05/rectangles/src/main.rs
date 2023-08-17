fn main() {
    let width1 = 30;
    let height1 = 50;

    println!("사각형의 면적: {} 제곱 픽셀", area(width1, height1));
    main0();
    main1();
    main2();
}
fn area(width: u32, height: u32) -> u32 {
    width * height
}

//튜플을 이용한 리팩토링
fn main0() {
    let rect1 = (30, 50);
    println!("(튜플) 사각형의 면적 : {} 제곱 픽셀", area0(rect1));
}
fn area0(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

//구조체를 이용한 리팩토링
struct Rectangle {
    width: u32,
    height: u32,
}
fn main1() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!("(구조체) 사각형의 면적: {} 제곱 픽셀", area1(&rect1)); //참조가 없어도 되지만 소유권의 안정성을 위해(main1함수가 소유권을 유지하기 위해) 참조하는 게 좋음
}
fn area1(rectangle: &Rectangle) -> u32 {
    //참조가 없어도 되지만 소유권의 안정성을 위해(main1함수가 소유권을 유지하기 위해) 참조하는 게 좋음
    rectangle.width * rectangle.height
}

// 구조체 디버그
#[derive(Debug)]
struct Rectangle2 {
    width: u32,
    height: u32,
}
fn main2() {
    let rect1 = Rectangle2 {
        width: 30,
        height: 50,
    };

    println!("(구조체 디버그)rect1: {:?}", rect1);
    println!("(구조체 디버그)rect1: {:#?}", rect1);
}

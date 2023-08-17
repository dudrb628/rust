#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, targetRect : &Rectangle) -> bool{
        if self.width >= targetRect.width && self.height >= targetRect.height{
            return true;
        }
        else{
            return false;
        }
    }//정답과 근사하나 더 짧게 가능. 한 줄. self.width>targetRect.width && self.height >targetRect.Height
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };
    
    println!("사각형의 면적: {} 제곱 픽셀", rect1.area());
    println!("rect1은 rect2를 포함하는가? {}", rect1.can_hold(&rect2));
    println!("rect1은 rect3을 포함하는가? {}", rect1.can_hold(&rect3));
    main0();
}

struct Rectangle0 {
    width: u32,
    height: u32,
}

impl Rectangle0 {
    fn Squeare(size : u32) -> Rectangle{
        Rectangle{width:size, height: size}
    }
}
fn main0(){
    let sq = Rectangle0::Squeare(3);
}



struct Rectangle1 {
    width: u32,
    height: u32,
}
impl Rectangle1{
    fn area(&self) ->u32{
        self.width*self.height
    } 
}
impl Rectangle1{
    fn can_hold(&self, targetRect: &Rectangle1) -> bool{
        self.width > targetRect.width && self.height > targetRect.height
    }
} 
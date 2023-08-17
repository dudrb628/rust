enum IpAddrKind{
    V4,
    V6,
}
//이 스크립트는 사실, 구조체를 사용한 예임. 6-1(p.121). 열거를 사용한 예는 다음(enum2) 스크립트 참조
struct IpAddr{
    kind: IpAddrKind,
    address: String,
}

fn main(){
    let home = IpAddr{
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr{
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };
}
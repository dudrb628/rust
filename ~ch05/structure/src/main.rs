struct User {
    //구조체 정의
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    println!("Hello, world!");
    let user1 = User {
        //불변
        email: String::from("someone@example.com"),
        username: String::from("Clare"),
        active: true,
        sign_in_count: 1,
    };
    let mut user2 = User {
        //가변
        email: String::from("someone0@example.com"),
        username: String::from("Leon"),
        active: true,
        sign_in_count: 2,
    };
    user2.email = String::from("handsomeLeon12@example.com");
    let user3 = User {
        email: String::from("another@example.com"),
        username: String::from("Criss567"),
        active: user1.active,
        sign_in_count: user1.sign_in_count,
    };
    let user4 = User {
        // ..문법은 명시적으로 나열하지 않은 나머지 필드에 대해서는 기존의 인스턴스의 필드와 같은 값을 사용하라는 의미
        email: String::from("anotherAgain@example.com"),
        username: String::from("Jill567"),
        ..user1
    };
}

fn build_user(email: String, username: String) -> User {
    User {
        username: username,
        email: email,
        sign_in_count: 1,
        active: true,
    }
}
fn build_user0(email: String, username: String) -> User {
    User {
        // 필드 초기화 단축 문법
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

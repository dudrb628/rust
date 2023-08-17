fn main() {
    // loop {
    //     println!("다시 실행~!");
    //&// }
    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {}", result);

    example_while();
}

fn example_while() {
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number = number - 1;
    }

    println!("발사!");

    example_for();
}

fn example_for() {
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("요소의 값: {}", element);
    }

    //리버스
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("발사!");
}

fn main() {
    const MAX_POINT: u32 = 100_000;
    println!("상수값: {}", MAX_POINT);

    let mut x = 5;
    println!("x의 값: {}", x);

    x = 6;
    println!("x의 값: {}", x);

    let spaces = "      ";
    let spaces = spaces.len();
    println!("{}", spaces); // 6

    let _guess: u32 = "42".parse().expect("숫자가 아닙니다.");
    // help: if this is intentional, prefix it with an underscore: `_guess`

    let tup = (500, 6.4, 1);
    // 튜플은 destruct 구문을 사용할 수도 있다
    let (x, y, z) = tup;
    println!("x: {}, y: {}, z: {}", x, y, z);

    // 또한 튜플은 각 원소의 명시적 타입 정의가 가능하다
    let tup2: (i32, f64, u8) = (500, 6.4, 1);
    println!("{}", tup2.0);

    let array = [1, 2, 3, 4, 5];
    println!("{}", array[array.len() - 1]);
    println!("{}", array[array.len() / 2]);

    another_function(32, 16);
    println!("five()의 값: {}", five());

    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("결과값은 {}", result);
    while_example();
    for_example();
    for_example_two();
}

// 러스트의 함수는 대부분 snake case를 사용한다
fn another_function(x: i32, y: i32) {
    println!("함수에 사용된 인자 x의 값 : {}, y의 값 : {}", x, y);
}

fn five() -> i32 {
    5
}

fn while_example() {
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }
    println!("완료!")
}

fn for_example() {
    let a = [10, 20, 30, 40, 50];

    for elem in a.iter() {
        println!("요소의 값: {}", elem);
    }
}

fn for_example_two() {
    for number in (1..4).rev() {
        println!("요소의 값: {}", number);
    }
}

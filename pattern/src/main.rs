fn main() {
    // 1. if let
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "32".parse();

    if let Some(color) = favorite_color {
        println!("선호하는 {}색을 배경으로 사용합니다.", color);
    } else if is_tuesday {
        println!("화요일엔 녹색");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("보라색을 배경으로");
        } else {
            println!("오렌지색 배경");
        }
    } else {
        println!("파란색 배경");
    }

    // 2. while let
    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{}", top);
    }

    let v = vec!['a', 'b', 'c'];

    for (index, value) in v.iter().enumerate() {
        println!("인덱스 {}의 값: {}", index, value);
    }

    // 3. let & match
    let point = (3, 5);
    print_coordinates(&point);

    let x = 1;
    match x {
        1 => println!("하나"),
        2 => println!("둘"),
        3 => println!("셋"),
        _ => println!("나머지"),
    }

    let y = Some(5);
    let z = 10;

    match y {
        Some(50) => println!("50"),
        Some(y) => println!("일치, y = {:?}", y),
        _ => println!("불일치, z = {:?}", z),
    }

    println!("결과: z = {:?}, y = {:?}", z, y);

    match x {
        1 | 2 => println!("1 또는 2"),
        _ => println!("그 외"),
    }
    match x {
        1..=5 => println!("1 ~ 5 사이 값"),
        _ => println!("그 외 나머지 값"),
    }

    let c = 'c';

    match c {
        'a'..='j' => println!("아스키 문자의 전반부"),
        'k'..='z' => println!("아스키 문자의 후반부"),
        _ => println!("그 외 나머지 값"),
    }

    // 4. 구조체의 해체
    let p = Point { x: 0, y: 7 };
    match p {
        Point { x, y: 0 } => println!("x축 {}에 위치하는 점", x),
        Point { x: 0, y } => println!("y축 {}에 위치하는 점", y),
        Point { x, y } => println!("({}, {})에 위치하는 점", x, y),
    }

    // 5. 열거자의 해체
    let msg = Message::Move { x: 160, y: 255 };

    match msg {
        Message::Quit => {
            println!("Quit: 해체할 값이 없습니다.");
        }
        Message::Move { x, y } => {
            println!("Move: x= {}, y = {}", x, y);
        }
        Message::Write(text) => println!("Write: {}", text),
        _ => {}
    }

    let msg_color = Message::ChangeColor(Color::Hsv(0, 160, 255));

    match msg_color {
        Message::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("ChangeColor: R({}), G({}), B({})", r, g, b);
        }
        Message::ChangeColor(Color::Hsv(h, s, v)) => {
            println!("ChangeColor: H({}), S({}), V({})", h, s, v);
        }
        _ => {}
    }

    foo(3, 4);
    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("이미 설정된 값을 덮어 쓸 수 없습니다.");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }
    println!("현재 설정: {:?}", setting_value);

    let numbers = (2, 4, 6, 8, 10);
    match numbers {
        (one, _, three, _, five) => {
            println!("숫자 세 개: {}, {}, {}", one, three, five);
        }
    };

    let rectangle = Rectangle {
        width: 120,
        height: 240,
        z_index: 999,
        x_axis: 50,
        y_axis: 50,
    };
    match rectangle {
        Rectangle { width, height, .. } => println!("넓이: {}, 높이: {}", width, height),
    }

    let num = Some(4);
    match num {
        Some(x) if x < 5 => println!("5보다 작은 값: {}", x),
        Some(x) => println!("{}", x),
        None => (),
    }

    let henry = Name::Gender { code: 10 };
    match henry {
        Name::Gender {
            code: code_variable @ 10..=19,
        } => {
            println!("henry의 성별 코드는 {}", code_variable)
        }
        Name::Gender { code: 20..=29 } => {
            println!("성별 코드가 여성입니다")
        }
        Name::Gender { code } => {
            println!("다른 코드를 찾았습니다. {}", code)
        }
    }
}

fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("현재 위치: ({}, {})", x, y);
}

struct Point {
    x: i32,
    y: i32,
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
}

enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

fn foo(_: i32, y: i32) {
    println!("이 함수는 매개변수로 y({})만 사용한다", y);
}

struct Rectangle {
    width: i32,
    height: i32,
    z_index: i32,
    x_axis: i32,
    y_axis: i32,
}

enum Name {
    Gender { code: i32 },
}

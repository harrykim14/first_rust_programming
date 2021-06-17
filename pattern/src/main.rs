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
}

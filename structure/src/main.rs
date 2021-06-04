/*
1. 일반적인 함수 (리팩토링 전)
fn area(width: u32, height: u32) -> u32 {
    width * height
}
*/

/*
2. 튜플을 이용한 리팩토링
fn area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}
*/

/*
3. 구조체를 이용한 리팩토링
fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
*/

// 러스트는 디버깅 정보를 출력하는 기능을 제공함
// 구조체는 이 기능을 명시적으로 구현해주어야 함
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// 함수를 구조체 내에 정의하기 -> prototype과 비슷
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // self 매개변수를 사용하지 않는 함수(연관 함수)의 정의
    // 이 연관함수는 구조체의 새로운 인스턴스를 리턴하는 생성자를 구현할 때 자주 사용됨
    fn square(width: u32, height: u32) -> Rectangle {
        Rectangle { width, height }
    }
}

fn main() {
    // let width1 = 30;
    // let height1 = 50;
    // rect1 = (30, 50);
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    // println!("rect1: {}", rect1);
    // `Rectangle` cannot be formatted with the default formatter

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };

    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };
    // println!("사각형의 면적: {} 제곱 픽셀", area(width1, height1));
    // println!("사각형의 면적: {} 제곱 픽셀", area(rect1));
    // println!("사각형의 면적: {} 제곱 픽셀", area(&rect1));
    println!("사각형의 면적: {} 제곱 픽셀", rect1.area());
    println!("rect1은 rect2를 포함하는가? {}", rect1.can_hold(&rect2));
    println!("rect1은 rect3를 포함하는가? {}", rect1.can_hold(&rect3));

    // 연관 함수는 :: 문법을 사용할 수 있다
    let sq = Rectangle::square(30, 50);
    println!("sq의 크기 : {} 제곱 픽셀", sq.area());
}

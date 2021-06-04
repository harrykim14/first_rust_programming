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

// 3. 구조체를 이용한 리팩토링
fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    // let width1 = 30;
    // let height1 = 50;
    // rect1 = (30, 50);
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    // println!("사각형의 면적: {} 제곱 픽셀", area(width1, height1));
    // println!("사각형의 면적: {} 제곱 픽셀", area(rect1));
    println!("사각형의 면적: {} 제곱 픽셀", area(&rect1));
}

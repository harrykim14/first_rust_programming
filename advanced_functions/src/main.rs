use std::ops::Add;
use std::slice;

static HELLO_WORLD: &str = "안녕하세요!";
static mut COUNTER: u32 = 0;
fn main() {
    // 참조에서 원시 포인터 생성
    let mut num = 5;
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    // 임의의 메모리에 접근하는 원시 포인터
    let address = 0x012345usize;
    let _r = address as *const i32;

    // unsafe 블록 안에서 원시 포인터 역참조하기
    unsafe {
        println!("r1 = {}", *r1);
        println!("r2 = {}", *r2);
        dangerous();
    }

    // 안전한 split_at_mut 함수 사용 예
    let mut v = vec![1, 2, 3, 4, 5, 6];
    let r = &mut v[..];

    let (a, b) = r.split_at_mut(3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);

    /*
    let rm = address as *mut i32;

    let slice: &[i32] = unsafe {
        slice::from_raw_parts_mut(r, 10000)
    }
    // 이 함수는 임의의 메모리에 위치한 데이터를 소유하지 않으므로
    // 이 코드가 생성하는 슬라이스는 올바른 i32 값을 포함하는지 보장할 수 없다
    // 따라서 결과를 예측할 수 없으며 컴파일 타임에 어느 정도의 크기가 되는지도 보장할 수 없으므로 에러를 표시한다
    */

    // 외부 코드를 호출하기 위한 extern 함수
    unsafe {
        println!("C언어에 따르면 -3의 절대값은 {}이다", abs(-3));
    }

    println!("{}", HELLO_WORLD);
    add_to_count(5);
    unsafe {
        println!("가변 변수 COUNTER: {}", COUNTER);
    }

    // + 연산자 기능을 오버로딩하기
    assert_eq!(
        Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
        Point { x: 3, y: 3 }
    );
}

unsafe fn dangerous() {
    println!("이 함수는 unsafe 내에서 실행됩니다.");
}

// 안전하지 않은 코드를 이용해 split_at_mut 함수를 구현
fn _split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    let ptr = slice.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.offset(mid as isize), len - mid),
        )
    }
}

extern "C" {
    fn abs(input: i32) -> i32;
}

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;
    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

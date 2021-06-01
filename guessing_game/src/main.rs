use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("숫자를 맞혀봅시다!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("사용자가 맞춰야 할 숫자: {}", secret_number);

    loop {
    println!("정답이라고 생각하는 숫자를 입력하세요.");
    // let은 변수를 생성하는 구문
    // 러스트에서 변수는 기본적으로 값을 변경할 수 없다 
    // 변수명 이전에 mut 키워드를 사용하면 가변 변수를 생성할 수 있다
    // String은 표준 라이브러리가 제공하는 문자열 타입으로 UTF-8 형식으로 인코딩된 텍스트를 표현한다
    // :: 문법은 new 함수가 String타입의 연관 함수라는 점을 의미함
    // 연관 함수는 특정한 인스턴스가 아니라 타입 자체에 구현된 함수로 '정적 메서드'라고도 부른다
    let mut guess = String::new();
    
    // io의 연관함수(::)인 stdin 함수를 호출하면 입력값을 읽을 수 있다
    // use std::io를 명시하지 않았다면 std::io::stdin과 같이 작성해도 된다
    io::stdin().read_line(&mut guess)
        .expect("입력한 값을 읽지 못했습니다.");
    // read_line() 메서드는 사용자가 입력한 값을 문자열에 대입함과 동시에 io::Result 타입의 값을 리턴하기도 함
    // 러스트는 표준 라이브러리 안에 범용의 Result 타입을 비롯해서 여러개의 Result 타입을 정의하고 있다

    // 러스트에서는 placeholder를 {}로 사용하며 여러개의 변수도 출력 할 수 있다
    println!("입력한 값: {}", guess);

    // guess는 String::new() 로 생성한 문자열이지만 secret_number는 32bit 정수이기 때문에 타입 불일치가 일어남
    // 러스트의 숫자타입에는 i32, u32, i64 등이 있으며 기본적으로 i32 타입을 사용한다
    // 타입 일치를 위해 guess 변수에 guess.trim().parse()의 결과를 바인딩하며 u32로 타입을 정의한다
    // 이 때 아무것도 입력하지 않으면 ParseIntError { kind: Empty } 가 출력되며 강제 종료된다
    // 이 예외를 처리하기 위해 parse() 메서드에 Ok, Err를 처리하는 구문을 추가한다
    let guess: u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => continue,
    };
        
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("입력한 숫자가 작습니다!"),
            Ordering::Greater => println!("입력한 숫자가 큽니다!"),
            Ordering::Equal => { 
                println!("정답!");
                break;
            }
        }
    }
}

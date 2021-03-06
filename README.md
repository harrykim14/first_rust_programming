## 러스트 배워보기

with 러스트 프로그래밍 공식 가이드 (스티브 클라브닉, 캐롤 니콜스 저 / 장현희 번)

### Chapter 1. 시작하기

<details>
<summary>열기</summary>
<div markdown="1">

1.1 설치하기

- rustup을 이용해 러스트를 내려받기
- 윈도우에 rustup 설치하기

1. [러스트 공식 문서](https://www.rust-lang.org/tools/install)에 설명된 단계를 따라 설치하기
2. 러스트를 설치하기 전에 미리 Visual Studio의 C++를 사용한 데스크톱 개발 패키지와 MSVCv142, Win10 SDK, CMake 도구를 설치하기
3. 환경 변수 내 Path에 `%USERPROFILE%\.cargo\bin` 를 추가하기
4. 러스트가 설치됐다면 `rustup update`를 실행하여 최신 버전을 체크하고 `rustc --version`, `cargo --version`, `rustup --version`을 각각 체크하여 최신 버전인지 체크하기
5. VS code에서 Code Runner, Rust 확장 프로그램을 설치하기
6. 러스트용 폴더를 추가하고 main.rs 파일을 생성, 아래와 같이 적는다

```rust
fn main() {
    println!("Hello, world!")
}
```

7. 오른쪽 클릭하여 실행하고 `Hello, world!`와 `[Done] exited with code=0 in 0.884 seconds`와 같은 문구가 정상적으로 출력됐다면 환경설정을 완료한 것이다
8. Cargo.toml이 없다는 오류가 뜰 경우에는 `cargo init` 명령어를 통해 Cargo.lock과 Cargo.toml를 생성하기
9. 릴리즈를 위한 빌드는 `cargo build --release`로 실행할 수 있다
</div>
</details>

### Chapter 2. 숫자 맞히기 게임의 구현

<details>
<summary>열기</summary>
<div markdown="2">

- 숫자 맞히기 게임을 구현하기 위한 첫 번째 단계는 플레이어에게 입력할 값을 묻고, 이 입력을 처리하고 이 값이 원하는 형태인지를 확인하는 것

```rust
use std::io;

fn main() {
    println!("숫자를 맞혀봅시다!");

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

    // 러스트에서는 placeholder를 {}로 사용한다
    println!("입력한 값: {}", guess);
}
```

- 크레이트는 소스 파일의 집합
- Cargo.toml 파일을 아래와 같이 수정한 후 `cargo build` 명령어를 실행하면 cargo는 해당 크레이트를 추가함

```
[dependencies]

rand = "0.6.1"
```

- `cargo build` 명령어를 실행하면 최초에 한해 Cargo.lock 파일을 생성함
- cargo는 `cargo build`가 실행될 때 Cargo.lock 파일에 기록된 의존 패키지의 버전을 사용함
- `cargo update` 명령어를 실행하면 시맨틱 버전으로 기록된 rand 패키지보다 최신 버전이 있다면 해당 버전을 다운로드 받게 됨
- 이 때 Cargo.lock에 저장되기 때문에 Cargo.toml에는 시밴틱 버전인 "0.6.1"이 그대로 적혀있고 Cargo.lock에는 "0.6.5"가 기록됨

```rust
use rand::Rng;

fn main() {
    // Rng 트레이트에서 thread_rng() 메서드를 호출하여 1~100 사이의 값을 랜덤으로 생성한다
    // 해당 변수는 불변 변수여야 하므로 mut 키워드 없이 사용한다
    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("사용자가 맞춰야 할 숫자: {}", secret_number);
    // ... 이하 동일
}
```

- 이제 입력받은 수도 있으며, 랜덤 생성한 수도 있으므로 이 두 값을 비교해야 한다

```rust
use std::cmp::Ordering;

fn main() {
    //...

    // guess는 String::new() 로 생성한 문자열이지만 secret_number는 32bit 정수이기 때문에 타입 불일치가 일어남
    // 러스트의 숫자타입에는 i32, u32, i64 등이 있으며 기본적으로 i32 타입을 사용한다
    // 타입 일치를 위해 guess 변수에 guess.trim().parse()의 결과를 바인딩하며 u32로 타입을 정의한다
    let guess: u32 = guess.trim().parse()
        .expect("입력한 값이 올바른 숫자가 안닙니다.");

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("입력한 숫자가 작습니다!"),
        Ordering::Greater => println!("입력한 숫자가 큽니다!"),
        Ordering::Equal => println!("정답!"),
    }

    // ...
}
```

- 이제 두 수의 비교를 반복문을 통해 정답을 출력할 때 까지 반복해야 함
- 또한 두 수가 같다면 "정답!"을 출력하고 반복문을 종료해야 하므로 break; 구문을 넣기

```rust
loop {
    // ...
    match guess.cmp(&secret_number) {
        Ordering::Less => println!("입력한 숫자가 작습니다!"),
        Ordering::Greater => println!("입력한 숫자가 큽니다!"),
        Ordering::Equal => {
            println!("정답!");
            break;
        }
    }
    // ...
}
```

- 나머지 개선점 : 숫자가 아닌 입력을 무시하고 재입력을 요청하기

```rust
// ...
let guess: u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => continue,
    };
// ...
```

</div>
</details>

### Chapter 3. 일반 프로그래밍 개념

<details>
<summary>열기</summary>
<div markdown="3">

- 러스트에도 개발자가 사용할 수 없는 예약된 키워드들이 있고 이 키워드들은 변수나 함수의 이름으로 사용할 수 없다

**3-1. 변수와 가변성**

```rust
fn main() {
    let x = 5;
    println!("x의 값: {}", x);

    x = 6;
    println!("x의 값: {}", x);
}
```

- 해당 파일을 실행하면 `cannot assign twice to immutable variable`라는 오류문을 볼 수 있다
- 출력된 오류에 따라서 변수 x를 가변 변수로 정의하면 실행된다

**3.2.1 러스트의 데이터 타입**

- 정수 타입: 각 타입은 부호가 있거나 없으며 명시적인 크기가 정해져있다

| 크기  | 부호 있음 | 부호 없음 |
| ----- | --------- | --------- |
| 8bit  | i8        | u8        |
| 16bit | i16       | u16       |
| 32bit | i32       | u32       |
| 64bit | i64       | u64       |
| arch  | isize     | usize     |

- 러스트는 소수점을 가진 숫자를 처리하기 위해 두 개의 부동 소수점 타입을 제공함(64비트 소수점 타입인 f64가 기본 타입)
- 사칙연산은 다른 프로그래밍 언어와 동일(+, -, \*, /, %)
- 불리언 타입은 `true`, `false`로 구성되어 있음
- 러스트의 문자타입인 char는 4byte 크기의 유니코드 스칼라 값이므로 ASCII보다 더 많은 문자를 표현할 수 있다

**3.2.2 러스트의 컴파운드 타입**

- 하나의 타입으로 여러개의 값을 그룹화한 타입으로 튜플과 배열 두 가지의 컴파운드 타입을 지원한다

- 튜플은 destruct 구문을 사용할 수도 있다
- 또한 튜플은 각 원소의 명시적 타입 정의가 가능하다

```rust
 let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("x: {}, y: {}, z: {}", x, y, z);
    // x: 500, y: 6.4, z: 1

    let tup2: (i32, f64, u8) = (500, 6.4, 1);
    println!("{}", tup2.0); // 500

    let array = [1, 2, 3, 4, 5];
    println!("{}", array[array.len() - 1]); // 5
    println!("{}", array[array.len() / 2]); // 3
```

**3.2.3 러스트의 함수**

- 러스트의 함수는 대부분 snake case를 사용한다
- 리턴값에는 이름을 부여하지는 않지만 리턴할 값의 타입은 화살표(->) 다음에 지정해주어야 한다

```rust
fn another_function(x: i32, y: i32) {
    println!("함수에 사용된 인자 x의 값 : {}, y의 값 : {}", x, y);
}

fn five() -> i32 {
    5
} // 여기서 five()는 let x = 5와 같은 표현이다

fn main() {
    println!("five()의 값: {}", five())
}
```

**3.2.4 러스트의 if문과 루프**

- if문은 조건에 따라 코드를 분기한다
- 반드시 불리언 타입 중 하나를 리턴해야 한다

```rust
fn main() {
    let number = 6;

    if number % 4 == 0 {
        println!("변수가 4로 나누어 떨어집니다.");
    } else if number % 3 == 0 {
        println!("변수가 3으로 나누어 떨어집니다.");
    } else if number % 2 == 0 {
        println!("변수가 2로 나누어 떨어집니다.");
    } else {
        println!("변수가 2, 3, 4로는 나누어 떨어지지 않습니다.")
    }

    let condition = true;
    let number = if condition {
        5
    } else {
        6 // 이때 조건에 따른 리턴값은 일치해야 한다
    };

    println!("number의 값: {}", number); // number의 값: 5
}
```

- 루프를 이용한 반복
- 러스트에서는 loop, while, for의 세 가지 루프를 제공함

```rust
fn main() {
    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        };
    }
    println!("result = {}", result) // result = 20
}
```

- while을 이용한 조건 루프

```rust
fn main() {
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }
    println!("완료!")
}
```

- for를 이용한 반복

```rust
fn for_iterator() {
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
```

</div>
</details>

### Chapter 4. 소유권

<details>
<summary>열기</summary>
<div markdown="4">

- 소유권은 러스트의 독특한 기능 중 하나로 가비지 컬렉터에 의존하지 않고도 메모리 안전성을 보장하려는 러스트만의 해법이다
- 소유권과 더불어 대여, 슬라이스, 그리고 메모리 관리법에 대해 알아두어야 한다

**4.1 소유권 규칙**

- 러스트가 다루는 각각의 값은 소유자라고 부르는 변수를 가지고 있다
- 특정 시점에 값의 소유자는 단 하나뿐
- 소유자가 범위를 벗어나면 그 값은 제거된다
- 변수의 유효범위

```rust
{   // 이 시점에서는 s를 선언하지 않았으므로 유효하지 않음
    let s = "hello"; // 이 지점부터 유효
    // 변수 s를 이용해 필요한 동작을 수행함
}   // 이 범위를 벗어나면 s는 유효하지 않음
```

- 타입들은 모두 스택에 저장되며 스코프를 벗어나면 스택에서 제거됨
- 힙에 저장되는 데이터들을 러스트가 어떻게 제거하는가?
- String 타입과 문자열 리터럴은 다르게 작동한다
- String 타입은 변경할 수 있지만 리터럴은 변경할 수 없다

```rust
fn string_literal() {
    let s = String::from("hello");
    println!("{}", s);
    // println!(s);
    // format argument must be a string literal
} // rust는 닫는 중괄호를 만나면 자동으로 drop 함수를 호출하여 메모리에서 해제한다

fn move_example() {
    // s1은 포인터, 길이, 용량으로 이루어져있다
    // 해당 포인터는 문자열의 인덱스와 값을 가지고 있음
    let s1 = String::from("hello");
    // s2 = s1을 실행하면 s1, s2의 포인터가 같은 인덱스를 가리키게 된다
    // 혹여나 s1이 drop으로 메모리에서 해제된다면 s2까지 해당 인덱스를 사용할 수 없게 되는데
    // 이런 경우 메모리의 불순화(이중 해제 에러)를 일으킬 수 있다
    let s2 = s1;
    // 따라서 s2에 s1을 대입한 경우 println!("{}, world", s1)를 실행하면
    // borrow of moved value: `s1` 와 같이 s1 값이 "이동됨(moved)"에 따라 실행 할 수 없게 된다
    // rust는 얕은 복사나 깊은 복사의 개념이 아니라 이런 식으로 첫 번째 변수(s1)를 무효화 시키므로 "이동했다"고 표현한다
    println!("{}, world", s2);
}

fn clone_example() {
    // 변수와 데이터가 상호작용하는 방식으로는 복제(clone)가 있다
    // 힙 데이터가 그대로 복사되기 때문에 복사하는 메모리의 크기에 따라서는 무거운 작업일 수도 있다
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("{}, world! {}!", s1, s2);
}

fn copy_example() {
    let x = 5;
    let y = x;
    println!("x = {}, y = {}", x, y);
    /*
        이 코드는 정상적으로 실행되는 것 처럼 보임
        정수형 타입은 스택에 저장되므로 힙에 저장되는 타입처럼 "이동"되지 않는다
        러스트는 이렇게 스택에 저장되는 정수형 타입에 적용할 수 있는 Copy trait라는 특별한 특성을 제공한다
        u32와 같은 모든 정수형 타입, bool, char, f64와 같은 부동 소수점 타입, (i32, i32)와 같은 Copy trait를 적용된 타입을 포함하는 튜플
        하지만 (i32, String)과 같은 튜플에는 적용되지 않는다
    */
}
```

**4.2 소유권(Ownership)과 범위**

```rust
fn owner_exmaple() {
    let s = String::from("hello");
    takes_ownership(s);

    let x = 5;
    makes_copy(x);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}
```

- 리턴값과 변수의 범위

```rust
fn return_example() {
    let s1 = gives_ownership(); // 리턴값이 s1으로 옮겨짐
    let s2 = String::from("hello"); // s2 변수 생성
    let s3 = takes_and_gives_back(s2); // s2는 함수 내로 옮겨지고 s3에 리턴값이 할당됨

    println!("{}? {}!", s1, s3);
} // s1, s3은 drop되고 s2는 함수로 옮겨졌기 때문에 아무것도 일어나지 않음

fn gives_ownership() -> String {
    // 변수 some_string이 생성
    let some_string = String::from("hello");
    some_string // 이 값이 리턴되면서 호출한 함수로 옮겨짐
}

fn takes_and_gives_back(a_string: String) -> String {
    // a_string이 생성되고 리턴되면서 호출한 함수로 옮겨짐
    a_string
}
```

**4.3 참조**

- 참조 변수와 가변 참조

```rust
fn lental_example() {
    let mut s1 = String::from("hello");
    // &로 참조할 수 있으며 &s1 문법을 이용하여 "소유권은 가져오지 않는 참조"를 생성할 수 있다
    // 이 경우에는 범위를 벗어나도 drop되지 않는다
    let len = calculate_length(&s1);
    // 따라서 여기서 호출 할 수 있다
    println!("'{}'의 길이는 {}입니다.", s1, len);
    change(&mut s1);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

// 변수가 기본적으로 불변인 것처럼 참조도 기본적으로 불변이다
// fn change(some_string: &String) {
// 따라서 &mut와 같이 가변 참조로 정의해주어야 한다
fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
```

- 데이터 경합은 **둘 혹은 그 이상의 포인터가 동시에 같은 데이터를 쓰거나 읽기 위해 접근할 때**, **최소한 하나의 포인터가 데이터를 쓰기 위해 사용될 때**, **데이터에 대한 접근을 동기화 할 수 있는 메커니즘이 없을 때** 일어난다
- 따라서 스코프를 임의로 설정하면 가변 참조를 여러번 사용할 수 있다

```rust
fn reference_example() {
    let mut s = String::from("hello");
    {
        let r1 = &mut s;
        println!("r1:{}", r1);
    } // scope를 임의로 설정함으로써 가변 참조를 여러 번 사용할 수 있도록 함
    let r2 = &mut s;

    // println!("r1:{}, r2:{}", r1, r2);
    println!("r2:{}", r2);
    // 따라서 cannot find value `r1` in this scope 와 같은 오류가 발생함
}
```

- 스코프를 벗어나면 메모리에서 drop되기 때문에 **죽은 참조**가 발생할 수 있다

```rust
fn death_example() -> &String {
    let s = String::from("hello");

    &s
} // 이 함수의 리턴 타입은 대여한 값을 리턴하고자 하지만 실제로 대여해 올 값이 존재하지 않는다.
```

- 어느 한 시점에 코드는 하나의 가변 참조 또는 여러 개의 불변 참조를 생성할 수는 있지만 둘 모두를 생성할 수는 없다
- 또한 참조는 항상 유효해야 한다

**4.4 슬라이스**

- 러스트에는 소유권을 갖지 않는 **슬라이스 타입**이 있다
- 이 슬라이스를 이용하면 컬렉션 전체가 아니라 컬렉션 내의 연속된 요소들을 참조할 수 있다

```rust
// 문자열 슬라이스
let s = String::from("hello world");
let hello = &s[0..5];
let world = &s[6..11];

// slice1과 slice2는 동일하게 동작함
let slice1 = &s[0..2];
let slice2 = &s[..2];

let len = s.len();
// slice3과 slice4도 동일하게 동작함
let slice3 = &s[0..len];
let slice4 = &s[..];
```

- 문자열 리터럴은 슬라이스이기 때문에 `let s = "Hello, world!;`와 같이 선언한다면 이 때 s의 타입은 `&str`이고 따라서 문자열 리터럴은 항상 불변이다

```rust
// 문자열 뿐만 하니라 배열의 슬라이스도 가능하다
let a = [1, 2, 3, 4, 5];
let slice = &a[1..3];
```

</div>
</details>

### Chapter 5. 구조체를 활용한 관련 데이터의 구조화

<details>
<summary>열기</summary>
<div markdown="5">

- 구조체는 서로 관련이 있는 여러 값을 의미 있는 하나로 모아 이름을 지정해 접근할 수 있는 사용자 정의 데이터 타입
- 구조체의 키워드는 struct로 TypeScript의 interface와 유사

```rust
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool
}

let mut user1 = User {
    email: String::from("someone@example.com"),
    username: String::from("someusername123"),
    active: true,
    sign_in_count: 1,
}

user1.email = String::from("anotheremail@example.com");

fn build_user(email: String, username: String) -> User {
    User {
        // email: email,
        // username: username,
        email,
        username,
        // 변수와 필드 이름이 동일할 때엔 필드 초기화 단축 문법을 사용할 수 있다
        active: true,
        sign_in_count: 1,
    }
}

let user2 = User {
    email: String::from("another@example.com"),
    username: String::from("anotherusername456"),
    // active: user1.active,
    // sign_in_count: user1.sign_in_count,
    ..user1 // 나머지 필드를 이렇게 정의할 수도 있다
}
```

- 튜플 구조체는 구조체에 이름을 부여하지만 필드에는 이름을 부여하지 않고 타입만 지정하는 경우를 말한다

```rust
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
// 튜플 내부가 같은 타입으로 정의되어 있지만 Color와 Point는 다른 타입임
let black Color(0, 0, 0);
let origin = Point(0, 0, 0);
```

- 러스트에서는 필드가 하나도 없는 구조체를 선언할 수도 있는데 이런 구조체를 **유사 유닛 구조체**라고 한다

```rust
struct User {
    username: &str, // missing lifetime specifier
    email: &str, // missing lifetime specifier
    sign_in_count: u64,
    active: bool
}

fn main() {
    let user1 = User {
        //...
    }
}
```

- 튜플을 이용한 사각형의 면적 계산 프로그램

```rust
fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn main() {
    let width1 = 30;
    let height1 = 50;

    println!("사각형의 면적: {} 제곱 픽셀", area(width1, height1));
}
```

- 튜플을 사용해 리팩토링하기

```rust
fn area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}
```

- 구조체를 이용한 리팩토링

```rust
fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
```

- 러스트는 디버깅 정보를 출력하는 기능을 제공하지만 구조체는 명시적으로 구현해주어야 함

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
```

- 함수를 구조체 내에 정의하는 법은 자바스크립트의 prototype과 비슷함

```rust
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
```

</div>
</details>

### Chapter 6. 열거자와 패턴 매칭

<details>
<summary>열기</summary>
<div markdown="6">

**6.1 열거자(emuns) 정의하기**

- 열거자(enums)는 사용 가능한 값만 나열한 타입을 정의할 때 사용한다
- 러스트의 열거자는 F#이나 하스켈 같은 함수형 언어의 대수식 데이터 타입에 가까움
- 열거자에 나열된 각각의 값은 서로 다른 타입과 다른 수의 연관 데이터를 보유할 수 있음

```rust
enum IpAddressType {
    V4, // 열거자의 열것값
    V6,
}

fn main() {
    let home = IpAddr {
        kind: IpAddressType::V4,
        address: String::from("127.0.0.1"),
    };
    let loopback = IpAddr {
        kind: IpAddressType::V6,
        address: String::from("::1"),
    };
}

struct IpAddr {
    kind: IpAddressType,
    address: String,
}

fn route(ip_type: IpAddressType) {}

```

- 열거자의 값에는 문자열, 숫자, 구조체 등 어떤 종류의 데이터도 저장할 수 있다

```rust
enum Message {
    Quit, // 연관 데이터를 갖지 않는 열거자
    Move { x: i32, y: i32 }, // 익명 구조체를 갖는 열거자
    Write(String), // 하나의 String 값을 갖는 열거자
    ChangeColor(i32, i32, i32), // 세 개의 i32 값을 포함하는 열거자
}
/* 구조체를 사용한다면 이렇게 나누어져야 한다 ↓ */
struct QuitMessage;
struct MoveMessage {
    x: i32,
    y: i32,
}

struct WriteMessage(String);
struct ChangeColorMessage(i32, i32, i32);
```

- 러스트에 널값이라는 개념은 없지만 존재 여부를 표현하는 열거자가 Option<T>이다
- Some 대신 None값을 이용하면 러스트에게 Option<T> 열거자의 타입이 무엇인지를 알려줘야 한다

```rust
enum Option<T> {
    Some(T),
    None,
}
```

**6.2 match 흐름 제어 연산자**

- 러스트는 match라는 매우 강력한 흐름 제어 연산자를 제공함
- 패턴은 리터럴, 변수 이름, 와일드카드를 비롯해 다양한 값으로 구성할 수 있다

```rust
enum Coin {
    Penny,
    Nickle,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickle => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
```

- match는 반드시 모든 경우를 처리해야 함

```rust
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i + 1),
        None => None,
    }
}
```

- 모든 경우를 다 처리하고 싶지 않을 때엔 \_ 자리지정자로 대체하면 된다

```rust
fn placeholder() {
    let some_u8_value = 0u8;
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (), // _ 패턴은 모든 값에 일치함
    }
}
```

- if let 문법은 여러 경우 중 한 가지만 처리하고 나머지는 고려하고 싶지 않을 때 사용한다
- 또한 if let 문법은 if let ~ else 표현식으로도 사용할 수 있다

```rust
fn iflet() {
    let some_u8_value = Some(0u8);
    if let Some(3) = some_u8_value {
        println!("three!");
    } else {
        println!("not three...");
    }
}
```

</div>
</details>

### Chapter 7. 패키지, 크레이트, 모듈로 프로젝트 관리하기

<details>
<summary>열기</summary>
<div markdown="7">

- 기능을 그룹화하는 것 외에도 구현을 캡슐화하면 코드를 재사용할 수 있다
- 러스트는 코드의 구조를 관리하기 위한 몇 가지 기능을 제공한다
  - 패키지: 크레이트를 빌드, 테스트, 공유할 수 있는 카고의 기능
  - 크레이트: 라이브러리나 실행 파일을 생성하는 모듈의 트리
  - 모듈과 use: 코드의 구조와 범위, 그리고 경로의 접근성을 제어하는 기능
  - 경로: 구조체, 함수, 혹은 모듈 등의 이름을 결정하는 방식

**7.1 패키지와 크레이트**

- 크레이트는 하나의 바이너리 혹은 라이브러리로 크레이트 루트는 러스트 컴파일러가 컴파일을 시작해서 크레이트의 루트 모듈을 만들어내는 소스 파일이다
- 패키지는 일련의 기능을 제공하는 하나 혹은 그 이상의 크레이트로 구성된다

```cmd
> cargo new my-project
> ls my-project

Mode                 LastWriteTime         Length Name
----                 -------------         ------ ----
d-----      2021-06-05   오후 9:36                src
-a----      2021-06-05   오후 9:36            229 Cargo.toml

> ls my-project/src

Mode                 LastWriteTime         Length Name
----                 -------------         ------ ----
-a----      2021-06-05   오후 9:36             45 main.rs
```

**7.2 모듈을 이용한 범위와 접근성 제어**

- 모듈은 크레이트의 코드를 그룹화해서 가독성과 재사용성을 향상하는 방법이다

```
cargo new --lib restaurant
> ls restaurant
Mode                 LastWriteTime         Length Name
----                 -------------         ------ ----
d-----      2021-06-05   오후 9:42                src
-a----      2021-06-05   오후 9:42            229 Cargo.toml

> ls restaurant/src
Mode                 LastWriteTime         Length Name
----                 -------------         ------ ----
-a----      2021-06-05   오후 9:42             95 lib.rs
```

- 레스토랑 시설을 구분하여 모듈로 정의해보자

```rust
mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}
```

**7.3 경로를 이용해 모듈 트리의 아이템 참조하기**

- 절대 경로: 크레이트 이름이나 crate 리터럴을 이용해 크레이트 루트부터 시작하는 경로
- 상대 경로: 현재 모듈로부터 시작해서 self, super 혹은 현재 모듈의 식별자를 이용함
- 하지만 절대경로나 상대경로는 해당 모듈이나 열거자가 pub으로 공개되지 않으면 참조할 수 없다
- 또한, 해당 모듈이 가진 함수나 하위 모듈 또한 기본적으로 은폐되기 때문에 사용하고자 할 때엔 pub 키워드로 열어주어야 함

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    // 절대 경로: 현재 crate -> front_of_house -> hosting -> add_to_waitlist();
    crate::front_of_house::hosting::add_to_waitlist();

    // 상대 경로: 같은 소스파일 내에 있는 front_of_house를 참조
    front_of_house::hosting::add_to_waitlist();
}
```

- 상대 경로는 `super` 키워드를 이용해 부모 모듈부터 시작할 수도 있다 (마치 파일 시스템 경로의 ..같은 것)

```rust
fn serve_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::serve_order();
        // super 키워드를 통해 루트 모듈 crate에 접근하여 serve_order()를 찾음
    }

    fn cook_order() {}
}
```

- 구조체를 정의할 때 `pub` 키워드를 사용하면 구조체는 공개되지만 구조체의 필드는 비공개임
- 반면 열거자를 공개하면 모든 열것값 또한 공개된다

```rust
mod back_of_house {
    // seasonal_fruit는 비공개 필드임
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("복숭아"),
            }
        }
    }

    // 열거자를 public처리하면 모든 열것값도 public 처리 된다
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

pub fn eat_at_restaurant() {
    let mut meal = back_of_house::Breakfast::summer("호밀빵");
    meal.toast = String::from("밀빵");
    // meal.seasonal_fruit;
    // field `seasonal_fruit` of struct `back_of_house::Breakfast` is private
    crate::front_of_house::hosting::add_to_waitlist(); // module `hosting` is private
    front_of_house::hosting::add_to_waitlist();
    // 아무런 문제 없이 열것값을 사용할 수 있다
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}
```

**7.4 모듈 사용하기, 내보내기**

- `use` 키워드를 사용하여 경로를 현재 범위로 가져오면 현재 범위의 아이템인 것 처럼 호출할 수 있다 (import와 비슷)

```rust
mod front_of_house {
    pub mod hosting {
    //...
    }
}
use crate::front_of_house::hosting;
// use self::front_of_house::hosting; 로도 정의할 수 있다
pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    // 'crate::front_of_house' 부분을 생략 가능하다
}
```

- `as` 키워드를 사용하면 새로운 이름을 부여할 수 있다

```rust
use std::io::Result as IoResult;

fn function1() -> IoResult<()> {
    //...
}
```

- `pub use` 키워드로 이름을 다시 내보내기

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

// export default와 비슷한 문법으로 해당 크레이터를 외부로 내보낼 수 있음
pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
```

**7.5 외부 패키지의 사용**

- Cargo.toml 파일 내 dependencies에 정의하고 카고를 통해 해당 패키지를 내려받으면 `use` 커맨드와 함께 어디서든 사용할 수 있다
- 중첩 경로의 사용

```rust
// use std::io;
// use std::cmp::Ordering
use std::{io, cmp::Ordering};

// use std::io;
// use std::io::Write;
use std::io::{self, Write};
```

- 글롭 연산자

```rust
use std::collections::*;
```

</div>
</details>

### Chapter 8. 범용 컬렉션

<details>
<summary>열기</summary>
<div markdown="8">

- 러스트의 표준 라이브러리는 여러 종류의 컬렉션을 포함하고 있다
- 그 중에 **벡터**, **문자열**, **해시 맵**이 대표적인 컬렉션

**8.1 벡터**

- 벡터 생성하기

```rust
let v: Vec<i32> = Vec::new();
let v1 = vec![1, 2, 3];
let mut v2 = Vec::new();
v2.push(5);
v2.push(6);
v2.push(7);
```

- 벡터 값 읽기

```rust
let third: &i32 = &v2[2];
println!("세 번째 원소: {}", third);

match v2.get(2) {
    Some(third) => println!("세 번째 원소: {}", third),
    None => println!("세 번째 원소가 없습니다."),
}

let v3 = vec![1, 2, 3, 4, 5];
// [] 방식은 패닉을 발생시킨다
let does_not_exist = &v3[100];
// 존재하지 않는 값에 접근할 때 get 메서드를 사용한다면 None값이 리턴된다
let does_not_exist = v3.get(100);

// for 루프를 이용해 벡터를 순회하기
let mut v4 = vec![1, 2, 3, 4, 5];
for i in &v4 {
    println!("{}", i);
}

// 가변 참조로 순회하기
for i in &mut v4 {
    *i += 50;
}

enum SpreadSheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn different_types_vector() {
    let row = vec![
        SpreadSheetCell::Int(3),
        SpreadSheetCell::Float(10.12),
        SpreadSheetCell::Text(String::from("벡터에 여러 타입을 저장하기")),
    ];
    row.get(0);
}
```

**8.2 문자열**

- 러스트는 언어 내에 문자열 슬라이스인 str 타입만을 지원한다
- String 타입은 표준 라이브러리가 제공하는 타입
- UTF-8 형식으로 인코딩된 문자열 타입을 사용하므로 어떤 언어를 쓰더라도 유효한 문자열이 된다

```rust
let mut s1 = String::new();
let data = "문자열 초깃값";
let s2 = data.to_string();
let s2 = "문자열 초깃값".to_string();
```

- 문자열을 이어붙일 때엔 push()나 push_str()를 사용한다

```rust
let mut s3 = String::from("foo");
let s4 = "bar";
s3.push_str(s4);
// push_str이 s4의 소유권을 가지게 되므로 s4는 출력되지 않아야 한다
// 하지만 문자열 슬라이스를 이용하기 때문에 소유권을 가지지 않는다

let s5 = String::from("Hello, ");
let s6 = String::from("world!");
// 이는 +가 add(self, s: &str) 메서드를 사용하기 때문
// self에는 &가 없으므로 소유권을 가져오고
// 따라서 self(여기서는 s5)는 이 메서드가 끝나면 메모리가 해제된다
let s7 = s5 + &s6;
```

- 문자열의 길이

```rust
let len_en = String::from("Hello").len();
let korean = String::from("안녕하세요");
let len_kr = korean.len();
println!("Hello의 길이: {}, 안녕하세요의 길이: {}", len_en, len_kr);
// Hello의 길이: 5, 안녕하세요의 길이: 15
```

- 러스트 관점에서 문자열은 크게 바이트, 스칼라값, 그래핌 클러스터의 세 가지로 구분한다
- 러스트에서 String의 인덱스 사용을 지원하지 않는 이유는 인덱스 처리에는 항상 O(1)이 소요되어야 하지만 String 타입은 러스트가 유효한 문자를 파악하기 위해 콘텐츠를 처음부터 스캔해야 하기 때문에 일정한 성능을 보장할 수 없기 때문

- 문자열 슬라이스하기

```rust
let korean = String::from("안녕하세요");
let sliced = &korean[0..3];
// 이 때 sliced에는 "안"이 저장되지만 [0..1] 처럼 글자가 될 수 없는 범위라면 오류를 일으킬 수 있다
```

- 문자열 순회하기

```rust
for c in "안녕하세요".chars() {
    print!("{} ", c);
}
```

**8.3 해시 맵**

- 벡터와 마찬가지로 해시맵은 데이터를 힙 메모리에 저장한다

```rust
use std::collections::HashMap;
let mut scores = HashMap::new();
scores.insert(String::from("블루"), 10);
scores.insert(String::from("옐로"), 50);
```

- collect 메서드와 zip 메서드를 사용하여 해시맵을 생성하기

```rust
let teams = vec![String::from("블루"), String::from("옐로")];
let initial_scores = vec![10, 50];
let hash_scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
```

- 해시맵의 소유권은 메서드를 사용할 때 넘어간다

```rust
let field_name = String::from("Favorite Color");
let field_value = String::from("블루");

let mut map = HashMap::new();
map.insert(field_name, field_value);
println!("name: {}, value: {}", field_name, field_value);
// borrow of moved value: `field_name`, `field_value`
```

- 해시맵에 접근하고 수정하기

```rust
// 해시맵에 접근하기
let team_name = String::from("블루");
let score = hash_scores.get(&team_name);

for (key, value) in &hash_scores {
    println!("{}, {}", key, value);
}

// 해시맵 수정하기
let mut new_scores = HashMap::new();
new_scores.insert(String::from("블루"), 10);
new_scores.insert(String::from("블루"), 25); // 값을 덮어씌움
println!("{:?}", new_scores); // {"블루":25}

// or_insert 메서드는 키가 존재하면 그 키에 연결된 값에 대한 가변 참조를 리턴함
new_scores.entry(String::from("옐로")).or_insert(50);
new_scores.entry(String::from("블루")).or_insert(10);
println!("{:?}", new_scores);

// 기존 값에 따라 값 수정하기
let text = "hello world wonderful world";
let mut map = HashMap::new();
// split_whitespace() : Splits a string slice by whitespace.
for word in text.split_whitespace() {
    let count = map.entry(word).or_insert(0);
    *count += 1;
}
println!("{:?}", map); // {"wonderful": 1, "hello": 1, "world": 2}
```

</div>
</details>

### Chapter 9. 에러 처리

<details>
<summary>열기</summary>
<div markdown="9">

- 러스트는 에러를 크게 '회복 가능한' 에러와 '회복 불가능한' 에러의 두 가지로 구분한다
- Result<T, E>와 panic!을 통한 에러 처리가 있다

**9.1 회복 불가능한 에러 처리**

- 기본적으로 패닉이 발생하면 프로그램은 스택을 역순으로 순회하면서 데이터를 정리하기 때문에 프로그램이 클 수록 해야 하는 작업량은 어마어마함
- 스택을 즉시 취소해서 애플리케이션을 종료하는 방법도 있는데 이 경우에는 운영체제가 메모리를 정리해야 한다

```
// Cargo.toml 파일 내
[profile.release]
panic = 'abort
```

- 패닉을 호출하는 법

```rust
painc!("crash and burn");
```

- 코드의 버그에 의해 일어나는 패닉
- 버퍼 오버리드(buffer overread)

```rust
let v = vec![1, 2, 3];
v[99];
// 벡터의 100번째값은 존재하지 않으므로 패닉을 발생시킴
```

- RUST_BACKTRACE 환경변수를 이용해 패닉의 원인을 역추적 할 수 있다
- `> RUST_BACKTRACE=1 cargo run` 처럼 RUST_BACKTRACE 환경변수에 값을 설정하여 실행하면

**9.2 Result 타입을 사용해 에러 처리하기**

- Result enum에는 Ok와 Err 열것값을 가지고 있는데 이를 이용해 에러를 처리할 수 있다

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

- match 표현식을 이용해 리턴된 Result 타입의 리턴값을 처리할 수도 있다

```rust
use std::fs::File;

fn main() {
    let f = File::open("hello.txt");
    // File::open() 메서드가 리턴하는 Err 열것값에 저장된 값을 타입은 io::Error 타입이다
    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() { // error.kind() 메서드는 io::ErrorKind 타입을 리턴한다
            ErrorKind::NotFound => match File::create("hello.txt") { // ErrorKind::NotFound를 처리하고
                Ok(fc) = fc,
                Err(e) => painc!("파일 생성 실패: {:?}", e),
            },
            other_error => panic!("파일 열기 실패: {:?}", other_error); // 나머지 에러를 처리한다
        }
    };
}
```

- match 표현식은 앞서 봤듯 중첩해서 사용되기 때문에 이럴 때엔 unwrap 메서드가 유용하다
- 혹은 unwrap 메서드 대신 expect 메서드를 사용하면 개발자의 의도를 더 명확하게 표현하는 동시에 패닉이 발생한 원인을 더 쉽게 추적할 수 있다

```rust
let f = File::open("hello.txt").unwrap();
let f = File::open("hello.txt").expect("파일을 열 수 없습니다.");
```

- 에러를 함수 안에서 처리하지 않고 호출하는 코드에 에러를 리턴하여 호출자가 에러를 처리하게 할 수 있다

```rust
use std::io;
use std::io::Read;
use std::fs::File;

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e), // 여기서 생긴 에러가 호출자에게 리턴됨
    };
    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e), // 마찬가지로 여기서 생긴 에러가 호출자에게 리턴됨
    }
}
```

- ? 연산자를 이용하면 더 간결하게 위 코드를 구현할 수 있다

```rust
fn read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s);
}
```

- fs::read_to_string() 메서드를 사용하기

```rust
use std::io;
use std::fs;

fn read_username_from_file() -> Result<String, io:Error> {
    fs::read_to_string("hello.txt")?;
}
```

- 하지만 이런 ? 연산자는 Result 타입을 리턴하는 함수에서만 사용할 수 있다

```rust
use std::error::Error;
use std::fs::File;

fn main() -> Result<(), Box<dyn Error>> {
    let f = File::open("hello.txt")?;
    Ok(())
}
```

**9.3 패닉에 빠질 것인가? 말 것인가?**

- 프로토타이핑을 할 때엔 unwrap과 expect 메서드를 사용할 것
- 컴파일러보다 개발자가 더 많은 정보를 가진 경우엔 unwrap 메서드를 호출하자
- 코드가 결국 잘못된 상태가 될 상황이라면 패닉을 발생시키는 것이 나쁜 선택이 아니다
- 러스트의 타입 시스템은 유효한 값을 전달한다고 보장하기 때문에 이것을 적극적으로 활용하자

```rust
// 리팩토링 전
loop {
    // ...
    let guess: i32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => continue,
    };

    if guess < 1 || guess > 100 {
        println!("1에서 100 사이의 값을 입력해주세요.");
        continue;
    }

    match guess.cmp(&secret_number);
    // ...
}

// 리팩토링 후
pub struct Guess {
    value: i32 // i32 타입의 value 필드를 가진 구조체를 정의
}

impl Guess {
    pub fn new(value: i32) -> Guess { // new 연관함수를 통해 1~100인지 검사하고 아니라면 panic! 매크로를 호출
        if value < 1 || value > 100 {
            panic!("유추한 값은 반드시 1에서 100 사이의 값이어야 합니다. 입력한 값: {}", value);
        }

        Guess {
            value
        }
    }

    pub fn value(&self) -> i32 {
        self.value // 유효성 검사를 통과한다면 i32 타입의 값을 리턴한다
    }
}
```

</div>
</details>

### Chapter 10. 제네릭 타입, 트레이트 그리고 수명

<details>
<summary>열기</summary>
<div markdown="10">

**10.1 함수로부터 중복 제거하기**

```rust
// 하나의 리스트에서 가장 큰 숫자 찾기
fn find_largest_number() {
    let number_list = vec![34, 50, 25, 100, 65];

    let mut largest = number_list[0];

    for number in number_list {
        if number > largest {
            largest = number;
        }
    }
    println!("가장 큰 수: {}", largest);
}

// 리스트를 매개변수로 받는 함수로 변경
fn find_largest_number(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}
```

**10.2 제네릭 데이터 타입**

- 제네릭은 여러 구체화된 타입을 사용할 수 있는 함수 시그너처나 구조체 같은 아이템을 정의할 때 사용함

```rust
fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}
```

- 두 함수의 구성은 매개변수만 제외하곤 완전히 동일하다
- 이 함수들을 제네릭을 사용해 하나의 함수로 변경해보자

```rust
fn largest<T>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}
```

- 제네릭으로 구조체 정의해서 사용하기

```rust
struct Point<T, U> {
    x: T,
    y: U,
}

fn main() {
    let intPoint = Point { x: 5, y: 10 };
    let floatPoint = Point { x: 1.0, y: 4.0 };
    let int_and_float = Point { x: 5, y: 4.0 };
}
```

- 구조체의 정의와는 다른 제네릭 타입을 사용하는 메서드

```rust
struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn mixup<V, W> (self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
    // p3.x = 5, p3.y = c
}
```

- 러스트는 컴파일 시점에 제네릭 코드를 '단일화'하기 때문에 성능이 떨어지지 않는다

**10.3 트레이트: 공유 가능한 행위를 정의하는 법**

- 트레이트: 러스트 컴파일러에게 특정 타입이 어떤 기능을 실행할 수 있으며, 어떤 타입과 이 기능을 공유할 수 있는지를 알려주는 방법
- 트레이트 선언하기: `trait` 키워드 다음에 트레이트의 이름을 지정한다

```rust
pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {}, ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("Start to learn Rust"),
        reply: false,
        retweet: false,
    }

    println!("새 트윗 1개: {}", tweet.summarize())
    // 새 트윗 1개: horse_ebooks: Start to learn Rust
}
```

- 트레이트 구현에 있어 한 가지 제약은 트레이트나 트레이트를 구현할 타입이 현재 크레이트의 로컬 타입이어야 한다는 점이다

- 트레이트를 이용해 여러 가지 타입을 수용하는 함수를 정의하기

```rust
// 1. 일반적인 impl 트레이트 문법
pub fn notify(item: impl Summary) {
    println!("속보! {}", item.summarize());
}

// 2. 트레이트 경계 문법
pub fn notify<T: Summary>(item: T) {
    println!("속보! {}", item.summarize());
}

// 3. + 문법으로 트레이트 경계 정의 (둘 다 같은 사용법임)
pub fn notify(item: imple Summary + Display) {
    println!("속보! {}", item.summarize());
}
pub fn notify<T: Summary + Display>(item: T) {
    println!("속보! {}", item.summarize());
}

// 4. where절을 이용한 트레이트 경계 정의 (역시 둘 다 같은 사용법)
fn some_function<T: Display + Clone, U: Clone + Debug>(t: T, u: U) -> i32 {}
fn some_function<T, U>(t: T, u: U) -> i32
    where T: Display + Clone,
          U: Clone + Debug
```

- 트레이트를 구현하는 값 리턴하기

```rust
// 리턴 타입으로 impl Summary를 정의했기 때문에
// 이 함수는 실제 타입 이름을 사용하지 않고도 Summary 트레이트를 구현하는 어떤 타입도 리턴할 수 있다
// 하지만 이 문법은 하나의 타입을 리턴하는 경우에만 사용할 수 있다
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("Start to learn Rust"),
        reply: false,
        retweet: false,
    }
}
```

- 트레이트 경계를 이용해 largest 함수를 수정해보기

```rust
// 1. PartialOrd 트레이트의 경계를 지정해서 largest 함수가 실제로 비교할 수 있는 타입의 슬라이스만을 처리할 수 있게 함
// 2. i32, char처럼 크기가 이미 정해진 타입은 스택에 저장되므로 Copy 트레이트를 추가하여 확실하게 해당 트레이트에 들어오는 값들을 정의해준다
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}
```

**10.4 수명을 이용해 참조 유효성 검사하기**

- 러스트의 모든 참조는 수명(참조가 유효한 범위)을 가지고 있다
- 수명을 이용해 죽은 참조의 발생을 방지하기

```rust
{
    let r;
    {
        let x = 5; // 'x' does not live long enough
        r = &x;
    }

    println!("r: {}", r);
}
```

- 러스트 컴파일러는 대여한 값이 현재 범위 내에서 유효한지 검사하는 대여 검사기를 탑재하고 있다
- 함수의 제네릭 수명

```rust
// error[E0106]: missing lifetime specifier
fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("더 긴 문자열: {}", result);
}

fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

- 함수 시그니처의 수명 애노테이션

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("아주 아주 긴 문자열");
    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("더 긴 문자열: {}", result);
    }
}
```

- 구조체 정의에서의 수명 애노테이션

```rust
struct ImportantExcerpt<'a> { // 구조체에 수명을 지정하면
    part: &'a str, // 필드값인 part에도 수명을 지정할 수 있다
}

fn main() {
    let novel = String::from("스타워즈. 오래 전 멀고 먼 은하계에...");
    let first_sentence = novel.split('.') // .으로 문자열을 나눠 next()로 [0]번째 항목에 접근
                              .next()
                              .expect("문장에서 마침표'.'를 찾을 수 없습니다.");
    let i = ImportantExcerpt { part: first_sentence };
    println!("첫번째 문장: {}", i.part); // 첫번째 문장: 스타워즈
}
```

- 현재의 러스트엔 수명 생략 규칙이 추가되어 수명을 명시적으로 지정하지 않아도 된다
- 메서드 정의에서의 수명 애노테이션

```rust
impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("주목해주세요! {}", announcement);
        self.part
    }
}
```

- 모든 문자열 리터럴의 수명은 'static이고 직접 명시할 수도 있다

```rust
let s: &'static str = "문자열은 정적 수명이다.";
```

</div>
</details>

### Chapter 11. 자동화 테스트 작성하기

<details>
<summary>열기</summary>
<div markdown="11">

**11.1 테스트의 작성**

- 러스트에는 테스트의 대상이 되는 코드가 의도된 동작을 실행하는지 확인하는 함수가 있다
- 필요한 데이터나 상태를 설정하고, 테스트할 코드를 실행하고, 의도한 결과가 출력되는지 검증하는 순서로 실행된다

- 테스트 코드와 실행

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
```

```
Compiling adder v0.1.0 (C:\Users\HarryKim\Documents\GitHub\first-rust-project\adder)
 Finished test [unoptimized + debuginfo] target(s) in 1.47s
  Running unittests (target\debug\deps\adder-a2c065fd1516b9c8.exe)

running 1 test
test tests::it_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests adder

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

- `assert!` 매크로 사용하기

```rust
#[derive(Debug)]
pub struct Rectangle {
    length: u32,
    width: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.length > other.length && self.width > other.width
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            length: 8,
            width: 7,
        };
        let smaller = Rectangle {
            length: 5,
            width: 1,
        };

        assert!(larger.can_hold(&smaller));
    }
}
```

- should_panic 매크로를 이용하기

```rust
pub struct Guess {
    value: u32,
}

impl Guess {
    pub fn new(value: u32) -> Guess {
        if value < 1 || value > 100 {
            panic!(
                "반드시 1과 100 사이의 값을 사용해야 합니다. 지정된 값: {}",
                value
            );
        }

        Guess { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn greater_than_100() {
        Guess::new(200);
    }
}
```

- should_panic 특성은 함수 내의 코드가 패닉이 발생해야 테스트가 성공하고 발생하지 않으면 실패하게 됨

**11.2 테스트 실행 제어하기**

- 테스트를 병렬이나 직렬로 실행하기
- `cargo test -- --test-threads=1`: 테스트 바이너리가 사용할 스레드의 개수를 정밀하게 제어 가능
- `cargo test [name]`: 특정 테스트 함수의 이름을 명령어로 전달하여 하나만 실행하거나 복수의 함수가 공통적으로 가진 일부단어를 입력하여 해당 테스트들을 실행할 수 있다
- `cargo test -- --ignore`: `#[ignore]` 특성을 함수 위에 추가한다면 이 명령어로 테스트를 실행했을 때 해당 함수를 무시한다

**11.3 테스트의 조직화**

- 단위 테스트와 통합 테스트
- `cargo new [name] --lib`으로 생성하였을 때 가장 처음 적용되는 `#[cfg(test)]` 특성은 configuration의 약자로 이후의 코드는 특정 설정 옵션이 지정되었을 때에만 포함되도록 한다
- 테스트 함수는 비공개 함수를 가져와 실행할 수 있다
- 러스트에서 통합 테스트는 완전히 라이브러리의 영역 바깥에서 진행된다
  - (1) tests 디렉터리를 최상위 수준에 생성한다

```rust
// tests/integration_test.rs 파일 생성
use adder;

#[test]
fn it_adds_two() {
    assert_eq!(4, adder::add_two(2));
}
```

- (2) 서브 모듈들을 tests 디렉터리의 서브 디렉터리에 작성한다면 테스트되지 않는다
- (3) `src/lib.rs` 파일이 아닌 `src/main.rs` 파일을 가진 바이너리 크레이트라면 tests 디렉터리에선 main.rs 파일의 함수를 테스트할 수 없다
- 러스트 프로젝트는 `src/lib.rs` 파일에 작성된 로직을 `src/main.rs`에서 직접 호출할 수 있는 것은 이 때문이다

</div>
</details>

### Chapter 12. I/O 프로젝트: 명령줄 프로그램 작성하기

<details>
<summary>열기</summary>
<div markdown="12">

- grep(globally search a regular expression and print)는 전통적인 명령줄 도구로 텍스트 검색 도구이다
- grep은 **코드의 구조**, **벡터와 문자열의 활용**, **에러 처리**, **트레이트와 수명의 적절한 활용**, **테스트 코드 작성**을 아우르는 프로젝트

**12.1 명령줄 인수 처리하기**

```cmd
> cargo new --bin minigrep
```

- 명령줄 인수를 읽기 위해 러스트의 표준 라이브러리 std::env::args 함수를 사용

```rust
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect(); // 명령줄을 읽고 벡터로 변환함
    println!("{:?}", args);

    let query = &args[1];
    let filename = &args[2];

    println!("검색어: {}", query);
    println!("대상 파일: {}", filename);
}
```

**12.2 파일 읽기**

- 파일 읽기를 위해 `std::fs` 라이브러리 사용
- 현재는 fs 모듈이 분리되었으며 다른 방식으로 사용해야 함 ([번역 페이지](https://rinthel.github.io/rust-lang-book-ko/ch12-02-reading-a-file.html))
- 파일을 읽고 쓰는데 필요한 `std::fs::File` 모듈과 파일 I/O를 포함한 I/O 작업을 위해 유용한 `use std::io::prelude::*` 를 사용해야 함

```rust
use std::fs::File;
use std::io::prelude::*;

fn main() {
    // ...생략
    let mut file = File::open(filename).expect("파일을 읽지 못했습니다.");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("파일을 읽는 도중 에러가 발생했습니다.");

    println!("파일 내용:\n{}", contents);
}
```

**12.3 모듈화와 에러 처리 향상을 위한 리팩토링**

- 프로그램 개선을 위해 몇 가지 문제를 수정하기
  (1) main 함수가 하나 이상의 작업을 수행하고 있음
  (2) 설정 변수는 하나의 구조체에 모아서 목적을 명확하게 할 것
  (3) 파일을 읽지 못했을 때의 에러 처리를 더 명확하게 제시할 것
  (4) 다른 종류의 에러 처리를 위해 expect를 남발하지 않기 (에러 처리 로직을 한 곳으로 모으기)

```rust
// 1. 인수 구문 정리하기
fn parse_config(args: &[String]) -> Config {
    let query = args[1].clone();
    let filename = args[2].clone();

    Config { query, filename }
}

// 2. 설정 변수를 하나의 구조체에 모아 목적을 명확히 할 것
struct Config {
    query: String,
    filename: String,
}
```

- parse_config 함수를 Config 구조체의 연관 함수로 리팩토링

```rust
impl Config {
    fn new(args: &[String]) -> Config {
        let query = args[1].clone();
        let filename = args[2].clone();
        Config { query, filename }
    }
}
```

- 에러 처리 개선하기

```rust
// 1. 분기 처리로 패닉 발생시키기
impl Config {
    fn new(args: &[String]) -> Config {
        if args.len() < 3 {
            panic!("필요한 인수가 지정되지 않았습니다.");
        }
        // ...
    }
}

// 2. 해당 구문을 Result를 사용하도록 변경
impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("필요한 인수가 지정되지 않았습니다.");
        }

        let query = args[1].clone();
        let filename = args[2].clone();
        Ok(Config { query, filename })
    }
}
```

- 설정이나 에러 처리에 관련된 부분이 아닌 나머지 코드를 분리

```rust
// 1. main 함수에서 run 함수를 분리
fn run(config: Config) {
    let mut file = File::open(config.filename).expect("파일을 읽지 못했습니다.");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("파일을 읽는 도중 에러가 발생했습니다.");

    println!("파일 내용:\n{}", contents);
}

```

- run 함수 내에서 에러 리턴하기

```rust
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // expect() 함수를 ? 연산자로 대체하기
    let mut file = File::open(config.filename)?;
    let mut contents = String::new();
    // ? 연산자에 걸린 에러는 Box<dyn Error>로 처리되고 Err(e)의 인수로 전달됨
    file.read_to_string(&mut contents)?;

    println!("파일 내용:\n{}", contents);

    Ok(())
}
```

- lib.rs로 모듈 분리하기

```rust
// src/lib.rs
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

pub struct Config { /*...*/ }
impl Config { /*...*/ }
pub fn run(config: Config) -> Result<(), Box<dyn Error>> { /*...*/ }

// src/main.rs
mod lib;

use lib::*;
use std::env;
use std::process;

fn main() { /*...*/ }
```

**12.4 테스트 주도 방법으로 라이브러리의 기능 개발하기**

(1) 실패하는 테스트 작성하기

```rust
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fase, productive.
Pick three.";

        assert_eq!(
            vec!["safe, fase, productive."],
            search(query, contents)
        );
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    vec![]
}
```

(2) 테스트가 성공하도록 코드 작성하기

```rust
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    // 열은 파일 내 여러 라인 순회
    for line in contents.lines() {
        // 각 줄이 검색어를 포함하는지 확인하기
        if line.contains(query) {
            // 검색어를 포함하는 줄 저장하기
            results.push(line);
        }
    }
    results
}
```

**12.5 환경 변수 다루기**

- 환경 변수로 사용하는 `case_sensitive`를 Config 구조체에 대소문자 구분을 위한 새로운 설정 옵션을 추가하기

```rust
pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}
```

- `case_sensitive`가 설정되면 사용할 `search_case_insensitive` 함수를 작성하기

```rust
pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        // to_lowercase()를 사용하여 비교하기
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}
```

</div>
</details>

### Chapter 13. 함수형 언어의 기능: 반복자와 클로저

<details>
<summary>열기</summary>
<div markdown="13">

- 러스트는 이미 존재하는 언어와 기법에 많은 영향을 받았고 그 중에서도 특히 함수형 프로그래밍의 영향을 가장 많이 받았다
- 러스트 내 함수형 스타일 문법에는 크게 클로저와 반복자가 있다

**13.1 클로저: 주변 환경을 캡처하는 익명 함수**

- 러스트의 클로저는 변수에 저장하거나 다른 함수에 인수로 전달하는 **익명 함수**이다
- 클로저를 이용해 동작을 추상화 할 수 있다
- 클로저 함수는 `|param (, param2): param type| -> retrun type { body }`와 같이 사용한다

```rust
let expensive_closure = |num: u32| -> u32 {
        println!("시간이 오래 걸리는 계산을 수행 중...");
        thread::sleep(Duration::from_secs(2));
        num
    };
```

- 제네릭 매개변수와 Fn 트레이트를 이용한 클로저 사용

```rust
struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}
```

- 명시한 구조체 Cacher로 메모이제이션 된 값을 사용하여 실행 시간을 줄이기
- 메모이제이션을 구현한 구조체의 인스턴스는 항상 처음 호출된 매개변수의 값만을 저장한다는 한계를 가짐

```rust
fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|num| {
        println!("시간이 오래 걸리는 계산을 수행 중...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!(
            "오늘은 {}번의 팔굽혀펴기를 하세요!",
            expensive_result.value(intensity)
        );
        println!(
            "그 다음엔 {}번의 윗몸 일으키기를 하세요!",
            expensive_result.value(intensity)
        )
    } else {
        if random_number == 3 {
            println!("오늘은 수분을 충분히 섭취하며 쉬세요...");
        } else {
            println!(
                "오늘은 {}분간 달리기를 하세요!",
                expensive_result.value(intensity)
            );
        }
    }
}
```

- FnOnce 트레이트는 같은 범위에 선언된 변수를 사용할 수 있다
- 이 범위를 클로저의 환경이라고 하며 클로저는 캡쳐된 변수를 사용하려면 이 변수들의 소유권을 가져야 한다
- 클로저를 선언하는 시점에 변수의 소유권은 클로저 안으로 이동한다
- Once라는 이름에서 알 수 있듯 이 트레이트는 소유권을 단 한 번만 갖는다
- FnMut 트레이트는 값을 가변으로 대여하므로 환경에서 가져온 값을 변경할 수 있다
- Fn 트레이트는 환경에서 값을 불변으로 대여한다

**13.2 반복자를 이용해 일련의 아이템 처리하기**

- 러스트에서 반복자는 지연 특성이 있어 반복자를 실제로 사용하는 메서드를 호출하기 전까지는 아무런 일도 일어나지 않는다

```rust
let v1 = vec![1, 2, 3];
let v1_iter = v1.iter();

for val in v1_iter {
    println!("값: {}", val);
}
```

- 표준 라이브러리에 정의된 Iterator 트레이트

```rust
pub trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
    //...
}
```

- 반복자의 next 메서드 호출

```rust
fn iterator_demonstration() {
    let v1 = vec![1, 2, 3];
    let mut v1_iter = v1.iter();

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);
}
```

- Iterator 트레이트는 표준 라이브러리가 제공하는 기본 구현과는 다른 여러 메서드를 제공한다
- 일부 메서드는 next 메서드를 호출하므로 Iterator 트레이트를 구현하려면 next 메서드를 반드시 구현해야 한다
- next 메서드를 호출하는 메서드는 내부적으로 반복자를 소비하기 때문에 **소비 어댑터**라고 부르기도 한다

```rust
#[test]
fn iterator_sum() {
    let v1: Vec<i32> = vec![1, 2, 3];
    let v1_iter = v1.iter().map(|x| x + 1);
}

#[test]
fn iterator_sum() {
    let v1: Vec<i32> = vec![1, 2, 3];
    // map 메서드로 새로운 반복자를 생성하여 collect 메서드로 벡터를 생성
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

    assert_eq!(v2, vec![2, 3, 4]);
}
```

- filter 반복자 어댑터를 이용한 환경을 캡처하는 클로저 생성

```rust
#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

// shoe_size 변수를 캡처하는 클로저를 filter로 넘겨 Shoe 구조체 인스턴스 컬렉션을 순회하기
// 1. 이 함수는 shoes 변수에 저장된 벡터와 shoe_size 매개변수의 소유권을 가짐
fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    // 2. into_iter 메서드를 이용해 벡터의 소유권이 있는 반복자를 생성
    // 3. filter 메서드를 호출해 클로저가 true를 리턴한 항목만을 가지는 새로운 반복자를 생성해 리턴한다
    // 4. collect 메서드를 호출하면 반복자 어댑터가 리턴한 반복자를 벡터에 저장해서 리턴한다(Vec<Shoe>)
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

#[test]
fn filters_by_size() {
    let shoes = vec![
        Shoe { size: 10, style: String::from("스니커즈"), },
        Shoe { size: 13, style: String::from("샌달"), },
        Shoe { size: 10, style: String::from("부츠"), },
    ];

    let in_my_size = shoes_in_my_size(shoes, 10);

    assert_eq!(
        in_my_size,
        vec![
            Shoe { size: 10, style: String::from("스니커즈") },
            Shoe { size: 10, style: String::from("부츠") },
        ]
    )
}
```

- Iterator 트레이트를 이용해 직접 반복자를 구현하기

```rust
// 1. Counter 구조체를 선언하고 count 필드에 0을 초깃값으로 대입해 새 인스턴스를 생성하는 new 함수를 구현
struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

// 2. Counter 구조체에 Iterator 트레이트를 구현하기
impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;

        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}

// 3. next() 메서드 구현 테스트
#[test]
fn calling_next_directly() {
    let mut counter = Counter::new();

    assert_eq!(counter.next(), Some(1));
    assert_eq!(counter.next(), Some(2));
    assert_eq!(counter.next(), Some(3));
    assert_eq!(counter.next(), Some(4));
    assert_eq!(counter.next(), Some(5));
    assert_eq!(counter.next(), None);
}

// 4. Iterator 트레이트가 지원하는 다른 메서드 활용해보기
#[test]
fn using_other_iterator_trait_methods() {
    let sum: u32 = Counter::new()
        .zip(Counter::new().skip(1))
        // map이나 filter와 같은 메서드는 클로저 구문이 들어가야 함
        .map(|(a, b)| a * b)
        .filter(|x| x % 3 == 0)
        .sum();
    assert_eq!(18, sum);
}
```

**13.3 입출력 프로젝트의 개선**

- 반복자를 이용해 clone 메서드 호출 제거하기

```rust
// src/main.rs
fn main() {
    // let args: Vec<String> = env::args().collect();
    // println!("{:?}", args);

    // env::args() 값을 Config::new() 함수에 그대로 전달
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        println!("인수를 구문분석하는 동안 오류가 발생했습니다: {}", err);
        process::exit(1);
    });

    // ...
}

// src/lib.rs
impl Config {
    // 따라서 이 부분도 변경되어야 함
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        args.next();

        // next() 함수를 사용하여 반복자를 순회하며 입력받은 변수를 각각 대입
        let query = match args.nexT() {
            Some(arg) => arg,
            None => return Err("검색어를 지정해야 합니다."),
        }
        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("파일명을 지정해야 합니다."),
        }
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}
```

- 반복자 어댑터를 이용해 리팩토링하기

```rust
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    /*
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results
    */
    contents.lines()
            .filter(|line| line.contains(query))
            .collection()
}
```

- 루프와 반복자의 성능을 비교하자면 반복자를 이용한 구현이 약간 더 빠르다
- 반복자는 러스트의 **무비용 추상화** 기능 중 하나로 추상화를 사용한다고 해서 추가적인 런타임 오버헤드가 발생하지 않는다

</div>
</details>

### Chapter 14. 카고와 crates.io

<details>
<summary>열기</summary>
<div markdown="14">

- cargo로 할 수 있는 더 많은 일들
- 릴리즈 프로필을 이용한 빌드 커스터마이징

```
> cargo build
> cargo build --release
```

```rust
// Cargo.toml
[profile.dev] // 개발 프로필의 최적화 수준
opt-level = 0 // opt-level = 1

[profile.release]
opt-level = 3 // 기본값은 3
```

- crates.io 사이트에 크레이트 발행하기
- 러스트의 문서 주석은 ///로 시작하며 텍스트 형식화를 위한 마크다운을 지원함

- **카고 작업공간**: 바이너리 크레이트의 크기가 증가해 여러 개의 라이브러리 크레이트로 나누어야 할 때 사용한다
- 디렉터리 구조를 통해 라이브러리를 나눌 때 사용한다

```
> cargo new adder
> cargo new add-one --lib

└ Cargo.lock
└ Cargo.toml
└ add-one
    └ Cargo.lock
    └ src
        └ lib.rs
└ adder
    └ Cargo.lock
    └ src
        └ main.rs
└ target
```

- Cargo.toml과 main.rs에 라이브러리 추가하기

```rust
// adder/Cargo.toml
[dependencies]
add-one = { path = "../add-one" }

// adder/src/main.rs
use add_one;
```

- 라이브러리 사용하기

```rust
> cargo build
> cargo run -p adder // -p 인수를 이용해 작업 공간 내에서 실행할 패키지의 이름을 지정하기
```

- 작업공간에 외부 크레이트 의존성 추가하기

```rust
[dependenceis]

rand = "0.6.5"
```

</div>
</details>

### Chapter 15. 스마트한 포인터

<details>
<summary>열기</summary>
<div markdown="15">

- 포인터는 메모리에 주소를 가지고 있는 변수를 일컫는 보편적인 개념
- **스마트 포인터**는 포인터처럼 동작할 뿐만 아니라 추가적인 메타데이터와 기능을 포함하는 **데이터 구조**이다
- 스마트 포인터는 C++에서 유래한 것이며 다른 언어에도 있다
- 이 포인터는 값의 소유권을 추척해서 여러 코드가 데이터의 소유권을 공유하고 소유권을 가진 코드가 더 없으면 해당 데이터를 해제한다
- `String` 타입이나 `Vec<T>`같은 타입들이 스마트 포인터에 해당하는데 이 두 타입은 모두 메모리를 소유하며 데이터를 갱신할 수 있다
- 스마트 포인터는 주로 구조체를 이용해 구현한다
- 러스트의 표준 라이브러리가 제공하는 가장 보편적인 스마트 포인터
  - 힙 메모리에 값을 할당하는 `Box<T>` 구조체
  - 다중 소유권을 지원하고자 참조 카운트를 수행하는 `Rc<T>` 구조체
  - 런타임이 아닌 컴파일 타임에 대여 규칙을 적용하는 타입인 `RefCell<T>`를 통해 접근하는 `Ref<T>`와 `RefMut<T>` 구조체
- 이 외에도 **내부 가변성** 패턴과 메모리 누수를 유발하는 **순환 참조**를 방지하는 방법도 있다

**15.1 `Box<T>`를 이용해 힙 메모리의 데이터 참조하기**

- 가장 직관적인 스마트 포인터는 `Box<T>` 타입으로 표현하는 박스
- 박스는 데이터를 스택이 아닌 힙 메모리에 저장하고 스택에는 힙 데이터를 가리키는 포인터만 저장한다

```rust
fn main() {
    let b = Box::new(5);
    println!("b = {}", b);
}
```

- 러스트는 컴파일 타임에 어떤 타입이 얼마나 많은 메모리를 사용하는지 알아야 한다
- 재귀 타입은 이 컴파일 타임에 크기를 알 수 없는 타입으로, 이 때 박스를 사용해 정해진 크기로 정의할 수 있다
- 콘스 리스트(cons list)

```rust
// List를 다시 넣기 때문에 무한하게 List가 생성된다
enum List {
    Cons(i32, List),
    Nil,
}

let normal_list = Cons(1,
        Cons(2,
            Cons(3, Nil)
        ));

// Box<T>를 사용해 재귀 타입의 크기를 미리 정해놓는다
enum List {
    Cons(i32, Box<List>),
    Nil,
}

let box_list = Cons(1,
    Box::new(Cons(2,
        Box::new(Cons(3, Nil)
    ))));
```

- Deref 트레이트를 이용해 스마트 포인터를 참조처럼 취급할 수 있다
- Deref 트레이트를 구현하면 역참조 연산자 \*의 동작을 변경할 수 있다

```rust
// 역참조 연산자를 이용해 포인터가 가리키는 값 읽어오기
fn main() {
    let x = 5;
    let y = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
}
```

- `Box<T>`와 유사한 스마트 포인터를 직접 구현해보기

```rust
struct MyBox<T>(T);

//
impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}
```

- 러스트의 표준 라이브러리의 Deref 트레이트는 deref라는 메서드가 있고 이를 구현해야 역참조를 구현할 수 있다

```rust
use std::ops::Deref;

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}
```

- Deref 트레이트가 불변 참조에 대한 _ 연산자의 동작을 재정의하는 것 처럼 DerefMut 트레이트는 가변 참조에 대한 _ 연산자의 동작을 재정의한다
- 러스트는 타입과 트레이트 구현이 다음 세 가지 경우에 해당하면 역참조를 강제 실행한다
  - `T: Deref<Target=U>`일 때 `&T`를 `&U`로 변환한다
  - `T: DerefMut<Target=U>`일 때 `&mut T`를 `&mut U`로 변환한다
  - `T: Deref<Target=U`일 때 `&mut T`를 `&U`로 변환한다

**15.2 Drop 트레이트를 구현해서 메모리를 해제할 때 코드 실행하기**

- Drop 트레이트는 값이 범위를 벗어날 때의 동작을 재정의한다
- 이 트레이트는 어떤 타입에도 구현할 수 있으며 파일이나 네트워크 연결 같은 자원을 해제한느 코드를 명시할 수 있다
- 스마트 포인터를 구현할 때 거의 항상 Drop 트레이트의 기능을 사용한다
- 러스트의 경우 값이 범위를 벗어날 때 호출되는 코드를 직접 명시할 수도 있고 컴파일러가 자동으로 이 코드를 삽일할 수도 있다

```rust
fn main() {
    let c = CustomSmartPointer {
            data: String::from("첫번째 데이터"),
        };
    let d = CustomSmartPointer {
            data: String::from("두번째 데이터"),
        };
    println!("CustomSmartPointer를 생성했습니다");
} // 명시적으로 drop을 호출하지 않아도 이 범위를 넘어가면 호출된다
/*
* CustomSmartPointer를 생성했습니다
* CustomSmartPointer의 데이터 '두번째 데이터'를 해제합니다
* CustomSmartPointer의 데이터 '첫번째 데이터'를 해제합니다
*/

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("CustomSmartPointer의 데이터 '{}'를 해제합니다", self.data);
    }
}
```

- drop 함수는 main 함수의 끝에 도달하면 자동으로 호출하므로 **명시적으로 호출할 수 없다**
- 값을 일찍 해제하려면 `std::mem::drop` 함수를 호출해야 한다

```rust
fn main() {
    //...
    drop(c);
    println!("변수 c를 해제하였습니다.");
}
```

**15.3 `Rc<T>` 참조 카운터 스마트 포인터**

- 러스트는 다중 소유권을 지원하기 위해 참조 카운터의 약어를 따온 `Rc<T>`타입을 지원한다
- `Rc<T>`타입은 프로그램의 여러 부분에서 데이터를 읽을 수 있도록 데이터를 힙 메모리에 저장할 때 사용한다

```rust
use std::rc::Rc;

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

fn main() {
    let e = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    let f = Cons(3, Rc::clone(&e));
    // Rc::clone 대신 .clone()을 사용해도 되지만 Rc::clone이 관례임
    let g = Cons(4, Rc::clone(&e));
```

- `Rc<T>`의 복제는 참조 카운터를 증가시킨다
- 참조 카운터는 `strong_count()` 함수로 호출할 수 있다

```rust
//...
let e = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
println!("e를 생성한 이후의 카운터 = {}", Rc::strong_count(&e)); // 1
let f = Cons(3, Rc::clone(&e));
println!("f를 생성한 이후의 카운터 = {}", Rc::strong_count(&e)); // 2
let g = Cons(4, Rc::clone(&e));
println!("g를 생성한 이후의 카운터 = {}", Rc::strong_count(&e)); // 3
drop(g);
println!("g를 드랍한 이후의 카운터 = {}", Rc::strong_count(&e)); // 2
```

- 내부 가변성은 러스트가 데이터의 불변 참조를 이용해 데이터를 가공할 수 있도록 지원하는 디자인 패턴이다
- 보통 data borrowed 와 같은 대여 규칙에 의해 차단되지만 이 패턴은 데이터 구조 안에 unsafe 코드를 사용해 값의 가공과 대여를 관장하는 러스트의 규칙을 우회한다

**15.4 `RefCell<T>` 타입과 내부 가변성 패턴**

```rust
/*
* `RefCell<T>` 타입을 이용해 런타임에 대여 규칙 강제하기
* Box<T> 타입의 경우 대여 규칙의 불변성질은 컴파일타임에 평가된다
* RefCell<T> 타입은 런타임에 적용된다
* 대부분의 대여 규칙을 컴파일 타임에 확인하는 것이 러스트로서는 최선
* 대여 규칙을 런타임에 검사하게 되면 메모리 안정성과 관련된 작업을 수행할 수 있다
* Rc<T>와 마찬가지로 RefCell<T> 또한 단일 스레드 환경에서만 사용해야 한다
* Rc<T>는 같은 데이터에 대한 다중 소유권을 지원하지만 Box<T>와 RefCell<T> 타입은 단일 소유권만 지원한다
*/

// 컴파일 되지 않는 코드
fn main() {
    let x = 5;
    let y = &mut x; // [error]: cannot borrow mutably
}

// 내부 가변성의 활용 예: 모조 객체
pub trait Messenger {
    // self의 불변참조와 전달할 텍스트를 매개변수로 선언하고 있다
    fn send(&self , msg: &str);
}

pub struct LimitTracker<'a, T: 'a + Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
    where T: Messenger {
        pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
            LimitTracker {
                messenger,
                value: 0,
                max,
            }
        }

        // 이 set_value 메서드는 아무것도 리턴하지 않음
        pub fn set_value(&mut self, value: usize) {
            self.value = value;

            let percentage_of_max = self.value as f64 / self.max as f64;

            if percentage_of_max >= 0.75 && percentage_of_max < 0.9 {
                self.messenger.send("경고: 최댓값의 75%를 사용했습니다.");
            } else if percentage_of_max >= 0.9 && percentage_of_max < 1 {
                self.messenger.send("긴급 경고: 최댓값의 90%를 사용했습니다.");
            } else if percentage_of_max >= 1.0 {
                self.messenger.send("에러: 최대값을 초과했습니다.");
            }
        }
    }

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {
        // `self` is a `&` reference, so the data it refers to cannot be borrowed as mutable
        fn send(&self, message: &str) {
            // self.sent_messages.push(String::from(message));
            // borrow_mut() 메서드를 사용하기
            self.sent_messages.borrow_mut().push(String::from(message));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);
        limit_tracker.set_value(80);
        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }
}
```

- `Rc<T>`와 `RefCell<T>`를 조합하면 가변 데이터에 다중 소유권을 적용할 수 있다

```rust
use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

fn main() {
    let value = Rc::new(RefCell::new(5));
    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));
    let b = Cons(Rc::new(RefCell::new(6)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(10)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    println!("a 수정 후 = {:?}", a);
    // a 수정 후 = Cons(RefCell { value: 15 }, Nil)
    println!("b 수정 후 = {:?}", b);
    // b 수정 후 = Cons(RefCell { value: 6 }, Cons(RefCell { value: 15 }, Nil))
    println!("c 수정 후 = {:?}", c);
    // c 수정 후 = Cons(RefCell { value: 10 }, Cons(RefCell { value: 15 }, Nil))
}
```

**15.5 메모리 누수의 원인이 되는 순환 참조**

- 러스트에서는 메모리 누수도 메모리 안전성의 일부
- 참조하는 Cons 열것값을 변경할 수 있도록 `RefCell<T>` 타입을 저장하는 콘스 리스트

```rust
use std::rc::Rc;
use std::cell::RefCell;
use List::{Cons, Nil};

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil
}

impl List {
    // 두 번째 원소에 쉽게 접근하기 위한 메서드
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match *self {
            Cons(_, ref item) => Some(item),
            Nil => None,
        }
    }
}

fn main() {
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));
    println!("a의 최초 rc 카운트 = {}", Rc::strong_count(&a));
    // a의 최초 rc 카운트 = 1
    println!("a의 다음 아이템 = {:?}", a.tail());
    // a의 다음 아이템 = Some(RefCell { value: Nil })

    let b = Rc::new(Cons(5, RefCell::new(Rc::clone(&a))));
    println!("b를 생성한 후 a의 rc 카운트 = {}", Rc::strong_count(&a));
    // b를 생성한 후 a의 rc 카운트 =
    println!("b의 최초 rc 카운트 = {}", Rc::strong_count(&b));
    // b의 최초 rc 카운트 = 1
    println!("b의 다음 아이템 = {:?}", b.tail());
    // b의 다음 아이템 = Some(RefCell { value: Cons(5, RefCell { value: Nil }) })

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("a를 변경한 후 b의 rc 카운트 = {}", Rc::strong_count(&b));
    // a를 변경한 후 b의 rc 카운트 = 2
    println!("a를 변경한 후 a의 rc 카운트 = {}", Rc::strong_count(&a));
    // a를 변경한 후 a의 rc 카운트 = 2
}
```

- 순환 참조를 방지하기 위해 `Rc<T>` 대신 `Week<T>`를 활용할 수 있다
- `Rc::clone` 메서드는 `Rc<T>` 인스턴스의 `strong_count` 값을 증가시키며, `Rc<T>` 타입은 `strong_count` 값이 0인 인스턴스만 해제한다
- `Rc::downgrade` 메서드에 `Rc<T>` 참조를 전달해 `Rc<T>`에 저장된 값에 대한 약한 참조를 생성할 수도 있다

```rust
use std::rc::Rc;
use std::cell:RefCell;

// 자식 노드를 갖는 노드로 구성된 트리 데이터 구조
#[derive(Debug)]
struct Node {
    value: i32,
    // 자식 노드가 부모 노드를 인식하도록 하려면 parent 필드를 추가하기
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

fn main() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf)
    ); // 1. leaf 인스턴스를 생성한 직후: leaf strong = 1, weak = 0

    // println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    // leaf parent = None

    {
        // 2. leaf 자식 노드를 갖는 branch 인스턴스 생성
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });

        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        println!(
            "branch strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf)
        );

        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf)
        );
    }

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf)
    );
}
```

</div>
</details>

### Chapter 16. 자신 있는 동시성

<details>
<summary>열기</summary>
<div markdown="16">

**16.1 코드를 동시에 실행하기 위한 스레드**

- 프로그램의 연산을 여러 개의 스레드로 분리하면 프로그램이 여러 작업을 한번에 실행할 수 있어 성능을 향상시킬 수 있지만 복잡도 역시 증가한다
- 스레드는 동시에 실행되므로 본질적으로 다른 스레드에서 실행되는 코드의 순서를 보장할 수 없다
- 그렇기 때문에 **경합상태**(일정하지 않은 순서로 데이터나 자원에 접근하는 상황)이나 **데드락**(두 스레드가 모두 서로가 자원의 사용을 마칠때까지 대기해서 두 스레드 모두 대기 상태에 놓이는 상황)이 일어날 수 있다
- 프로그래밍 언어가 제공하는 스레드는 M:N 모델의 그린 스레드이며 러스트는 저수준의 언어이기 때문에 러스트의 표준 라이브러리는 1:1 스레드 구현만을 지원한다
- 새 스레드를 생성하는 것은 `thread::spawn` 함수를 호출하고 새 스레드에서 실행하기를 원하는 코드를 담고 있는 클로저를 전달하면 된다

```rust
use std::thread;
use std::time::Duration;

fn main() {
    thread::spawn(||{
        for i in 1..10 {
            println!("새 스레드: {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("주 스레드: {}", i);
        thread::sleep(Duration::from_millis(1));
    }
}
/*
주 스레드: 1
새 스레드: 1
새 스레드: 2
주 스레드: 2
주 스레드: 3
새 스레드: 3
새 스레드: 4
주 스레드: 4
새 스레드: 5
*/
```

- `join`메서드를 사용한다면 `thread::spawn` 함수가 리턴한 값을 변수에 저장해야 한다
- `thread::spawn` 함수는 `JoinHandle` 타입을 리턴하고 `join` 메서드를 호출하면 그 스레드가 종료될 때 까지 기다린다

```rust
use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("새 스레드: {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("주 스레드: {}", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
}
/*
주 스레드: 1
새 스레드: 1
새 스레드: 2
주 스레드: 2
새 스레드: 3
주 스레드: 3
주 스레드: 4
새 스레드: 4
새 스레드: 5
새 스레드: 6
새 스레드: 7
새 스레드: 8
새 스레드: 9
*/
```

- `move` 클로저는 한 스레드의 데이터를 다른 스레드에서 사용할 수 있게 한다
- `thread::spawn` 함수에 전달한 클로저는 매개변수가 없다
- 자식 스레드의 코드는 주 스레드의 데이터를 전혀 활용하지 않는다
- 주 스레드의 데이터를 사용하려면 자식 스레드의 클로저가 그 값을 캡처해야 한다
- 클로저 앞에 `move` 키워드를 추가하면 클로저가 필요로 하는 값을 대여하려는 러스트의 동작을 변경해 클로저 값의 소유권을 가질 수 있다

```rust
fn main() {
    let v = vec![1, 2, 3];
    let handle = thread::spawn(move || {
                println!("벡터: {:?}", v);
    });
    handle.join().unwrap();
}
```

- 동시성의 안정을 보장하려는 방법 중에는 **메시지 전달**이 빠르게 대중화되고 있다
- 러스트가 가진 도구 중 메시지 전달 동시성을 구현하기 위한 것은 **채널**이다
- 채널은 **전달자**와 **수신자**로 구성된다 전달자는 상류에 해당하고 수신자가 하류에 해당한다
- 채널은 전달자나 수신자가 해제되면 함께 닫힌다

```rust
use std::thread;
use std::sync::mpsc;

fn main() {
    // mpsc::channel()은 (Sender<T>, Receiver<T>) 튜플을 리턴함
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("안녕하세요");
        tx.send(val).unwrap();
    });

    //
    let received = rx.recv().unwrap();
    println!("수신: {}", received);
    // 수신: 안녕하세요
}
```

- 안전한 동시성 코드의 작성을 돕기 위해 소유권 규칙은 메시지 전송에 중요한 역할을 한다

```rust
use std::thread;
use std::sync::mpsc;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("자식 스레드가"),
            String::from("안녕하세요"),
            String::from("라고"),
            String::from("인사를 합니다"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("수신: {}", received);
    }
}
```

- 스레드를 생성하기 전 `mpsc::Sender::clone()` 메서드를 통해 채널의 전달자를 복제해 놓으면 두 개의 스레드에 각기 다른 메세지를 채널의 수신자에게 전송할 수 있다

```rust
// ...
let (tx, rx) = mpsc::channel();

let tx1 = mpsc::Sender::clone(&tx); // 전달자를 복제

thread::spawn(move || {
    let vals = vec![
        String::from("자식 스레드가"),
        String::from("안녕하세요"),
        String::from("라고"),
        String::from("인사를 합니다"),
    ];

    for val in vals {
        tx1.send(val).unwrap();
        thread::sleep(Duration::from_secs(1));
    }
});

thread::spawn(move || {
    let vals = vec![
        String::from("그리고"),
        String::from("더 많은"),
        String::from("메시지를"),
        String::from("보냅니다."),
    ];

    for val in vals {
        tx.send(val).unwrap();
        thread::sleep(Duration::from_secs(1));
    }
});

for received in rx {
    println!("수신: {}", received);
}

// ...
```

**16.2 공유 상태 동시성**

- 대부분 프로그래밍 언어에서 채널은 단일 소유권을 의미한다
- 러스트에서는 스마트 포인터를 이용하면 다중 소유권을 적용할 수 있다
- 특정 시점에 한 스레드만 데이터에 접근하도록 허용하려면 뮤텍스를 사용해야 한다
- 뮤텍스는 데이터를 사용하기 전에 반드시 락을 획득해야 하며, 뮤텍스가 보호하는 데이터르 사용한 후에는 다른 스레드가 락을 얻을 수 있도록 데이터를 언락해야 한다

```rust
use std::sync::Mutex;

fn main() {
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("m = {:?}", m);
}
```

- 다중 스레드 간에 `Mutex<T>`를 공유하려면 해당 타입의 다중 소유권을 해결해야 하며 러스트에서는 `Arc<T>` 타입을 사용할 수 있다

```rust
use std::thread;
use std::sync::{Arc, Mutex};

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
    println!("결과: {}", *counter.lock().unwrap());
}
```

**16.3 Sync와 Send 트레이트로 동시성 확장하기**

- 러스트에서는 `std::marker` 트레이트의 Sync와 Send 트레이트를 사용하면 동시성 구현할 수 있다
- Send 트레이트는 이 트레이트를 구현하는 타입의 소유권이 다른 스레드로 이전될 수 있음을 표시하는 **마커**이다
- `Rc<T>`를 복제해서 다른 스레드로 소유권을 이전하면 참조 카운터를 동시에 변경할 수 있기 때문에 이를 사용할 수는 없다
- `Sync` 마커 트레이트는 여러 스레드가 타입을 안전하게 참조할 수 있음을 표시하기 위한 트레이트로 타입 T가 Sync 트레이트를 구현하고 있으며 &T가 Send 트레이트를 구현하고 있다면 이 참조는 다른 스레드로 안전하게 전달할 수 있다

</div>
</details>

### Chapter 17. 러스트의 객체지향 프로그래밍 기능

<details>
<summary>열기</summary>
<div markdown="17">

- 객체지향 언어의 특징: 데이터와 행위를 정의하는 객체, 상세 구현을 숨기는 캡슐화, 타입 시스템으로서의 상속과 코드 공유를 위한 상속 등
- 러스트에서는 다른 언어의 객체와 구분하고자 구조체와 열거자를 '객체'로 부르지 않는다
- 트레이트 객체는 데이터와 행위가 결합하므로 다른 언어에서 말하는 객체와 유사하다
- 단, 트레이트 객체가 전통적인 객체와 다른 점은 트레이트 객체에 데이터를 추가할 수 없다는 점이다
- 트레이트 객체의 목적은 **공통된 행위에 대한 추상화를 제공하는 것**
- 러스트는 트레이트 객체를 사용할 때 반드시 동적 호출을 사용한다
- 트레이트 객체는 객체 안정성을 가진 트레이트만 사용할 수 있다
- 트레이트에 선언된 모든 메서드의 리턴 타입이 Self가 아니며 제네릭 타입의 매개변수가 없을 때 객체 안정성을 확보했다고 할 수 있다

**객체지향 디자인 패턴 구현**

- 동작을 구현하기

```rust
mod blog;
use blog::Post;

fn main() {
     // 1) 블로그의 새 초고를 작성하기
    let mut post = Post::new();

    // 2) 텍스트를 추가할 수 있는 함수를 제공
    post.add_text("나는 오늘 점심으로 샐러드를 먹었다.");
    assert_eq!("", post.content());

    // 3) 블로그의 리뷰를 요청하기
    post.request_review();
    assert_eq!("", post.content());
    // 아직 게재가 승인된 상태가 아니므로 빈 문자열을 리턴해야 한다

    // 4) 포스트가 승인을 받는다면 앞서 입력했던 text와 같아야 할 것
    post.approve();
    assert_eq!("나는 오늘 점심으로 샐러드를 먹었다.", post.content());
}
```

- Post 구조체와 new 함수, State 트레이트와 Draft 구조체를 정의하기

```rust
// src/blog/mod.rs
pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    // 새 인스턴스를 생성하는 new 함수
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }

    // 텍스트를 추가하는 add_text 메서드
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    // content 메서드는 초고 상태일 때엔 빈 문자열을 리턴한다
    pub fn content(&self) -> &str {
        ""
    }

    // 리뷰를 요청하여 상태를 변경하게 하는 request_review 메서드
    // 이 메서드가 실행되면
    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review());
        }
    }
}


trait State {
    fn request_review(self: Box<Self>) -> Box<State>;
}

struct Draft {}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<State> {
        Box::new(PendingReview {})
    }
}

struct PendingReview {}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<State> {
        self
    }
}
```

- Post 구조체와 DraftPost 구조체로 나누기

```rust
pub struct Post {
    content: String,
}

impl Post {
    pub fn new() -> DraftPost {
        DraftPost {
            content: String::new(),
        }
    }

    pub fn content(&self) -> &str {
        &self.content
    }
}

pub struct DraftPost {
    content: String,
}

impl DraftPost {
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn request_review(self) -> PendingReviewPost {
        PendingReviewPost {
            content: self.content,
        }
    }
}

pub struct PendingReviewPost {
    content: String,
}

impl PendingReviewPost {
    pub fn approve(self) -> Post {
        Post {
            content: self.content,
        }
    }
}
// ...
```

</div>
</details>

### Chapter 18. 패턴과 매칭

<details>
<summary>열기</summary>
<div markdown="18">

- 패턴은 러스트가 간단한 타입과 복잡한 타입의 구조 모두를 매칭할 수 있는 특별한 문법
- 패턴을 match 표현식과 더불어 다른 구조와 함께 사용하면 프로그램의 흐름을 더욱 세밀하게 제어할 수 있다
- 패턴은 **리터럴**, **해체된 배열, 열거자, 구조체 혹은 튜플**, **변수**, **와일드카드**, **자리지정자**의 조합으로 구성된다

**18.1 패턴을 활용할 수 있는 위치**

- 패턴은 match 표현식의 가지에서 사용할 수 있다
- match 표현식의 요구사항 중 하나는 match 표현식에 사용된 값에 대한 모든 가능성을 반드시 처리해야 한다는 점이다

```rust
match VALUE {
    PATTERN => EXPRESSION,
    PATTERN => EXPRESSION,
    PATTERN => EXPRESSION,
}
```

- match 표현식을 더 짧게 표현하는 용도로 `if let` 표현식을 사용할 수 있다

```rust
fn main() {
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
}
```

- `while let` 조건부 루프 또한 패턴이 일치하는 동안 루프를 실행한다

```rust
let mut stack = Vec::new();

stack.push(1);
stack.push(2);
stack.push(3);

while let Some(top) = stack.pop() {
    println!("{}", top);
}
```

- for 루프는 러스트의 가장 일반적인 루프의 형태이며 for 루프 또한 패턴을 적용할 수 있다

```rust
let v = vec!['a', 'b', 'c'];

for (index, value) in v.iter().enumerate() {
    println!("인덱스 {}의 값: {}", index, value);
}
```

- let 키워드로 튜플을 해체하여 지정할 때엔 변수의 개수가 튜플 원소의 수와 같아야 한다

```rust
let (x, y) = (1, 2) // Ok
let (x, y) = (1, 2, 3) // Error
```

- 튜플을 해체하여 변수로서 사용하는 패턴은 함수의 매개변수일 때도 가능하다

```rust
fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("현재 위치: ({}, {})", x, y);
}

fn main() {
    let point = (3, 5);
    print_coordinates(&point);
}
```

**18.2 부인 가능성: 패턴이 일치할수도, 그렇지 않을 수도 있다**

- 패턴에는 **부인할 수 있는 패턴**과, **부인할 수 없는 패턴**으로 나뉜다

```rust
// let 구문에 부인할 수 있는 패턴을 적용하기 (컴파일 되지 않음)
let some_option_value = Some(5);
let Some(x) = some_option_value;

// 부인할 수 있는 패턴을 적용하기 위해 if let 구문으로 수정
if let Some(x) = some_option_value {
    println!("{}", x);
}

// 부인할 수 없는 패턴을 if let 구문에서 사용 (경고가 출력됨)
if let x = 5 {
    println!("{}", x);
}
```

**18.3 패턴 문법**

- 패턴은 리터럴에 직접 비교할 수 있다

```rust
let x = 1;
// 부인할 수 있는 패턴을 if let 구문에서 사용한다면
// match는 부인할 수 없는 패턴을 사용한다
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
    _ => println!("불일치, zx = {:?}", z),
}

println!("결과: z = {:?}, y = {:?}", z, y);
```

- match 표현식에는 `|` 문법(or)을 이용해 여러개의 패턴을 비교할 수 있다
- `...` 문법은 가장 자리의 값을 포함하는 값의 범위와 비교한다
- 이 범위 문법은 숫자나 char 값만 사용할 수 있다
- `...` 문법은 `..=` 문법으로 교체되었다 ([Rust 공식문서](https://doc.rust-lang.org/std/ops/struct.RangeInclusive.html))

```rust
let x = 1;

match x {
    1 | 2 => println!("1 또는 2"),
    _ => println!("그 외"),
}

match x {
    1..=5 => println!("1 ~ 5 사이 값"),
    _ println!("그 외 나머지 값"),
}

let c = 'c';

match c {
    'a'..='j' => println!("아스키 문자의 전반부"),
    'k'..='z' => println!("아스키 문자의 후반부"),
    _ => println!("그 외 나머지 값"),
}
```

- 구조체, 열거자의 해체문법과 match 문법의 혼용

```rust
// 구조체의 해체
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p = Point { x: 0, y: 7 };
    match p {
        Point { x, y: 0 } => println!("x축 {}에 위치하는 점", x),
        Point { x: 0, y } => println!("y축 {}에 위치하는 점", y),
        Point { x, y } => println!("({}, {})에 위치하는 점", x, y),
    }
}

// 열거자의 해체
enum Message {
    Quit,
    Move { x: i32, y: i32},
    Write(String),
    ChangeColor(Color),
}

fn main() {
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
}
```

- 패턴의 값은 `_`를 사용함으로써 일부나 전체를 무시할 수 있다
- 변수 앞에 `_`를 붙임으로써 경고 출력을 막을 수 있다
- 하지만 변수 앞에 `_`가 붙어도 값은 바인딩 되며 소유권도 있다
- `..`을 이용해도 값의 일부만 검사하고 나머지는 무시할 수 있다

```rust
// _의 기본 사용 예시
fn foo(_: i32, y: i32) {
    println!("이 함수는 매개변수로 y({})만 사용한다", y);
}

// _를 중첩해서 값의 일부만 무시하기
let mut setting_value = Some(5);
let new_setting_value = Some(10);

match(setting_value, new_setting_value) {
    // 패턴 안에서 _로 Some 열것값을 확인하지만 그 안에 저장된 실제 값은 사용하지 않음
    (Some(_), Some(_)) => {
        println!("이미 설정된 값을 덮어 쓸 수 없습니다.");
    }
    _ => {
        setting_value = new_setting_value;
    }
}

println!("현재 설정: {:?}", setting_value);

// 튜플의 부분 무시하기
let numbers = (2, 4, 6, 8, 10);
match numbers {
    (one, _, three, _, five) => {
        println!("숫자 세 개: {}, {}, {}", one, three, five);
        // 숫자 세 개: 2, 6, 10
    }
}

// .. 사용하기
struct Rectangle {
    width: i32,
    height: i32,
    z_index: i32,
    x_axis: i32,
    y_axis: i32,
}

let rectangle = Rectangle {
    width: 120,
    height: 240,
    z_index: 999,
    x_axis: 50,
    y_axis: 50,
}

match rectangle {
    Rectangle { width, height, .. } => println!("넓이: {}, 높이: {}", width, height);
}
```

- match 표현식의 가지에 일치해야 할 패턴 외에도 추가적인 if 조건을 지정해서 그 조건이 일치할 때만 해당 가지를 실행할 수 있는데 이것을 **매치 가드**라고 한다

```rust
let num = Some(4);
match num {
    Some(x) if x < 5 => println!("5보다 작은 값: {}", x),
    Some(x) => println!("{}", x),
    None => (),
}
```

- `@`연산자는 어떤 값이 패턴과 일치하는지를 비교함과 동시에 그 값을 가진 변수를 생성한다

```rust
enum Name {
    Gender { code: i32 },
}

let henry = Name::Gender { code : 10 };

match henry {
    Name::Gender { code: code_variable @ 10..=19 } => {
        // code 값을 체크하며 code_variable에 해당 값 바인딩하기
        println!("henry의 성별 코드는 {}", code_variable)
    },
    Name::Gender { code: 20..=29 } => {
        println!("성별 코드가 여성입니다")
    },
    Name::Gender { code } => {
        println!("다른 코드를 찾았습니다. {}", code)
    }
}
```

</div>
</details>

### Chapter 19. 러스트의 고급 기능

<details>
<summary>열기</summary>
<div markdown="19">

**19.1 안전하지 않은 러스트**

- 러스트에서는 `unsafe` 키워드를 이용하여 **원시 포인터의 역참조**, **안전하지 않ㅇ느 함수나 메서드 호출**, **가변 정적 변수에 대한 접근이나 변경**, **안전하지 않은 트레이트의 구현** 등을 할 수 있다
- `unsafe` 키워드가 반드시 메모리 안전성 관련 문제가 발생하는 것은 아니며 이는 `unsafe` 블록 내 코드가 유효한 방법으로 메모리에 접근한다는 것을 명시하기 위함임

```rust
fn main() {
    // 참조에서 원시 포인터 생성
    let mut num = 5;
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    // 임의의 메모리에 접근하는 원시 포인터
    let address = 0x012345usize;
    let r = address as *const i32;

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
}
unsafe fn dangerous() {
    println!("이 함수는 unsafe 내에서 실행됩니다.")
}
// 안전한 코드로 구현한 split_at_mut 함수
fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();

    assert!(mid <= len);

    (&mut slice[..mid], &mut slice[mid..])
    // 이 함수는 slice를 두 번 대여할 수 없다는 에러를 출력함
}

// 안전하지 않은 코드를 이용해 split_at_mut 함수를 구현
fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
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
```

- 러스트 코드를 이용해 다른 언어로 작성된 코드를 호출해야 할 때가 있는데 이 때에 `extern` 키워드를 사용할 수 있다
- 러스트 함수를 다른 언어에서 호출할 때에도 이 `extern` 키워드를 사용할 수 있다

```rust
extern "C" {
    fn abs(input: i32) -> i32;
}

fn main() {
    unsafe {
        println!("C언어에 따르면 -3의 절대값은 {}이다", abs(-3));
    } // C언어에 따르면 -3의 절대값은 3이다
}
```

- 러스트는 전역 변수를 허용하기는 하지만 소유권 규칙 문제가 발생할 수도 있다 정적변수는 `static` 키워드를 사용한다
- 정적 변수의 값은 메모리의 고정된 주소를 갖지만 상수는 선언 때마다 데이터가 중복되어 생성된다
- 또한, 정적 변수는 가변 변수가 될 수 있다

```rust
static HELLO_WORLD: &str = "안녕하세요!";
static mut COUNTER: u32 = 0;

fn main() {
    println!("{}", HELLO_WORLD);

    add_to_count(5);
    unsafe {
        println!("가변 변수 COUNTER: {}", COUNTER);
    }
}

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}
```

- `unsafe` 키워드를 사용한다면 안전하지 않은 트레이트를 선언하고 구현할 수 있다

```rust
unsafe trait Foo {}
unsafe impl Foo for i32 {}
```

**19.2 고급 트레이트**

- **연관 타입**은 타입 자리지정자를 트레이트와 연결하며 이 자리지정자 타입을 사용해 트레이트의 메서드 시그니처를 정의할 수 있다

```rust
pub trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
    // Item 타입이 자리지정자 타입으로 Option<Self::Item>을 리턴한다
    // 제네릭을 사용한다면 구현 할 때마다 타입을 명시해야 하지만
    // 자리지정자 타입을 사용한다면 여러번 구현하지 않도고 다른 타입을 리턴하는 메서드를 쉽게 구현할 수 있다
}
```

- 이 문법은 연산자를 오버로딩하는 경우 자주 사용된다

```rust
use std::ops::Add;

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

// Add 트레이트를 구현하여 + 연산자 기능 오버로딩
impl Add for Point {
    type Output = Point;
    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

fn main() {
    assert_eq!(Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
               Point { x: 3, y: 3 });
}

// 서로 다른 구조체를 더하는 Add 트레이트를 구현하기
struct Millimeters(u32);
struct Meters(u32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}
```

- 러스트의 트레이트는 다른 트레이트에 선언된 것과 같은 이름의 메서드를 선언할 수 있고 하나의 타입에 두 트레이트를 모두 구현할 수도 있다

```rust
trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("안녕하세요 기장입니다.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("날아라! 얍!");
    }
}

impl Human {
    fn fly(&self) {
        println!("*양 팔을 펄럭이며 새를 흉내낸다*")
    }
}
// ---- Human 구조체를 -> Pilot, Wizard가 상속(비스무리한)
fn main() {
    let person = Human;

    Pilot::fly(&person); // 안녕하세요 기장입니다.
    Wizard::fly(&person); // 날아라! 얍!
    person.fly(); // *양 팔을 펄럭이며 새를 흉내낸다*
}
```

- 완전한 식별자 문법은 `<타입명 as 트레이트명>::함수(매개변수 들)` 과 같이 사용한다

```rust
trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("점박이")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("멍멍이")
    }
}

fn main() {
    println!("새끼 강아지 이름은 {}", Dog::baby_name());
    // 새끼 강아지 이름은 점박이
    println!("새끼 강아지 이름은 {}", <Dog as Animal>::baby_name());
    // 새끼 강아지 이름은 멍멍이
}
```

- 러스트에는 한 트레이트에서 다른 트레이트의 기능을 요청하는 수퍼 트레이트가 있다

```rust
use std::fmt;

trait OutlinePrint: fmt::Display {
    // 값을 애스터리스크로 꾸며서 출력하는 메서드
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();

        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

struct Point {
    x: i32,
    y: i32,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
```

- 뉴타입 패턴이란 튜플 구조체에 새로운 타입을 생성하는 것

```rust
use std::fmt;
struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

fn main() {
    let w = Wrapper(vec![
        String::from("안녕하세요"),
        String::from("러스트입니다."),
    ]);
    println!("w = {}", w);
}
```

**19.3 고급 타입 시스템**

- **뉴타입 패턴**은 값을 정적으로 명확히 구분하고 값의 단위를 표시하는 등의 용도로도 사용한다
- 뉴타입 패턴의 또 다른 예로 상세 구현에 대한 추상화 제공이 있다
- 러스트는 타입 이론에서 사용하는 잘 알려지지 않은 ! 타입을 지원하며 이 타입은 아무런 값도 없는 빈 타입처럼 동작한다
- 이 **네버 타입**은 함수가 값을 리턴하지 않을 때 리턴 타입 자리에 사용하며 네버를 리턴하는 함수를 **발산 함수**라고 한다

```rust
fn bar() -> ! {
    //...
}
```

- 러스트는 특정 타입의 값에 어느 정도의 메모리를 할당해야 하는지 알아야 하며 타입의 모든 값은 반드시 같은 양의 메모리를 사용해야 한다
- 동적 사이즈 타입을 다루기 위해 러스트는 `Sized` 트레이트를 지원한다

```rust
fn generic<T>(t: T) {}
// 위 함수는 이렇게 작성된 것 처럼 취급된다
fn generic<T: Sized>(t: T) {}
// 이 제한을 완화하려면 ?Sized와 &T를 활용할 수 있다
fn generic<T: ?Sized>(t: &T) {}
// ?Sized 트레이트 경계는 Sized 트레이트 경계와는 반대 개념으로 T는 Sized 트레이트를 구현할 수도 있고 아닐 수도 있다
// 타입이 Sized 트레이트를 구현하지 않을 수도 있기 때문에 T가 아닌 &T로 지정해주어야 한다
```

**19.4 고급 함수와 클로저**

- 함수를 다른 함수의 인수로 전달하려면 **함수 포인터**를 이용한다
- 클로저와 달리 fn 타입은 트레이트가 아닌 타입이기 때문에 fn을 매개변수의 타입으로 직접 명시할 수 있다

```rust
// 기본적인 함수 포인터
fn add_one(x: i32) -> i32{
    x + 1
}
fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

fn main() {
    let answer = do_twice(add_one, 5);
    println!("정답은 {}", answer);
    // map 함수는 클로저와 함수를 모두 허용한다
    let list_of_numbers = vec![1, 2, 3];
    let list_of_strings: Vec<String>
        = list_of_numbers.iter()
                         .map(|i| i.to_string())
                         .collect();
    let list_of_strings2: Vec<String>
        = list_of_numbers.iter()
                         .map(ToString::to_string)
                         .collect();
}
```

- 클로저는 트레이트로 표현하기 때문에 클로저를 직접 리턴할 수는 없고 따라서 Box와 같은 트레이트 객체를 사용해 우회하여 리턴해야 한다

**19.5 매크로**

- 매크로란 러스트에서 `macro_rules!`로 정의하는 선언적 매크로와 절차적 매크로가 있다
  - `#[derive]` 매크로는 구조체와 열거자에 적용된 특성을 상속해서 삽입될 코드를 명시한다
  - 특성형 매크로는 어떤 아이템에도 적용할 수 있는 사용자 정의 특성을 정의한다
  - 함수형 매크로는 함수 호출처럼 보이지만 인수로 전달된 토큰에 적용된다
- 기본적으로 매크로는 다른 코드를 작성하는 코드로서 **메타프로그래밍**이라고도 한다
- 메타프로그래밍은 개발자가 작성하고 관리해야 하는 코드양을 줄이기 위한 것이고 함수 또한 이런 역할을 수행한다
- 함수의 시그너처는 반드시 함수에 필요한 매개변수 개수와 타입을 선언해야 하지만 매크로는 매개변수가 가변적이다
- 하지만 매크로는 정의하기 어렵고 관리하기 어렵다
- 선언적 매크로를 정의하려면 `macro_rules!`를 사용한다

```rust
// 1. 이 매크로를 선언한 크레이트를 가져올 때 매크로도 함께 사용할 수 있도록 지시하는 macro_export 애노테이션
#[macro_export]
// 2. 매크로의 이름을 macro_rules!와 함께 지정하며 이 때 매크로의 이름 뒤에 !는 붙이지 않는다
macro_rules! vec {
    // 3. 본문에는 여러 가지 패턴을 작성할 수 있으며 이 가지에는 ($($x:expr),*) 패턴과 이 패턴에 적용되는 코드 블록이 작성되었다
    // 매크로는 러스트의 코드 구조와 일치해야 하기 때문에 패턴 문법과는 다르게 작성된다
    ( $( $x:expr ),* ) => {
        // 전체 패턴은 () 괄호로 시작한다  그 다음 $()로 시작하고 이 괄호 안의 패턴과 일치하는 값을 캡처해 코드로 대체한다
        {
            let mut temp_vec = Vec::new();

            $(
                // $x:expr로 캡쳐한 값을 여기서 사용한다
                temp_vec.push($x);
            )*
            temp_vec
        }
    }
}
```

- 절차적 매크로를 생성할 때에는 반드시 각자의 크레이트 안에 정의해야 하며 특별한 크레이트 타입을 사용해야 한다

```rust
use proc_macro;
#[some_attribute]
    pub fn some_name(input: TokenStream) -> TokenStream {

    }
```

- 사용자 정의 상속 매크로는 `#[derive(HelloMacro)]` 처럼 연관 함수를 포함하는 트레이트를 `#[derive()]`내에 표기한다

```rust
// hello_macro/src/main.rs
use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;

#[derive(HelloMacro)]
struct Pancakes;

fn main() {
    Pancakes::hello_macro();
}
// hello_macro/scr/lib.rs
pub trait HelloMacro {
    fn hello_macro();
}

// hello_macro_derive/Cargo.toml
[lib]
proc-macro = true

[dependencies]
syn = "0.14.4"
quote = "0.6.3"

// hello_macro_derive/src/lib.rs
extern crate proc_macro;

use crate::proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    // TokenStream로 전달된 러스트 코드를 조작할 수 있도록 문법 트리를 구성
    let ast = syn::parse(input).unwrap();
    // 트레이트 구현체를 빌드
    // syn 크레이트는 문자열 안의 러스트 코드를 구문분석해서 필요한 작업을 수행할 데이터 구조로 변환한다
    // quote 크레이트는 syn 크레이트의 데이터 구조를 전달 받아 러스트 코드로 다시 변환한다
    impl_hello_macro(&ast)
}

/*
매크로의 특성이 적용된 코드를 구문분석한 후 얻게 된 인스턴스의 구조
DeriveInput {
    ident: Ident {
        ident: "Pancakes",
        span: #0 bytes(95..103)
    },
    data: Struct(
        DataStruct {
            struct_token: Struct,
            fields: Unit,
            semi_token: Some(
                Semi
            )
        }
    )
}
*/

fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    // ast.ident 필드로부터 애노테이션이 적용된 타입의 이름을 가진 Ident 구조체의 인스턴스를 얻어온다
    // ident: Ident { ident: "Pancakes", span: #0 bytes(95..103) }
    let gen = quote! {
        // quote! 매크로는 리턴할 러스트 코드를 정의한다
        impl HelloMacro for #name {
            // #name 템플릿을 이용하면 quote! 매크로는 이 템플릿을 변수 name의 값으로 교체한다
            fn hello_macro() {
                println!("안녕하세요 매크로! 내 이름은 {}입니다!", stringify!(#name));
                // 러스트에 내장된 매크로로 문자열 리터럴로 변환한다
            }
        }
        gen.into()
    }
}
```

- 특성형 매크로는 사용자 정의 상속 매크로와 유사하지만 `derive` 특성을 위한 코드를 생성하는 것이 아니라 새로운 특성을 생성한다

```rust
#[route(GET, "/")]
fn index() {}

#[proc_macro_attribute]
pub fn route(attr: TokenStream, item: TokenStream) -> TokenStream {}
```

- 함수형 매크로는 `TokenStream` 타입 매개변수를 사용하며 다른 두 가지 절차적 매크로와 마찬가지로 러스트 코드를 이용해서 `TokenStream`을 조작한다

```rust
let sql = sql!(SELECT * FROM postes WHERE id = 1);
```

</div>
</details>

### Chapter 20. 최종 프로젝트: 다중 스레드 웹서버 구축

<details>
<summary>열기</summary>
<div markdown="20">

- 본 챕터는 바뀐 부분이 있어 해당 [공식 문서](https://doc.rust-lang.org/book/ch20-01-single-threaded.html)를 참고하였음
- 웹서버를 구현하기 위한 단계
  - TCP와 HTTP에 대한 사전 지식 학습
  - 소켓으로 TCP 연결 대기하기
  - HTTP 요청 구문 분석하기
  - 적절한 HTTP 응답 생성하기
  - 스레드 풀을 이용해 처리량 개선하기

**20.1 단일 스레드 웹서버 구현하기**

- 웹서버를 구현하는 데 주축이 되는 두 개의 프로토콜은 HTTP와 TCP이다

```rust
use std::net::TcpListener;

fn main() {
    // 127.0.0.1:7878 주소로 TCP 연결을 대기하기
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    // incoming 메서드는 연속된 스트림에 대한 반복자를 리턴한다
    for stream in listener.incoming() {
        // unwrap 메서드를 호출해서 스트림에 문제가 있으면 프로그램을 중단하도록 하기
        let stream = stream.unwrap();
        // handle_connection 함수로 stream 인스턴스를 전달한다
        handle_connection(stream);
    }
}

// stream 매개변수를 가변 매개변수로 선언해야 TcpStream 인스턴스 내부적으로 변경되는 것들에 대응할 수 있음
fn handle_connection(mut stream: TcpStream) {
    // 스택에 데이터를 저장할 buffer 인스턴스(1Mb)를 생성한다
    let mut buffer = [0; 1024];
    // stream.read 메서드로 이 버퍼를 전달하여 데이터를 버퍼에 채운다
    stream.read(&mut buffer).unwrap();
    // String::from_utf8_lossy 함수는 &[u8] 타입을 매개변수로 전달 받아 그 배열로부터 String 인스턴스를 생성한다
    // 함수 이름 내 'lossy'의 뜻은 이 함수가 유효하지 않은 UTF-8 값을 U+FFFD REPLACEMENT CHARACTER로 대체한다는 뜻이다
    println!("요청: {}", String::from_utf8_lossy(&buffer[..]));
}

/* 요청 결과
요청: GET / HTTP/1.1
Host: 127.0.0.1:7878
Connection: keep-alive
Cache-Control: max-age=0
sec-ch-ua: " Not;A Brand";v="99", "Google Chrome";v="91", "Chromium";v="91"
sec-ch-ua-mobile: ?0
DNT: 1
Upgrade-Insecure-Requests: 1
User-Agent: Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.106 Safari/537.36
Accept: text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,* /*;q=0.8,application/signed-exchange;v=b3;q=0.9
Sec-Fetch-Sit
*/
```

- HTTP 요청/응답 형식

```rust
// 1. HTTP 요청 형식
Method Request-URI HTTP-Version CRLF
// request line: 클라이언트가 요청하는 정보를 표현
// 요청 줄의 마지막은 CRLF(carriage return and line feed)로 끝을 맺는다
headers CRLF
// 클라이언트가 요청하는 통합 자원 식별자(URI): URL과 유사하지만 서로 다른 개념
message-body

// 2. HTTP 응답 형식
HTTP-Version Status-Code Reason-Phrase CRLF
// status line: 응답에 사용한 HTTP 버전과 요청의 결과를 의미하는 상태 코드와 상태코드를 설명하는 응답 구문으로 구성된다
// 예) HTTP/1.1 200 OK\r\n\r\n
headers CRLF
message-body
```

- html 파일을 불러와 요청에 대한 응답으로 서버에 송신하기

```html
<!-- index.html -->
<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="UTF-8" />
    <meta http-equiv="X-UA-Compatible" content="IE=edge" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <title>Index Page</title>
  </head>
  <body>
    <h1>러스트로 짠 첫 페이지</h1>
    <p>마지막 챕터의 프로젝트!</p>
  </body>
</html>
```

```rust
// ...
use std::fs;

// ...
fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let contents = fs::read_to_string("index.html").unwrap();
    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
        contents.len(),
        contents
    );
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
```

- 요청을 검증하여 선택적으로 응답하기

```rust
// 1. 단순히 응답을 if else 문법을 사용해 두 가지로 분류하기
fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/`.`\r\n";
    // 성공했을 때엔 정상적으로 출력하기
    if buffer.starts_with(get) {
        let contents = fs::read_to_string("index.html").unwrap();
        let response = format!(
            "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
            contents.len(),
            contents
        );
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
    // 실패했을 때엔 404 페이지를 표시하기
    else {
        let status_line = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
        let contents = fs::read_to_string("404.html").unwrap();
        let response = format!("{}{}", status_line, contents);
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}

// 2. 응답만을 리팩토링하여 중복되는 구문을 줄이기
fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    // println!("요청: {}", String::from_utf8_lossy(&buffer[..]));

    // 튜플에 포함된 두 값을 let 구문을 이용해 해체하기
    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "index.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };

    let contents = fs::read_to_string("404.html").unwrap();
    let response = format!("{}{}", status_line, contents);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
```

**20.2 다중 스레드 서버로 전환하기**

- 1. 요청별로 스레드를 생성하는 코드 작성하기

```rust
fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        //각 스트림마다 새 스레드 생성
        // `thread::spawn` 함수는 새로운 스레드를 생성한 후 전달된 클로저를 실행한다
        thread::spawn(|| {
            handle_connection(stream);
        });
    }
}
```

- 2. 제한된 스레드를 사용하는 인터페이스 구현하기

```rust
fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    // thread::spawn 함수 대신 가상의 ThreadPool 구조체의 인터페이스를 사용
    let pool = ThreadPool::new(4);
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        // thread::spawn 함수와 유사한 인터페이스의 pool.execute 메서드를 호출하여 클로저를 전달
        pool.execute(|| {
            handle_connection(stream);
        });
    }
}
```

- 3. 컴파일러 주도 개발 방식으로 ThreadPool 구조체 작성하기
  - 연관 함수 new를 작성해야 하며 이 함수는 4라는 인수를 전달 받아야 한다
  - 타입 매개변수 F에는 Send 트레이트 경계와 'static 수명 경계도 지정되어 있다
  - FnOnce()는 매개변수와 리턴값이 없는 클로저를 표현하기 때문에 ()가 필요하다

```rust
// src/lib.rs
pub struct ThreadPool {}

impl ThreadPool {
    //
    pub fn new(size: usize) -> ThreadPool { ThreadPool }
    /* thread::spawn 함수의 정의
    pub fn spawn<F, T>(f: F) -> JoinHandle<T>
        where
                // 이
            F: FnOnce() -> T + Send + 'static,
            T: Send + 'static
    */
    // spawn과 유사한 동작을 구현하기
    pub fn execute<F>(&self, f: F)
        where
            F: FnOnce() + Send + 'static
    {
    }
}

// src/bin/main.rs
use #[PROJECT_NAME]::ThreadPool; // 프로젝트 폴더명 넣기
```

- 4. new 함수가 지정된 개수의 스레드를 생성하는지 검증하기

```rust
// src/lib.rs
use std::thread;

pub struct ThreadPool {
    threads: Vec<thread::JoinHandle<()>>,
}

impl ThreadPool {
    /// 새 ThreadPool 인스턴스를 생성
    ///
    /// size 매개변수는 풀의 스레드 개수를 지정한다
    ///
    /// # Panics
    ///
    /// size 매개변수의 값이 0이면 패닉을 발생시킴
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        ThreadPool
    }
    //...
}
```

- 5. 스레드를 저장할 공간 생성하기
  - spawn 함수는 `JoinHandle<T>` 타입을 리턴한다
  - ThreadPool 구조체에 지정된 크기의 벡터를 생성하여 for 루프에서 지정된 수 만큼 스레드를 생성하고 `thread::JoinHandle<()` 타입의 벡터 인스턴스에 저장하고 이를 ThreadPool 인스턴스에 담아 리턴한다

```rust
// src/lib.rs
use std::thread;

pub struct ThreadPool {
    // 벡터에 저장하는 타입은 thread::JoinHandle 타입
    threads: Vec<thread::JoinHandle<()>>,
}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        // 스레드를 저장할 벡터를 새로 생성
        // with_capacity 함수는 Vec::new와 같은 동작을 하지만 벡터의 공간을 미리 할당한다는 차이가 있다
        let mut threads = Vec::with_capacity(size);

        for _ in 0..size {
            // 스레드를 생성해서 벡터에 저장한다
        }

        ThreadPool {
            threads
        }
    }
    // 생략
}
```

- 6. 스레드 풀에서 스레드로 코드를 보내는 Worker 구조체
  - 풀링을 구현할 때 보편적으로 사용하는 Worker 구조체를 구현
  - 이 구조체를 JoinHandle<()>의 벡터 대신 저장한다
  - Worker 구조체에 실행할 클로저를 전달받는 메서드를 구현
  - 각 Worker 인스턴스는 id를 부여해 각 작업자를 구분할 것

```rust
pub struct ThreadPool {
    // 필드를 workers 로 변경
    workers: Vec<Worker>,
}


impl ThreadPool {
    pub fn new(size: usize) -> Result<ThreadPool, PoolCreationError> {
        assert!(size > 0);
        let mut workers = Vec::with_capacity(size);

        // for 루프의 카운터를 이용해서 Worker의 new 함수로 id 인수를 전달
        // 새로 만들어진 Worker 인스턴스를 workers 벡터에 저장한다
        for id in 0..size {
            workers.push(Worker::new(id))
        }

        ThreadPool {
            threads
        }
    }

    pub fn execute<F>(&self, f: F)
        where
            F: FnOnce() + Send + 'static
    {
    }
}

// 이 구조체는 외부에서 사용되지 않으므로 pub 선언하지 않음
struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

// Worker::new 함수는 전달된 id 매개변수와 빈 클로저를 이용해 새로 생성한 스레드에 대한 JoinHandle<()> 인스턴스를 이용해 새 Worker 인스턴스를 생성한다
impl Worker {
    fn new(id: usize) -> Worker {
        let thread = thread::spawn(|| {});

        Worker {
            id,
            thread,
        }
    }
}
```

- 7. 채널을 이용해 스레드로 요청 전달하기
  - ThreadPool 구조체는 채널을 생성하고 채널의 전달자를 저장
  - Worker 인스턴스들은 채널의 수신자를 저장
  - 채널에 전달할 클로저를 저장할 새로운 Job 구조체를 선언
  - execute 메서드에서는 실행할 작업을 채널에 전달
  - Worker 구조체는 자신이 보유한 스레드에서 채널의 수신자를 이용해 수신한 클로저를 실행

```rust
impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        let (sender, receiver) = mpsc::channel();
        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            // Worker::new 메서드에 수신자도 전달
            workers.push(Worker::new(id, reveiver));
        }

        ThreadPool { workers, sender }
    }
    //...
}

impl Worker {
    // 수신자도 매개변수로 받도록 추가
    fn new(id: usize, reviever: mpsc::Receiver<Job>) -> Worker {
        let thread = thread::spawn(|| {
            reviever;
        });

        Worker { id, thread }
    }
}
```

- 모든 작업자가 하나의 수신자를 공유하면서 작업을 여러 스레드로 분산하고자 한다면 `Arc<Mutex<T>>` 타입을 사용해야 한다

```rust
use std::sync::Arc;
use std::sync::Mutex;

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        let (sender, receiver) = mpsc::channel();

        let reveiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            // Worker::new 함수는 Arc::clone 함수로 수신자의 주소를 복사하여 전달
            workers.push(Worker::new(id, Arc::clone(&reveiver)));
        }

        ThreadPool { workers, sender }
    }
    //...
}

impl Worker {
    // Worker::new 수신자의 주소에 저장된 타입은 Arc<Mutex<T>> 타입이다
    // (ThreadPool 구조체 내에서 reveiver를 Arc::new(Mutex::new(receiver))로 정의함)
    fn new(id: usize, reviever: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(|| {
            reviever;
        });

        Worker { id, thread }
    }
}
```

- 8. execute 메서드 구현하기

```rust
type Job = Box<FnOnce() + Send + 'static>;

impl ThreadPool {
    //...
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        // 전달된 클로저를 이용해 Job 인스턴스를 생성하고 채널에 전달하기
        let job = Box::new(f);
        self.sender.send(job).unwrap();
    }
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let job = receiver.lock() // receiver 매개변수의 lock 메서드를 호출하여 뮤텍스로부터 락을 획득
                              .unwrap() // 락을 획득하지 못한다면 패닉을 리턴하기 위한 1차 unwrap 메서드
                              .recv() // 락을 획득했다면 recv 메서드를 호출하여 Job 인스턴스를 수신
                              .unwrap(); // recv 메서드의 에러 처리를 위한 2차 unwrap 메서드
            println!("시작: 작업자 {}", id);
            (*job)();
        });

        Worker { id, thread }
    }
}
```

- 이 때, `Box<T>` 내에 있는 FnOnce 클로저를 호출하기 위해서는 `Box<T>` 바깥으로 이동해야 하지만 러스트는 `Box<T>` 내에 저장된 값의 크기를 모르므로 이를 우회하기 위해 `self: Box<Self>` 문법을 이용한다

```rust
// FnBox 트레이트는 call_box 메서드를 갖는다
trait FnBox {
    // 이 메서드는 Box<T>의 바깥으로 옮기며 self 매개변수에 대한 소유권을 갖는다
    fn call_box(self: Box<Self>);
}

// FnOnce() 트레이트를 구현하는 모든 F 타입 매개변수에 대해 FnBox 트레이트를 구현하여 call_box 메서드를 호출할 수 있도록 선언
impl<F: FnOnce()> FnBox for F {
    fn call_box(self: Box<F>) {
        // call_box()를 호출하면 self의 소유권을 갖는다
        (*self)()
    }
}

type Job = Box<dyn FnBox + Send + 'static>;
```

- 이 작업자 스레드를 `while let` 구문으로 구현하지 않는 이유는 뮤텍스의 락을 획득하고 해제하는 작업이 원활하지 않을 수 있기 때문

**20.3 우아한 종료와 해제**

- 스레드 풀에 Drop 트레이트를 구현하여 풀이 해제되면 스레드를 모두 조인해서 작업이 완료될 때 까지 기다린다

```rust
// 스레드 풀이 범위를 벗어날 때 각 스레드를 조인시키기
impl Drop for ThreadPool {
    fn drop(&mut self) {
        // 각 스레드의 workers를 순회
        for worker in &mut self.workers {
            println!("종료: 작업자 {}", worker.id);
            // 여기서 스레드의 join 메서드를 호출
            worker.thread.join().unwrap();
            // 여기서 worker 인스턴스에 대한 가변 대여값만을 가지고 있어서 join 메서드를 호출할 수 없다
        }
    }
}
```

- Worker 구조체가 `Option<thread::JoinHandle()>` 타입이라면 take 메서드를 호출하여 Some 열것값에 저장된 값을 바깥으로 꺼내오고 그 자리에 None 값을 대신 할당할 수 있다

```rust
struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
    // thread 필드의 타입을 Option<thread::JoinHandle<()>>로 변경했으므로
}

impl Worker {
    //...
        Worker {
            id,
            thread: Some(thread), // 이 부분 또한 변경되어야 한다
        }
    }
}

// 그리고 if let 구문으로 타입 가드 및 Some 열것값을 가져오게 변경한다
impl ThreadPool {
    //...
     fn drop(&mut self) {
        for worker in &mut self.workers {
            println!("종료: 작업자 {}", worker.id);
            /* 이 부분 */
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}
```

- 이 작업들은 Worker 인스턴스의 스레드가 실행하는 클로저 내의 로직이 무한 루프에 빠지기 때문에 스레드가 종료되지 않는다
- 이를 해결하기 위해서는 실행할 작업을 큐에서 가져오거나 대기를 중지하고 무한 루프를 빠져나오는 작업으로 수정해야 한다

```rust
// 스레드의 메세지를 주고받을 수 있게 열거자 Message를 새로 정의
enum Message {
    NewJob(Job),
    Terminate,
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
    // 수신자의 제네릭을 Message로 변경하여 Job 타입 대신 Message 타입을 전달하도록 함
}

impl ThreadPool {
    // ...
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        // 따라서 ThreadPool 에서도 수신자가 Message의 NewJob 열것값을 통해 Job을 가지도록 한다
        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

impl Worker {
    // Worker도 이 메세지를 수신받기 위해서 reviver의 타입이 Arc<Mutex<mpsc::Receiver<Message>>>가 되고 (이전엔 Job 타입이었음)
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv().unwrap();

            // message의 열것값인 NewJob(job)과 Terminate 값에 따라 match 값을 설정하고 Terminate 메세지가 온다면 break 하여 무한 루프에서 벗어난다
            match message {
                Message::NewJob(job) => {
                    println!("시작: 작업자 {}", id);
                    job.call_box();
                }
                Message::Terminate => {
                    println!("종료 메세지 수신: 작업자 {}", id);
                    break;
                }
            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}

// 각 작업자 스레드의 join 메서드를 호출하기 전에 Message::Terminate 값을 보내야 하는데 이는 drop 메서드에서 실행한다
impl ThreadPool {
    fn drop(&mut self) {
        println!("모든 작업자 종료");
        // workers 벡터를 두 번 순회한다
        // 종료 메세지를 처음에 보내게 되면 종료 메세지를 수신한 후로는 더 이상 채널로부터 요청을 수신하지 않는다
        for _ in &mut self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }
        for worker in &mut self.workers {
            println!("종료: 작업자 {}", worker.id);
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}
```

</div>
</details>

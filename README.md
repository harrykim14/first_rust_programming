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

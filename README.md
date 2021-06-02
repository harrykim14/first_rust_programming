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

<style>
div.numberTable th { 
    background: #CCC;
}
</style>
<div class="numberTable">

| 크기  | 부호 있음 | 부호 없음 |
| ----- | --------- | --------- |
| 8bit  | i8        | u8        |
| 16bit | i16       | u16       |
| 32bit | i32       | u32       |
| 64bit | i64       | u64       |
| arch  | isize     | usize     |

</div>

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

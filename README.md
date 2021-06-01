## 러스트 배워보기

with 러스트 프로그래밍 공식 가이드 (스티브 클라브닉, 캐롤 니콜스 저 / 장현희 번)

### Chapter 1. 시작하기

<details>
<summary>열기</summary>
<div markdown="1">

1.1 설치하기

- rustup을 이요해 러스트를 내려받기
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

</div>
</details>

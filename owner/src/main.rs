fn main() {
    string_literal();
    move_example();
    clone_example();
    copy_example();
    owner_exmaple();
    return_example();
    lental_example();
    borrow_example();
    reference_example();
    let mut s = String::from("hello");
    let len = first_word(&mut s);
    println!("hello의 길이: {}", len);
    slice_word();
}

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

fn owner_exmaple() {
    // 변수 s가 범위 내에 생성됨
    let s = String::from("hello");
    // 변수 s값이 함수 내로 이동하고 s는 더이상 유효하지 않음
    takes_ownership(s);
    // println!("{}", s); -> borrow of moved value: `s`

    // 변수 x가 범위 내에 생성됨
    let x = 5;
    // 변수 x는 i32 타입이므로 복사를 수행하므로도 x는 이 이후에도 유효함
    makes_copy(x);
} // 변수 x가 범위를 벗어나고 s도 벗어남 x는 drop이 수행되지만 s는 함수 내로 이동했기 때문에 아무런 일도 일어나지 않는다

fn takes_ownership(some_string: String) {
    // some_string 변수가 범위 내에 생성됨
    println!("{}", some_string);
} // 이 시점에서 some_string의 drop이 일어나면서 메모리에서 해제됨

fn makes_copy(some_integer: i32) {
    // some_integer 변수가 범위 내에 생성됨
    println!("{}", some_integer);
} // some_integer 변수가 범위를 벗어나도 아무런 일도 일어나지 않는다(i32 타입)

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

fn borrow_example() {
    let mut s = String::from("hello");
    let r1 = &mut s;
    // let r2 = &mut s; -> second mutable borrow occurs here
    println!("{}", r1);
}

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

// fn death_example() -> &String {
//     let s = String::from("hello");

//     &s
// }

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

fn slice_word() {
    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];
    println!("{}, {}!", hello, world);
    let slice1 = &s[0..5];
    let slice2 = &s[..5];
    println!("{}? {}!", slice1, slice2);

    let len = s.len();
    let slice3 = &s[0..len];
    let slice4 = &s[..];
    println!("{}? {}!", slice3, slice4);
}

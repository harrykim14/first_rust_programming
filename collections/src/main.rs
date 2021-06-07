fn main() {
    vector_example();
    different_types_vector();
    string_example();
    hashmap_example();
}

fn vector_example() {
    // Vec<T> 타입은 어떤 타입이든 저장할 수 있다
    let v1: Vec<i32> = Vec::new();
    // 값을 포함하는 벡터를 생성하는 매크로가 존재함
    let v1 = vec![1, 2, 3];
    println!("v1: [{}, {}, {}]", v1[0], v1[1], v1[2]);
    // 벡터 수정하기
    let mut v2 = Vec::new();

    v2.push(5);
    v2.push(6);
    v2.push(7);

    // 벡터의 값 읽기
    let third: &i32 = &v2[2];
    println!("세 번째 원소: {}", third);

    match v2.get(2) {
        Some(third) => println!("세 번째 원소: {}", third),
        None => println!("세 번째 원소가 없습니다."),
    }

    let v3 = vec![1, 2, 3, 4, 5];
    // [] 방식은 패닉을 발생시킨다
    // let does_not_exist = &v3[100];
    // 존재하지 않는 값에 접근할 때 get 메서드를 사용한다면 None값이 리턴된다
    let does_not_exist = v3.get(100);
    if does_not_exist == None {
        println!("값이 없습니다");
    }

    // for 루프를 이용해 벡터를 순회하기
    let mut v4 = vec![1, 2, 3, 4, 5];
    for i in &v4 {
        println!("{}", i);
    }

    // 가변 참조로 순회하기
    for i in &mut v4 {
        *i += 50;
    }
} // v가 범위를 벗어나면 drop되는 것은 변수와 동일하다

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

fn string_example() {
    // 러스트는 언어 내에 문자열 슬라이스인 str 타입만을 지원한다
    // String 타입은 표준 라이브러리가 제공하는 타입
    // UTF-8 형식으로 인코딩된 문자열 타입을 사용하므로 어떤 언어를 쓰더라도 유효한 문자열이 된다
    let mut s1 = String::new();
    let data = "문자열 초깃값";
    let s2 = data.to_string();
    let s2 = "문자열 초깃값".to_string();
    let mut s3 = String::from("foo");
    let s4 = "bar";
    s3.push_str(s4);
    // push_str이 s4의 소유권을 가지게 되므로 s4는 출력되지 않아야 한다
    // 하지만 문자열 슬라이스를 이용하기 때문에 소유권을 가지지 않는다
    println!("s3: {}", s3);
    println!("s4: {}", s4);
    // 메서드를 사용하지 않고 이어붙인다면 s5는 메모리가 해제된다
    let s5 = String::from("Hello, ");
    let s6 = String::from("world!");
    // 이는 +가 add(self, s: &str) 메서드를 사용하기 때문
    // self에는 &가 없으므로 소유권을 가져오고, 따라서 self는 이 메서드가 끝나면 메모리가 해제된다
    let s7 = s5 + &s6;
    println!("s7: {}", s7);

    let len_en = String::from("Hello").len();
    let korean = String::from("안녕하세요");
    let len_kr = korean.len();
    println!("Hello의 길이: {}, 안녕하세요의 길이: {}", len_en, len_kr);
    // Hello의 길이: 5, 안녕하세요의 길이: 15
    // 러스트 관점에서 문자열은 크게 바이트, 스칼라값, 그래핌 클러스터의 세 가지로 구분한다
    // 러스트에서 String의 인덱스 사용을 지원하지 않는 이유는 인덱스 처리에는 항상 O(1)이 소요되어야 하지만
    // String 타입은 러스트가 유효한 문자를 파악하기 위해 콘텐츠를 처음부터 스캔해야 하기 때문에 일정한 성능을 보장할 수 없기 때문

    // 문자열 슬라이스하기
    let sliced = &korean[0..3]; // 이 때 sliced에는 "안"이 저장되지만 [0..1] 처럼 글자가 될 수 없는 범위라면 오류를 일으킬 수 있다
    println!("{}녕하세요", sliced);

    // 문자열을 순회하는 법
    for c in "안녕하세요".chars() {
        print!("{} ", c);
    }
    println!();
}

use std::collections::HashMap;
fn hashmap_example() {
    // 벡터와 마찬가지로 해시맵은 데이터를 힙 메모리에 저장한다
    let mut scores = HashMap::new();
    scores.insert(String::from("블루"), 10);
    scores.insert(String::from("옐로"), 50);

    // collect 메서드와 zip 메서드를 사용하여 해시맵을 생성하기
    let teams = vec![String::from("블루"), String::from("옐로")];
    let initial_scores = vec![10, 50];
    let hash_scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

    // 해시맵의 소유권
    let field_name = String::from("Favorite Color");
    let field_value = String::from("블루");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // println!("name: {}, value: {}", field_name, field_value);
    // borrow of moved value: `field_name`, `field_value`

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
    println!("{:?}", map);
    // {"wonderful": 1, "hello": 1, "world": 2}
}

fn main() {
    vector_example();
    different_types_vector();
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

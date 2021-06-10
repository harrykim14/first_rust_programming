mod lib;

use lib::*;
use std::env; // 명령줄 인수를 읽기 위해 러스트의 표준 라이브러리 std::env::args 함수를 사용
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect(); // 명령줄을 읽고 벡터로 변환함
    println!("{:?}", args);

    // unwrap_or_else()는 Result<T, E> 타입에 정의된 메서드로 익명 함수가 파이프 문자(|) 사이에 선언한 err 인수로 전달됨
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("인수를 구문분석하는 동안 오류가 발생했습니다: {}", err);
        process::exit(1);
    });

    println!("검색어: {}", config.query);
    println!("대상 파일: {}", config.filename);

    if let Err(e) = run(config) {
        println!("애플리케이션 에러: {}", e);
        process::exit(1);
    }
}

/* 인수 구문 정리하기 -> 이 함수를 Config 구조체의 연관 함수로 이사시킴
fn parse_config(args: &[String]) -> Config {
}
*/

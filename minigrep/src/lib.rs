use std::env;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

mod test {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fase, productive.
Pick three.";

        assert_eq!(vec!["safe, fase, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        )
    }
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

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

// 설정 변수를 하나의 구조체에 모아 목적을 명확히 할 것
pub struct Config {
    pub query: String,
    pub filename: String,
    // Config 구조체에 대소문자 구분을 위한 새로운 설정 옵션을 추가하기
    pub case_sensitive: bool,
}

impl Config {
    // parse_config 함수를 Config 구조체의 연관 함수로 리팩토링
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("필요한 인수가 지정되지 않았습니다.");
        }

        let query = args[1].clone();
        let filename = args[2].clone();
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}

// 설정이나 에러 처리에 관련된 부분이 아닌 나머지 코드를 분리
// Box<dyn Error> 함수는 Error 트레이트를 구현하는 타입을 리턴함
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // expect() 함수를 ? 연산자로 대체하기
    let mut file = File::open(config.filename)?;
    let mut contents = String::new();
    // ? 연산자에 걸린 에러는 Box<dyn Error>로 처리되고 Err(e)의 인수로 전달됨
    file.read_to_string(&mut contents)?;

    let _result = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in search(&config.query, &contents) {
        println!("{}", line)
    }

    // println!("파일 내용:\n{}", contents);

    Ok(())
}

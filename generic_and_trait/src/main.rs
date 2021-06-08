fn main() {
    let numbers = vec![34, 50, 25, 100, 65];
    let chars = vec!['y', 'm', 'a', 'q'];

    let num_result = largest(&numbers);
    let char_result = largest(&chars);

    println!(
        "가장 큰 숫자: {}, 가장 큰 글자: {}",
        num_result, char_result
    );

    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("Start to learn Rust"),
        reply: false,
        retweet: false,
    };

    println!("새 트윗 1개: {}", tweet.summarize());

    let string1 = String::from("아주 아주 긴 문자열");
    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("더 긴 문자열: {}", result);
    }
    // println!("더 긴 문자열: {}", result);

    let novel = String::from("스타워즈. 오래 전 멀고 먼 은하계에...");
    let first_sentence = novel
        .split('.') // .으로 문자열을 나눠
        .next()
        .expect("문장에서 마침표'.'를 찾을 수 없습니다.");
    let i = ImportantExcerpt {
        part: first_sentence,
    };

    println!("첫번째 문장: {}", i.part);
}

struct ImportantExcerpt<'a> {
    // 구조체에 수명을 지정하면
    part: &'a str, // 필드값인 part에도 수명을 지정할 수 있다
}

struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}
// error[E0369]: binary operation `>` cannot be applied to type `T`

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

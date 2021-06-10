use std::thread;
use std::time::Duration;

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_user_specified_value, simulated_random_number);

    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();

    for val in v1_iter {
        println!("값: {}", val);
    }
}
/*
fn simulated_expensive_calculate(intensity: u32) -> u32 {
    println!("시간이 오래 걸리는 계산을 수행 중...");
    thread::sleep(Duration::from_secs(2));
    intensity
}
*/

struct Cacher<T>
where
    T: Fn(u32) -> u32, // calculation 필드의 타입이 클로저임을 명시
{
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T> {
        // new 함수는 당연히 구조체와 같을 것이고 Cacher 인스턴스를 리턴한다
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        // 클로저의 실행 결과는 클로저를 직접 호출하지 않고 이 value 메서드를 호출하면 된다
        // 미리 self.value 필드에 실행 결과를 저장하여 Some 열것값에 저장해두고 이 값을 리턴함
        // 메모이제이션 같은 효과를 냄
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

/*
1. generate_workout() 원본 함수
fn generate_workout(intensity: u32, random_number: u32) {
    if intensity < 25 {
        println!(
            "오늘은 {}번의 팔굽혀펴기를 하세요!",
            simulated_expensive_calculate(intensity)
        );
        println!(
            "그 다음엔 {}번의 윗몸 일으키기를 하세요!",
            simulated_expensive_calculate(intensity)
        )
    } else {
        if random_number == 3 {
            println!("오늘은 수분을 충분히 섭취하며 쉬세요...");
        } else {
            println!(
                "오늘은 {}분간 달리기를 하세요!",
                simulated_expensive_calculate(intensity)
            );
        }
    }
}
*/

/*
2. generate_workout() 1차 리팩토링: simulated_expensive_calculate()를 한 번만 호출하도록 변경
fn generate_workout(intensity: u32, random_number: u32) {
    let expensive_result = simulated_expensive_calculate(intensity);

    if intensity < 25 {
        println!("오늘은 {}번의 팔굽혀펴기를 하세요!", expensive_result);
        println!("그 다음엔 {}번의 윗몸 일으키기를 하세요!", expensive_result)
    } else {
        if random_number == 3 {
            println!("오늘은 수분을 충분히 섭취하며 쉬세요...");
        } else {
            println!("오늘은 {}분간 달리기를 하세요!", expensive_result);
        }
    }
}
*/

//3. 클로저를 사용하여 해당 함수가 필요할 때만 부르기
fn generate_workout(intensity: u32, random_number: u32) {
    // 클로저의 매개변수와 리턴값에 타입 애노테이션을 추가할 수 있다
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

use crate::List::{Cons, Nil};
use std::ops::Deref;
use std::rc::Rc;

fn main() {
    // 무한한 Cons 열것값으로 구성된 무한한 List 열거자
    /*
    let normal_list = Cons(1,
        Cons(2,
            Cons(3, Nil)
        ));
    let _box_list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    let x = 5;
    let y = Box::new(x);

    println!("y: {}", *y);
    */

    let a = 5;
    let b = MyBox::new(a);

    println!("b: {}", *(b.deref()));

    let c = CustomSmartPointer {
        data: String::from("첫번째 데이터"),
    };
    let _d = CustomSmartPointer {
        data: String::from("두번째 데이터"),
    };
    println!("CustomSmartPointer를 생성했습니다");
    drop(c);
    println!("변수 c를 해제하였습니다.");

    let e = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("e를 생성한 이후의 카운터 = {}", Rc::strong_count(&e));
    let f = Cons(3, Rc::clone(&e));
    println!("f를 생성한 이후의 카운터 = {}", Rc::strong_count(&e));
    let g = Cons(4, Rc::clone(&e));
    println!("g를 생성한 이후의 카운터 = {}", Rc::strong_count(&e));
    drop(g);
    println!("g를 드랍한 이후의 카운터 = {}", Rc::strong_count(&e));
}

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("CustomSmartPointer의 데이터 '{}'를 해제합니다", self.data);
    }
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}
/*
enum List {
    Cons(i32, Box<List>),
    Nil,
}
*/

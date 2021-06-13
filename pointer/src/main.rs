use crate::List::{Cons, Nil};
use std::ops::Deref;

fn main() {
    // 무한한 Cons 열것값으로 구성된 무한한 List 열거자
    /*
    let normal_list = Cons(1,
        Cons(2,
            Cons(3, Nil)
        ));
    */

    let _box_list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    let x = 5;
    let y = Box::new(x);

    println!("y: {}", *y);

    let a = 5;
    let b = MyBox::new(a);

    println!("b: {}", *(b.deref()));
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

enum List {
    Cons(i32, Box<List>),
    Nil,
}

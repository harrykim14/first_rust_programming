// use crate::List::{Cons, Nil};
use std::cell::RefCell;
// use std::ops::Deref;
use std::rc::{Rc, Weak};

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
    let _f = Cons(3, Rc::clone(&e));
    println!("f를 생성한 이후의 카운터 = {}", Rc::strong_count(&e));
    let g = Cons(4, Rc::clone(&e));
    println!("g를 생성한 이후의 카운터 = {}", Rc::strong_count(&e));
    drop(g);
    println!("g를 드랍한 이후의 카운터 = {}", Rc::strong_count(&e));

    let value = Rc::new(RefCell::new(5));
    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));
    let b = Cons(Rc::new(RefCell::new(6)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(10)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    println!("a 수정 후 = {:?}", a);
    println!("b 수정 후 = {:?}", b);
    println!("c 수정 후 = {:?}", c);

    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));
    println!("a의 최초 rc 카운트 = {}", Rc::strong_count(&a));
    println!("a의 다음 아이템 = {:?}", a.tail());

    let b = Rc::new(Cons(5, RefCell::new(Rc::clone(&a))));
    println!("b를 생성한 후 a의 rc 카운트 = {}", Rc::strong_count(&a));
    println!("b의 최초 rc 카운트 = {}", Rc::strong_count(&b));
    println!("b의 다음 아이템 = {:?}", b.tail());

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("a를 변경한 후 b의 rc 카운트 = {}", Rc::strong_count(&b));
    println!("a를 변경한 후 a의 rc 카운트 = {}", Rc::strong_count(&a));
    */
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf)
    );

    // println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });

        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        println!(
            "branch strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf)
        );

        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf)
        );
    }

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf)
    );
}

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

/*
#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match *self {
            Cons(_, ref item) => Some(item),
            Nil => None,
        }
    }
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

enum List {
    Cons(i32, Box<List>),
    Nil,
}
*/

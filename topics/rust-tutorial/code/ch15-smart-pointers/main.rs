use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

fn main() {
    let boxed = Box::new(5);
    println!("boxed = {boxed}");

    let value = Rc::new(RefCell::new(5));
    let list = Rc::new(List::Cons(Rc::clone(&value), Rc::new(List::Nil)));

    let _list2 = List::Cons(Rc::new(RefCell::new(3)), Rc::clone(&list));

    *value.borrow_mut() += 10;

    if let List::Cons(v, _) = &*list {
        println!("list head = {}", v.borrow());
    }
}

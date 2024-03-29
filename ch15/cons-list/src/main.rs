enum List {
    Cons(i32, Rc<List>),
    Nil,
}

impl Drop for List {
    fn drop(&mut self) {
        if let Cons(v, _) = self {
            println!("Dropping List with data `{}`!", v);
        } else {
            println!("Dropping List with data Nil!");
        }
    }
}

use std::rc::Rc;

use crate::List::{Cons, Nil};

fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let _b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let _c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}

#![allow(unused)]
// #![allow(dead_code)]
use std::cell::RefCell;
use std::rc::Rc;

struct MyStruct { s: u32 }

fn main() {
    // let mut x = Rc::new(RefCell::new(MyStruct{ s: 5u32 }));
    // let y = x.clone();
    // x.borrow_mut().s = 6;
    // println!("{}", x.borrow().s);

    let data = RefCell::new(5);
    // Immutable borrow
    {
        let borrow1 = data.borrow();
        println!("borrow1: {}", *borrow1);

        // Another immutable borrow
        let borrow2 = data.borrow();
        println!("borrow2: {}", *borrow2);
    } // borrow1 and borrow2 go out of scope

    // Mutable borrow
    {
        let mut borrow_mut = data.borrow_mut();
        *borrow_mut += 1;
        println!("borrow_mut: {}", borrow_mut);
    } // borrow_mut goes out of scope

    {
        let mut borrow_mut = data.borrow_mut();
        *borrow_mut += 1;
        println!("borrow_mut: {}", borrow_mut);
    }

    println!("data: {}", data.borrow());

    // Example of a panic (commented out to avoid runtime error)
    // let mut borrow_mut = data.borrow_mut();
    // let _borrow2 = data.borrow(); // This will cause a panic
}
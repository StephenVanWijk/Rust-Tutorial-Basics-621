#![allow(unused)]
#![allow(dead_code)]
// 20250706 1121CET SDvW
// Experiments with Rust borrowing and ownership

#[derive(Copy, Clone)]
struct Point { x: i32, y: i32 }

fn immutable_mutalble_ref_one_value() {
    let mut p1 = Point{ x: -1, y: 2 };
    let p2 = p1;
    p1.x = 1;
    println!("p1: {}, {}", p1.x, p1.y);
    println!("p2: {}, {}", p2.x, p2.y);
}
#![no_std]
#![no_main]

use core::arch::asm;

#[macro_use]
extern crate user_lib;

#[no_mangle]
fn main() -> i32 {
    println!("yfshuaibi22301107");
    
    // 计算并输出
    let a = 5;
    let b = 3;
    println!("{} + {} = {}", a, b, a + b);
    println!("{} - {} = {}", a, b, a - b);
    println!("{} * {} = {}", a, b, a * b);
    
    // 循环输出
    for i in 0..5 {
        println!("count_yf: {}", i);
    }
    
    unsafe {
        asm!("sret");
    }
    0
}
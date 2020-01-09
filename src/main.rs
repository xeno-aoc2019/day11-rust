extern crate num_bigint;
extern crate num_traits;

mod panel;
mod input;
mod virtualmachine;

use std::fmt;
use std::path::Path;
use std::fmt::Formatter;
use num_bigint::BigInt;
use num_traits::{Zero, One, ToPrimitive};
use std::ops::Add;
use std::collections::HashMap;

use panel::Panel;
use panel::Color;


fn _0() -> BigInt { Zero::zero() }

fn _1() -> BigInt { One::one() }

fn _2() -> BigInt { BigInt::from(2) }

fn _3() -> BigInt { BigInt::from(3) }

struct Instruction {
    opcode: i32,
    steps_next: usize,
}


fn main() {
    let program = input::read_program("input.txt");
    let mut vm = virtualmachine::VM::new(program.clone(), vec!());
    let mut panel = Panel::new();
    vm.run();
    while !vm.is_halted() {
        if panel.is_on_white() {
            vm.add_input(_1());
        } else {
            vm.add_input(_0());
        }
//        println!("{}", vm);
        vm.resume();
        let out_1 = vm.read_output();
        let out_2 = vm.read_output();
        if out_1 == _0() {
            panel.paint(Color::BLACK);
        } else if out_1 == _1() {
            panel.paint(Color::WHITE);
        } else {
            panic!("out_1 = {}", out_1);
        }
        if out_2 == _0() {
            panel.turn_left();
        } else if out_2 == _1() {
            panel.turn_right();
        } else {
            panic!("out_2 = {}", out_2);
        }
        panel.step();
    }
    println!("WHITE COUNT {}", panel.white_count());
    println!("PAINT COUNT {}", panel.paint_count());
}


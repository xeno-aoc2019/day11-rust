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
    vm.run();
    vm.add_input(_1());
    println!("{}", vm);
    vm.resume();
    vm.add_input(BigInt::from(4));
    vm.resume();
    println!("Output: {}", vm.read_output());
    let mut panel = Panel::new();
    panel.paint(Color::WHITE);
    panel.turn_left();
    panel.step();
    panel.turn_right();
    panel.paint(Color::WHITE);
    panel.paint(Color::BLACK);
    panel.step();
    panel.paint(Color::BLACK);
    println!("White: count: {}", panel.white_count());
    println!("Painted: count: {}", panel.paint_count());
    println!("Position: ({},{})", panel.pos().0, panel.pos().1);
}


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
    task_2();
}

fn task_1() {
    let program = input::read_program("input.txt");
    let mut vm = virtualmachine::VM::new(program.clone(), vec!());
    let mut panel = Panel::new();
    run_vm(&mut vm, &mut panel);
    println!("Part 1: WHITE COUNT {}", panel.white_count());
    println!("Part 1: PAINT COUNT {}", panel.paint_count());
}

fn task_2() {
    let program = input::read_program("input.txt");
    let mut vm = virtualmachine::VM::new(program.clone(), vec!());
    let mut panel = Panel::new();
    panel.paint(Color::WHITE);
    run_vm(&mut vm, &mut panel);
    display_panel(&panel)
    // println!("Part 1: WHITE COUNT {}", panel.white_count());
    // println!("Part 1: PAINT COUNT {}", panel.paint_count());
}


fn display_panel(pan: &panel::Panel) {
//    for w in pan.get_whites().iter() {
//        println!("({},{})", w.0, w.1);
//    }
    println!("Part 2: WHITE COUNT {}", pan.white_count());
    println!("Part 2: PAINT COUNT {}", pan.paint_count());

//    let mut whites: Vec<(i64,i64)> = pan.get_whites().iter()
//        .map(|x| x.clone())
//        .map(|x| (-x.1, x.0)).collect();
//    whites.sort();

    for y in -4..8 {
        for x in 0..40 {
            if pan.is_white(x, -y) { print!("#"); } else { print!(" "); };
        }
        println!();
    }
}


fn run_vm(vm: &mut virtualmachine::VM, panel: &mut Panel) {
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
}
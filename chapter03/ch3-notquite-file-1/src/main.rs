#![allow(unused_variables)]

use std::fmt::Debug;

type File = String;

fn open(f: &mut File) -> bool {
    true
}

fn close(f: &mut File) -> bool {
    true
}

#[allow(dead_code)]
fn read(f: &mut File, save_to: &mut Vec<u8>) -> ! {
    unimplemented!()
}

#[allow(dead_code)]
fn report<T: Debug>(item: T) {
    println!("{:?}", item);
}

#[allow(dead_code)]
fn clear(text: &mut String) -> () {
    *text = String::from("");
}

#[allow(dead_code)]
fn dead_end() -> ! {
    panic!("you have reached a dead end");
}

#[allow(dead_code)]
fn forever() -> ! {
    loop {}
}

fn main() {
    let mut f1 = File::from("f1.txt");
    open(&mut f1);
    // read(f1, vec![]);
    close(&mut f1);
}

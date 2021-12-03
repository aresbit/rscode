

// a half hour to learn rust
// affine type : consumed at most once
// Linear type : consumed exactly once

// rust types are affine 
// 仿射类型系统： 只能消费一次，不能多次消费
// 这是  move 语义的一个花哨名称，它确保了如果一个类型没有标记为 copyable，那么它只能被移动，而不能被复制。
let x;
x = 5;

let y = 6;

let x : i32 = 5;

let x : f64;
x = 5.0;

foobar(x);

let _ = 43;
let _ = fetthing();

let _x = 5; // unused, but compiler doesn't know

let x = 5;
let x = x + 1;

// tuples, char * int
let pair = ('a', 5);

pair.0;
pair.1;
let pair: (char, i32) = ('a', 5);

let (x, y) = (1, 2);

let (_, r) = slice.split_at(3);

let x = 5;
let y = x;
let z = y + x;

let x = vec![1, 2, 3]
      .iter()
      .map(|x| x + 1)
      .fold(0, |a, b| a + b);

fn greet(){
    println!("Hello, world!");
}


fn fair_you(x: i32) -> i32 {
    x + 1
}

fn main() {
    let x = 5;
    let y = {
        let x = 3;
        x + 1

    let x = 5;
    let y ={
      let x = 3;
      x + 1
    }
    };
    let z = {
        let x = 3;
        x + 1
    };
  println!("{}", x);


}

if some {4} else {5}

  fn fair_dice() -> i32{

    match rand::random::<i32>() % 4 {
        0 => 1,
        1 => 2,
        2 => 3,
  }

}

let least = std::cmp::min(1, 2);

use std::cmp::min;

let least = min(1, 2);

use std::cmp::{min, max};

use std::{cmp::min, cmp::max};

use std::cmp::*;

// type are namespaces too

let x = "adfg".len();

let x = str::len("adfg");

let v = Vec::new();

let v = std::vec::Vec::new();

use std::prelude::v1::*;

struct Point {
    x: i32,
    y: i32,
}


let p = Point { x: 0, y: 7 };

let p2 = Point{
  x : 14.0,
  ..p
};

let p3 = Point {..p};

let Point {a, b} = p;

let Point {x, ..} = p;


struct Number {
  odd : bool,
  value: i32,
}

fn main() {
  let one = Number {
    odd: true,
    value: 5,
  };

  let two = Number {
    odd: false,
    value: 6,
  };

  print_number(one);
  print_number(two);
}

fn print_number(n: Number) {

  if let Number {odd: true, value: v} = n {
    println!("odd {}", v);
  } else if let NUmber {odd: false, value} = n {
    println!("even {}", value);
  }
}

// match  just like if let
fn print_number(n: Number) {

  match n {
    Number {odd: true, value: v} => println!("odd {}", v),
    Number {odd: false, value} => println!("even {}", value),
  }
}


fn print_number(n: Number) {

  match n {
    Number {odd: true, value: v} => println!("odd {}", v),
    Number {odd: false, value} => println!("even {}", value),
  }
}


fn print_number(n: Number) {

 match n.value{
    1 => println!("one"),
    2 => println!("two"),
    _ => println!("something else"),
 }
}



impl Number {
  fn is_odd(self) -> bool {
    self.odd

  fn is_strictly_positive(self) -> bool {
    self.value > 0
  }

}


fn main() {
  let one = Number {
    odd: true,
    value: 5,
  };

  let two = Number {
    odd: false,
    value: 6,
  };

println!("{}", one.is_strictly_positive());


let mut n = Number {
  odd: true,
  value: 5,
};



}


trait Signed {
  fn is_positive(self) -> bool;
}

impl Signed for Number {
  fn is_positive(self) -> bool {
    self.value > 0
  }
}

impl Signed for i32 {
  fn is_positive(self) -> bool {
    self < 0
  }
}


// trait -> impl trait for type.


//  immutable reference

fn print_number(n: &Number) {
  println!("{}", n.value);
}

fn main() {
  let one = Number {
    odd: true,
    value: 5
  };

  print_number(&one);
}

fn invert (n: &mut Number) {
  n.odd = !n.odd;
}

// a mutable reference -> borrow
fn main() {
  let mut one = Number {
    odd: true,
    value: 5
  };

  invert(&mut one);
  println!("{}", one.odd);
}




impl std::clone::Clone for NUmber{
  fn clone(&self) -> Self{
    Self { ..*self}
  }
}

fn main(){

  let n = Number {
    odd: true,
    value: 5
  };

  let mut m = n.clone();

  print_number(&n);

}


#[derive(Copy, Clone)]
struct Point {
  x: i32,
  y: i32,
}

// generic

fn foobar<T>(x: T) {
  println!("{}", x);
}

// <T> means type variable, just like 'a in ocaml

fn foobar<L, R>(x: L, y: R) {
  println!("{} {}", x, y);
}

// The simplest constraints are just trait names

fn print<T: Display>(x: T) {
  println!("{}", x);
}


fn foobar<T>(x: T)
where
  T: Display,
{
  println!("{}", x);
}

fn main(){

  use std::any::typename;

  println!("{}", typename::<i32, char>());


}



struct Pair<T>{
  a: T,
  b: T,
}


// mut -> &
// &mut -> borrow

// macro
let v1 = vec![1, 2, 3];

fn main(){

  use std::io::{self, Write};
  io::stdout().lock().write_all(b"hello\n").unwrap();

  panic!("this is a panic");
}

// option

enum Option<T> {
  Some(T),
  None,
}


// Implement some functionality for a type.

impl<T> Option<T> {
  fn unwrap(self) -> T {
    match self {
      Option::Some(x) => x,
      Option::None => panic!("called `Option::unwrap()` on a `None` value"),
    }
  }
}


use self::Option::{Some, None};

// "lifetime" just is a type for time domain in Rust
// Lifetimes are generic parameters for time domain types. type is space domain types

fn main() {

  {
    let x = 5;
    println!("{}", x);
  }

}

// references have a lifetime
fn main() {

  let x = 5;
  let x_ref = &x; // borrow x, means T:x_ref < T:x
  println!("{}", x_ref);

}
// 引用的生存期不能超过其借用的变量绑定的生存期

fn main() {

  let x_ref = {
    let x = 5;
    &x
  };

  println!("{}", x_ref);// error
}

// 一个变量绑定可以被多次借用

fn main() {
  let x = 42;
  let x_ref1 = &x;
  let x_ref2 = &x;
  let x_ref3 = &x;
  println!("{} {} {}", x_ref1, x_ref2, x_ref3);
}

//While borrowed, a variable binding cannot be mutated

fn main() {
  let mut x = 5;
  let x_ref = &x;
   x = 6;
  println!("{}", x_ref); // error
}

// cannot borrow `x` as mutable because it is also borrowed as immutable

// References in function arguments also have lifetimes:
fn print(x: &i32) {
  println!("{}", x);
}

//这允许返回的引用的生存期依赖于参数的生存期
fn print<'a>(x: &'a i32){

}

struct Number {
  value: i32
}

//这允许返回引用的生存期依赖于参数的生存期:
fn nummber_value<'a>(num: &'a Number) -> &'a i32 {
  &num.value
}

fn main() {
  let num = Number { value: 3 };
  let num_value = nummber_value(&num);
  println!("{}", num_value);
}

//两点注意:
//1）返回值v的生存期依赖于参数n的生存期。
//2）v 活着，所有权在v,n就不能被变卖，转移。

struct NumRef<'a> {
  value: &'a i32,
}

fn main() {
  let num = 5;
  let num_ref = NumRef { value: &num };
  println!("{}", num_ref.value);
}


impl<'a> NumRef<'a> {
  fn asref(value: &'a self) -> &'a i32 {
      self.x
  }
}

fn main() {
  let num: i32= 5;
  let num_ref = NumRef { value: &num };
  let num_i32_ref = num_ref.asref();
  println!("{}", num_ref.asref());
}

//
impl<'a> NumRef<'a> {
  fn asref(&self) -> &i32 {
      self.value
  }
}






impl NumRef<'_>{

  fn asref(&self) -> &i32 {
      self.value
  }
}

struct Person{
  name: &'static str,
}

// string字面量是静态的

fn main() {
  let person = Person { name: "name" };

}



//但字符串不是静态的
struct Person{
  name: &'static str,
}
fn main() {



  let name = format!("faster{}", "name");
  let p = Person { name: &name };
}










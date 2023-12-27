# Learnings

## Basic data types
- Boolean - true, false
- Integer - 2, -9
- Float / Double - 0.3423, 1.1
- Character - 'A', '8',
- String - "Hello", "!"

## Variable
- Used to assign data to a temporary memory location, helps us easily work with memory
- Defined using `let` keyword
- Are by default immutable, but we can make them mutable with `mut` keyword
```rust
let two = 2;
let mut my_name = "Nikit";
let my_name = gaming_name;
```

## Functions
- Declared with `fn` keyword
- Last statement is returned by default
```rust
fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

## Macros
- Macros expand into additional code. `println!` is a macro which execute some code to print the out on terminal.
- Difference between function and macro is `!` at the end of it's name when calling it.

### println macro
```rust
let life = true;
println!("{}, life"); // This can only display the trait which implements Display trait
println!("{life}");
println!("{:?}, life"); // This can display any value that implements Debug trait
println!("{life:?}");
```

## Control flow

### if else
- Condition doesn't require to be wrapped in brackets
```rust
let a = 100
if a > 100 {
    println!("Let's go");
} else if a < 100 {
    println!("Let's not go");
} else {
    println!("Huh!?");
}
```

### infinite loop
```rust
let mut a = 0;
loop {
    if a == 5 {
        break;
    }
    println!({:?}, a);
    a = a + 1;
}
```

### while loop
```rust
let mut a = 0;
while a != 5 {
    println!({:?}, a);
    a = a + 1;
}
```

## Running program
```bash
cargo run
```

- If you add files to a `bin` directory in the `src` directory, then you can run those files as
```bash
cargo run --bin filename
```

## Match Expression
- Similar to switch, but should be exaustive (All options should be accounted for)
```rust
let some_bool = false;
match some_bool {
    true => println!("its true"),
    false => println!("its false"),
}
let some_int = 3;
match some_int {
    1 => println!("its 1"),
    2 => println!("its 2"),
    3 => println!("its 3"),
    _ => println!("its something else!!"),
}
```

## Enums
- Enums are used to describe that a value is one of a possible set of values.
```rust
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn which_way(go: Direction) {
    match go {  
        Direction::Up => "up",
        Direction::Down => "down",
        Direction::Left => "left",
        Direction::Right => "right",
    }
}
```

## Struct
- A type that contains multiple pieces of data
- Cannot have partial values, either all data should be there or none
- Each piece of data is called field
```rust
struct ShippingBox {
    depth: i32,
    height: i32,
    width: i32,
}

let my_box = ShippingBox {
    depth: 3,
    width: 2,
    height: 5,
}

println!("{:?}", my_box.height);
```

## Tuple
- A type of record
- useful to return data from functions and can be easily destructured
```rust
fn one_two_three() -> (i32, i32, i32) {
    (1, 2, 3)
}
let num = one_two_three();
let (one, two, three) = one_two_three();
println!("{:?} {:?}", num.0, one);
```

## Expressions
- Rust is a expression based language
- Most things are evaluated and return something
```rust
let my_num = 3;
let is_lt_4 = if my_num < 5 {
    true
} else {
    false
};
// or
let is_lt_5 = my_num < 5;

let val = 3;
let greeting = matxh val {
    1 => "hello",
    _ => "goodbye",
}
```

## Ownership
- Rust uses "Ownership" model to manage memory instead of other methods like "Garbage collection" or manually managing memory
- It means the owner is responsible for cleaning the memory
- Memory can either be "moved" or "borrowed"
```rust
// Ownership
fn main() {
    let x = 5;
    // here the main fn transfers the ownership to some_fn and later when some_fn finishes
    // executing the x var is removed from memory
    some_fn(x);
    some_fn2(x); // This will give error as only one function can own the variable at once
}
// Borrowing
fn main() {
    let x = 5;
    // now the x variable will be deleted from memory only when the main fn execution completes
    some_fn(&x);
    some_fn2(&x); // This will work as the value of x now is borrowed instead of being transferred to some_fn
}
```

## impl
- This keyword is used to define implementations on types.
- This is where you define methods associated with a type. 

```rust
// 1. Implement methods on structs or enums:
// In this example, `impl` is used to define a method `area` on the `Circle` struct.
struct Circle {
    radius: f64,
}

impl Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * (self.radius * self.radius)
    }
}


// 2. Implement traits for types:
// In this example, `impl` is used to implement the `HasArea` trait for the `Circle` type.
trait HasArea {
    fn area(&self) -> f64;
}

impl HasArea for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * (self.radius * self.radius)
    }
}
```

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

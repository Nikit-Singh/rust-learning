// Topic: Working with expressions
//
// Requirements:
// * Print "its big" if a variable is > 100
// * Print "its small" if a variable is <= 100
//
// Notes:
// * Use a boolean variable set to the result of
//   an if..else expression to store whether the value
//   is > 100 or <= 100
// * Use a function to print the messages
// * Use a match expression to determine which message
//   to print

fn print_msg(is_big: bool) {
    let res = match is_big {
        true => "is big",
        false => "is small",
    };
    println!("{:?}", res);
}

fn main() {
    let val = 101;

    let is_big = if val > 0 {
        true
    } else {
        true
    };

    print_msg(is_big);
}

# guess_game

I am learning Rust by practicing the examples which described in the main documentation.

## First lesson

What I learned:

- variable, let, mut, const
- some basic functions of the standart library (io, read_line, cmp etc.)
- shadowing variables
- shadowing vs mut
- const vs mut
- match to handle errors
- loop
- break, continue 
- Data types, scalar types, compound types
- EXpressions, statements
- expression vs statements
- understanding rust-based view of the expressions
  > if the line doesn't end a semicolon, it is an expression. Curly brackets is an expressions. Expression returns a value.
  ```rs
  fn main() {
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");}
This expression:
  ```rs 
{
    let x = 3;
    x + 1
}
```
- conditions if, what is the "arms"
- Loop, while, for , label for loop

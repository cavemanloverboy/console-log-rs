# `console-log-rs`

For when `console.log` strikes again...

## Example

```rust
use console_log_rs::console_log;

#[console_log]
mod test_module {
    pub fn test_function() {
        console.log("This is a test");
    }

    pub fn test_function_formatted() {
        console.log("This is a test {}", 5);
    }
}

#[console_log(msg!)]
mod test_msg {
    macro_rules! msg {
        ($msg:expr) => {
            println!("using msg macro");
            println!($msg)
        };
        ($($arg:tt)*) => {
            println!("using msg macro");
            println!($($arg)*);
        }
    }

    pub fn test_function() {
        console.log("This is a test");
    }

    pub fn test_function_formatted() {
        console.log("This is a test {}", 5);
    }
}

fn main() {
    test_module::test_function();
    test_module::test_function_formatted();
    println!();
    test_msg::test_function();
    test_msg::test_function_formatted();
}
```

Output:

```txt
This is a test
This is a test 5

using msg macro
This is a test
using msg macro
This is a test 5
```

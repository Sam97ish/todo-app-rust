# TODO app in Rust
A simple todo app that writes todos to a file and reads them to cli.
Done as refresher for Rust programming.

## Usage
todo-app [OPTIONS] --action <ACTION>

## Help
todo-app -h or --help

### Options
  * -a, --action <ACTION>    Action to be taken (add or list), add is used to modify by given the same task again
  * -t, --todo <TODO>        The job to be added to the TODO list [default: ]
  * -i, --is-done <IS_DONE>  Is the job done or not (1 for true or  0 for false) default is false [default: 0]
  * -h, --help               Print help information
  * -V, --version            Print version information

### Example
```rust
todo-app -a add -t work OR cargo run -- -a add -t work
todo-app -a add -t sleep -i 1 OR cargo run -- -a add -t sleep -i 1
todo-app -a list OR cargo run -- -a list
```
**Expected output**
```rust
Todo     Done
work     [ ]
sleep    [X]
```
```rust
todo-app -a add -t work -i 1 OR cargo run -- -a add -t work -i 1
todo-app -a list OR cargo run -- -a list
```
**Expected output**
```rust
Todo     Done
sleep    [X]
work     [X]
```
fn main() {
    let x = 123;
    let some_string = String::from("hello world worktree");
    let mut splits = some_string.split(' ');

    splits.next();

    println!("Hello, from worktree!");
}

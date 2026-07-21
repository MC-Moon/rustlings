fn main() {
    let mut res = 42;
    let option = Some(12);
    // TODO: Fix the Clippy lint.

    match option{
        Some(x)=> {
            res += x;
        }
        None => panic!(),
    };
    println!("{res}");
}

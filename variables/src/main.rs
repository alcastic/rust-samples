fn main() {
    const SCOPED_CONSTANT: u32 = 200_000;
    println!("SCOPED_CONSTANT={}", SCOPED_CONSTANT);

    let immutable_var: u32 = 100_000_000;
    println!("immutable_var={}", immutable_var);

    let mut mutable_var: u32 = 100_000_000;
    mutable_var = mutable_var + 1;
    println!("mutable_var={}", mutable_var);

    let x:i8 = 10; // declares x variable
    println!("initial x={}", x);
    let x:i8 = 11; // shadowing variable x
    println!("previous x value shadowed by x={}", x);
    let x:i32 = 12; // shadowing variable x and changing its type
    println!("previous x value and type shadowed by x={}", x);

    let mut x:u32 = 100;
    x = x + 1;
    println!("previous x value and type shadowed by mutable x={}", x);
}

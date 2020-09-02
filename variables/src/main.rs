fn main() {
    const SCOPED_CONSTANT: u32 = 200_000;
    println!("SCOPED_CONSTANT={}", SCOPED_CONSTANT);

    let immutable_var: u32 = 100_000_000;
    println!("immutable_var={}", immutable_var);

    let mut mutable_var: u32 = 100_000_000;
    mutable_var = mutable_var + 1;
    println!("mutable_var={}", mutable_var);
}

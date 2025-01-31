use rust_macro::my_vec;
fn main() {
    let v: Vec<i32> = my_vec![];
    let v2: Vec<i32> = my_vec![1, 2, 3,];
    println!("{:?}", v);
    println!("{:?}", v2);
}

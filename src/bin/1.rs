fn foo()-> Option<i32>{
    return Some(100);
}

fn main() {
    match foo() {
        Some(_)=>{
            println!("1");
        }
        _=>{
            panic!("None");
        }
    }
}
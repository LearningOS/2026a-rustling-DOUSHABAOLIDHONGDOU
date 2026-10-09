// lifetimes2.rs
//
// So if the compiler is just validating the references passed to the annotated
// parameters and the return type, what do we need to change?
//
// Execute `rustlings hint lifetimes2` or use the `hint` watch subcommand for a
// hint.



fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("long string is long"); //寿命main
    let string2 = String::from("xyz");
    let result;
    {
        //let string2 = String::from("xyz"); //寿命只到作用域结束
        result = longest(string1.as_str(), string2.as_str());
    }
    println!("The longest string is '{}'", result); //寿命整个main，但是result引用了string2，string2可能已经被释放，所以这里会报错
}

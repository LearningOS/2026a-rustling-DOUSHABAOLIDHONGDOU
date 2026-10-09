// modules1.rs
//
// Execute `rustlings hint modules1` or use the `hint` watch subcommand for a
// hint.



mod sausage_factory {
    // Don't let anybody outside of this module see this!
    fn get_secret_recipe() -> String {
        String::from("Ginger")
    }

    pub fn make_sausage() {
        get_secret_recipe();
        println!("sausage!");
    }
}

// rust里所有东西都是默认私有的，模块里的函数、结构体、枚举等都是默认私有的，只有在模块里使用pub关键字声明的函数、结构体、枚举等才是公有的，才能被模块外部访问。

fn main() {
    sausage_factory::make_sausage();
}

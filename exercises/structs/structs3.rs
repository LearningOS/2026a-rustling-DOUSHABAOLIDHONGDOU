// structs3.rs
//
// Structs contain data, but can also have logic. In this exercise we have
// defined the Package struct and we want to test some logic attached to it.
// Make the code compile and the tests pass!
//
// Execute `rustlings hint structs3` or use the `hint` watch subcommand for a
// hint.


#[derive(Debug)]
struct Package {
    sender_country: String,
    recipient_country: String,
    weight_in_grams: i32,
}

impl Package { //impl 是给结构体类型挂函数的语法，块里的函数叫方法或者是关联函数
    fn new(sender_country: String, recipient_country: String, weight_in_grams: i32) -> Package {
        if weight_in_grams <= 0 {
            panic!("Can not ship a weightless package.")
        } else {
            Package {
                sender_country,
                recipient_country,
                weight_in_grams,
            }
        }
    }
    // rust 中的函数签名中，返回值类型是通过 -> 来指定的，??? 是占位符，表示需要你来填写正确的返回值类型。
    // rust 函数的最后一个表达式就是返回值，不需要写return
    fn is_international(&self) -> bool { 
        self.sender_country != self.recipient_country
    }
    
    // 究竟需要怎么补请看后面的函数调用时怎么调用的
    fn get_fees(&self, cents_per_gram: i32) -> i32{
        self.weight_in_grams * cents_per_gram
        
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn fail_creating_weightless_package() {
        let sender_country = String::from("Spain");
        let recipient_country = String::from("Austria");

        Package::new(sender_country, recipient_country, -2210);
    }

    #[test]
    fn create_international_package() {
        let sender_country = String::from("Spain");
        let recipient_country = String::from("Russia");

        let package = Package::new(sender_country, recipient_country, 1200);

        assert!(package.is_international());
    }

    #[test]
    fn create_local_package() {
        let sender_country = String::from("Canada");
        let recipient_country = sender_country.clone();

        let package = Package::new(sender_country, recipient_country, 1200);

        assert!(!package.is_international());  //因为这里的比较是布尔值，所以函数的返回值是bool
    }

    #[test]
    fn calculate_transport_fees() {
        let sender_country = String::from("Spain");
        let recipient_country = String::from("Spain");

        let cents_per_gram = 3;

        let package = Package::new(sender_country, recipient_country, 1500);

        assert_eq!(package.get_fees(cents_per_gram), 4500);// 这里比较的事两个数值是否相等，所以返回值是数值
        assert_eq!(package.get_fees(cents_per_gram * 2), 9000);
    }
}

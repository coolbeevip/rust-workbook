#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {

        // 进制格式化
        let foo = 100;
        println!("{}", foo);
        println!("0x{:X}", foo);
        println!("0o{:o}", foo);

        // 浮点数精度
        let lat:f32 = 1.23;
        println!("{}", lat);
        println!("{:.1}", lat);
        println!("{:.3}", lat);

        // Without a suffix, 31 becomes an i32. You can change what type 31 is
        // by providing a suffix. The number 31i64 for example has the type i64.

        // There are various optional patterns this works with. Positional
        // arguments can be used.
        println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

        // As can named arguments.
        println!(
            "{subject} {verb} {object}",
            object = "the lazy dog",
            subject = "the quick brown fox",
            verb = "jumps over"
        );

        // Special formatting can be specified after a `:`.
        println!(
            "{} of {:b} people know binary, the other half doesn't",
            1, 2
        );

        // You can right-align text with a specified width. This will output
        // "     1". 5 white spaces and a "1".
        println!("{number:>width$}", number = 1, width = 6);

        // You can pad numbers with extra zeroes. This will output "000001".
        println!("{number:0>width$}", number = 1, width = 6);

        // Rust even checks to make sure the correct number of arguments are
        // used.
        println!("My name is {0}, {1} {0}", "Bond", "James");
        // FIXME ^ Add the missing argument: "James"
    }
}

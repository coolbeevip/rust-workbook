#[derive(Debug)] // 如果此处没有推导 Debug，则下边打印 Animal 的会编译出错
struct Animal<'a> {
    name: &'a str,
    age: u8,
}

#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8,
}

#[cfg(test)]
mod tests {
    use crate::debug::{Animal, Person};

    #[test]
    fn it_works() {
        // 使用 `{:?}` 打印和使用 `{}` 类似。
        println!("{:?} months in a year.", 12);
        println!(
            "{1:?} {0:?} is the {actor:?} name.",
            "Slater",
            "Christian",
            actor = "actor's"
        );

        // `Structure` 也可以打印！
        println!(
            "Now {:?} will print!",
            Animal {
                name: "Peter",
                age: 27
            }
        );

        // 使用 `derive` 的一个问题是不能控制输出的形式。
        println!(
            "Now {:?} will print!",
            Person {
                name: "Peter",
                age: 27
            }
        );

        // 美化打印
        let peter = Person {
            name: "Peter",
            age: 27,
        };
        println!("{:#?}", peter);
    }
}

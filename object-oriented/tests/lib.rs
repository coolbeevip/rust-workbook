#[cfg(test)]
mod tests {
    use object_oriented::{Animal, Cat, Dog};

    #[test]
    fn ping_test() {
        let animal = Animal {
            comp: Box::new(Dog {
                name: "dog".to_string(),
            }),
        };
        assert_eq!(animal.ping(), "Woof");

        let animal = Animal {
            comp: Box::new(Cat {
                name: "cat".to_string(),
            }),
        };
        assert_eq!(animal.ping(), "Miaow");
    }
}

// 定义一个接口
pub trait Say {
    fn say(&self) -> String;
}

// 定义一个类
pub struct Animal<T: Say> {
    pub comp: Box<T>,
}

// 定义类方法
impl<T> Animal<T>
where
    T: Say,
{
    pub fn ping(&self) -> String {
        self.comp.say()
    }
}

// 定义类
pub struct Dog {
    pub name: String,
}
impl Say for Dog {
    fn say(&self) -> String {
        println!("{} say {}", self.name, "Woof");
        "Woof".to_string()
    }
}

// 定义类
pub struct Cat {
    pub name: String,
}
impl Say for Cat {
    fn say(&self) -> String {
        println!("{} say {}", self.name, "Miaow");
        "Miaow".to_string()
    }
}

pub struct MyData {
    pub name: String,
    pub id: i32,
}

impl MyData {
    fn do_something(&self) {
        // do something
    }

    fn do_something_else(&self) {
        // do something else
    }
}

pub fn test_function(my_data: MyData) {
    println!("Hello {}, my id is {}", my_data.name, my_data.id);
}

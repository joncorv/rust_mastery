mod code_practice;
mod imports;
use imports::MyData;
use imports::test_function;

fn main() {
    let my_data = MyData {
        name: "John Smith".to_string(),
        id: 10093,
    };

    test_function(my_data);
}

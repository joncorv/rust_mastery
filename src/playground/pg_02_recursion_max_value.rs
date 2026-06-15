pub fn pg_02_recursion_max_value() {
    let vals = [0, 2, 4, 6, 8, 9, 20, 9];
    let mut visited: Vec<i32> = Vec::new();
    let mut max_value = 0;

    fn dfs(vals: &[i32], current: i32, visited: &mut Vec<i32>, max_value: &mut i32) {
        visited.push(current);
        if current > *max_value {
            *max_value = current;
        }
        println!("we're now on {:?}", current.clone());

        for val in vals {
            if !visited.contains(val) {
                dfs(vals, *val, visited, max_value);
            }
        }
    }

    dfs(&vals, 0, &mut visited, &mut max_value);

    println!("max value is: {max_value:?}");
}

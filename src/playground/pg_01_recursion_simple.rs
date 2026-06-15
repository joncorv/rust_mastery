pub fn pg_01_recursion_simple() {
    let vals = [0, 2, 4, 6, 8, 9];
    let mut visited: Vec<i32> = Vec::new();

    fn dfs(vals: &[i32], current: i32, visited: &mut Vec<i32>) {
        visited.push(current);
        println!("we're now on {:?}", current.clone());

        for val in vals {
            if !visited.contains(val) {
                dfs(vals, *val, visited);
            }
        }
    }

    dfs(&vals, 0, &mut visited);
}

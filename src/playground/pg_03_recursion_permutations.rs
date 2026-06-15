pub fn pg_02_recursion_permutations() {
    let vals = [1, 2, 3, 4];
    let mut visited: Vec<i32> = Vec::new();
    let mut permutations: Vec<Vec<i32>> = Vec::new();

    fn dfs(vals: &[i32], current: i32, visited: &mut Vec<i32>, permutations: &mut Vec<Vec<i32>>) {
        visited.push(current);
        println!("we're now on {:?}", current.clone());

        for val in vals {
            if !visited.contains(val) {
                dfs(vals, *val, visited, permutations);
            }
        }
    }

    dfs(&vals, 0, &mut visited, &mut permutations);

    // println!("max value is: {max_value:?}");
}

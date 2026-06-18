pub fn pg_02_recursion_permutations_v2() {
    let vals = [1, 2, 3, 4];
    let mut permutations: Vec<Vec<i32>> = Vec::new();
    let mut visited: Vec<i32> = Vec::new();

    fn dfs(vals: &[i32], visited: &mut Vec<i32>, permutations: &mut Vec<Vec<i32>>) {
        // success case
        if visited.len() == vals.len() {
            permutations.push(visited.clone());
        }

        // send all neighbor/adjacent nodes to the stack
        for val in vals {
            if !visited.contains(val) {
                visited.push(*val);
                dfs(vals, visited, permutations);
                visited.pop();
            }
        }
    }

    dfs(&vals, &mut visited, &mut permutations);

    // println!("max value is: {max_value:?}");
    println!("here is all the permutations here: {permutations:?}");
}

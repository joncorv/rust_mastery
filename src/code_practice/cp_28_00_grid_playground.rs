// here we have a function that takes in a nxn grid, and row/columnn number of a king. The goal is
// to return an array listing all the possible moves.
pub fn chess_moves(n: i32, r: i32, c: i32) -> Vec<Vec<usize>> {
    // code here

    let mut result: Vec<Vec<usize>> = Vec::new();

    let directions: Vec<Vec<i32>> = vec![
        vec![1, 0],
        vec![1, 1],
        vec![0, 1],
        vec![-1, 1],
        vec![-1, 0],
        vec![-1, -1],
        vec![0, -1],
        vec![1, -1],
    ];

    for dir in directions {
        let new_r = dir[0] + r;
        let new_c = dir[1] + c;
        if is_valid(n, new_r, new_c) {
            result.push(vec![new_r as usize, new_c as usize]);
        }
    }

    return result;
}

fn is_valid(n: i32, r: i32, c: i32) -> bool {
    return r >= 0 && r < n && c >= 0 && c < n;
}

pub fn test_grid_playground() {
    println!("Testing valid chess moves");

    // create a basic grid

    let size_grid: usize = 8;

    let row = vec!['.'; size_grid];
    let mut room = vec![row.clone(); size_grid];
    println!("Here is a {size_grid} x {size_grid} room:");
    room[0][0] = 'K';
    for row in room.clone() {
        println!("{row:?}");
    }

    let test_result = chess_moves(size_grid as i32, 0, 0);
    // let changes = vec![
    //     vec![0, 0],
    //     vec![1, 1],
    //     vec![2, 2],
    //     vec![3, 3],
    //     vec![4, 4],
    //     vec![5, 5],
    //     vec![6, 6],
    //     vec![7, 7],
    // ];

    println!("test_result: {test_result:?}");

    for ch in test_result {
        room[ch[0]][ch[1]] = '0';
    }

    println!("room has now been changed");
    for row in room.clone() {
        println!("{row:?}");
    }
}

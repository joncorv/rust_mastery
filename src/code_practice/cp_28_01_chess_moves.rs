pub fn chess_moves(room: Vec<Vec<char>>, piece: String, r: usize, c: usize) -> Vec<Vec<usize>> {
    let result: Vec<Vec<usize>> = Vec::new();

    if room.is_empty() {
        return result;
    }

    let len_rows = room[0].len();
    let len_cols= room.len();

    let king_directions: Vec<Vec<i32>> = vec![
        vec![1, 0],
        vec![1, 1],
        vec![0, 1],
        vec![-1, 1],
        vec![-1, 0],
        vec![-1, -1],
        vec![0, -1],
        vec![1, -1],
    ];

    for dirs in king_directions {
        let new_r = r + dirs[0];
        let new_c = c + dirs[1];

        if is_valid(n, r, c)
    }

    return result;
}

fn is_valid(n: i32, r: i32, c: i32) -> bool {
    return r >= 0 && r < n && c >= 0 && c < n;
}

pub fn test_chess_moves() {
    println!("Testing valid chess moves");

    // define square grid size
    let size_grid: usize = 9;
    let piece = "king".to_string();
    let (r, c): (usize, usize) = (4, 3);

    // iterate to build grid with default values
    let row = vec![' '; size_grid];
    let mut room = vec![row.clone(); size_grid];

    // create occupied
    let occupied = vec![
        vec![0, 0],
        vec![1, 1],
        vec![2, 2],
        vec![3, 3],
        vec![4, 4],
        vec![5, 5],
        vec![6, 6],
        vec![7, 7],
        vec![8, 8],
        vec![8, 0],
        vec![7, 1],
        vec![6, 2],
        vec![5, 3],
        vec![4, 4],
        vec![3, 5],
        vec![2, 6],
        vec![1, 7],
        vec![0, 8],
    ];

    // apply changes to room
    for opd in occupied {
        room[opd[0]][opd[1]] = 'x';
    }

    room[r][c] = 'K';
    // print new room
    println!("This is the testing room:");
    for row in room.clone() {
        println!("{row:?}");
    }

    let test_result: Vec<Vec<usize>> = chess_moves(room.clone(), piece, r, c);

    // set test_results to room
    for tr in test_result {
        room[tr[0]][tr[1]] = '0';
    }

    println!("This is the test result room:");
    for row in room.clone() {
        println!("{row:?}");
    }
}

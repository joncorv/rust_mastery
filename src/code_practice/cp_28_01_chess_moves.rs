pub fn chess_moves(room: Vec<Vec<char>>, piece: String, r: usize, c: usize) -> Vec<Vec<usize>> {
    let mut result: Vec<Vec<usize>> = Vec::new();

    if room.is_empty() {
        return result;
    }

    let queen = "queen".to_string();
    let king = "king".to_string();
    let knight = "knight".to_string();

    let royal_directions: Vec<Vec<i32>> = vec![
        vec![1, 0],
        vec![1, 1],
        vec![0, 1],
        vec![-1, 1],
        vec![-1, 0],
        vec![-1, -1],
        vec![0, -1],
        vec![1, -1],
    ];

    let knight_directions: Vec<Vec<i32>> = vec![
        vec![-2, -1],
        vec![-1, -2],
        vec![-2, 1],
        vec![-1, 2],
        vec![2, -1],
        vec![1, -2],
        vec![2, 1],
        vec![1, 2],
    ];

    if piece == "king".to_string() {
        for dirs in royal_directions {
            let r = r as i32;
            let c = c as i32;
            let new_r = r + dirs[0];
            let new_c = c + dirs[1];

            if is_valid(room.clone(), new_r, new_c) {
                result.push(vec![new_r as usize, new_c as usize]);
            }
        }
    } else if piece == "knight".to_string() {
        for dirs in knight_directions {
            let r = r as i32;
            let c = c as i32;
            let new_r = r + dirs[0];
            let new_c = c + dirs[1];

            if is_valid(room.clone(), new_r, new_c) {
                result.push(vec![new_r as usize, new_c as usize]);
            }
        }
    } else if piece == "queen".to_string() {
        for dirs in royal_directions {
            let r = r as i32;
            let c = c as i32;
            let mut new_r = r + dirs[0];
            let mut new_c = c + dirs[1];

            'check_validity: loop {
                if is_valid(room.clone(), new_r, new_c) {
                    result.push(vec![new_r as usize, new_c as usize]);
                    new_r += dirs[0];
                    new_c += dirs[1];
                } else {
                    break 'check_validity;
                }
            }
        }
    }

    return result;
}

fn is_valid(room: Vec<Vec<char>>, r: i32, c: i32) -> bool {
    return r >= 0
        && r < room[0].len() as i32
        && c >= 0
        && c < room.len() as i32
        && room[r as usize][c as usize] != 'x';
}

pub fn test_chess_moves() {
    println!("Testing valid chess moves");

    // define square grid size
    let size_grid: usize = 9;
    let piece = "queen".to_string();
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

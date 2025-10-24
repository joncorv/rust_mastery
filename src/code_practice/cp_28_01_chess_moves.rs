pub fn chess_moves() {
    // code here
}

pub fn test_chess_moves() {
    println!("Testing valid chess moves");

    // create a basic grid

    let size_grid: usize = 8;

    let row = vec!['x'; size_grid];
    let mut room = vec![row.clone(); size_grid];
    println!("Here is a {size_grid} x {size_grid} room:");
    for row in room.clone() {
        println!("{row:?}");
    }

    let changes = vec![
        vec![0, 0],
        vec![1, 1],
        vec![2, 2],
        vec![3, 3],
        vec![4, 4],
        vec![5, 5],
        vec![6, 6],
        vec![7, 7],
    ];

    for ch in changes {
        room[ch[0]][ch[1]] = '0';
    }

    println!("room has now been changed");
    for row in room.clone() {
        println!("{row:?}");
    }
}

#[derive(Debug, Clone)]
pub struct Context {
    cell: (i8, i8),
    path: Vec<(i8, i8)>,
    success: bool,
    success_path: Vec<(i8, i8)>,
}

fn get_valid_neighbors(maze: &Vec<Vec<i8>>, cell: (i8, i8)) -> Vec<(i8, i8)> {
    let mut valid_neighbors = Vec::new();
    let dirs: Vec<(i8, i8)> = vec![(-1, 0), (1, 0), (0, 1), (0, -1)];
    for dir in dirs {
        let test_cell = (dir.0 + cell.0, dir.1 + cell.1);
        let in_bounds = test_cell.0 >= 0 && test_cell.0 < (maze.len() as i8) && test_cell.1 >= 0 && test_cell.1 < (maze[0].len() as i8);
        // println!("test cell {test_cell:?} is {in_bounds}");
        if in_bounds {
            let valid_cell = maze[test_cell.0 as usize][test_cell.1 as usize] == 1;
            if valid_cell {
                // println!("cell: {cell:?} and test_cell: {test_cell:?} is valid? {valid_cell}");
                valid_neighbors.push(test_cell);
            }
        }
    }
    valid_neighbors
}

pub fn graph_maze(maze: Vec<Vec<i8>>, start: (i8, i8), end: (i8, i8)) -> Context {
    let mut cell: (i8, i8);
    let mut path: Vec<(i8, i8)>;
    let mut context = Context { cell: start, path: Vec::new(), success: false, success_path: Vec::new() };

    fn dfs(maze: &Vec<Vec<i8>>, end: &(i8, i8), context: &mut Context) {
        // base case
        if context.success {
            return;
        }

        // success found
        if context.cell == *end {
            context.success = true;
            context.success_path = context.path.clone();
            // println!("success found!");
            return;
        }

        // find valid neighbors over current cell
        let valid_neighbors = get_valid_neighbors(maze, context.cell);
        // println!("valid neighbors: {:?}", valid_neighbors.clone());

        // recursion over valid neighbors
        for neighbor in valid_neighbors {
            if !context.path.contains(&neighbor) {
                context.cell = neighbor;
                context.path.push(context.cell);
                dfs(maze, end, context);
            }
        }

        // backtrack
        context.path.pop();
    }

    dfs(&maze, &end, &mut context);

    context
}

pub fn test_graph_maze() {
    let maze: Vec<Vec<i8>> = vec![
        vec![1, 1, 1, 1, 0, 1, 0, 0, 0, 0],
        vec![0, 0, 0, 1, 0, 1, 0, 0, 0, 0],
        vec![0, 0, 0, 1, 0, 1, 0, 1, 0, 0],
        vec![0, 0, 0, 1, 1, 1, 1, 1, 1, 0],
        vec![0, 0, 0, 1, 0, 0, 0, 1, 1, 0],
        vec![0, 0, 1, 1, 0, 0, 0, 0, 1, 0],
        vec![1, 1, 1, 0, 0, 0, 1, 1, 1, 0],
        vec![0, 0, 0, 0, 0, 0, 1, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 1, 1, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 1],
    ];
    let start = (0, 0);
    let end = (9, 9);
    println!("\nProessing has begun");
    let result = graph_maze(maze.clone(), start, end);

    if result.success {
        println!("HOORAY, We found a path out of the maze!!! 😃")
    } else {
        println!("CRAP. We couldn't find a path out of the maze 😢")
    }
    // println!("graph maze result num success path: {:?}", result.success_path);

    // show question
    println!("\nOriginal Question:");
    for row in maze.clone() {
        println!("{row:?}");
    }

    // show result
    println!("\nSuccessful Path:");
    let mut result_view = maze.clone();
    for res in result.success_path {
        result_view[res.0 as usize][res.1 as usize] = 2;
    }
    for row in result_view {
        println!("{row:?}");
    }
}

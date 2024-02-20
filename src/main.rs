fn main() {


    let size = 4;

    let mut matrix = vec![vec![0; size]; size];


    preset_matrix(&mut matrix);

    loop {
        print_matrix(&matrix);
        println!();

        //wait for 1 second
        matrix = apply_rules(&matrix);
        std::thread::sleep(std::time::Duration::from_millis(1000));
    }

}


fn preset_matrix(matrix: &mut Vec<Vec<i32>>) {
    let size = matrix.len() as usize;
    for (i, row) in matrix.iter_mut().enumerate() {
        for (j, cell) in row.iter_mut().enumerate() {
            if (i > 0 && i < size-1) && (j > 0 && j < size-1){
                *cell = 1;
            }
        }
    }
}


fn print_matrix(matrix: &Vec<Vec<i32>>) {
    // clear the screen
    print!("\x1B[2J\x1B[1;1H");
    for row in matrix.iter() {
        for cell in row.iter() {
            print!("{} ", cell);
        }
        println!();
    }
}


fn apply_rules(matrix: &Vec<Vec<i32>>) -> Vec<Vec<i32>>{
    let mut new_matrix = vec![vec![0; matrix.len()]; matrix.len()];
    for (i, row) in matrix.iter().enumerate() {
        for (j, cell) in row.iter().enumerate() {
            let count = count_neighbors(&matrix, i , j);


            if *cell == 1 {
                if count < 2 || count > 3 {
                    new_matrix[i][j] = 0;
                } else {
                    new_matrix[i][j] = 1;
                }
            } else {
                if count == 3 {
                    new_matrix[i][j] = 1;
                } else {
                    new_matrix[i][j] = 0;
                }
            }
        }
    }

    new_matrix
}


fn count_neighbors(matrix: &Vec<Vec<i32>>, i: usize, j: usize) -> i32 {
    let mut count = 0;
    for x in -1..2 {
        for y in -1..2 {
            if x == 0 && y == 0 {
                continue;
            }
            let new_i = i as i32 + x;
            let new_j = j as i32 + y;
            if new_i >= 0 && new_i < matrix.len() as i32 && new_j >= 0 && new_j < matrix.len() as i32 {
                count += matrix[new_i as usize][new_j as usize];
            }
        }
    }
    count
}


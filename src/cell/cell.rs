use std::error::Error;


#[derive(Default)]
pub struct Cells {
    pub cells: Vec<Vec<bool>>,
}

#[derive(Debug)]
pub struct CellError {
   pub message: String,
}


impl std::fmt::Display for CellError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "CellError: {}", self.message)
    }
}

impl Error for CellError {
    fn description(&self) -> &str {
        &self.message
    }
}



impl Cells {
    pub fn new(preset: &str) -> Result<Cells, CellError>{
        match preset.to_lowercase().as_str() {
            "blinker"=> {
                let cell = Cells::blinker();
                Ok(cell)
            },
            "toad"=> {
                let cell = Cells::toad();
                Ok(cell)
            },
            "r_pentomino"=> {
                let cell = Cells::r_pentomino();
                Ok(cell)
            },
            default => return Err(CellError{message: format!("Unknown preset: {}", default)}),
        }
    }

    pub fn get_presets(&mut self, preset: &str) -> (){
        match preset.to_lowercase().as_str() {
            "blinker"=> {
                self.cells = Cells::blinker().cells;
            },
            "toad"=> {
                self.cells = Cells::toad().cells;
            },
            "r_pentomino"=> {
                self.cells = Cells::r_pentomino().cells;
            },
            _ => (),
        }
    }




    pub fn apply_rules(&mut self) {
        let mut new_cells = self.cells.clone();
        for (i, row) in self.cells.iter().enumerate() {
            for (j, cell) in row.iter().enumerate() {
                let count = self.count_neighbors(i , j);


                if *cell == true {
                    if count < 2 || count > 3 {
                        new_cells[i][j] = false;
                    } else {
                        new_cells[i][j] = true;
                    }
                } else {
                    if count == 3 {
                        new_cells[i][j] = true;
                    } else {
                        new_cells[i][j] = false;
                    }
                }
            }
        }


        self.cells = new_cells;


    }



    fn count_neighbors(&self, i: usize, j: usize) -> i32 {
        let mut count = 0;
        for x in -1..2 {
            for y in -1..2 {
                if x == 0 && y == 0 {
                    continue;
                }
                let new_i = i as i32 + x;
                let new_j = j as i32 + y;
                if new_i >= 0 && new_i < self.cells.len() as i32 && new_j >= 0 && new_j < self.cells.len() as i32 {
                    if self.cells[new_i as usize][new_j as usize] {
                        count += 1;
                    }
                }
            }
        }
        count
    }








    // Preset Cells for the game
    // They all need to be 10x10

    fn blinker() -> Cells{
        // set to be 10x10
        let cell = Cells{ cells: vec![
            vec![false, false, false, false, false, false, false, false, false, false],
            vec![false, false, false, false, false, false, false, false, false, false],
            vec![false, false, false, false, false, false, false, false, false, false],
            vec![false, false, false, true, true, true, false, false, false, false],
            vec![false, false, false, false, false, false, false, false, false, false],
            vec![false, false, false, false, false, false, false, false, false, false],
            vec![false, false, false, false, false, false, false, false, false, false],
            vec![false, false, false, false, false, false, false, false, false, false],
            vec![false, false, false, false, false, false, false, false, false, false],
            vec![false, false, false, false, false, false, false, false, false, false],
        ]
        };

        cell
    }

    fn toad() -> Cells{
        // set to be 10x10
        let cell = Cells{ cells: vec![
            vec![false, false, false, false, false, false, false, false, false, false],
            vec![false, false, false, false, false, false, false, false, false, false],
            vec![false, false, false, false, false, false, false, false, false, false],
            vec![false, false, false, false, true, true, true, false, false, false],
            vec![false, false, false, true, true, true, false, false, false, false],
            vec![false, false, false, false, false, false, false, false, false, false],
            vec![false, false, false, false, false, false, false, false, false, false],
            vec![false, false, false, false, false, false, false, false, false, false],
            vec![false, false, false, false, false, false, false, false, false, false],
            vec![false, false, false, false, false, false, false, false, false, false],
        ]
        };

        cell
    }

    fn r_pentomino() -> Cells{
        // set to be 10x10
        let cell = Cells{ cells: vec![
            vec![false, false, false, false, false, false, false, false, false, false],
            vec![false, false, false, false, false, false, false, false, false, false],
            vec![false, false, false, false, false, false, false, false, false, false],
            vec![false, false, false, false, true, true, false, false, false, false],
            vec![false, false, false, true, true, false, false, false, false, false],
            vec![false, false, false, false, true, false, false, false, false, false],
            vec![false, false, false, false, false, false, false, false, false, false],
            vec![false, false, false, false, false, false, false, false, false, false],
            vec![false, false, false, false, false, false, false, false, false, false],
            vec![false, false, false, false, false, false, false, false, false, false],
        ]
        };

        cell
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_blinker() {
        let mut cell = Cells::blinker();
        cell.apply_rules();

        assert_eq!(cell.cells, vec![
            vec![false, false, false, false, false],
            vec![false, false, true, false, false],
            vec![false, false, true, false, false],
            vec![false, false, true, false, false],
            vec![false, false, false, false, false],
        ]);
    }

    #[test]
    fn test_toad() {
        let mut cell = Cells::toad();
        cell.apply_rules();

        assert_eq!(cell.cells, vec![
            vec![false, false, false, false, false, false],
            vec![false, false, false, true, false, false],
            vec![false, true, false, false, true, false],
            vec![false, true, false, false, true, false],
            vec![false, false, true, false, false, false],
            vec![false, false, false, false, false, false],
        ]);
    }

}

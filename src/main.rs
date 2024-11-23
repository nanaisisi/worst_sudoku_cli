use rand::Rng;
use std::collections::HashSet;

struct Grid {
    data: [[u32; 9]; 9],
}

impl Grid {
    fn new() -> Self {
        Grid { data: [[0; 9]; 9] }
    }

    fn generate(&mut self) -> Result<(), &'static str> {
        let mut rng = rand::thread_rng();
        for i in 0..9 {
            for j in 0..9 {
                let mut available_numbers: HashSet<u32> = (1..=9).collect();

                while self.data[i].contains(&random_number)
                    || self.data.iter().any(|row| row[j] == random_number)
                {
                    let random_number = rng.gen_range(1..=9);
                    self.data[i][j] = random_number;
                }
            }
        }
        Ok(())
    }

    fn display(&self) {
        for row in &self.data {
            for &num in row {
                print!("{} ", num);
            }
            println!();
        }
    }
}

fn main() {
    let mut grid = Grid::new();
    match grid.generate() {
        Ok(_) => grid.display(),
        Err(_) => println!("破綻した"),
    }
}

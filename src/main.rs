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

                // 行と列の既存の数字を除外
                for k in 0..9 {
                    available_numbers.remove(&self.data[i][k]);
                    available_numbers.remove(&self.data[k][j]);
                }

                // 利用可能な数字からランダムに選択
                let random_number = *available_numbers
                    .iter()
                    .nth(rng.gen_range(0..available_numbers.len()))
                    .unwrap();
                self.data[i][j] = random_number;
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

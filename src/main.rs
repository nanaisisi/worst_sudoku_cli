use rand::Rng;

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
                println!("{}行{}列目の数字を生成します", i + 1, j + 1);
                let mut random_number = rng.gen_range(1..=9);
                while self.data[i].contains(&random_number)
                    || self.data.iter().any(|row| row[j] == random_number)
                {
                    random_number = rng.gen_range(1..=9);
                }
                self.data[i][j] = random_number;
                println!(
                    "{}行{}列目の数字{}を生成しました",
                    i + 1,
                    j + 1,
                    self.data[i][j]
                );
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
        Err(e) => println!("破綻した: {}", e),
    }
}

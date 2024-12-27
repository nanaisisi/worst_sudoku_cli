mod grid;

use grid::Grid;

fn main() {
    let mut grid = Grid::new();
    match grid.generate() {
        Ok(_) => grid.display(),
        Err(e) => println!("破綻した: {}", e),
    }
}

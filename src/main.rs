use rand::Rng;
use std::thread;
use std::time::Duration;

struct Grid {
    data: [[u32; 9]; 9],
}

impl Grid {
    fn new() -> Self {
        Grid { data: [[0; 9]; 9] }
    }

    fn generate(&mut self) -> Result<(), &'static str> {
        let mut rng = rand::thread_rng();
        let mut random_number;
        let mut re_random_flag = false;
        let mut re_generate_flag = false;
        let mut re_i_flag = false;
        let mut re_j_flag = false;
        let mut re_block_flag = false;
        loop {
            for i in 0..9 {
                for j in 0..9 {
                    if re_generate_flag == true {
                        break;
                    }
                    if cfg!(debug_assertions) {
                        println!("{}行{}列目の数字を生成します", i + 1, j + 1);
                    }
                    loop {
                        dbg!(
                            re_i_flag,
                            re_j_flag,
                            re_block_flag,
                            re_generate_flag,
                            re_random_flag
                        );
                        random_number = rng.gen_range(1..10);
                        if cfg!(debug_assertions) {
                            println!("乱数: {}", random_number);
                            self.display();
                        }
                        if re_random_flag == true {
                            re_random_flag = false;
                            re_generate_flag = false;
                            re_i_flag = false;
                            re_j_flag = false;
                            re_block_flag = false;
                            println!("再生成します。");
                            if cfg!(debug_assertions) {
                                self.display();
                            }
                            continue;
                        }
                        for k in 0..9 {
                            // 同じ行に同じ数字があるかチェック
                            if self.data[i][k] == random_number {
                                re_random_flag = true;
                                re_i_flag = true;
                                if cfg!(debug_assertions) {
                                    println!(
                                        "{}行目チェック{}列目の数字{}はすでに存在します",
                                        i + 1,
                                        j + 1,
                                        random_number
                                    );
                                    println!(
                                        "{}行目チェック{}列目の{}が重複しているため再生成します",
                                        i + 1,
                                        k + 1,
                                        random_number
                                    );
                                }
                            }
                            // 同じ列に同じ数字があるかチェック
                            if self.data[k][j] == random_number {
                                re_random_flag = true;
                                re_j_flag = true;
                                if cfg!(debug_assertions) {
                                    println!(
                                        "{}列目チェック{}行目の数字{}はすでに存在します",
                                        j + 1,
                                        i + 1,
                                        random_number
                                    );
                                    println!(
                                        "{}列目チェック{}行目の{}が重複しているため再生成します",
                                        k + 1,
                                        j + 1,
                                        random_number
                                    );
                                }
                            }
                        }
                        // 3x3のブロック内での重複チェック
                        // 1-3行目
                        if i < 3 {
                            // 1-3列目
                            // 1-1ブロック
                            if j < 3 {
                                for l in 0..3 {
                                    for m in 0..3 {
                                        if self.data[l][m] == random_number {
                                            if re_i_flag == false && re_j_flag == false {
                                                re_block_flag = true;
                                            }
                                            re_random_flag = true;
                                            if cfg!(debug_assertions) {
                                                println!(
                                                        "1-1ブロックチェック{}行目{}列目の数字{}はすでに存在します",
                                                        i + 1,
                                                        j + 1,
                                                        random_number
                                                    );
                                                println!(
                                                    "1-1ブロックチェック{}行目{}列目の{}が重複しているため再生成します",
                                                    l + 1,
                                                    m + 1,
                                                    random_number
                                                );
                                            }
                                        }
                                    }
                                }

                                // 3-6列目
                                // 1-2ブロック
                            } else if 2 < j && j < 6 {
                                for l in 0..3 {
                                    for m in 3..6 {
                                        if self.data[l][m] == random_number {
                                            if re_i_flag == false && re_j_flag == false {
                                                re_block_flag = true;
                                            }
                                            re_random_flag = true;
                                            if cfg!(debug_assertions) {
                                                println!(
                                                        "1-2ブロックチェック{}行目{}列目の数字{}はすでに存在します",
                                                        i + 1,
                                                        j + 1,
                                                        random_number
                                                    );
                                                println!(
                                                    "1-2ブロックチェック{}行目{}列目の{}が重複しているため再生成します",
                                                    l + 1,
                                                    m + 1,
                                                    random_number
                                                );
                                            }
                                        }
                                    }
                                }
                                // 6-9列目
                                // 1-3ブロック
                            } else if 5 < j {
                                for l in 0..3 {
                                    for m in 6..9 {
                                        if self.data[l][m] == random_number {
                                            if re_i_flag == false && re_j_flag == false {
                                                re_block_flag = true;
                                            }
                                            re_random_flag = true;
                                            if cfg!(debug_assertions) {
                                                println!(
                                                        "1-3ブロックチェック{}行目{}列目の数字{}はすでに存在します",
                                                        i + 1,
                                                        j + 1,
                                                        random_number
                                                    );
                                                println!(
                                                    "1-3ブロックチェック{}行目{}列目の{}が重複しているため再生成します",
                                                    l + 1,
                                                    m + 1,
                                                    random_number
                                                );
                                            }
                                        }
                                    }
                                }
                            }
                            // 4-6行目
                        } else if 2 < i && i < 6 {
                            // 1-3列目
                            // 2-1ブロック
                            if j < 3 {
                                for l in 3..6 {
                                    for m in 0..3 {
                                        if self.data[l][m] == random_number {
                                            if re_i_flag == false && re_j_flag == false {
                                                re_block_flag = true;
                                            }
                                            re_random_flag = true;
                                            if cfg!(debug_assertions) {
                                                println!(
                                                        "2-1ブロックチェック{}行目{}列目の数字{}はすでに存在します",
                                                        i + 1,
                                                        j + 1,
                                                        random_number
                                                    );
                                                println!(
                                                        "2-1ブロックチェック{}行目{}列目の{}が重複しているため再生成します",
                                                        l + 1,
                                                        m + 1,
                                                        random_number
                                                    );
                                            }
                                        }
                                    }
                                }
                            }
                            // 3-6列目
                            // 2-2ブロック
                            else if 2 < j && j < 6 {
                                for l in 3..6 {
                                    for m in 3..6 {
                                        if self.data[l][m] == random_number {
                                            if re_i_flag == false && re_j_flag == false {
                                                re_block_flag = true;
                                            }
                                            re_random_flag = true;
                                            if cfg!(debug_assertions) {
                                                println!(
                                                        "2-2ブロックチェック{}行目{}列目の数字{}はすでに存在します",
                                                        i + 1,
                                                        j + 1,
                                                        random_number
                                                    );
                                                println!(
                                                        "2-2ブロックチェック{}行目{}列目の{}が重複しているため再生成します",
                                                        l + 1,
                                                        m + 1,
                                                        random_number
                                                    );
                                            }
                                        }
                                    }
                                }
                            }
                            // 7-9列目
                            // 2-3ブロック
                            else if 5 < j {
                                for l in 3..6 {
                                    for m in 6..9 {
                                        if self.data[l][m] == random_number {
                                            if re_i_flag == false && re_j_flag == false {
                                                re_block_flag = true;
                                            }
                                            re_random_flag = true;
                                            if cfg!(debug_assertions) {
                                                println!(
                                                        "2-3ブロックチェック{}行目{}列目の数字{}はすでに存在します",
                                                        i + 1,
                                                        j + 1,
                                                        random_number
                                                    );
                                                println!(
                                                        "2-3ブロックチェック{}行目{}列目の{}が重複しているため再生成します",
                                                        l + 1,
                                                        m + 1,
                                                        random_number
                                                    );
                                            }
                                        }
                                    }
                                }
                            }
                            // 7-9行目
                        } else if 5 < i {
                            // 1-3列目
                            // 3-1ブロック
                            if j < 3 {
                                for l in 6..9 {
                                    for m in 0..3 {
                                        if self.data[l][m] == random_number {
                                            if re_i_flag == false && re_j_flag == false {
                                                re_block_flag = true;
                                            }
                                            re_random_flag = true;
                                            if cfg!(debug_assertions) {
                                                println!(
                                                        "3-1ブロックチェック{}行目{}列目のの数字{}はすでに存在します",
                                                        i + 1,
                                                        j + 1,
                                                        random_number
                                                    );
                                                println!(
                                                        "3-1ブロックチェック{}行目{}列目の{}が重複しているため再生成します",
                                                        l + 1,
                                                        m + 1,
                                                        random_number
                                                    );
                                            }
                                        }
                                    }
                                }
                            }
                            // 3-6列目
                            // 3-2ブロック
                            else if 2 < j && j < 6 {
                                for l in 6..9 {
                                    for m in 3..6 {
                                        if self.data[l][m] == random_number {
                                            if re_i_flag == false && re_j_flag == false {
                                                re_block_flag = true;
                                            }
                                            re_random_flag = true;
                                            if cfg!(debug_assertions) {
                                                println!(
                                                        "3-2ブロックチェック{}行目{}列目の数字{}はすでに存在します",
                                                        i + 1,
                                                        j + 1,
                                                        random_number
                                                    );
                                                println!(
                                                        "3-2ブロックチェック{}行目{}列目の{}が重複しているため再生成します",
                                                        l + 1,
                                                        m + 1,
                                                        random_number
                                                    );
                                            }
                                        }
                                    }
                                }
                            }
                            // 7-9列目
                            // 3-3ブロック
                            else if 5 < j {
                                for l in 6..9 {
                                    for m in 6..9 {
                                        if self.data[l][m] == random_number {
                                            if re_i_flag == false && re_j_flag == false {
                                                re_block_flag = true;
                                            }
                                            re_random_flag = true;
                                            if cfg!(debug_assertions) {
                                                println!(
                                                        "3-3ブロックチェック{}行目{}列目の数字{}はすでに存在します",
                                                        i + 1,
                                                        j + 1,
                                                        random_number
                                                    );
                                                println!(
                                                        "3-3ブロックチェック{}行目{}列目の{}が重複しているため再生成します",
                                                        l + 1,
                                                        m + 1,
                                                        random_number
                                                    );
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        if re_i_flag == true && re_j_flag == true
                            || re_i_flag == true && re_block_flag == true
                            || re_j_flag == true && re_block_flag == true
                        {
                            dbg!(
                                re_i_flag,
                                re_j_flag,
                                re_block_flag,
                                re_generate_flag,
                                re_random_flag
                            );
                            re_i_flag = false;
                            re_j_flag = false;
                            re_block_flag = false;
                            re_random_flag = false;
                            re_generate_flag = true;
                            self.data = [[0; 9]; 9];
                            break;
                        }
                        if re_random_flag == false {
                            dbg!(
                                re_i_flag,
                                re_j_flag,
                                re_block_flag,
                                re_generate_flag,
                                re_random_flag
                            );
                            break;
                        }
                    }
                    self.data[i][j] = random_number;
                    if cfg!(debug_assertions) {
                        println!(
                            "{}行{}列目の数字{}を生成しました",
                            i + 1,
                            j + 1,
                            self.data[i][j]
                        );
                    }
                }
                if cfg!(debug_assertions) {
                    println!("{:?}行目の数字生成完了", i + 1);
                }
                if re_generate_flag == true {
                    dbg!(re_generate_flag);
                    println!("完全に再生成します。");
                    if cfg!(debug_assertions) {
                        self.display();
                    }
                    break;
                }
            }
            if re_generate_flag == true {
                dbg!(
                    re_i_flag,
                    re_j_flag,
                    re_block_flag,
                    re_generate_flag,
                    re_random_flag
                );
                re_generate_flag = false;
                self.data = [[0; 9]; 9];
                continue;
            }
            break;
        }
        println!("数字生成完了");
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

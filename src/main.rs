use rand::Rng;

struct Grid {
    data: [[u32; 9]; 9],
}

impl Grid {
    fn new() -> Self {
        Grid { data: [[0; 9]; 9] }
    }

    fn z_for(for_range_min: u8, for_range_max: u8, vertex: u8, bottom: u8, twist : u8){
        for i in for_range_min..for_range_max {
            z_if();
    }
};

    fn z_if(vertex: u8){if self.data[vertex][k] == random_number {
        re_random_flag = true;
        re_i_flag = true;
        if cfg!(debug_assertions) {
            println!(
                "{}行{}列目の数字{}はすでに存在します",
                vertex + 1,
                bottom + 1,
                random_number
            );
            println!(
                "{}行{}列目の{}が重複しているため再生成します",
                vertex + 1,
                twist + 1,
                random_number
            );
        }
    }};

    fn generate(&mut self) -> Result<(), &'static str> {
        let mut rng = rand::thread_rng();
        let mut random_number;
        let mut re_random_flag = false;
        let mut re_generate_flag = false;
        let mut re_i_flag = false;
        let mut re_j_flag = false;
        let mut re_k_flag = false;
        loop {
            for i in 0..9 {
                for j in 0..9 {
                    if re_generate_flag == true {
                        break;
                    }
                    re_i_flag = false;
                    re_j_flag = false;
                    re_k_flag = false;
                    if cfg!(debug_assertions) {
                        println!("{}行{}列目の数字を生成します", i + 1, j + 1);
                    }
                    loop {
                        random_number = rng.gen_range(1..10);
                        if cfg!(debug_assertions) {
                            println!("乱数: {}", random_number);
                        }
                        if re_random_flag == true {
                            re_random_flag = false;
                            println!("再生成します。");
                            if cfg!(debug_assertions) {
                                self.display();
                            }
                            continue;
                        }
                        for k in 0..9 {
                            if self.data[i][k] == random_number {
                                re_random_flag = true;
                                re_i_flag = true;
                                if cfg!(debug_assertions) {
                                    println!(
                                        "{}行{}列目の数字{}はすでに存在します",
                                        i + 1,
                                        j + 1,
                                        random_number
                                    );
                                    println!(
                                        "{}行{}列目の{}が重複しているため再生成します",
                                        i + 1,
                                        k + 1,
                                        random_number
                                    );
                                }
                            }
                            if self.data[k][j] == random_number {
                                re_random_flag = true;
                                re_j_flag = true;
                                if cfg!(debug_assertions) {
                                    println!(
                                        "{}行{}列目の数字{}はすでに存在します",
                                        i + 1,
                                        j + 1,
                                        random_number
                                    );
                                    println!(
                                        "{}行{}列目の{}が重複しているため再生成します",
                                        k + 1,
                                        j + 1,
                                        random_number
                                    );
                                }
                            }
                            // 3x3のブロック内での重複チェック
                            if i < 3 {
                                if j < 3 {
                                    for l in 0..3 {
                                        for m in 0..3 {
                                            if self.data[l][m] == random_number {
                                                re_k_flag = true;
                                                re_random_flag = true;
                                                if cfg!(debug_assertions) {
                                                    println!(
                                                        "{}行{}列目の数字{}はすでに存在します",
                                                        i + 1,
                                                        j + 1,
                                                        random_number
                                                    );
                                                    println!(
                                                    "{}行{}列目の{}が重複しているため再生成します",
                                                    l + 1,
                                                    m + 1,
                                                    random_number
                                                );
                                                }
                                            }
                                        }
                                    }
                                } else if j < 6 {
                                    for l in 0..3 {
                                        for m in 3..6 {
                                            if self.data[l][m] == random_number {
                                                re_k_flag = true;
                                                re_random_flag = true;
                                                if cfg!(debug_assertions) {
                                                    println!(
                                                        "{}行{}列目の数字{}はすでに存在します",
                                                        i + 1,
                                                        j + 1,
                                                        random_number
                                                    );
                                                    println!(
                                                    "{}行{}列目の{}が重複しているため再生成します",
                                                    l + 1,
                                                    m + 1,
                                                    random_number
                                                );
                                                }
                                            }
                                        }
                                    }
                                } else {
                                    for l in 0..3 {
                                        for m in 6..9 {
                                            if self.data[l][m] == random_number {
                                                re_k_flag = true;
                                                re_random_flag = true;
                                                if cfg!(debug_assertions) {
                                                    println!(
                                                        "{}行{}列目の数字{}はすでに存在します",
                                                        i + 1,
                                                        j + 1,
                                                        random_number
                                                    );
                                                    println!(
                                                    "{}行{}列目の{}が重複しているため再生成します",
                                                    l + 1,
                                                    m + 1,
                                                    random_number
                                                );
                                                }
                                            }
                                        }
                                    }
                                }
                            } else if i < 6 {
                                if j < 3 {
                                    for l in 3..6 {
                                        for m in 0..3 {
                                            if self.data[l][m] == random_number {
                                                re_k_flag = true;
                                                re_random_flag = true;
                                                if cfg!(debug_assertions) {
                                                    println!(
                                                        "{}行{}列目の数字{}はすでに存在します",
                                                        i + 1,
                                                        j + 1,
                                                        random_number
                                                    );
                                                }
                                            }
                                        }
                                    }
                                }
                            } else {
                                if j < 3 {
                                    for l in 6..9 {
                                        for m in 0..3 {
                                            if self.data[l][m] == random_number {
                                                re_k_flag = true;
                                                re_random_flag = true;
                                                if cfg!(debug_assertions) {
                                                    println!(
                                                        "{}行{}列目の数字{}はすでに存在します",
                                                        i + 1,
                                                        j + 1,
                                                        random_number
                                                    );
                                                }
                                            }
                                        }
                                    }
                                } else if j < 6 {
                                    for l in 6..9 {
                                        for m in 3..6 {
                                            if self.data[l][m] == random_number {
                                                re_k_flag = true;
                                                re_random_flag = true;
                                                if cfg!(debug_assertions) {
                                                    println!(
                                                        "{}行{}列目の数字{}はすでに存在します",
                                                        i + 1,
                                                        j + 1,
                                                        random_number
                                                    );
                                                }
                                            }
                                        }
                                    }
                                } else {
                                    for l in 6..9 {
                                        for m in 6..9 {
                                            if self.data[l][m] == random_number {
                                                re_k_flag = true;
                                                re_random_flag = true;
                                                if cfg!(debug_assertions) {
                                                    println!(
                                                        "{}行{}列目の数字{}はすでに存在します",
                                                        i + 1,
                                                        j + 1,
                                                        random_number
                                                    );
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        if re_i_flag == true && re_j_flag == true
                            || re_i_flag == true && re_k_flag == true
                            || re_j_flag == true && re_k_flag == true
                        {
                            re_i_flag = false;
                            re_j_flag = false;
                            re_k_flag = false;
                            re_generate_flag = true;
                            continue;
                        }
                        if re_random_flag == false {
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
                    println!("完全に再生成します。");
                    if cfg!(debug_assertions) {
                        self.display();
                    }
                    break;
                }
            }
            if re_generate_flag == true {
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

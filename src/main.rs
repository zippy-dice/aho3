use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(help = "デフォルトだとxでアホになるかどうかを調べる")]
    x: u128,
    #[arg(short, long, help = "x番目にアホになる数字を出力")]
    nth: bool,
    #[arg(short, long, help = "1~xにアホになる数字がいくつあるかを出力")]
    count: bool,
}

fn is_aho(x: u128) -> bool {
    x % 3 == 0 || x.to_string().contains("3")
}

struct DP {
    // mem[i][j][k][l] := i桁目まで見て、jがtrueなら桁の制約あり、桁和のmod3がk、lがtrueなら3が含まれる
    mem: Vec<Vec<Vec<Vec<Option<u128>>>>>,
    digits: Vec<u8>,
}

impl DP {
    fn new(x: u128) -> DP {
        let size = x.to_string().len() + 1;
        DP {
            mem: vec![vec![vec![vec![None; 2]; 3]; 2]; size],
            digits: x.to_string().bytes().map(|x| x - b'0').collect(),
        }
    }

    fn rec(&mut self, i: usize, j: usize, k: usize, l: usize) -> u128 {
        if let Some(x) = self.mem[i][j][k][l] {
            return x;
        }

        if i == self.digits.len() {
            return (k == 0 || l == 1) as u128;
        }

        let mut res = 0;
        let lim = if j != 1 { 9 } else { self.digits[i] };
        for x in 0..=lim {
            let nj = if j == 1 && x == lim { 1 } else { 0 };
            let nk = (k + x as usize) % 3;
            let nl = if x == 3 { 1 } else { l };
            res += self.rec(i + 1, nj, nk, nl);
        }

        self.mem[i][j][k][l] = Some(res);
        return res;
    }

    fn solve(x: u128) -> u128 {
        let mut dp = DP::new(x);
        // 0が含まれるので1引く
        dp.rec(0, 1, 0, 0) - 1
    }
}

fn count(x: u128) -> u128 {
    DP::solve(x)
}

fn nth(n: u128) -> u128 {
    let mut lb = 0;
    let mut ub = 3 * n;

    while ub - lb > 1 {
        let mid = (lb + ub) / 2;
        if count(mid) < n {
            lb = mid;
        } else {
            ub = mid;
        }
    }

    ub
}

fn main() {
    let cli = Cli::parse();

    if cli.nth {
        let ans = nth(cli.x);
        println!("{}", ans);
    } else if cli.count {
        let ans = count(cli.x);
        println!("{}", ans);
    } else {
        std::process::exit(!is_aho(cli.x) as i32);
    }
}

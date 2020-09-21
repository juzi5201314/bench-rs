use std::time::Instant;

pub use bencher_macro::*;

#[derive(Debug)]
pub struct Bencher {
    name: String,
    count: usize,
    steps: Vec<Step>,
    pub bytes: usize,
    n: usize
}

impl Bencher {
    pub fn new(name: impl AsRef<str>, count: usize, bytes: usize) -> Self {
        Bencher {
            name: name.as_ref().to_owned(),
            count,
            steps: Vec::with_capacity(count),
            bytes,
            n: 0
        }
    }

    pub fn bench_once<T>(&mut self, f: &mut impl FnMut() -> T, n: usize) -> u128 {
        let now = Instant::now();
        for _ in 0..n {
            f();
        }
        now.elapsed().as_nanos()
    }

    pub fn iter<T>(&mut self, mut f: impl FnMut() -> T) {
        let single = self.bench_once(&mut f, 1);
        // 1_000_000ns : 1ms
        self.n = (1_000_000 / single.max(1)).max(1) as usize;
        self.steps = (0..self.count).map(|_| Step {
            time: self.bench_once(&mut f, self.n) / self.n as u128
        }).collect()
    }

    pub fn finish(&self) {
        let times = self.steps.iter().map(|step| step.time).collect::<Vec<u128>>();
        let iter = times.iter();
        let average = iter.clone().sum::<u128>() / self.count as u128;
        let min = iter.clone().cloned().min().unwrap_or_default();
        let max = iter.clone().cloned().max().unwrap_or_default();
        bunt::println!(
            "{$bg:white+blue+bold}{}{/$} ... {$green}{}{/$} ns/iter (+/- {$red}{}{/$}) = {$#FFA500}{:.2}{/$} MB/s @Total: {} * {} iter",
             &self.name,
             fmt_thousands_sep(average as usize, ','),
             fmt_thousands_sep((max - min) as usize, ','),
             (self.bytes as f64 * (1_000_000_000f64 / average as f64)) / 1000f64 / 1000f64,
             self.count,
             self.n
        );
    }
}

#[derive(Debug)]
pub struct Step {
    time: u128
}

// Format a number with thousands separators
fn fmt_thousands_sep(mut n: usize, sep: char) -> String {
    use std::fmt::Write;
    let mut output = String::new();
    let mut trailing = false;
    for &pow in &[9, 6, 3, 0] {
        let base = 10_usize.pow(pow);
        if pow == 0 || trailing || n / base != 0 {
            if !trailing {
                output.write_fmt(format_args!("{}", n / base)).unwrap();
            } else {
                output.write_fmt(format_args!("{:03}", n / base)).unwrap();
            }
            if pow != 0 {
                output.push(sep);
            }
            trailing = true;
        }
        n %= base;
    }

    output
}
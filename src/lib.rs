pub use bencher::Bencher;
pub use bencher_macro::*;

mod timing_future;
mod bencher;
mod track_allocator;

#[derive(Debug)]
pub struct Stats {
    pub times_average: usize,
    pub times_min: usize,
    pub times_max: usize,

    pub mem_average: usize,
    pub mem_min: usize,
    pub mem_max: usize,
}

#[derive(Debug)]
pub struct Step {
    time: u128,
    mem: usize
}

impl From<&Vec<Step>> for Stats {
    fn from(steps: &Vec<Step>) -> Self {
        let count = steps.len();

        let times = steps.iter().map(|step| step.time).collect::<Vec<u128>>();
        let times_iter = times.iter();

        let mem = steps.iter().map(|step| step.mem).collect::<Vec<usize>>();
        let mem_iter = mem.iter();

        Stats {
            times_average: (times_iter.clone().sum::<u128>() / count as u128) as usize,
            times_min: times_iter.clone().cloned().min().unwrap_or_default() as usize,
            times_max: times_iter.clone().cloned().max().unwrap_or_default() as usize,
            mem_average: mem_iter.clone().sum::<usize>() / count,
            mem_min: mem_iter.clone().cloned().min().unwrap_or_default(),
            mem_max: mem_iter.clone().cloned().max().unwrap_or_default()
        }
    }
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
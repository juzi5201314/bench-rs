use std::future::Future;
use std::time::Instant;

use crate::{GLOBAL, Stats, Step};
use crate::fmt_thousands_sep;
use crate::timing_future::TimingFuture;

pub struct Bencher {
    pub name: String,
    pub count: usize,
    pub steps: Vec<Step>,
    pub bytes: usize,
    pub n: usize,
    pub poll: usize,
    pub format_fn: fn(&Stats, &Bencher),
}

impl Bencher {
    pub fn new(name: impl AsRef<str>, count: usize, bytes: usize) -> Self {
        Bencher {
            name: name.as_ref().to_owned(),
            count,
            steps: Vec::with_capacity(count),
            bytes,
            n: 0,
            poll: 0,
            format_fn: |s, b| Self::default_format(s, b)
        }
    }

    // (time, memory_usage)
    pub fn bench_once<T>(&self, f: &mut impl FnMut() -> T, n: usize) -> (u128, usize) {
        let now = Instant::now();
        GLOBAL.reset();

        for _ in 0..n {
            let _output = f();
        }

        (now.elapsed().as_nanos(), GLOBAL.get())
    }

    pub fn iter<T>(&mut self, mut f: impl FnMut() -> T) {
        let single = self.bench_once(&mut f, 1).0;
        // 1_000_000ns : 1ms
        self.n = (1_000_000 / single.max(1)).max(1) as usize;
        (0..self.count).for_each(|_| {
            let res = self.bench_once(&mut f, self.n);
            self.steps.push(Step {
                time: res.0 / self.n as u128,
                mem: res.1 / self.n
            })
        });
    }

    pub fn async_iter<'a, T, Fut: Future<Output=T>>(&'a mut self, mut f: impl FnMut() -> Fut + 'a) -> impl Future + 'a {
        async move {
            let single = TimingFuture::new(f()).await.elapsed_time.as_nanos();
            // 1_000_000ns : 1ms
            self.n = (1_000_000 / single.max(1)).max(1) as usize;

            let mut polls = Vec::with_capacity(self.count);

            for _ in 0..self.count {
                let mut mtime = 0u128;
                GLOBAL.reset();
                
                for _ in 0..self.n {
                    let tf = TimingFuture::new(f()).await;
                    mtime += tf.elapsed_time.as_nanos();
                    polls.push(tf.poll);
                }

                self.steps.push(Step {
                    time: mtime / self.n as u128,
                    mem: GLOBAL.get() / self.n
                });
            }

            self.poll = polls.iter().sum::<usize>() / polls.len();
        }
    }

    pub fn finish(&self) {
        let stats = Stats::from(&self.steps);
        (self.format_fn)(&stats, self)
    }

    fn default_format(stats: &Stats, bencher: &Bencher) {
        bunt::println!(
            "{[bg:white+blue+bold]} ... {[green]} ns/iter (+/- {[red]}) = {[#FFA500]:.2} MB/s\
            \n\t memory usage: {[green]} bytes/iter (+/- {[red]})\
            \n\t {$bold}{}@Total: {} * {} iters{/$}",
             &bencher.name,
             fmt_thousands_sep(stats.times_average, ','),
             fmt_thousands_sep(stats.times_max - stats.times_min, ','),
             (bencher.bytes as f64 * (1_000_000_000f64 / stats.times_average as f64)) / 1000f64 / 1000f64,

             fmt_thousands_sep(stats.mem_average, ','),
             fmt_thousands_sep(stats.mem_max - stats.mem_min, ','),

             if bencher.poll > 0 {
                format!(
                    "@avg {} polls ",
                    bencher.poll
                 )
             } else {
                String::new()
             },
             bencher.count,
             bencher.n
        );
    }
}

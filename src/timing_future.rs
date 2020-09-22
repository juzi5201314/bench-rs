use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::{Instant, Duration};

use pin_project::pin_project;

pub struct TimingResult<T> {
    pub output: T,
    pub elapsed_time: Duration,
    pub poll: usize
}

#[pin_project]
pub struct TimingFuture<Fut> where Fut: Future {
    #[pin]
    inner: Fut,
    elapsed: Option<Instant>,
    poll: usize
}

impl<Fut> TimingFuture<Fut> where Fut: Future {
    pub fn new(fut: Fut) -> Self {
        TimingFuture {
            inner: fut,
            elapsed: None,
            poll: 0
        }
    }
}

impl<Fut> Future for TimingFuture<Fut> where Fut: Future {
    type Output = TimingResult<Fut::Output>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.project();
        let time = this.elapsed.get_or_insert_with(std::time::Instant::now);

        *this.poll += 1;

        match this.inner.poll(cx) {
            Poll::Pending => Poll::Pending,
            Poll::Ready(t) => {
                Poll::Ready(TimingResult {
                    output: t,
                    elapsed_time: time.elapsed(),
                    poll: *this.poll
                })
            }
        }
    }
}
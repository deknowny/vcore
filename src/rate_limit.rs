extern crate queues;
extern crate time;

use queues::*;
use tokio;

use std::time::SystemTime;


pub struct RateLimitSemaphore<T> {
    calls: Queue<SystemTime>,
    delay: f64
}


impl<T> RateLimitSemaphore<T> {
    fn new(rate: usize, delay: f64) -> Self {
        let queue = queue![];
        for _ in 0..usize {
            queue.add(SystemTime::now());
        }
        RateLimitSemaphore {
            calls: queue,
            delay
        }
    }

    async fn acquire(&mut self) {
        let last_call = self.queue.remove();
        let now = SystemTime::now();
        self.calls.add(now);
        let timedif = now - last_call;
        if timedif < self.delay {
            let need_to_wait = self.delay - timedif;
            tokio::time::sleep(need_to_wait).await;
        }
    }

}



use futures::Stream;
use iced::Subscription;

use super::{Event, worker};

pub fn events() -> Subscription<Event> {
    Subscription::run(stream)
}

fn stream() -> impl Stream<Item = Event> {
    iced::stream::channel(64, |output| async move {
        // presage's receive futures are huge (multi-MB in debug builds); the
        // default 2 MiB thread stack overflows while polling them.
        std::thread::Builder::new()
            .name("signal".into())
            .stack_size(64 * 1024 * 1024)
            .spawn(move || worker::run(output))
            .expect("spawn signal worker thread");
        std::future::pending::<()>().await
    })
}

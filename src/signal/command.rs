use crate::data::Thread;

#[derive(Debug, Clone)]
pub enum Command {
    SendText {
        thread: Thread,
        body: String,
        timestamp: u64,
    },
    LoadThread(Thread),
}

pub extern crate tonic;

pub mod chatroom {
    use std::time::SystemTime;

    tonic::include_proto!("chatroom");

    impl ChatLogEntry {
        pub fn new(msg: Message) -> Self {
            let now = SystemTime::now();
            Self {
                msg: Some(msg),
                timestamp: Some(now.into()),
            }
        }
    }
}

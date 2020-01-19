use e2e_grpc::chatroom::chatroom_client::ChatroomClient;
use e2e_grpc::tonic::transport::Channel;
use neon::prelude::*;
use tokio::runtime::Runtime;

type Client = ChatroomClient<Channel>;

pub struct ClientWrapper {
    client: Client,
}

impl ClientWrapper {
    pub fn new(url: String) -> Self {
        let mut rt = Runtime::new().unwrap();

        Self {
            client: rt.block_on(ChatroomClient::connect(url)).unwrap(),
        }
    }
}

declare_types! {
    /// JS class wrapping Employee records.
    pub class JsChatroomClient for ClientWrapper {

        init(mut cx) {
            let url: String = cx.argument::<JsString>(0)?.value();

            Ok(ClientWrapper::new(url))
        }

        method getIdentity(mut ctx) {
            todo!();
        }

        method getMessages(mut ctx) {
            todo!();
        }

        method createMessage(mut ctx) {
            todo!();
        }
    }
}
register_module!(mut m, {
    m.export_class::<JsChatroomClient>("ChatroomClient")?;
    Ok(())
});

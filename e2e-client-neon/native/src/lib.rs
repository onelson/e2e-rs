use e2e_grpc::chatroom::{
    chatroom_client::ChatroomClient, IdentityCreateRequest, IdentityResponse,
};
use e2e_grpc::tonic::transport::Channel;
use neon::prelude::*;
use tokio::runtime::Runtime;

type Client = ChatroomClient<Channel>;

pub struct ClientWrapper {
    rt: Runtime,
    client: Client,
}

impl ClientWrapper {
    pub fn new(url: String) -> Self {
        let mut rt = Runtime::new().unwrap();
        let client = rt.block_on(ChatroomClient::connect(url)).unwrap();
        Self { rt, client }
    }

    pub fn get_identity(&mut self) -> IdentityResponse {
        self.rt
            .block_on(self.client.get_identity(IdentityCreateRequest {}))
            .unwrap()
            .into_inner()
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
            let mut this = ctx.this();
            let identity = {
                let guard = ctx.lock();
                let resp = this.borrow_mut(&guard).get_identity();
                resp
            };
            Ok(ctx.string(identity.username).upcast())
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

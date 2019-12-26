use e2e_core::{ChatLogEntry, Message, MessageListResponse, TypeScriptifyTrait};

fn main() {
    println!("{}", ChatLogEntry::type_script_ify());
    println!("{}", Message::type_script_ify());
    println!("{}", MessageListResponse::type_script_ify());
}

use crate::{INIT_VALUE, REPLY_REPLY, SEND_REPLY};
use gstd::msg;

#[no_mangle]
extern "C" fn init() {
    msg::send_bytes(msg::source(), [], INIT_VALUE).expect("Failed to send message");
}

#[no_mangle]
extern "C" fn handle() {
    msg::reply(SEND_REPLY, 0).unwrap();
}

#[no_mangle]
extern "C" fn handle_reply() {
    msg::reply(REPLY_REPLY, 0).unwrap();
}

pub mod pb {
    #![allow(dead_code)]
    #![allow(unused_imports)]
    include!(concat!(env!("OUT_DIR"), "/proxy.rs"));
}

pub mod prelude {
    pub use crate::pb::{Entry, EntryType, Message, MessageType};
}

pub mod util {
    use crate::pb::{Entry, EntryType, Message, MessageType};
    pub fn new_msg(msg_type: MessageType, data: &str) -> Message {
        let m = Message {
            msg_type: msg_type as i32,
            entries: vec!(Entry {
                entry_type: EntryType::EntryNormal as i32,
                data: data.as_bytes().to_vec(),
                ..Default::default()
            }),

            ..Default::default()
        };
        m
    }

    pub fn get_entry0(m: Message) -> Option<String> {
        if let Some(e) = m.entries.get(0) {
            if let Ok(ee) = String::from_utf8(e.data.clone()) {
                return Some(ee)
            }
        }
        None
    }

}




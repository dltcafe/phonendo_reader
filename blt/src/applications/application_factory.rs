use crate::ping_pong::PingPong;
use crate::BltApplication;

pub fn get_blt_application(name: &str) -> Option<Box<dyn BltApplication>> {
    match name {
        "PingPong" => Some(Box::new(PingPong::default())),
        _ => None,
    }
}

use std::os::unix::net::UnixStream;

pub mod load;

pub struct Entity {
    path: String,
    connection: UnixStream,
}

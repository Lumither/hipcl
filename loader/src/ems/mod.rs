use std::error::Error;
#[cfg(unix)]
use std::os::unix::net::UnixStream;

#[cfg(windows)]
use uds_windows::UnixStream;

pub mod load;

pub struct Entity {
    path: String,
    connection: Option<UnixStream>,
}

impl Entity {
    pub fn new(entity_path: &str) -> Result<Entity, Box<dyn Error>> {
        todo!()
    }
}

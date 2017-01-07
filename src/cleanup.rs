use std::fs;
use std::ops::Drop;

pub enum Cleanup {
    File(String),
}

impl Drop for Cleanup {
    fn drop(&mut self) {
        match self {
            &mut Cleanup::File(ref path) => {
                match fs::remove_file(path) {
                    Ok(_) => {}
                    Err(e) => {
                        println!("warning: failed to remove file {}: {}", path, e);
                    }
                }
            }
        }
    }
}

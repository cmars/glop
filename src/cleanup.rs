extern crate fs2;

use std;
use std::ops::Drop;

use self::fs2::FileExt;

pub enum Cleanup {
    #[allow(dead_code)]
    Empty,
    File(String),
    #[allow(dead_code)]
    Dir(String),
    Lock(std::fs::File, String),
}

impl Drop for Cleanup {
    fn drop(&mut self) {
        match self {
            &mut Cleanup::Empty => {}
            &mut Cleanup::File(ref path) => {
                match std::fs::remove_file(path) {
                    Ok(_) => {}
                    Err(e) => {
                        warn!("failed to remove file {}: {}", path, e);
                    }
                }
            }
            &mut Cleanup::Dir(ref path) => {
                match std::fs::remove_dir_all(path) {
                    Ok(_) => {}
                    Err(e) => {
                        warn!("failed to remove file {}: {}", path, e);
                    }
                }
            }
            &mut Cleanup::Lock(ref f, ref path) => {
                match f.unlock() {
                    Ok(_) => {}
                    Err(e) => {
                        warn!("failed to unlock file {}: {}", path, e);
                    }
                }
                match std::fs::remove_file(path) {
                    Ok(_) => {}
                    Err(e) => {
                        warn!("failed to remove file {}: {}", path, e);
                    }
                }
            }
        }
    }
}

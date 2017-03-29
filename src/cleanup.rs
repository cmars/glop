use std;
use std::ops::Drop;

pub enum Cleanup {
    #[allow(dead_code)]
    Empty,
    File(String),
    #[allow(dead_code)]
    Dir(String),
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
        }
    }
}

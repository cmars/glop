#![cfg(unix)]

extern crate futures;
extern crate libc;

extern crate tokio_core;
extern crate tokio_signal;

use std::sync::mpsc::channel;
use std::sync::{Once, ONCE_INIT, Mutex, MutexGuard};
use std::thread;

use self::tokio_core::reactor::Core;
use self::tokio_signal::unix::Signal;

static INIT: Once = ONCE_INIT;
static mut LOCK: *mut Mutex<()> = 0 as *mut _;

pub fn lock() -> MutexGuard<'static, ()> {
    unsafe {
        INIT.call_once(|| {
            LOCK = Box::into_raw(Box::new(Mutex::new(())));
            let (tx, rx) = channel();
            thread::spawn(move || {
                let mut lp = Core::new().unwrap();
                let handle = lp.handle();
                let _signal = lp.run(Signal::new(libc::SIGALRM, &handle)).unwrap();
                tx.send(()).unwrap();
                drop(lp.run(futures::empty::<(), ()>()));
            });
            rx.recv().unwrap();
        });
        (*LOCK).lock().unwrap()
    }
}

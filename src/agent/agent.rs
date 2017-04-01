extern crate futures;

use std;

use self::futures::sync::mpsc;

use super::*;

pub struct Agent<S: runtime::Storage> {
    matches: Vec<runtime::Match>,
    st: runtime::State<S>,
    receiver: mpsc::Receiver<Message>,
    match_index: usize,
}

impl<S: runtime::Storage> Agent<S> {
    pub fn new(glop: &ast::Glop,
               st: runtime::State<S>,
               receiver: mpsc::Receiver<Message>)
               -> Result<Agent<S>, Error> {
        let name = st.name().to_string();
        let mut st = st;
        let (seq, _) = st.mut_storage().load()?;
        if seq == 0 {
            st.mut_storage()
                .push_msg(Message {
                    src: "".to_string(),
                    src_role: None,
                    topic: "init".to_string(),
                    dst: name,
                    contents: Obj::new(),
                })?;
        }
        let m_excs = glop.matches
            .iter()
            .map(|m_ast| runtime::Match::new_from_ast(&m_ast))
            .collect::<Vec<_>>();
        Ok(Agent {
            matches: m_excs,
            st: st,
            receiver: receiver,
            match_index: 0,
        })
    }

    fn poll_matches(&mut self) -> futures::Poll<Option<()>, Error> {
        let i = self.match_index % self.matches.len();
        let m = &self.matches[i];
        // TODO: intelligent selection of next match?
        self.match_index = self.match_index + 1;
        let mut txn = match self.st.eval(m.clone()) {
            Ok(Some(txn)) => txn,
            Ok(None) => return Ok(futures::Async::Ready(Some(()))),
            Err(e) => return Err(e),
        };
        // TODO: graceful agent termination (nothing left to do)?
        let result = self.st.commit(&mut txn);
        match result {
            Ok(_) => {}
            Err(e) => {
                error!("transaction seq={} failed: {}", txn.seq, e);
                match self.st.rollback(txn) {
                    Ok(_) => {}
                    Err(e) => return Err(e),
                }
            }
        }
        Ok(futures::Async::Ready(Some(())))
    }
}

impl<S: runtime::Storage> futures::stream::Stream for Agent<S> {
    type Item = ();
    type Error = Error;

    fn poll(&mut self) -> futures::Poll<Option<Self::Item>, Self::Error> {
        // TODO: poll mpsc channel (receiver end) for state changes & apply?
        match self.receiver.poll() {
            Ok(futures::Async::Ready(Some(msg))) => {
                self.st.mut_storage().push_msg(msg)?;
            }
            Ok(futures::Async::Ready(None)) => return Ok(futures::Async::Ready(None)),
            Ok(futures::Async::NotReady) => {}
            Err(_) => return Ok(futures::Async::Ready(None)),
        }
        let result = self.poll_matches();
        std::thread::sleep(std::time::Duration::from_millis(100));
        result
    }
}

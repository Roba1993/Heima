use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, Mutex};

// Internal data for the bus
struct BusData<M: 'static + PartialEq + Clone + Send, I: 'static + PartialEq + Clone + Send> {
    sender_list: Vec<(Sender<BusMessage<M, I>>, Vec<I>)>,
    sender: Sender<BusMessage<M, I>>,
}

#[derive(Clone)]
pub struct Bus<M: 'static + PartialEq + Clone + Send, I: 'static + PartialEq + Clone + Send> {
    data: Arc<Mutex<BusData<M, I>>>,
}

impl<M: 'static + PartialEq + Clone + Send, I: 'static + PartialEq + Clone + Send> Bus<M, I> {
    pub fn new() -> Bus<M, I> {
        // create the default channel
        let (sender, receiver): (Sender<BusMessage<M, I>>, Receiver<BusMessage<M, I>>) = channel();

        // create a new bus-data type
        let bus_data = Arc::new(Mutex::new(BusData {
            sender_list: Vec::new(),
            sender: sender,
        }));

        // copy a reference for the execution thread
        let bus_data_thread = bus_data.clone();

        // start the endless thread to send a received massage to all receivers
        std::thread::spawn(move || {
            // endless loop the handle the data
            loop {
                let res = Bus::reactor_handler_intern(&bus_data_thread, &receiver);
                if res.is_err() {
                    println!("Bus error: {:?}", res);
                }
            }
        });

        // return the bus
        Bus { data: bus_data }
    }

    fn reactor_handler_intern(
        data: &Arc<Mutex<BusData<M, I>>>,
        receiver: &Receiver<BusMessage<M, I>>,
    ) -> Result<(), Error> {
        // receive message from the bus
        let message = receiver.recv()?;

        // get the data
        let mut bus_data = data.lock().map_err(|_| Error::Lock)?;

        // clean up array
        let mut clean = vec![];

        // loop over the senders and multiply the message to each of them
        for (i, (sender, subscription)) in bus_data.sender_list.iter_mut().enumerate() {
            if subscription.contains(&message.ident) {
                // send the message and run clean up code when an error occoured
                if let Err(_) = sender.send(message.clone()) {
                    clean.push(i);
                }
            }
        }

        // loop over the clean up array and remove the senders which sent out of context
        for i in clean {
            remove_item(&mut bus_data.sender_list, i);
        }

        Ok(())
    }

    pub fn channel(
        &self,
        idents: Vec<I>,
    ) -> Result<(Sender<BusMessage<M, I>>, Receiver<BusMessage<M, I>>), Error> {
        // get the bus data
        let mut bus_data = self.data.lock().map_err(|_| Error::Lock)?;

        // create a new channel to communicate with the bus
        let (sender, receiver) = channel();

        // push the new sender to the sender list
        bus_data.sender_list.push((sender, idents));

        // return the bus
        Ok((bus_data.sender.clone(), receiver))
    }
}

/// A generic Bus message.
///
/// The I defines the Ident, to compare if a message should be sent to a specific receiver or not
/// The M defines the Message which get's sent to all receivers
#[derive(PartialEq, Clone)]
pub struct BusMessage<M: 'static + PartialEq + Clone + Send, I: 'static + PartialEq + Clone + Send>
{
    pub message: M,
    pub ident: I,
}

impl<M: 'static + PartialEq + Clone + Send, I: 'static + PartialEq + Clone + Send> BusMessage<M, I> {
    pub fn new(message: M, ident: I) -> Self {
        BusMessage {
            message,
            ident
        }
    }
}

/// Save remove of an object from an array is nightly right now
///
/// Please track this rust commit to check when it's get available to remove this function
/// #40062 remove_item --> Bad Rust, this should be avilable already OOTB ;)
fn remove_item<T>(vec: &mut Vec<T>, index: usize) -> Option<T> {
    match vec.get(index) {
        Some(_) => Some(vec.remove(index)),
        None => None,
    }
}

/// Generic easy Error for the Bus
#[derive(Debug, Clone)]
pub enum Error {
    Bus(String),
    Lock,
    RecvError(std::sync::mpsc::RecvError),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::Bus(s) => write!(f, "{}", s),
            Error::Lock => write!(f, "Lock error"),
            Error::RecvError(_) => write!(f, "Recv Error"),
        }
    }
}

// This is important for other errors to wrap this one.
impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        // Generic error, underlying cause isn't tracked.
        None
    }
}

// from std mpsc recv error
impl From<std::sync::mpsc::RecvError> for Error {
    fn from(err: std::sync::mpsc::RecvError) -> Self {
        Error::RecvError(err)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mapping_one() {
        let bus = Bus::new();

        let (tx1, rx1) = bus.channel(vec!["Channel 1"]).unwrap();
        let (_tx2, rx2) = bus.channel(vec!["Channel 1", "Channel 2"]).unwrap();

        tx1.send(BusMessage::new("Message 1", "Channel 1")).unwrap();
        tx1.send(BusMessage::new("Message 2", "Channel 1")).unwrap();
        tx1.send(BusMessage::new("Message 3", "Channel 2")).unwrap();
        tx1.send(BusMessage::new("Message 4", "Channel 2")).unwrap();

        assert_eq!("Message 1", rx1.recv().unwrap().message);
        assert_eq!("Message 2", rx1.recv().unwrap().message);
        assert_eq!(true, rx1.recv_timeout(std::time::Duration::from_millis(1)).is_err());

        assert_eq!("Message 1", rx2.recv().unwrap().message);
        assert_eq!("Message 2", rx2.recv().unwrap().message);
        assert_eq!("Message 3", rx2.recv().unwrap().message);
        assert_eq!("Message 4", rx2.recv().unwrap().message);
        assert_eq!(true, rx2.recv_timeout(std::time::Duration::from_millis(1)).is_err());
    }
}

use std::sync::{Arc, Mutex};

/// Bus which receives data with a specific ident and sends the message
/// to all receivers which have subscriped for the same ident.
///
/// This bus is build upon the rust OOTB MPSC and combines them to a MPMC where
/// messages get cloned for every receiver who has subscriped for it.
///
/// The internal structure looks something like this:
/// ```compile_fail
/// Sender 1 --->                                     Bus Sender ---> Receiver 1
///                Bus Receiver -> Filter & Clone ->
/// Sender 2 --->                                     Bus Sender ---> Receiver 2
/// ```
#[derive(Clone)]
pub struct Bus<M: 'static + PartialEq + Clone + Send, I: 'static + PartialEq + Clone + Send> {
    data: Arc<Mutex<BusData<M, I>>>,
}

impl<M: 'static + PartialEq + Clone + Send, I: 'static + PartialEq + Clone + Send> Bus<M, I> {
    /// Creates a new bus, which is providing a channel function, to
    /// create new channel to receive and send from and with the bus.
    ///
    /// ```rust
    /// // Create the new bus
    /// use bus::{Bus, BusMessage};
    ///
    /// let bus = Bus::new();
    ///
    /// // Create two new channels
    /// let (Sender1, Receiver1) = bus.channel(vec!["Channel 1"]).unwrap();
    /// let (Sender2, Receiver2) = bus.channel(vec!["Channel 1"]).unwrap();
    ///
    /// // Send a message from each sender
    /// Sender1.send(BusMessage::new("Message 1", "Channel 1")).unwrap();
    /// Sender2.send(BusMessage::new("Message 2", "Channel 1")).unwrap();
    ///
    /// // Check if the messages got received
    /// assert_eq!("Message 1", Receiver1.recv().unwrap().message);
    /// assert_eq!("Message 1", Receiver2.recv().unwrap().message);
    /// assert_eq!("Message 2", Receiver1.recv().unwrap().message);
    /// assert_eq!("Message 2", Receiver2.recv().unwrap().message);
    /// ```
    pub fn new() -> Self {
        // create the default channel
        let (sender, receiver) = std::sync::mpsc::channel();

        // create a new bus-data type
        let bus_data = Arc::new(Mutex::new(BusData {
            sender_list: Vec::new(),
            sender,
        }));

        // copy a reference for the execution thread
        let bus_data_thread = bus_data.clone();

        // start the endless thread to send a received massage to all receivers
        std::thread::spawn(move || {
            // endless loop the handle the data
            loop {
                let res = Bus::bus_handler_intern(&bus_data_thread, &receiver);
                if res.is_err() {
                    println!("Bus error: {:?}", res);
                }
            }
        });

        // return the bus
        Bus { data: bus_data }
    }

    /// Internal bus handler which runs within the thread and
    /// performs the filter and copy tasks for each message
    fn bus_handler_intern(
        data: &Arc<Mutex<BusData<M, I>>>,
        receiver: &BusReceiver<M, I>,
    ) -> Result<()> {
        // receive message from the bus
        let message = receiver.recv()?;

        // get the data
        let mut bus_data = data.lock().map_err(|_| BusError::Lock)?;

        // clean up array
        let mut clean = vec![];

        // loop over the senders and multiply the message to each of them
        for (i, (sender, subscription)) in bus_data.sender_list.iter_mut().enumerate() {
            if subscription.contains(&message.ident) {
                // send the message and run clean up code when an error occoured
                if sender.send(message.clone()).is_err() {
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

    /// Create a new channel to and from the bus, returning a Sender and Receiver.
    /// All data sendet to the Sender get availble to all Receivers which have provided
    /// the same ident.
    ///
    /// ```rust
    /// use bus::{Bus, BusMessage};
    ///
    /// // Create the new bus
    /// let bus = Bus::new();
    ///
    /// // Create two new channels
    /// let (Sender1, Receiver1) = bus.channel(vec!["Channel 1"]).unwrap();
    /// let (Sender2, Receiver2) = bus.channel(vec!["Channel 1", "Channel 2"]).unwrap();
    ///
    /// // Send a message from each sender
    /// Sender1.send(BusMessage::new("Message 1", "Channel 1")).unwrap();
    /// Sender2.send(BusMessage::new("Message 2", "Channel 2")).unwrap();
    ///
    /// // Check if the messages got received
    /// assert_eq!("Message 1", Receiver1.recv().unwrap().message);
    /// assert_eq!("Message 1", Receiver2.recv().unwrap().message);
    /// assert_eq!("Message 2", Receiver2.recv().unwrap().message);
    /// ```
    pub fn channel(&self, idents: Vec<I>) -> Result<(BusSender<M, I>, BusReceiver<M, I>)> {
        // get the bus data
        let mut bus_data = self.data.lock().map_err(|_| BusError::Lock)?;

        // create a new channel to communicate with the bus
        let (sender, receiver) = std::sync::mpsc::channel();

        // push the new sender to the sender list
        bus_data.sender_list.push((sender, idents));

        // return the bus
        Ok((bus_data.sender.clone(), receiver))
    }
}

impl<M: 'static + PartialEq + Clone + Send, I: 'static + PartialEq + Clone + Send> Default
    for Bus<M, I>
{
    fn default() -> Self {
        Bus::new()
    }
}

/// Internal data for the bus
struct BusData<M: 'static + PartialEq + Clone + Send, I: 'static + PartialEq + Clone + Send> {
    sender_list: Vec<(BusSender<M, I>, Vec<I>)>,
    sender: BusSender<M, I>,
}

/// Custom type definition for a sender for BusMessages
pub type BusSender<M, I> = std::sync::mpsc::Sender<BusMessage<M, I>>;

/// Custom type definition for a receiver for BusMessages
pub type BusReceiver<M, I> = std::sync::mpsc::Receiver<BusMessage<M, I>>;

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

impl<M: 'static + PartialEq + Clone + Send, I: 'static + PartialEq + Clone + Send>
    BusMessage<M, I>
{
    /// Creates a new `BusMessage`, which is holding the message itself and the definition
    /// which is defining the filtering. Every channel who has subscribed to the same
    /// ident, will receive the the `BusMessage` when send to the bus.
    ///
    /// ```rust
    /// // Create the new bus
    /// use bus::BusMessage;
    ///
    /// BusMessage::new("Message 1", "Channel 1");
    /// ```
    pub fn new(message: M, ident: I) -> Self {
        BusMessage { message, ident }
    }
}

/// Save remove of an object from an vec is nightly right now
///
/// Please track this rust commit to check when it's get available to remove this function
/// #40062 remove_item --> Bad Rust, this should be avilable already OOTB ;)
fn remove_item<T>(vec: &mut Vec<T>, index: usize) -> Option<T> {
    match vec.get(index) {
        Some(_) => Some(vec.remove(index)),
        None => None,
    }
}

/// Custom type definition for Result with a BusError
pub type Result<R> = std::result::Result<R, BusError>;

/// Generic easy Error for the Bus
#[derive(Debug, Clone)]
pub enum BusError {
    Lock,
    RecvError(std::sync::mpsc::RecvError),
}

impl std::fmt::Display for BusError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            BusError::Lock => write!(f, "Lock error"),
            BusError::RecvError(_) => write!(f, "Recv Error"),
        }
    }
}

// This is important for other errors to wrap this one.
impl std::error::Error for BusError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        // Generic error, underlying cause isn't tracked.
        None
    }
}

// from std mpsc recv error
impl From<std::sync::mpsc::RecvError> for BusError {
    fn from(err: std::sync::mpsc::RecvError) -> Self {
        BusError::RecvError(err)
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
        assert_eq!(
            true,
            rx1.recv_timeout(std::time::Duration::from_millis(1))
                .is_err()
        );

        assert_eq!("Message 1", rx2.recv().unwrap().message);
        assert_eq!("Message 2", rx2.recv().unwrap().message);
        assert_eq!("Message 3", rx2.recv().unwrap().message);
        assert_eq!("Message 4", rx2.recv().unwrap().message);
        assert_eq!(
            true,
            rx2.recv_timeout(std::time::Duration::from_millis(1))
                .is_err()
        );
    }
}

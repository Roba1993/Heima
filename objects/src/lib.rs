/// Ident enum for the Messages used within Heima
///
/// It allows to receive all messages, no messages at all
/// or just matching ones by string.
pub enum Ident {
    All,
    None,
    Name(String),
}

impl PartialEq for Ident {
    /// PartialEq implementation for the ident.
    ///
    /// None is always returning false
    /// All is returning true against All and Name
    /// Name is returning true when compared against matching name or All
    fn eq(&self, other: &Self) -> bool {
        // match operator on a tuple - Rust is just awesome
        match (self, other) {
            (Ident::All, Ident::All) => true,
            (Ident::All, Ident::Name(_)) => true,
            (Ident::Name(_), Ident::All) => true,
            (Ident::Name(a), Ident::Name(b)) => a == b,
            _ => false,
        }
    }
}

/// A device which holds multiple activities which a providing
/// specific functionalities.
pub struct Device {
    pub id: u32,
    pub name: String,
    pub activities: Vec<DeviceActivity>,
}

/// The device activity defines, what kind of action it is capable
/// of and holds the actual value with the information when it was set.
pub struct DeviceActivity {
    pub id: u32,
    pub name: String,
    pub action: DeviceAction,
    pub value: Value,
    pub value_time: u32,
}

/// The device action defines what functionality a device supports.
pub enum DeviceAction {
    Light,
    LightDim,
    LightColor,
    ShadowOpen,
    ShadowAngle,
    Temperature,
}

/// The actual value store
pub enum Value {
    Light(Light),
    Temperature(Temperature),
}

/// The temperatur which can be set
pub enum Temperature {
    Celsius(f32),
    Fahrenheit(f32),
}

/// The light which can be set
pub struct Light {
    pub hue: f32,
    pub saturation: f32,
    pub lightness: f32,
}

/// A room which holds multiple devices
/// and has a name to group them.
pub struct Room {
    pub id: u32,
    pub name: String,
    pub devices: Vec<u8>,
}

/// A Appartement combines multiple rooms and
/// allows to control different areas or houses.
pub struct Appartement {
    pub id: u32,
    pub name: String,
    pub rooms: Vec<u8>,
}

/// The general Message, which is
/// internally a specific one.
pub enum Message {
    DeviceValueUpdate(DeviceValueUpdate),
    AddDeviceMessage(AddDeviceMessage),
    RemoveDeviceMessage(RemoveDeviceMessage),
    RenameDeviceMessage(RenameDeviceMessage),
    AllRoomsMessage(AllRoomsMessage),
    AddRoomMessage(AddRoomMessage),
    RenameRoomMessage(RenameRoomMessage),
    RemoveRoomMessage(RemoveRoomMessage),
    RoomAddDevicesMessage(RoomAddDevicesMessage),
    RoomRemoveDevicesMessage(RoomRemoveDevicesMessage),
    AllAppartementsMessage(AllAppartementsMessage),
    AppartementAddRoomMessage(AppartementAddRoomMessage),
    AppartementRemoveRoomMessage(AppartementRemoveRoomMessage),
}

/// Specific message to update a device value
pub struct DeviceValueUpdate {
    pub timestamp: u32,
    pub device_id: u32,
    pub device_activity_id: u32,
    pub value: Value,
    pub trigger: String,
}

/// Specific message to add a device to the system
///
/// This enables, that device get listet in the structure etc.
pub struct AddDeviceMessage {
    pub timestamp: u32,
    pub device: Device,
}

/// Specific message to remove a message
pub struct RemoveDeviceMessage {
    pub timestamp: u32,
    pub device_id: u32,
}

/// Specific message to rename a device
pub struct RenameDeviceMessage {
    pub timestamp: u32,
    pub device_id: u32,
    pub name: String,
}

/// Specific message to provide all rooms available
pub struct AllRoomsMessage {
    pub timestamp: u32,
    pub rooms: Vec<Room>,
}

/// Specific message to add a room to the structure
pub struct AddRoomMessage {
    pub timestamp: u32,
    pub room: Room,
}

/// Specific message to rename a room
pub struct RenameRoomMessage {
    pub timestamp: u32,
    pub room_id: u32,
    pub name: String,
}

/// Specific message to remove a room
pub struct RemoveRoomMessage {
    pub timestamp: u32,
    pub room_id: u32,
}

/// Specific message to add a device to a room
pub struct RoomAddDevicesMessage {
    pub timestamp: u32,
    pub room_id: u32,
    pub device_id: u32,
}

/// Specific message to remove device from a room
pub struct RoomRemoveDevicesMessage {
    pub timestamp: u32,
    pub room_id: u32,
    pub device_id: u32,
}

/// Specific message with all appartments
pub struct AllAppartementsMessage {
    pub timestamp: u32,
    pub rooms: Vec<Appartement>,
}

/// Specific message to add a room to a appartment
pub struct AppartementAddRoomMessage {
    pub timestamp: u32,
    pub appartement_id: u32,
    pub room_id: u32,
}

/// Specific message to remove a room from a appartment
pub struct AppartementRemoveRoomMessage {
    pub timestamp: u32,
    pub appartement_id: u32,
    pub room_id: u32,
}
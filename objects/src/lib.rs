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
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Device {
    pub id: u32,
    pub name: String,
    pub activities: Vec<DeviceActivity>,
}

/// The device activity defines, what kind of action it is capable
/// of and holds the actual value with the information when it was set.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DeviceActivity {
    pub id: u32,
    pub name: String,
    pub action: DeviceAction,
    pub value: Value,
    pub value_time: u32,
}

/// The device action defines what functionality a device supports.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum DeviceAction {
    Light,
    LightDim,
    LightColor,
    ShadowOpen,
    ShadowAngle,
    Temperature,
}

/// The actual value store
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum Value {
    Light(Light),
    Temperature(Temperature),
}

/// The temperatur which can be set
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum Temperature {
    Celsius(f32),
    Fahrenheit(f32),
}

/// The light which can be set
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Light {
    pub hue: f32,
    pub saturation: f32,
    pub lightness: f32,
}

/// A room which holds multiple devices
/// and has a name to group them.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Room {
    pub id: u32,
    pub name: String,
    pub devices: Vec<u8>,
}

/// A Appartement combines multiple rooms and
/// allows to control different areas or houses.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Appartement {
    pub id: u32,
    pub name: String,
    pub rooms: Vec<u8>,
}

/// The general Message, which is
/// internally a specific one.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
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
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
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
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AddDeviceMessage {
    pub timestamp: u32,
    pub device: Device,
}

/// Specific message to remove a message
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RemoveDeviceMessage {
    pub timestamp: u32,
    pub device_id: u32,
}

/// Specific message to rename a device
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RenameDeviceMessage {
    pub timestamp: u32,
    pub device_id: u32,
    pub name: String,
}

/// Specific message to provide all rooms available
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AllRoomsMessage {
    pub timestamp: u32,
    pub rooms: Vec<Room>,
}

/// Specific message to add a room to the structure
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AddRoomMessage {
    pub timestamp: u32,
    pub room: Room,
}

/// Specific message to rename a room
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RenameRoomMessage {
    pub timestamp: u32,
    pub room_id: u32,
    pub name: String,
}

/// Specific message to remove a room
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RemoveRoomMessage {
    pub timestamp: u32,
    pub room_id: u32,
}

/// Specific message to add a device to a room
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RoomAddDevicesMessage {
    pub timestamp: u32,
    pub room_id: u32,
    pub device_id: u32,
}

/// Specific message to remove device from a room
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RoomRemoveDevicesMessage {
    pub timestamp: u32,
    pub room_id: u32,
    pub device_id: u32,
}

/// Specific message with all appartments
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AllAppartementsMessage {
    pub timestamp: u32,
    pub rooms: Vec<Appartement>,
}

/// Specific message to add a room to a appartment
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AppartementAddRoomMessage {
    pub timestamp: u32,
    pub appartement_id: u32,
    pub room_id: u32,
}

/// Specific message to remove a room from a appartment
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AppartementRemoveRoomMessage {
    pub timestamp: u32,
    pub appartement_id: u32,
    pub room_id: u32,
}
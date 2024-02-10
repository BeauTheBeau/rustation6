use serde::{Deserialize, Serialize};
use crate::schemas::items::Item;


/// CommandUsed struct represents a command used by a user in the system, intended for use with the analytics.
///
/// The CommandUsed struct contains the following fields:
///
/// - `id`: Represents the unique identifier of the command.
/// - `command`: Represents the command used by the user.
/// - `timestamp`: Represents the timestamp when the command was used.
///
/// # Examples
///
/// ```
/// use serde::{Deserialize, Serialize};
///
/// #[derive(Serialize, Deserialize, Debug)]
/// pub struct CommandUsed {
///     pub id: u64,
///     pub command: String,
///     pub timestamp: i64,
/// }
/// ```
#[derive(Serialize, Deserialize, Debug)]
pub struct CommandUsed {
    pub id: u64,
    pub command: String,
    pub timestamp: i64,
}

/// User struct represents a user in the system.
///
/// The User struct contains the following fields:
///
/// - `id`: Represents the unique identifier of the user.
/// - `xp`: Represents the experience points of the user.
/// - `messages_sent`: Represents the number of messages sent by the user.
/// - `last_message`: Represents the timestamp of the last message sent by the user.
/// - `balance`: Represents the amount of currency the user has in the bank.
/// - `cash`: Represents the amount of currency the user has on hand.
/// - `inventory`: Represents the items owned by the user.
/// - `equipped`: Represents the items equipped by the user.
/// - `last_command_list`: Represents the list of commands used and when they were used by the user.
///
/// The fields related to leveling, currency, and analytics are included for possible future use.
///
/// # Examples
///
/// ```
/// use serde::{Deserialize, Serialize};
///
/// #[derive(Serialize, Deserialize, Debug)]
/// pub struct User {
///     pub id: u64,
///     pub xp: i32,
///     pub messages_sent: i32,
///     pub last_message: i64,
///     pub balance: i32,
///     pub cash: i32,
///     pub inventory: Vec<Item>,
///     pub equipped: Vec<Item>,
///     pub last_command_list: Vec<CommandUsed>,
/// }
/// ```
#[derive(Serialize, Deserialize, Debug)]
pub struct User {

    pub id: u64,

    // for leveling
    //      there is no explicit level field as it is derived from xp
    pub xp: i32,
    pub messages_sent: i32,
    pub last_message: i64,

    // for currency
    //      balance is the amount of currency the user has in the bank, cash is the amount of currency
    //      the user has on hand
    pub balance: i32,
    pub cash: i32,

    // for inventory
    pub inventory: Vec<Item>,
    pub equipped: Vec<Item>,

    // for analytics
    //      these fields are not used in the current implementation
    //      they are included for possible future use
    pub last_command_list: Vec<CommandUsed>,


}

impl User {
    pub fn new(id: u64) -> User {
        User {
            id,
            xp: 0,
            messages_sent: 0,
            last_message: 0,
            balance: 0,
            cash: 0,
            inventory: Vec::new(),
            equipped: Vec::new(),
            last_command_list: Vec::new(),
        }
    }
}
use serde::{Serialize, de::DeserializeOwned};

pub trait ReqPacket: DeserializeOwned + Serialize + Send + Sync + 'static {}

pub trait ResPacket: DeserializeOwned + Serialize + Send + Sync + 'static {}

pub mod room;

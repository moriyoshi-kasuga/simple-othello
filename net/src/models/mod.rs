use serde::{Serialize, de::DeserializeOwned};

pub mod login;

pub trait ReqPacket: DeserializeOwned + Serialize + Send + Sync + 'static {}

impl<T> ReqPacket for T where T: DeserializeOwned + Serialize + Send + Sync + 'static {}

pub trait ResPacket: DeserializeOwned + Serialize + Send + Sync + 'static {}

impl<T> ResPacket for T where T: DeserializeOwned + Serialize + Send + Sync + 'static {}

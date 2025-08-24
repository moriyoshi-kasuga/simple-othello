use serde::{Serialize, de::DeserializeOwned};

pub mod models;

pub mod request;
pub mod response;

pub trait Packet: DeserializeOwned + Serialize + Send + Sync + 'static {
    const PACKET_ID: u8;

    fn encode(&self) -> Option<Vec<u8>> {
        let mut buf = vec![Self::PACKET_ID];
        let json = serde_json::to_vec(self).ok()?;
        buf.extend(json);
        Some(buf)
    }

    /// Decode from a buffer with packet id
    fn decode(buf: &[u8]) -> Option<Self> {
        if buf.is_empty() {
            return None;
        }
        let id = buf[0];
        Self::decode_by_id(id, &buf[1..])
    }

    fn decode_by_id(id: u8, buf: &[u8]) -> Option<Self> {
        if id != Self::PACKET_ID {
            return None;
        }
        Self::decode_raw(buf)
    }

    /// Decode from a buffer without packet id
    /// If you want to decode from a buffer with packet id, use [`Packet::decode`]
    fn decode_raw(buf: &[u8]) -> Option<Self> {
        serde_json::from_slice(buf).ok()
    }
}

pub trait ReqPacket: Packet {}

pub trait ResPacket: Packet {}

macro_rules! definition_packet {
    (@res $name:ident) => {
        impl $crate::ResPacket for $name {}
    };
    (@req $name:ident) => {
        impl $crate::ReqPacket for $name {}
    };
    (@impl
        $(#[$struct_attr:meta])*
        $struct_pub:ident $struct:ident $name:ident {
            $($tt:tt)*
        }
    ) => {
        #[derive(Clone, serde::Deserialize, serde::Serialize)]
        $struct_pub $struct $name {
            $($tt)*
        }
    };
    (
        #[res]
        $(#[$struct_attr:meta])*
        $struct_pub:ident $struct:ident $name:ident {
            $($tt:tt)*
        }
    ) => {
        definition_packet!(@impl
        $(#[$struct_attr])*
        $struct_pub $struct $name {
            $($tt)*
        });

        definition_packet!(@res $name);
    };
    (
        #[req]
        $(#[$struct_attr:meta])*
        $struct_pub:ident $struct:ident $name:ident {
            $($tt:tt)*
        }
    ) => {
        definition_packet!(@impl
        $(#[$struct_attr])*
        $struct_pub $struct $name {
            $($tt)*
        });

        definition_packet!(@req $name);
    };
    ($(
        #[$target:ident]
        $(#[$struct_attr:meta])*
        $struct_pub:ident $struct:ident $name:ident {
            $($tt:tt)*
        }
    )+) => {
        $(
            definition_packet!(
                #[$target]
                $(#[$struct_attr])*
                $struct_pub $struct $name {
                    $($tt)*
                }
            );
        )+
    };
}

macro_rules! definition_packets {
    (
        $(#[$enum_attr:meta])*
        $pub:ident $enum:ident $name:ident {
            $(
                $(#[$variant_attr:meta])*
                $variant:ident($ty:ident) = $id:literal,
            )*
        }
    ) => {
        $(#[$enum_attr])*
        $pub $enum $name {
            $($(#[$variant_attr])*
            $variant($ty)),*
        }

        impl $name {
            pub const fn id(&self) -> u8 {
                match self {
                    $(Self::$variant(_) => $id),*
                }
            }

            pub fn encode(&self) -> Option<Vec<u8>> {
                use $crate::Packet;

                match self {
                    $(Self::$variant(v) => v.encode()),*
                }
            }

            pub fn decode(buf: &[u8]) -> Option<Self> {
                if buf.is_empty() {
                    return None;
                }
                let id = buf[0];
                Self::decode_by_id(id, &buf[1..])
            }

            pub fn decode_by_id(id: u8, buf: &[u8]) -> Option<Self> {
                use $crate::Packet;

                match id {
                    $($id => {
                        let v: $ty = $ty::decode_raw(buf)?;
                        Some(Self::$variant(v))
                    }),*,
                    _ => None,
                }
            }
        }

        $(
            impl From<$ty> for $name {
                #[inline]
                fn from(value: $ty) -> Self {
                    Self::$variant(value)
                }
            }

            impl $crate::Packet for $ty {
                const PACKET_ID: u8 = $id;
            }
        )*
    };
}

pub(crate) use definition_packet;
pub(crate) use definition_packets;

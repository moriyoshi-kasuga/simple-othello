pub mod login;
pub mod models;

pub mod request;
pub mod response;

macro_rules! definition_packet {
    (@res $name:ident) => {
        impl $crate::models::ResPacket for $name {}
    };
    (@req $name:ident) => {
        impl $crate::models::ReqPacket for $name {}
    };
    (
        #[res]
        $(#[$struct_attr:meta])*
        $struct_pub:ident $struct:ident $name:ident {
            $($tt:tt)*
        }
    ) => {
        $(#[$struct_attr])*
        #[derive(serde::Deserialize, serde::Serialize)]
        $struct_pub $struct $name {
            $($tt)*
        }

        definition_packet!(@res $name);
    };
    (
        #[req]
        $(#[$struct_attr:meta])*
        $struct_pub:ident $struct:ident $name:ident {
            $($tt:tt)*
        }
    ) => {
        $(#[$struct_attr])*
        #[derive(serde::Deserialize, serde::Serialize)]
        $struct_pub $struct $name {
            $($tt)*
        }

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
                $variant:ident($ty:ty) = $id:literal,
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
                let mut buf = vec![self.id()];
                let json = match self {
                    $(Self::$variant(v) => serde_json::to_vec(v)),*
                };
                let json = json.ok()?;
                buf.extend(json);
                Some(buf)
            }

            pub fn decode(buf: &[u8]) -> Option<Self> {
                if buf.is_empty() {
                    return None;
                }
                let id = buf[0];
                Self::decode_by_id(id, &buf[1..])
            }

            pub fn decode_by_id(id: u8, buf: &[u8]) -> Option<Self> {
                match id {
                    $($id => {
                        let v: $ty = serde_json::from_slice(buf).ok()?;
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
        )*
    };
}

pub(crate) use definition_packet;
pub(crate) use definition_packets;

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
            $($field_pub:ident $(#[$field_attr:meta])* $field:ident: $ty:ty),* $(,)*
        }
    ) => {
        $(#[$struct_attr])*
        #[derive(serde::Deserialize, serde::Serialize)]
        $struct_pub $struct $name {
            $($field_pub $(#[$field_attr])* $field: $ty),*
        }

        definition_packet!(@res $name);
    };
    (
        #[req]
        $(#[$struct_attr:meta])*
        $struct_pub:ident $struct:ident $name:ident {
            $($field_pub:ident $(#[$field_attr:meta])* $field:ident: $ty:ty),* $(,)*
        }
    ) => {
        $(#[$struct_attr])*
        #[derive(serde::Deserialize, serde::Serialize)]
        $struct_pub $struct $name {
            $($field_pub $(#[$field_attr])* $field: $ty),*
        }

        definition_packet!(@req $name);
    };
    ($(
        #[$target:ident]
        $(#[$struct_attr:meta])*
        $struct_pub:ident $struct:ident $name:ident {
            $($field_pub:ident $(#[$field_attr:meta])* $field:ident: $ty:ty),* $(,)*
        }
    )+) => {
        $(
            definition_packet!(
                #[$target]
                $(#[$struct_attr])*
                $struct_pub $struct $name {
                    $($field_pub $(#[$field_attr])* $field: $ty),*
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

            pub fn decode(id: u8, buf: &[u8]) -> Option<Self> {
                match id {
                    $($id => {
                        let v: $ty = serde_json::from_slice(buf).ok()?;
                        Some(Self::$variant(v))
                    }),*,
                    _ => None,
                }
            }
        }
    };
}

pub(crate) use definition_packet;
pub(crate) use definition_packets;

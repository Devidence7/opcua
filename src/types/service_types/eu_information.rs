// OPCUA for Rust
// SPDX-License-Identifier: MPL-2.0
// Copyright (C) 2017-2022 Adam Lock
//
// This file was autogenerated from Opc.Ua.Types.bsd by tools/schema/gen_types.js
//
// DO NOT EDIT THIS FILE
#![allow(unused_attributes)]
use std::io::{Read, Write};
#[allow(unused_imports)]
use crate::{
    encoding::*,
    basic_types::*,
    service_types::impls::MessageInfo,
    node_ids::ObjectId,
    string::UAString,
    localized_text::LocalizedText,
};

#[derive(Debug, Clone, PartialEq)]
pub struct EUInformation {
    pub namespace_uri: UAString,
    pub unit_id: i32,
    pub display_name: LocalizedText,
    pub description: LocalizedText,
}

impl MessageInfo for EUInformation {
    fn object_id(&self) -> ObjectId {
        ObjectId::EUInformation_Encoding_DefaultBinary
    }
}

impl BinaryEncoder<EUInformation> for EUInformation {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += self.namespace_uri.byte_len();
        size += self.unit_id.byte_len();
        size += self.display_name.byte_len();
        size += self.description.byte_len();
        size
    }

    #[allow(unused_variables)]
    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += self.namespace_uri.encode(stream)?;
        size += self.unit_id.encode(stream)?;
        size += self.display_name.encode(stream)?;
        size += self.description.encode(stream)?;
        Ok(size)
    }

    #[allow(unused_variables)]
    fn decode<S: Read>(stream: &mut S, decoding_options: &DecodingOptions) -> EncodingResult<Self> {
        let namespace_uri = UAString::decode(stream, decoding_options)?;
        let unit_id = i32::decode(stream, decoding_options)?;
        let display_name = LocalizedText::decode(stream, decoding_options)?;
        let description = LocalizedText::decode(stream, decoding_options)?;
        Ok(EUInformation {
            namespace_uri,
            unit_id,
            display_name,
            description,
        })
    }
}
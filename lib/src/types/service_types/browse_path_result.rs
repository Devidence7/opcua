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
use crate::types::{
    encoding::*,
    basic_types::*,
    service_types::impls::MessageInfo,
    node_ids::ObjectId,
    status_codes::StatusCode,
    service_types::BrowsePathTarget,
};

#[derive(Debug, Clone, PartialEq)]
pub struct BrowsePathResult {
    pub status_code: StatusCode,
    pub targets: Option<Vec<BrowsePathTarget>>,
}

impl MessageInfo for BrowsePathResult {
    fn object_id(&self) -> ObjectId {
        ObjectId::BrowsePathResult_Encoding_DefaultBinary
    }
}

impl BinaryEncoder<BrowsePathResult> for BrowsePathResult {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += self.status_code.byte_len();
        size += byte_len_array(&self.targets);
        size
    }

    #[allow(unused_variables)]
    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += self.status_code.encode(stream)?;
        size += write_array(stream, &self.targets)?;
        Ok(size)
    }

    #[allow(unused_variables)]
    fn decode<S: Read>(stream: &mut S, decoding_options: &DecodingOptions) -> EncodingResult<Self> {
        let status_code = StatusCode::decode(stream, decoding_options)?;
        let targets: Option<Vec<BrowsePathTarget>> = read_array(stream, decoding_options)?;
        Ok(BrowsePathResult {
            status_code,
            targets,
        })
    }
}
// @generated
// This file is @generated by prost-build.
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct PaginationRequest {
    #[prost(uint32, tag="1")]
    pub page: u32,
    #[prost(uint32, tag="2")]
    pub size: u32,
}
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct PaginationResponse {
    #[prost(uint32, tag="1")]
    pub page: u32,
    #[prost(uint32, tag="2")]
    pub size: u32,
    #[prost(uint32, tag="3")]
    pub total_elements: u32,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Direction {
    Unspecified = 0,
    Asc = 1,
    Desc = 2,
}
impl Direction {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Self::Unspecified => "DIRECTION_UNSPECIFIED",
            Self::Asc => "DIRECTION_ASC",
            Self::Desc => "DIRECTION_DESC",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "DIRECTION_UNSPECIFIED" => Some(Self::Unspecified),
            "DIRECTION_ASC" => Some(Self::Asc),
            "DIRECTION_DESC" => Some(Self::Desc),
            _ => None,
        }
    }
}
/// Encoded file descriptor set for the `sited_io.types.query.v1` package
pub const FILE_DESCRIPTOR_SET: &[u8] = &[
    0x0a, 0xd3, 0x07, 0x0a, 0x23, 0x73, 0x69, 0x74, 0x65, 0x64, 0x5f, 0x69, 0x6f, 0x2f, 0x74, 0x79,
    0x70, 0x65, 0x73, 0x2f, 0x71, 0x75, 0x65, 0x72, 0x79, 0x2f, 0x76, 0x31, 0x2f, 0x71, 0x75, 0x65,
    0x72, 0x79, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x17, 0x73, 0x69, 0x74, 0x65, 0x64, 0x5f,
    0x69, 0x6f, 0x2e, 0x74, 0x79, 0x70, 0x65, 0x73, 0x2e, 0x71, 0x75, 0x65, 0x72, 0x79, 0x2e, 0x76,
    0x31, 0x22, 0x3b, 0x0a, 0x11, 0x50, 0x61, 0x67, 0x69, 0x6e, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x52,
    0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x12, 0x0a, 0x04, 0x70, 0x61, 0x67, 0x65, 0x18, 0x01,
    0x20, 0x01, 0x28, 0x0d, 0x52, 0x04, 0x70, 0x61, 0x67, 0x65, 0x12, 0x12, 0x0a, 0x04, 0x73, 0x69,
    0x7a, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x04, 0x73, 0x69, 0x7a, 0x65, 0x22, 0x63,
    0x0a, 0x12, 0x50, 0x61, 0x67, 0x69, 0x6e, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x52, 0x65, 0x73, 0x70,
    0x6f, 0x6e, 0x73, 0x65, 0x12, 0x12, 0x0a, 0x04, 0x70, 0x61, 0x67, 0x65, 0x18, 0x01, 0x20, 0x01,
    0x28, 0x0d, 0x52, 0x04, 0x70, 0x61, 0x67, 0x65, 0x12, 0x12, 0x0a, 0x04, 0x73, 0x69, 0x7a, 0x65,
    0x18, 0x02, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x04, 0x73, 0x69, 0x7a, 0x65, 0x12, 0x25, 0x0a, 0x0e,
    0x74, 0x6f, 0x74, 0x61, 0x6c, 0x5f, 0x65, 0x6c, 0x65, 0x6d, 0x65, 0x6e, 0x74, 0x73, 0x18, 0x03,
    0x20, 0x01, 0x28, 0x0d, 0x52, 0x0d, 0x74, 0x6f, 0x74, 0x61, 0x6c, 0x45, 0x6c, 0x65, 0x6d, 0x65,
    0x6e, 0x74, 0x73, 0x2a, 0x4d, 0x0a, 0x09, 0x44, 0x69, 0x72, 0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e,
    0x12, 0x19, 0x0a, 0x15, 0x44, 0x49, 0x52, 0x45, 0x43, 0x54, 0x49, 0x4f, 0x4e, 0x5f, 0x55, 0x4e,
    0x53, 0x50, 0x45, 0x43, 0x49, 0x46, 0x49, 0x45, 0x44, 0x10, 0x00, 0x12, 0x11, 0x0a, 0x0d, 0x44,
    0x49, 0x52, 0x45, 0x43, 0x54, 0x49, 0x4f, 0x4e, 0x5f, 0x41, 0x53, 0x43, 0x10, 0x01, 0x12, 0x12,
    0x0a, 0x0e, 0x44, 0x49, 0x52, 0x45, 0x43, 0x54, 0x49, 0x4f, 0x4e, 0x5f, 0x44, 0x45, 0x53, 0x43,
    0x10, 0x02, 0x42, 0xa4, 0x01, 0x0a, 0x1b, 0x63, 0x6f, 0x6d, 0x2e, 0x73, 0x69, 0x74, 0x65, 0x64,
    0x5f, 0x69, 0x6f, 0x2e, 0x74, 0x79, 0x70, 0x65, 0x73, 0x2e, 0x71, 0x75, 0x65, 0x72, 0x79, 0x2e,
    0x76, 0x31, 0x42, 0x0a, 0x51, 0x75, 0x65, 0x72, 0x79, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x50, 0x01,
    0xa2, 0x02, 0x03, 0x53, 0x54, 0x51, 0xaa, 0x02, 0x16, 0x53, 0x69, 0x74, 0x65, 0x64, 0x49, 0x6f,
    0x2e, 0x54, 0x79, 0x70, 0x65, 0x73, 0x2e, 0x51, 0x75, 0x65, 0x72, 0x79, 0x2e, 0x56, 0x31, 0xca,
    0x02, 0x16, 0x53, 0x69, 0x74, 0x65, 0x64, 0x49, 0x6f, 0x5c, 0x54, 0x79, 0x70, 0x65, 0x73, 0x5c,
    0x51, 0x75, 0x65, 0x72, 0x79, 0x5c, 0x56, 0x31, 0xe2, 0x02, 0x22, 0x53, 0x69, 0x74, 0x65, 0x64,
    0x49, 0x6f, 0x5c, 0x54, 0x79, 0x70, 0x65, 0x73, 0x5c, 0x51, 0x75, 0x65, 0x72, 0x79, 0x5c, 0x56,
    0x31, 0x5c, 0x47, 0x50, 0x42, 0x4d, 0x65, 0x74, 0x61, 0x64, 0x61, 0x74, 0x61, 0xea, 0x02, 0x19,
    0x53, 0x69, 0x74, 0x65, 0x64, 0x49, 0x6f, 0x3a, 0x3a, 0x54, 0x79, 0x70, 0x65, 0x73, 0x3a, 0x3a,
    0x51, 0x75, 0x65, 0x72, 0x79, 0x3a, 0x3a, 0x56, 0x31, 0x4a, 0xf2, 0x03, 0x0a, 0x06, 0x12, 0x04,
    0x00, 0x00, 0x13, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x0c, 0x12, 0x03, 0x00, 0x00, 0x12, 0x0a, 0x08,
    0x0a, 0x01, 0x02, 0x12, 0x03, 0x02, 0x00, 0x20, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04,
    0x04, 0x00, 0x07, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x04, 0x08, 0x19,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x05, 0x02, 0x12, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x05, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x05, 0x09, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x00, 0x03, 0x12, 0x03, 0x05, 0x10, 0x11, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12,
    0x03, 0x06, 0x02, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x05, 0x12, 0x03, 0x06,
    0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x06, 0x09, 0x0d,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x06, 0x10, 0x11, 0x0a, 0x0a,
    0x0a, 0x02, 0x04, 0x01, 0x12, 0x04, 0x09, 0x00, 0x0d, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01,
    0x01, 0x12, 0x03, 0x09, 0x08, 0x1a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x00, 0x12, 0x03,
    0x0a, 0x02, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x05, 0x12, 0x03, 0x0a, 0x02,
    0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x0a, 0x09, 0x0d, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x03, 0x12, 0x03, 0x0a, 0x10, 0x11, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x01, 0x02, 0x01, 0x12, 0x03, 0x0b, 0x02, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x01, 0x05, 0x12, 0x03, 0x0b, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01,
    0x01, 0x12, 0x03, 0x0b, 0x09, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x03, 0x12,
    0x03, 0x0b, 0x10, 0x11, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x02, 0x12, 0x03, 0x0c, 0x02,
    0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x05, 0x12, 0x03, 0x0c, 0x02, 0x08, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x01, 0x12, 0x03, 0x0c, 0x09, 0x17, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x02, 0x03, 0x12, 0x03, 0x0c, 0x1a, 0x1b, 0x0a, 0x0a, 0x0a, 0x02, 0x05,
    0x00, 0x12, 0x04, 0x0f, 0x00, 0x13, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x05, 0x00, 0x01, 0x12, 0x03,
    0x0f, 0x05, 0x0e, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x00, 0x12, 0x03, 0x10, 0x02, 0x1c,
    0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x10, 0x02, 0x17, 0x0a, 0x0c,
    0x0a, 0x05, 0x05, 0x00, 0x02, 0x00, 0x02, 0x12, 0x03, 0x10, 0x1a, 0x1b, 0x0a, 0x0b, 0x0a, 0x04,
    0x05, 0x00, 0x02, 0x01, 0x12, 0x03, 0x11, 0x02, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02,
    0x01, 0x01, 0x12, 0x03, 0x11, 0x02, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x01, 0x02,
    0x12, 0x03, 0x11, 0x12, 0x13, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x02, 0x12, 0x03, 0x12,
    0x02, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x12, 0x02, 0x10,
    0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x02, 0x02, 0x12, 0x03, 0x12, 0x13, 0x14, 0x62, 0x06,
    0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
];
// @@protoc_insertion_point(module)
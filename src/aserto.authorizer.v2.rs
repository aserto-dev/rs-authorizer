// @generated
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InfoRequest {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InfoResponse {
    #[prost(string, tag="1")]
    pub version: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub commit: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub date: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub os: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub arch: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPolicyRequest {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub field_mask: ::core::option::Option<::pbjson_types::FieldMask>,
    #[prost(message, optional, tag="3")]
    pub policy_instance: ::core::option::Option<api::PolicyInstance>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPolicyResponse {
    #[prost(message, optional, tag="1")]
    pub result: ::core::option::Option<api::Module>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListPoliciesRequest {
    #[prost(message, optional, tag="1")]
    pub field_mask: ::core::option::Option<::pbjson_types::FieldMask>,
    #[prost(message, optional, tag="2")]
    pub policy_instance: ::core::option::Option<api::PolicyInstance>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListPoliciesResponse {
    #[prost(message, repeated, tag="1")]
    pub result: ::prost::alloc::vec::Vec<api::Module>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DecisionTreeRequest {
    #[prost(message, optional, tag="1")]
    pub policy_context: ::core::option::Option<api::PolicyContext>,
    #[prost(message, optional, tag="2")]
    pub identity_context: ::core::option::Option<api::IdentityContext>,
    #[prost(message, optional, tag="3")]
    pub options: ::core::option::Option<DecisionTreeOptions>,
    #[prost(message, optional, tag="4")]
    pub resource_context: ::core::option::Option<::pbjson_types::Struct>,
    #[prost(message, optional, tag="5")]
    pub policy_instance: ::core::option::Option<api::PolicyInstance>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DecisionTreeOptions {
    #[prost(enumeration="PathSeparator", tag="1")]
    pub path_separator: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DecisionTreeResponse {
    #[prost(string, tag="1")]
    pub path_root: ::prost::alloc::string::String,
    #[prost(message, optional, tag="3")]
    pub path: ::core::option::Option<::pbjson_types::Struct>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IsRequest {
    #[prost(message, optional, tag="1")]
    pub policy_context: ::core::option::Option<api::PolicyContext>,
    #[prost(message, optional, tag="2")]
    pub identity_context: ::core::option::Option<api::IdentityContext>,
    #[prost(message, optional, tag="3")]
    pub resource_context: ::core::option::Option<::pbjson_types::Struct>,
    #[prost(message, optional, tag="4")]
    pub policy_instance: ::core::option::Option<api::PolicyInstance>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Decision {
    #[prost(string, tag="1")]
    pub decision: ::prost::alloc::string::String,
    #[prost(bool, tag="2")]
    pub is: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IsResponse {
    #[prost(message, repeated, tag="1")]
    pub decisions: ::prost::alloc::vec::Vec<Decision>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryOptions {
    /// default false
    #[prost(bool, tag="1")]
    pub metrics: bool,
    /// default false
    #[prost(bool, tag="2")]
    pub instrument: bool,
    /// default ExplainOffV1
    #[prost(enumeration="TraceLevel", tag="3")]
    pub trace: i32,
    /// default false
    #[prost(bool, tag="4")]
    pub trace_summary: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryRequest {
    #[prost(string, tag="1")]
    pub query: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub input: ::prost::alloc::string::String,
    #[prost(message, optional, tag="3")]
    pub options: ::core::option::Option<QueryOptions>,
    #[prost(message, optional, tag="4")]
    pub policy_context: ::core::option::Option<api::PolicyContext>,
    #[prost(message, optional, tag="5")]
    pub identity_context: ::core::option::Option<api::IdentityContext>,
    #[prost(message, optional, tag="6")]
    pub resource_context: ::core::option::Option<::pbjson_types::Struct>,
    #[prost(message, optional, tag="7")]
    pub policy_instance: ::core::option::Option<api::PolicyInstance>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CompileRequest {
    #[prost(string, tag="1")]
    pub query: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub input: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="3")]
    pub unknowns: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag="4")]
    pub disable_inlining: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(message, optional, tag="5")]
    pub options: ::core::option::Option<QueryOptions>,
    #[prost(message, optional, tag="6")]
    pub policy_context: ::core::option::Option<api::PolicyContext>,
    #[prost(message, optional, tag="7")]
    pub identity_context: ::core::option::Option<api::IdentityContext>,
    #[prost(message, optional, tag="8")]
    pub resource_context: ::core::option::Option<::pbjson_types::Struct>,
    #[prost(message, optional, tag="9")]
    pub policy_instance: ::core::option::Option<api::PolicyInstance>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CompileResponse {
    #[prost(message, optional, tag="1")]
    pub result: ::core::option::Option<::pbjson_types::Struct>,
    #[prost(message, optional, tag="2")]
    pub metrics: ::core::option::Option<::pbjson_types::Struct>,
    #[prost(message, repeated, tag="3")]
    pub trace: ::prost::alloc::vec::Vec<::pbjson_types::Struct>,
    #[prost(string, repeated, tag="4")]
    pub trace_summary: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryResponse {
    #[prost(message, optional, tag="1")]
    pub response: ::core::option::Option<::pbjson_types::Struct>,
    #[prost(message, optional, tag="2")]
    pub metrics: ::core::option::Option<::pbjson_types::Struct>,
    #[prost(message, repeated, tag="3")]
    pub trace: ::prost::alloc::vec::Vec<::pbjson_types::Struct>,
    #[prost(string, repeated, tag="4")]
    pub trace_summary: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PathSeparator {
    /// Value not set.
    Unknown = 0,
    /// Dot "." path separator
    Dot = 1,
    /// Slash "/" path separtor
    Slash = 2,
}
impl PathSeparator {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            PathSeparator::Unknown => "PATH_SEPARATOR_UNKNOWN",
            PathSeparator::Dot => "PATH_SEPARATOR_DOT",
            PathSeparator::Slash => "PATH_SEPARATOR_SLASH",
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TraceLevel {
    /// Value not set.
    Unknown = 0,
    /// ExplainOffV1   ExplainModeV1 = "off"
    Off = 1,
    /// ExplainFullV1  ExplainModeV1 = "full"
    Full = 2,
    /// ExplainNotesV1 ExplainModeV1 = "notes"
    Notes = 3,
    /// ExplainFailsV1 ExplainModeV1 = "fails"
    Fails = 4,
}
impl TraceLevel {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            TraceLevel::Unknown => "TRACE_LEVEL_UNKNOWN",
            TraceLevel::Off => "TRACE_LEVEL_OFF",
            TraceLevel::Full => "TRACE_LEVEL_FULL",
            TraceLevel::Notes => "TRACE_LEVEL_NOTES",
            TraceLevel::Fails => "TRACE_LEVEL_FAILS",
        }
    }
}
include!("aserto.authorizer.v2.serde.rs");
include!("aserto.authorizer.v2.tonic.rs");
// @@protoc_insertion_point(module)
// @generated
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PolicyContext {
    /// policy path aka package name
    #[prost(string, tag="1")]
    pub path: ::prost::alloc::string::String,
    /// list (1..N) of policy decisions (aka rules)
    #[prost(string, repeated, tag="2")]
    pub decisions: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IdentityContext {
    #[prost(string, tag="1")]
    pub identity: ::prost::alloc::string::String,
    #[prost(enumeration="IdentityType", tag="2")]
    pub r#type: i32,
}
/// Identity types, describes the payload type of the identity field inside the IdentityContext message.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum IdentityType {
    /// Unknown, value not set, requests will fail with identity type not set error.
    Unknown = 0,
    /// None, no explicit identity context set, equals anonymous.
    None = 1,
    /// Sub(ject), identity field contains an oAUTH subject.
    Sub = 2,
    /// JWT, identity field contains a JWT access token.
    Jwt = 3,
}
impl IdentityType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            IdentityType::Unknown => "IDENTITY_TYPE_UNKNOWN",
            IdentityType::None => "IDENTITY_TYPE_NONE",
            IdentityType::Sub => "IDENTITY_TYPE_SUB",
            IdentityType::Jwt => "IDENTITY_TYPE_JWT",
        }
    }
}
/// represents a decision that an authorizer performed in the past
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Decision {
    /// unique id, replay a decision starting with this, also useful to de-dup
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    /// UTC time when the decision was made
    #[prost(message, optional, tag="2")]
    pub timestamp: ::core::option::Option<::pbjson_types::Timestamp>,
    /// Policy path used in decision
    #[prost(string, tag="3")]
    pub path: ::prost::alloc::string::String,
    /// info about user for whom the decision as made
    #[prost(message, optional, tag="4")]
    pub user: ::core::option::Option<DecisionUser>,
    /// info about policy used for the decision    
    #[prost(message, optional, tag="5")]
    pub policy: ::core::option::Option<DecisionPolicy>,
    /// outcome of the decisions specified in the policy context
    #[prost(map="string, bool", tag="6")]
    pub outcomes: ::std::collections::HashMap<::prost::alloc::string::String, bool>,
    /// the resource context used in a decision
    #[prost(message, optional, tag="7")]
    pub resource: ::core::option::Option<::pbjson_types::Struct>,
    /// annotations that may be added to a decision    
    #[prost(map="string, string", tag="8")]
    pub annotations: ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
}
/// information about a user on behalf of whom a decision was made
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DecisionUser {
    /// identity context used in the decision
    #[prost(message, optional, tag="1")]
    pub context: ::core::option::Option<IdentityContext>,
    /// id of the user the identity resolved to    
    #[prost(string, tag="2")]
    pub id: ::prost::alloc::string::String,
    /// convinience human-readable identifier
    #[prost(string, tag="3")]
    pub email: ::prost::alloc::string::String,
}
/// information about a policy used in a decision
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DecisionPolicy {
    /// policy context used in the decision
    #[prost(message, optional, tag="1")]
    pub context: ::core::option::Option<PolicyContext>,
    /// registry service where policy was retrieved from (e.g. opcr.io)
    #[prost(string, tag="2")]
    pub registry_service: ::prost::alloc::string::String,
    /// image of the policy in the registry, including org (e.g. acmecorp/peoplefinder-abac)
    #[prost(string, tag="3")]
    pub registry_image: ::prost::alloc::string::String,
    /// tag of the policy image (e.g. 0.8.2 or latest)
    #[prost(string, tag="4")]
    pub registry_tag: ::prost::alloc::string::String,
    /// digest of the policy image 
    #[prost(string, tag="5")]
    pub registry_digest: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Module {
    #[prost(string, optional, tag="1")]
    pub id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="2")]
    pub raw: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="3")]
    pub package_path: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag="4")]
    pub ast: ::core::option::Option<::pbjson_types::Value>,
    #[prost(string, optional, tag="5")]
    pub package_root: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PolicyInstance {
    /// policy name
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// label identifying the instance of the policy
    #[prost(string, tag="2")]
    pub instance_label: ::prost::alloc::string::String,
}
include!("aserto.authorizer.v2.api.serde.rs");
// @@protoc_insertion_point(module)
// @generated
impl serde::Serialize for Decision {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.id.is_empty() {
            len += 1;
        }
        if self.timestamp.is_some() {
            len += 1;
        }
        if !self.path.is_empty() {
            len += 1;
        }
        if self.user.is_some() {
            len += 1;
        }
        if self.policy.is_some() {
            len += 1;
        }
        if !self.outcomes.is_empty() {
            len += 1;
        }
        if self.resource.is_some() {
            len += 1;
        }
        if !self.annotations.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("aserto.authorizer.v2.api.Decision", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if let Some(v) = self.timestamp.as_ref() {
            struct_ser.serialize_field("timestamp", v)?;
        }
        if !self.path.is_empty() {
            struct_ser.serialize_field("path", &self.path)?;
        }
        if let Some(v) = self.user.as_ref() {
            struct_ser.serialize_field("user", v)?;
        }
        if let Some(v) = self.policy.as_ref() {
            struct_ser.serialize_field("policy", v)?;
        }
        if !self.outcomes.is_empty() {
            struct_ser.serialize_field("outcomes", &self.outcomes)?;
        }
        if let Some(v) = self.resource.as_ref() {
            struct_ser.serialize_field("resource", v)?;
        }
        if !self.annotations.is_empty() {
            struct_ser.serialize_field("annotations", &self.annotations)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Decision {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "timestamp",
            "path",
            "user",
            "policy",
            "outcomes",
            "resource",
            "annotations",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            Timestamp,
            Path,
            User,
            Policy,
            Outcomes,
            Resource,
            Annotations,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "id" => Ok(GeneratedField::Id),
                            "timestamp" => Ok(GeneratedField::Timestamp),
                            "path" => Ok(GeneratedField::Path),
                            "user" => Ok(GeneratedField::User),
                            "policy" => Ok(GeneratedField::Policy),
                            "outcomes" => Ok(GeneratedField::Outcomes),
                            "resource" => Ok(GeneratedField::Resource),
                            "annotations" => Ok(GeneratedField::Annotations),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Decision;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct aserto.authorizer.v2.api.Decision")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<Decision, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut timestamp__ = None;
                let mut path__ = None;
                let mut user__ = None;
                let mut policy__ = None;
                let mut outcomes__ = None;
                let mut resource__ = None;
                let mut annotations__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map.next_value()?);
                        }
                        GeneratedField::Timestamp => {
                            if timestamp__.is_some() {
                                return Err(serde::de::Error::duplicate_field("timestamp"));
                            }
                            timestamp__ = map.next_value()?;
                        }
                        GeneratedField::Path => {
                            if path__.is_some() {
                                return Err(serde::de::Error::duplicate_field("path"));
                            }
                            path__ = Some(map.next_value()?);
                        }
                        GeneratedField::User => {
                            if user__.is_some() {
                                return Err(serde::de::Error::duplicate_field("user"));
                            }
                            user__ = map.next_value()?;
                        }
                        GeneratedField::Policy => {
                            if policy__.is_some() {
                                return Err(serde::de::Error::duplicate_field("policy"));
                            }
                            policy__ = map.next_value()?;
                        }
                        GeneratedField::Outcomes => {
                            if outcomes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("outcomes"));
                            }
                            outcomes__ = Some(
                                map.next_value::<std::collections::HashMap<_, _>>()?
                            );
                        }
                        GeneratedField::Resource => {
                            if resource__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resource"));
                            }
                            resource__ = map.next_value()?;
                        }
                        GeneratedField::Annotations => {
                            if annotations__.is_some() {
                                return Err(serde::de::Error::duplicate_field("annotations"));
                            }
                            annotations__ = Some(
                                map.next_value::<std::collections::HashMap<_, _>>()?
                            );
                        }
                    }
                }
                Ok(Decision {
                    id: id__.unwrap_or_default(),
                    timestamp: timestamp__,
                    path: path__.unwrap_or_default(),
                    user: user__,
                    policy: policy__,
                    outcomes: outcomes__.unwrap_or_default(),
                    resource: resource__,
                    annotations: annotations__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("aserto.authorizer.v2.api.Decision", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DecisionPolicy {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.context.is_some() {
            len += 1;
        }
        if !self.registry_service.is_empty() {
            len += 1;
        }
        if !self.registry_image.is_empty() {
            len += 1;
        }
        if !self.registry_tag.is_empty() {
            len += 1;
        }
        if !self.registry_digest.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("aserto.authorizer.v2.api.DecisionPolicy", len)?;
        if let Some(v) = self.context.as_ref() {
            struct_ser.serialize_field("context", v)?;
        }
        if !self.registry_service.is_empty() {
            struct_ser.serialize_field("registryService", &self.registry_service)?;
        }
        if !self.registry_image.is_empty() {
            struct_ser.serialize_field("registryImage", &self.registry_image)?;
        }
        if !self.registry_tag.is_empty() {
            struct_ser.serialize_field("registryTag", &self.registry_tag)?;
        }
        if !self.registry_digest.is_empty() {
            struct_ser.serialize_field("registryDigest", &self.registry_digest)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DecisionPolicy {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "context",
            "registry_service",
            "registryService",
            "registry_image",
            "registryImage",
            "registry_tag",
            "registryTag",
            "registry_digest",
            "registryDigest",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Context,
            RegistryService,
            RegistryImage,
            RegistryTag,
            RegistryDigest,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "context" => Ok(GeneratedField::Context),
                            "registryService" | "registry_service" => Ok(GeneratedField::RegistryService),
                            "registryImage" | "registry_image" => Ok(GeneratedField::RegistryImage),
                            "registryTag" | "registry_tag" => Ok(GeneratedField::RegistryTag),
                            "registryDigest" | "registry_digest" => Ok(GeneratedField::RegistryDigest),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DecisionPolicy;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct aserto.authorizer.v2.api.DecisionPolicy")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<DecisionPolicy, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut context__ = None;
                let mut registry_service__ = None;
                let mut registry_image__ = None;
                let mut registry_tag__ = None;
                let mut registry_digest__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Context => {
                            if context__.is_some() {
                                return Err(serde::de::Error::duplicate_field("context"));
                            }
                            context__ = map.next_value()?;
                        }
                        GeneratedField::RegistryService => {
                            if registry_service__.is_some() {
                                return Err(serde::de::Error::duplicate_field("registryService"));
                            }
                            registry_service__ = Some(map.next_value()?);
                        }
                        GeneratedField::RegistryImage => {
                            if registry_image__.is_some() {
                                return Err(serde::de::Error::duplicate_field("registryImage"));
                            }
                            registry_image__ = Some(map.next_value()?);
                        }
                        GeneratedField::RegistryTag => {
                            if registry_tag__.is_some() {
                                return Err(serde::de::Error::duplicate_field("registryTag"));
                            }
                            registry_tag__ = Some(map.next_value()?);
                        }
                        GeneratedField::RegistryDigest => {
                            if registry_digest__.is_some() {
                                return Err(serde::de::Error::duplicate_field("registryDigest"));
                            }
                            registry_digest__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(DecisionPolicy {
                    context: context__,
                    registry_service: registry_service__.unwrap_or_default(),
                    registry_image: registry_image__.unwrap_or_default(),
                    registry_tag: registry_tag__.unwrap_or_default(),
                    registry_digest: registry_digest__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("aserto.authorizer.v2.api.DecisionPolicy", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DecisionUser {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.context.is_some() {
            len += 1;
        }
        if !self.id.is_empty() {
            len += 1;
        }
        if !self.email.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("aserto.authorizer.v2.api.DecisionUser", len)?;
        if let Some(v) = self.context.as_ref() {
            struct_ser.serialize_field("context", v)?;
        }
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if !self.email.is_empty() {
            struct_ser.serialize_field("email", &self.email)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DecisionUser {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "context",
            "id",
            "email",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Context,
            Id,
            Email,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "context" => Ok(GeneratedField::Context),
                            "id" => Ok(GeneratedField::Id),
                            "email" => Ok(GeneratedField::Email),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DecisionUser;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct aserto.authorizer.v2.api.DecisionUser")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<DecisionUser, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut context__ = None;
                let mut id__ = None;
                let mut email__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Context => {
                            if context__.is_some() {
                                return Err(serde::de::Error::duplicate_field("context"));
                            }
                            context__ = map.next_value()?;
                        }
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map.next_value()?);
                        }
                        GeneratedField::Email => {
                            if email__.is_some() {
                                return Err(serde::de::Error::duplicate_field("email"));
                            }
                            email__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(DecisionUser {
                    context: context__,
                    id: id__.unwrap_or_default(),
                    email: email__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("aserto.authorizer.v2.api.DecisionUser", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for IdentityContext {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.identity.is_empty() {
            len += 1;
        }
        if self.r#type != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("aserto.authorizer.v2.api.IdentityContext", len)?;
        if !self.identity.is_empty() {
            struct_ser.serialize_field("identity", &self.identity)?;
        }
        if self.r#type != 0 {
            let v = IdentityType::from_i32(self.r#type)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.r#type)))?;
            struct_ser.serialize_field("type", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for IdentityContext {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "identity",
            "type",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Identity,
            Type,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "identity" => Ok(GeneratedField::Identity),
                            "type" => Ok(GeneratedField::Type),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = IdentityContext;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct aserto.authorizer.v2.api.IdentityContext")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<IdentityContext, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut identity__ = None;
                let mut r#type__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Identity => {
                            if identity__.is_some() {
                                return Err(serde::de::Error::duplicate_field("identity"));
                            }
                            identity__ = Some(map.next_value()?);
                        }
                        GeneratedField::Type => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type__ = Some(map.next_value::<IdentityType>()? as i32);
                        }
                    }
                }
                Ok(IdentityContext {
                    identity: identity__.unwrap_or_default(),
                    r#type: r#type__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("aserto.authorizer.v2.api.IdentityContext", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for IdentityType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unknown => "IDENTITY_TYPE_UNKNOWN",
            Self::None => "IDENTITY_TYPE_NONE",
            Self::Sub => "IDENTITY_TYPE_SUB",
            Self::Jwt => "IDENTITY_TYPE_JWT",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for IdentityType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "IDENTITY_TYPE_UNKNOWN",
            "IDENTITY_TYPE_NONE",
            "IDENTITY_TYPE_SUB",
            "IDENTITY_TYPE_JWT",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = IdentityType;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                use std::convert::TryFrom;
                i32::try_from(v)
                    .ok()
                    .and_then(IdentityType::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                use std::convert::TryFrom;
                i32::try_from(v)
                    .ok()
                    .and_then(IdentityType::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "IDENTITY_TYPE_UNKNOWN" => Ok(IdentityType::Unknown),
                    "IDENTITY_TYPE_NONE" => Ok(IdentityType::None),
                    "IDENTITY_TYPE_SUB" => Ok(IdentityType::Sub),
                    "IDENTITY_TYPE_JWT" => Ok(IdentityType::Jwt),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for Module {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.id.is_some() {
            len += 1;
        }
        if self.raw.is_some() {
            len += 1;
        }
        if self.package_path.is_some() {
            len += 1;
        }
        if self.ast.is_some() {
            len += 1;
        }
        if self.package_root.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("aserto.authorizer.v2.api.Module", len)?;
        if let Some(v) = self.id.as_ref() {
            struct_ser.serialize_field("id", v)?;
        }
        if let Some(v) = self.raw.as_ref() {
            struct_ser.serialize_field("raw", v)?;
        }
        if let Some(v) = self.package_path.as_ref() {
            struct_ser.serialize_field("packagePath", v)?;
        }
        if let Some(v) = self.ast.as_ref() {
            struct_ser.serialize_field("ast", v)?;
        }
        if let Some(v) = self.package_root.as_ref() {
            struct_ser.serialize_field("packageRoot", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Module {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "raw",
            "package_path",
            "packagePath",
            "ast",
            "package_root",
            "packageRoot",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            Raw,
            PackagePath,
            Ast,
            PackageRoot,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "id" => Ok(GeneratedField::Id),
                            "raw" => Ok(GeneratedField::Raw),
                            "packagePath" | "package_path" => Ok(GeneratedField::PackagePath),
                            "ast" => Ok(GeneratedField::Ast),
                            "packageRoot" | "package_root" => Ok(GeneratedField::PackageRoot),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Module;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct aserto.authorizer.v2.api.Module")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<Module, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut raw__ = None;
                let mut package_path__ = None;
                let mut ast__ = None;
                let mut package_root__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = map.next_value()?;
                        }
                        GeneratedField::Raw => {
                            if raw__.is_some() {
                                return Err(serde::de::Error::duplicate_field("raw"));
                            }
                            raw__ = map.next_value()?;
                        }
                        GeneratedField::PackagePath => {
                            if package_path__.is_some() {
                                return Err(serde::de::Error::duplicate_field("packagePath"));
                            }
                            package_path__ = map.next_value()?;
                        }
                        GeneratedField::Ast => {
                            if ast__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ast"));
                            }
                            ast__ = map.next_value()?;
                        }
                        GeneratedField::PackageRoot => {
                            if package_root__.is_some() {
                                return Err(serde::de::Error::duplicate_field("packageRoot"));
                            }
                            package_root__ = map.next_value()?;
                        }
                    }
                }
                Ok(Module {
                    id: id__,
                    raw: raw__,
                    package_path: package_path__,
                    ast: ast__,
                    package_root: package_root__,
                })
            }
        }
        deserializer.deserialize_struct("aserto.authorizer.v2.api.Module", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PolicyContext {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.path.is_empty() {
            len += 1;
        }
        if !self.decisions.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("aserto.authorizer.v2.api.PolicyContext", len)?;
        if !self.path.is_empty() {
            struct_ser.serialize_field("path", &self.path)?;
        }
        if !self.decisions.is_empty() {
            struct_ser.serialize_field("decisions", &self.decisions)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PolicyContext {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "path",
            "decisions",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Path,
            Decisions,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "path" => Ok(GeneratedField::Path),
                            "decisions" => Ok(GeneratedField::Decisions),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PolicyContext;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct aserto.authorizer.v2.api.PolicyContext")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<PolicyContext, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut path__ = None;
                let mut decisions__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Path => {
                            if path__.is_some() {
                                return Err(serde::de::Error::duplicate_field("path"));
                            }
                            path__ = Some(map.next_value()?);
                        }
                        GeneratedField::Decisions => {
                            if decisions__.is_some() {
                                return Err(serde::de::Error::duplicate_field("decisions"));
                            }
                            decisions__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(PolicyContext {
                    path: path__.unwrap_or_default(),
                    decisions: decisions__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("aserto.authorizer.v2.api.PolicyContext", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PolicyInstance {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.name.is_empty() {
            len += 1;
        }
        if !self.instance_label.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("aserto.authorizer.v2.api.PolicyInstance", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.instance_label.is_empty() {
            struct_ser.serialize_field("instanceLabel", &self.instance_label)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PolicyInstance {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "instance_label",
            "instanceLabel",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            InstanceLabel,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "name" => Ok(GeneratedField::Name),
                            "instanceLabel" | "instance_label" => Ok(GeneratedField::InstanceLabel),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PolicyInstance;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct aserto.authorizer.v2.api.PolicyInstance")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<PolicyInstance, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut instance_label__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map.next_value()?);
                        }
                        GeneratedField::InstanceLabel => {
                            if instance_label__.is_some() {
                                return Err(serde::de::Error::duplicate_field("instanceLabel"));
                            }
                            instance_label__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(PolicyInstance {
                    name: name__.unwrap_or_default(),
                    instance_label: instance_label__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("aserto.authorizer.v2.api.PolicyInstance", FIELDS, GeneratedVisitor)
    }
}

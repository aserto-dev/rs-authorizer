// @generated
impl serde::Serialize for CompileRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.query.is_empty() {
            len += 1;
        }
        if !self.input.is_empty() {
            len += 1;
        }
        if !self.unknowns.is_empty() {
            len += 1;
        }
        if !self.disable_inlining.is_empty() {
            len += 1;
        }
        if self.options.is_some() {
            len += 1;
        }
        if self.policy_context.is_some() {
            len += 1;
        }
        if self.identity_context.is_some() {
            len += 1;
        }
        if self.resource_context.is_some() {
            len += 1;
        }
        if self.policy_instance.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("aserto.authorizer.v2.CompileRequest", len)?;
        if !self.query.is_empty() {
            struct_ser.serialize_field("query", &self.query)?;
        }
        if !self.input.is_empty() {
            struct_ser.serialize_field("input", &self.input)?;
        }
        if !self.unknowns.is_empty() {
            struct_ser.serialize_field("unknowns", &self.unknowns)?;
        }
        if !self.disable_inlining.is_empty() {
            struct_ser.serialize_field("disableInlining", &self.disable_inlining)?;
        }
        if let Some(v) = self.options.as_ref() {
            struct_ser.serialize_field("options", v)?;
        }
        if let Some(v) = self.policy_context.as_ref() {
            struct_ser.serialize_field("policyContext", v)?;
        }
        if let Some(v) = self.identity_context.as_ref() {
            struct_ser.serialize_field("identityContext", v)?;
        }
        if let Some(v) = self.resource_context.as_ref() {
            struct_ser.serialize_field("resourceContext", v)?;
        }
        if let Some(v) = self.policy_instance.as_ref() {
            struct_ser.serialize_field("policyInstance", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CompileRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "query",
            "input",
            "unknowns",
            "disable_inlining",
            "disableInlining",
            "options",
            "policy_context",
            "policyContext",
            "identity_context",
            "identityContext",
            "resource_context",
            "resourceContext",
            "policy_instance",
            "policyInstance",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Query,
            Input,
            Unknowns,
            DisableInlining,
            Options,
            PolicyContext,
            IdentityContext,
            ResourceContext,
            PolicyInstance,
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
                            "query" => Ok(GeneratedField::Query),
                            "input" => Ok(GeneratedField::Input),
                            "unknowns" => Ok(GeneratedField::Unknowns),
                            "disableInlining" | "disable_inlining" => Ok(GeneratedField::DisableInlining),
                            "options" => Ok(GeneratedField::Options),
                            "policyContext" | "policy_context" => Ok(GeneratedField::PolicyContext),
                            "identityContext" | "identity_context" => Ok(GeneratedField::IdentityContext),
                            "resourceContext" | "resource_context" => Ok(GeneratedField::ResourceContext),
                            "policyInstance" | "policy_instance" => Ok(GeneratedField::PolicyInstance),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CompileRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct aserto.authorizer.v2.CompileRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<CompileRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut query__ = None;
                let mut input__ = None;
                let mut unknowns__ = None;
                let mut disable_inlining__ = None;
                let mut options__ = None;
                let mut policy_context__ = None;
                let mut identity_context__ = None;
                let mut resource_context__ = None;
                let mut policy_instance__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Query => {
                            if query__.is_some() {
                                return Err(serde::de::Error::duplicate_field("query"));
                            }
                            query__ = Some(map.next_value()?);
                        }
                        GeneratedField::Input => {
                            if input__.is_some() {
                                return Err(serde::de::Error::duplicate_field("input"));
                            }
                            input__ = Some(map.next_value()?);
                        }
                        GeneratedField::Unknowns => {
                            if unknowns__.is_some() {
                                return Err(serde::de::Error::duplicate_field("unknowns"));
                            }
                            unknowns__ = Some(map.next_value()?);
                        }
                        GeneratedField::DisableInlining => {
                            if disable_inlining__.is_some() {
                                return Err(serde::de::Error::duplicate_field("disableInlining"));
                            }
                            disable_inlining__ = Some(map.next_value()?);
                        }
                        GeneratedField::Options => {
                            if options__.is_some() {
                                return Err(serde::de::Error::duplicate_field("options"));
                            }
                            options__ = map.next_value()?;
                        }
                        GeneratedField::PolicyContext => {
                            if policy_context__.is_some() {
                                return Err(serde::de::Error::duplicate_field("policyContext"));
                            }
                            policy_context__ = map.next_value()?;
                        }
                        GeneratedField::IdentityContext => {
                            if identity_context__.is_some() {
                                return Err(serde::de::Error::duplicate_field("identityContext"));
                            }
                            identity_context__ = map.next_value()?;
                        }
                        GeneratedField::ResourceContext => {
                            if resource_context__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resourceContext"));
                            }
                            resource_context__ = map.next_value()?;
                        }
                        GeneratedField::PolicyInstance => {
                            if policy_instance__.is_some() {
                                return Err(serde::de::Error::duplicate_field("policyInstance"));
                            }
                            policy_instance__ = map.next_value()?;
                        }
                    }
                }
                Ok(CompileRequest {
                    query: query__.unwrap_or_default(),
                    input: input__.unwrap_or_default(),
                    unknowns: unknowns__.unwrap_or_default(),
                    disable_inlining: disable_inlining__.unwrap_or_default(),
                    options: options__,
                    policy_context: policy_context__,
                    identity_context: identity_context__,
                    resource_context: resource_context__,
                    policy_instance: policy_instance__,
                })
            }
        }
        deserializer.deserialize_struct("aserto.authorizer.v2.CompileRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CompileResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.result.is_some() {
            len += 1;
        }
        if self.metrics.is_some() {
            len += 1;
        }
        if !self.trace.is_empty() {
            len += 1;
        }
        if !self.trace_summary.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("aserto.authorizer.v2.CompileResponse", len)?;
        if let Some(v) = self.result.as_ref() {
            struct_ser.serialize_field("result", v)?;
        }
        if let Some(v) = self.metrics.as_ref() {
            struct_ser.serialize_field("metrics", v)?;
        }
        if !self.trace.is_empty() {
            struct_ser.serialize_field("trace", &self.trace)?;
        }
        if !self.trace_summary.is_empty() {
            struct_ser.serialize_field("traceSummary", &self.trace_summary)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CompileResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "result",
            "metrics",
            "trace",
            "trace_summary",
            "traceSummary",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Result,
            Metrics,
            Trace,
            TraceSummary,
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
                            "result" => Ok(GeneratedField::Result),
                            "metrics" => Ok(GeneratedField::Metrics),
                            "trace" => Ok(GeneratedField::Trace),
                            "traceSummary" | "trace_summary" => Ok(GeneratedField::TraceSummary),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CompileResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct aserto.authorizer.v2.CompileResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<CompileResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut result__ = None;
                let mut metrics__ = None;
                let mut trace__ = None;
                let mut trace_summary__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Result => {
                            if result__.is_some() {
                                return Err(serde::de::Error::duplicate_field("result"));
                            }
                            result__ = map.next_value()?;
                        }
                        GeneratedField::Metrics => {
                            if metrics__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metrics"));
                            }
                            metrics__ = map.next_value()?;
                        }
                        GeneratedField::Trace => {
                            if trace__.is_some() {
                                return Err(serde::de::Error::duplicate_field("trace"));
                            }
                            trace__ = Some(map.next_value()?);
                        }
                        GeneratedField::TraceSummary => {
                            if trace_summary__.is_some() {
                                return Err(serde::de::Error::duplicate_field("traceSummary"));
                            }
                            trace_summary__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(CompileResponse {
                    result: result__,
                    metrics: metrics__,
                    trace: trace__.unwrap_or_default(),
                    trace_summary: trace_summary__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("aserto.authorizer.v2.CompileResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Decision {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.decision.is_empty() {
            len += 1;
        }
        if self.is {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("aserto.authorizer.v2.Decision", len)?;
        if !self.decision.is_empty() {
            struct_ser.serialize_field("decision", &self.decision)?;
        }
        if self.is {
            struct_ser.serialize_field("is", &self.is)?;
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
            "decision",
            "is",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Decision,
            Is,
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
                            "decision" => Ok(GeneratedField::Decision),
                            "is" => Ok(GeneratedField::Is),
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
                formatter.write_str("struct aserto.authorizer.v2.Decision")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<Decision, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut decision__ = None;
                let mut is__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Decision => {
                            if decision__.is_some() {
                                return Err(serde::de::Error::duplicate_field("decision"));
                            }
                            decision__ = Some(map.next_value()?);
                        }
                        GeneratedField::Is => {
                            if is__.is_some() {
                                return Err(serde::de::Error::duplicate_field("is"));
                            }
                            is__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(Decision {
                    decision: decision__.unwrap_or_default(),
                    is: is__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("aserto.authorizer.v2.Decision", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DecisionTreeOptions {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.path_separator != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("aserto.authorizer.v2.DecisionTreeOptions", len)?;
        if self.path_separator != 0 {
            let v = PathSeparator::from_i32(self.path_separator)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.path_separator)))?;
            struct_ser.serialize_field("pathSeparator", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DecisionTreeOptions {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "path_separator",
            "pathSeparator",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PathSeparator,
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
                            "pathSeparator" | "path_separator" => Ok(GeneratedField::PathSeparator),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DecisionTreeOptions;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct aserto.authorizer.v2.DecisionTreeOptions")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<DecisionTreeOptions, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut path_separator__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::PathSeparator => {
                            if path_separator__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pathSeparator"));
                            }
                            path_separator__ = Some(map.next_value::<PathSeparator>()? as i32);
                        }
                    }
                }
                Ok(DecisionTreeOptions {
                    path_separator: path_separator__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("aserto.authorizer.v2.DecisionTreeOptions", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DecisionTreeRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.policy_context.is_some() {
            len += 1;
        }
        if self.identity_context.is_some() {
            len += 1;
        }
        if self.options.is_some() {
            len += 1;
        }
        if self.resource_context.is_some() {
            len += 1;
        }
        if self.policy_instance.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("aserto.authorizer.v2.DecisionTreeRequest", len)?;
        if let Some(v) = self.policy_context.as_ref() {
            struct_ser.serialize_field("policyContext", v)?;
        }
        if let Some(v) = self.identity_context.as_ref() {
            struct_ser.serialize_field("identityContext", v)?;
        }
        if let Some(v) = self.options.as_ref() {
            struct_ser.serialize_field("options", v)?;
        }
        if let Some(v) = self.resource_context.as_ref() {
            struct_ser.serialize_field("resourceContext", v)?;
        }
        if let Some(v) = self.policy_instance.as_ref() {
            struct_ser.serialize_field("policyInstance", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DecisionTreeRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "policy_context",
            "policyContext",
            "identity_context",
            "identityContext",
            "options",
            "resource_context",
            "resourceContext",
            "policy_instance",
            "policyInstance",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PolicyContext,
            IdentityContext,
            Options,
            ResourceContext,
            PolicyInstance,
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
                            "policyContext" | "policy_context" => Ok(GeneratedField::PolicyContext),
                            "identityContext" | "identity_context" => Ok(GeneratedField::IdentityContext),
                            "options" => Ok(GeneratedField::Options),
                            "resourceContext" | "resource_context" => Ok(GeneratedField::ResourceContext),
                            "policyInstance" | "policy_instance" => Ok(GeneratedField::PolicyInstance),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DecisionTreeRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct aserto.authorizer.v2.DecisionTreeRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<DecisionTreeRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut policy_context__ = None;
                let mut identity_context__ = None;
                let mut options__ = None;
                let mut resource_context__ = None;
                let mut policy_instance__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::PolicyContext => {
                            if policy_context__.is_some() {
                                return Err(serde::de::Error::duplicate_field("policyContext"));
                            }
                            policy_context__ = map.next_value()?;
                        }
                        GeneratedField::IdentityContext => {
                            if identity_context__.is_some() {
                                return Err(serde::de::Error::duplicate_field("identityContext"));
                            }
                            identity_context__ = map.next_value()?;
                        }
                        GeneratedField::Options => {
                            if options__.is_some() {
                                return Err(serde::de::Error::duplicate_field("options"));
                            }
                            options__ = map.next_value()?;
                        }
                        GeneratedField::ResourceContext => {
                            if resource_context__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resourceContext"));
                            }
                            resource_context__ = map.next_value()?;
                        }
                        GeneratedField::PolicyInstance => {
                            if policy_instance__.is_some() {
                                return Err(serde::de::Error::duplicate_field("policyInstance"));
                            }
                            policy_instance__ = map.next_value()?;
                        }
                    }
                }
                Ok(DecisionTreeRequest {
                    policy_context: policy_context__,
                    identity_context: identity_context__,
                    options: options__,
                    resource_context: resource_context__,
                    policy_instance: policy_instance__,
                })
            }
        }
        deserializer.deserialize_struct("aserto.authorizer.v2.DecisionTreeRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DecisionTreeResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.path_root.is_empty() {
            len += 1;
        }
        if self.path.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("aserto.authorizer.v2.DecisionTreeResponse", len)?;
        if !self.path_root.is_empty() {
            struct_ser.serialize_field("pathRoot", &self.path_root)?;
        }
        if let Some(v) = self.path.as_ref() {
            struct_ser.serialize_field("path", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DecisionTreeResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "path_root",
            "pathRoot",
            "path",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PathRoot,
            Path,
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
                            "pathRoot" | "path_root" => Ok(GeneratedField::PathRoot),
                            "path" => Ok(GeneratedField::Path),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DecisionTreeResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct aserto.authorizer.v2.DecisionTreeResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<DecisionTreeResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut path_root__ = None;
                let mut path__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::PathRoot => {
                            if path_root__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pathRoot"));
                            }
                            path_root__ = Some(map.next_value()?);
                        }
                        GeneratedField::Path => {
                            if path__.is_some() {
                                return Err(serde::de::Error::duplicate_field("path"));
                            }
                            path__ = map.next_value()?;
                        }
                    }
                }
                Ok(DecisionTreeResponse {
                    path_root: path_root__.unwrap_or_default(),
                    path: path__,
                })
            }
        }
        deserializer.deserialize_struct("aserto.authorizer.v2.DecisionTreeResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetPolicyRequest {
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
        if self.field_mask.is_some() {
            len += 1;
        }
        if self.policy_instance.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("aserto.authorizer.v2.GetPolicyRequest", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if let Some(v) = self.field_mask.as_ref() {
            struct_ser.serialize_field("fieldMask", v)?;
        }
        if let Some(v) = self.policy_instance.as_ref() {
            struct_ser.serialize_field("policyInstance", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetPolicyRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "field_mask",
            "fieldMask",
            "policy_instance",
            "policyInstance",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            FieldMask,
            PolicyInstance,
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
                            "fieldMask" | "field_mask" => Ok(GeneratedField::FieldMask),
                            "policyInstance" | "policy_instance" => Ok(GeneratedField::PolicyInstance),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetPolicyRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct aserto.authorizer.v2.GetPolicyRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<GetPolicyRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut field_mask__ = None;
                let mut policy_instance__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map.next_value()?);
                        }
                        GeneratedField::FieldMask => {
                            if field_mask__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fieldMask"));
                            }
                            field_mask__ = map.next_value()?;
                        }
                        GeneratedField::PolicyInstance => {
                            if policy_instance__.is_some() {
                                return Err(serde::de::Error::duplicate_field("policyInstance"));
                            }
                            policy_instance__ = map.next_value()?;
                        }
                    }
                }
                Ok(GetPolicyRequest {
                    id: id__.unwrap_or_default(),
                    field_mask: field_mask__,
                    policy_instance: policy_instance__,
                })
            }
        }
        deserializer.deserialize_struct("aserto.authorizer.v2.GetPolicyRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetPolicyResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.result.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("aserto.authorizer.v2.GetPolicyResponse", len)?;
        if let Some(v) = self.result.as_ref() {
            struct_ser.serialize_field("result", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetPolicyResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "result",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Result,
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
                            "result" => Ok(GeneratedField::Result),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetPolicyResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct aserto.authorizer.v2.GetPolicyResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<GetPolicyResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut result__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Result => {
                            if result__.is_some() {
                                return Err(serde::de::Error::duplicate_field("result"));
                            }
                            result__ = map.next_value()?;
                        }
                    }
                }
                Ok(GetPolicyResponse {
                    result: result__,
                })
            }
        }
        deserializer.deserialize_struct("aserto.authorizer.v2.GetPolicyResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for InfoRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("aserto.authorizer.v2.InfoRequest", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for InfoRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
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
                            Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = InfoRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct aserto.authorizer.v2.InfoRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<InfoRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(InfoRequest {
                })
            }
        }
        deserializer.deserialize_struct("aserto.authorizer.v2.InfoRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for InfoResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.version.is_empty() {
            len += 1;
        }
        if !self.commit.is_empty() {
            len += 1;
        }
        if !self.date.is_empty() {
            len += 1;
        }
        if !self.os.is_empty() {
            len += 1;
        }
        if !self.arch.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("aserto.authorizer.v2.InfoResponse", len)?;
        if !self.version.is_empty() {
            struct_ser.serialize_field("version", &self.version)?;
        }
        if !self.commit.is_empty() {
            struct_ser.serialize_field("commit", &self.commit)?;
        }
        if !self.date.is_empty() {
            struct_ser.serialize_field("date", &self.date)?;
        }
        if !self.os.is_empty() {
            struct_ser.serialize_field("os", &self.os)?;
        }
        if !self.arch.is_empty() {
            struct_ser.serialize_field("arch", &self.arch)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for InfoResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "version",
            "commit",
            "date",
            "os",
            "arch",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Version,
            Commit,
            Date,
            Os,
            Arch,
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
                            "version" => Ok(GeneratedField::Version),
                            "commit" => Ok(GeneratedField::Commit),
                            "date" => Ok(GeneratedField::Date),
                            "os" => Ok(GeneratedField::Os),
                            "arch" => Ok(GeneratedField::Arch),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = InfoResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct aserto.authorizer.v2.InfoResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<InfoResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut version__ = None;
                let mut commit__ = None;
                let mut date__ = None;
                let mut os__ = None;
                let mut arch__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Version => {
                            if version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("version"));
                            }
                            version__ = Some(map.next_value()?);
                        }
                        GeneratedField::Commit => {
                            if commit__.is_some() {
                                return Err(serde::de::Error::duplicate_field("commit"));
                            }
                            commit__ = Some(map.next_value()?);
                        }
                        GeneratedField::Date => {
                            if date__.is_some() {
                                return Err(serde::de::Error::duplicate_field("date"));
                            }
                            date__ = Some(map.next_value()?);
                        }
                        GeneratedField::Os => {
                            if os__.is_some() {
                                return Err(serde::de::Error::duplicate_field("os"));
                            }
                            os__ = Some(map.next_value()?);
                        }
                        GeneratedField::Arch => {
                            if arch__.is_some() {
                                return Err(serde::de::Error::duplicate_field("arch"));
                            }
                            arch__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(InfoResponse {
                    version: version__.unwrap_or_default(),
                    commit: commit__.unwrap_or_default(),
                    date: date__.unwrap_or_default(),
                    os: os__.unwrap_or_default(),
                    arch: arch__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("aserto.authorizer.v2.InfoResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for IsRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.policy_context.is_some() {
            len += 1;
        }
        if self.identity_context.is_some() {
            len += 1;
        }
        if self.resource_context.is_some() {
            len += 1;
        }
        if self.policy_instance.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("aserto.authorizer.v2.IsRequest", len)?;
        if let Some(v) = self.policy_context.as_ref() {
            struct_ser.serialize_field("policyContext", v)?;
        }
        if let Some(v) = self.identity_context.as_ref() {
            struct_ser.serialize_field("identityContext", v)?;
        }
        if let Some(v) = self.resource_context.as_ref() {
            struct_ser.serialize_field("resourceContext", v)?;
        }
        if let Some(v) = self.policy_instance.as_ref() {
            struct_ser.serialize_field("policyInstance", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for IsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "policy_context",
            "policyContext",
            "identity_context",
            "identityContext",
            "resource_context",
            "resourceContext",
            "policy_instance",
            "policyInstance",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PolicyContext,
            IdentityContext,
            ResourceContext,
            PolicyInstance,
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
                            "policyContext" | "policy_context" => Ok(GeneratedField::PolicyContext),
                            "identityContext" | "identity_context" => Ok(GeneratedField::IdentityContext),
                            "resourceContext" | "resource_context" => Ok(GeneratedField::ResourceContext),
                            "policyInstance" | "policy_instance" => Ok(GeneratedField::PolicyInstance),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = IsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct aserto.authorizer.v2.IsRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<IsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut policy_context__ = None;
                let mut identity_context__ = None;
                let mut resource_context__ = None;
                let mut policy_instance__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::PolicyContext => {
                            if policy_context__.is_some() {
                                return Err(serde::de::Error::duplicate_field("policyContext"));
                            }
                            policy_context__ = map.next_value()?;
                        }
                        GeneratedField::IdentityContext => {
                            if identity_context__.is_some() {
                                return Err(serde::de::Error::duplicate_field("identityContext"));
                            }
                            identity_context__ = map.next_value()?;
                        }
                        GeneratedField::ResourceContext => {
                            if resource_context__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resourceContext"));
                            }
                            resource_context__ = map.next_value()?;
                        }
                        GeneratedField::PolicyInstance => {
                            if policy_instance__.is_some() {
                                return Err(serde::de::Error::duplicate_field("policyInstance"));
                            }
                            policy_instance__ = map.next_value()?;
                        }
                    }
                }
                Ok(IsRequest {
                    policy_context: policy_context__,
                    identity_context: identity_context__,
                    resource_context: resource_context__,
                    policy_instance: policy_instance__,
                })
            }
        }
        deserializer.deserialize_struct("aserto.authorizer.v2.IsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for IsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.decisions.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("aserto.authorizer.v2.IsResponse", len)?;
        if !self.decisions.is_empty() {
            struct_ser.serialize_field("decisions", &self.decisions)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for IsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "decisions",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
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
            type Value = IsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct aserto.authorizer.v2.IsResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<IsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut decisions__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Decisions => {
                            if decisions__.is_some() {
                                return Err(serde::de::Error::duplicate_field("decisions"));
                            }
                            decisions__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(IsResponse {
                    decisions: decisions__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("aserto.authorizer.v2.IsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListPoliciesRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.field_mask.is_some() {
            len += 1;
        }
        if self.policy_instance.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("aserto.authorizer.v2.ListPoliciesRequest", len)?;
        if let Some(v) = self.field_mask.as_ref() {
            struct_ser.serialize_field("fieldMask", v)?;
        }
        if let Some(v) = self.policy_instance.as_ref() {
            struct_ser.serialize_field("policyInstance", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListPoliciesRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "field_mask",
            "fieldMask",
            "policy_instance",
            "policyInstance",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            FieldMask,
            PolicyInstance,
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
                            "fieldMask" | "field_mask" => Ok(GeneratedField::FieldMask),
                            "policyInstance" | "policy_instance" => Ok(GeneratedField::PolicyInstance),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListPoliciesRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct aserto.authorizer.v2.ListPoliciesRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ListPoliciesRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut field_mask__ = None;
                let mut policy_instance__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::FieldMask => {
                            if field_mask__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fieldMask"));
                            }
                            field_mask__ = map.next_value()?;
                        }
                        GeneratedField::PolicyInstance => {
                            if policy_instance__.is_some() {
                                return Err(serde::de::Error::duplicate_field("policyInstance"));
                            }
                            policy_instance__ = map.next_value()?;
                        }
                    }
                }
                Ok(ListPoliciesRequest {
                    field_mask: field_mask__,
                    policy_instance: policy_instance__,
                })
            }
        }
        deserializer.deserialize_struct("aserto.authorizer.v2.ListPoliciesRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListPoliciesResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.result.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("aserto.authorizer.v2.ListPoliciesResponse", len)?;
        if !self.result.is_empty() {
            struct_ser.serialize_field("result", &self.result)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListPoliciesResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "result",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Result,
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
                            "result" => Ok(GeneratedField::Result),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListPoliciesResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct aserto.authorizer.v2.ListPoliciesResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ListPoliciesResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut result__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Result => {
                            if result__.is_some() {
                                return Err(serde::de::Error::duplicate_field("result"));
                            }
                            result__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(ListPoliciesResponse {
                    result: result__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("aserto.authorizer.v2.ListPoliciesResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PathSeparator {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unknown => "PATH_SEPARATOR_UNKNOWN",
            Self::Dot => "PATH_SEPARATOR_DOT",
            Self::Slash => "PATH_SEPARATOR_SLASH",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for PathSeparator {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "PATH_SEPARATOR_UNKNOWN",
            "PATH_SEPARATOR_DOT",
            "PATH_SEPARATOR_SLASH",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PathSeparator;

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
                    .and_then(PathSeparator::from_i32)
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
                    .and_then(PathSeparator::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "PATH_SEPARATOR_UNKNOWN" => Ok(PathSeparator::Unknown),
                    "PATH_SEPARATOR_DOT" => Ok(PathSeparator::Dot),
                    "PATH_SEPARATOR_SLASH" => Ok(PathSeparator::Slash),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for QueryOptions {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.metrics {
            len += 1;
        }
        if self.instrument {
            len += 1;
        }
        if self.trace != 0 {
            len += 1;
        }
        if self.trace_summary {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("aserto.authorizer.v2.QueryOptions", len)?;
        if self.metrics {
            struct_ser.serialize_field("metrics", &self.metrics)?;
        }
        if self.instrument {
            struct_ser.serialize_field("instrument", &self.instrument)?;
        }
        if self.trace != 0 {
            let v = TraceLevel::from_i32(self.trace)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.trace)))?;
            struct_ser.serialize_field("trace", &v)?;
        }
        if self.trace_summary {
            struct_ser.serialize_field("traceSummary", &self.trace_summary)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryOptions {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "metrics",
            "instrument",
            "trace",
            "trace_summary",
            "traceSummary",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Metrics,
            Instrument,
            Trace,
            TraceSummary,
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
                            "metrics" => Ok(GeneratedField::Metrics),
                            "instrument" => Ok(GeneratedField::Instrument),
                            "trace" => Ok(GeneratedField::Trace),
                            "traceSummary" | "trace_summary" => Ok(GeneratedField::TraceSummary),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryOptions;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct aserto.authorizer.v2.QueryOptions")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<QueryOptions, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut metrics__ = None;
                let mut instrument__ = None;
                let mut trace__ = None;
                let mut trace_summary__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Metrics => {
                            if metrics__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metrics"));
                            }
                            metrics__ = Some(map.next_value()?);
                        }
                        GeneratedField::Instrument => {
                            if instrument__.is_some() {
                                return Err(serde::de::Error::duplicate_field("instrument"));
                            }
                            instrument__ = Some(map.next_value()?);
                        }
                        GeneratedField::Trace => {
                            if trace__.is_some() {
                                return Err(serde::de::Error::duplicate_field("trace"));
                            }
                            trace__ = Some(map.next_value::<TraceLevel>()? as i32);
                        }
                        GeneratedField::TraceSummary => {
                            if trace_summary__.is_some() {
                                return Err(serde::de::Error::duplicate_field("traceSummary"));
                            }
                            trace_summary__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(QueryOptions {
                    metrics: metrics__.unwrap_or_default(),
                    instrument: instrument__.unwrap_or_default(),
                    trace: trace__.unwrap_or_default(),
                    trace_summary: trace_summary__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("aserto.authorizer.v2.QueryOptions", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for QueryRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.query.is_empty() {
            len += 1;
        }
        if !self.input.is_empty() {
            len += 1;
        }
        if self.options.is_some() {
            len += 1;
        }
        if self.policy_context.is_some() {
            len += 1;
        }
        if self.identity_context.is_some() {
            len += 1;
        }
        if self.resource_context.is_some() {
            len += 1;
        }
        if self.policy_instance.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("aserto.authorizer.v2.QueryRequest", len)?;
        if !self.query.is_empty() {
            struct_ser.serialize_field("query", &self.query)?;
        }
        if !self.input.is_empty() {
            struct_ser.serialize_field("input", &self.input)?;
        }
        if let Some(v) = self.options.as_ref() {
            struct_ser.serialize_field("options", v)?;
        }
        if let Some(v) = self.policy_context.as_ref() {
            struct_ser.serialize_field("policyContext", v)?;
        }
        if let Some(v) = self.identity_context.as_ref() {
            struct_ser.serialize_field("identityContext", v)?;
        }
        if let Some(v) = self.resource_context.as_ref() {
            struct_ser.serialize_field("resourceContext", v)?;
        }
        if let Some(v) = self.policy_instance.as_ref() {
            struct_ser.serialize_field("policyInstance", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "query",
            "input",
            "options",
            "policy_context",
            "policyContext",
            "identity_context",
            "identityContext",
            "resource_context",
            "resourceContext",
            "policy_instance",
            "policyInstance",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Query,
            Input,
            Options,
            PolicyContext,
            IdentityContext,
            ResourceContext,
            PolicyInstance,
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
                            "query" => Ok(GeneratedField::Query),
                            "input" => Ok(GeneratedField::Input),
                            "options" => Ok(GeneratedField::Options),
                            "policyContext" | "policy_context" => Ok(GeneratedField::PolicyContext),
                            "identityContext" | "identity_context" => Ok(GeneratedField::IdentityContext),
                            "resourceContext" | "resource_context" => Ok(GeneratedField::ResourceContext),
                            "policyInstance" | "policy_instance" => Ok(GeneratedField::PolicyInstance),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct aserto.authorizer.v2.QueryRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<QueryRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut query__ = None;
                let mut input__ = None;
                let mut options__ = None;
                let mut policy_context__ = None;
                let mut identity_context__ = None;
                let mut resource_context__ = None;
                let mut policy_instance__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Query => {
                            if query__.is_some() {
                                return Err(serde::de::Error::duplicate_field("query"));
                            }
                            query__ = Some(map.next_value()?);
                        }
                        GeneratedField::Input => {
                            if input__.is_some() {
                                return Err(serde::de::Error::duplicate_field("input"));
                            }
                            input__ = Some(map.next_value()?);
                        }
                        GeneratedField::Options => {
                            if options__.is_some() {
                                return Err(serde::de::Error::duplicate_field("options"));
                            }
                            options__ = map.next_value()?;
                        }
                        GeneratedField::PolicyContext => {
                            if policy_context__.is_some() {
                                return Err(serde::de::Error::duplicate_field("policyContext"));
                            }
                            policy_context__ = map.next_value()?;
                        }
                        GeneratedField::IdentityContext => {
                            if identity_context__.is_some() {
                                return Err(serde::de::Error::duplicate_field("identityContext"));
                            }
                            identity_context__ = map.next_value()?;
                        }
                        GeneratedField::ResourceContext => {
                            if resource_context__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resourceContext"));
                            }
                            resource_context__ = map.next_value()?;
                        }
                        GeneratedField::PolicyInstance => {
                            if policy_instance__.is_some() {
                                return Err(serde::de::Error::duplicate_field("policyInstance"));
                            }
                            policy_instance__ = map.next_value()?;
                        }
                    }
                }
                Ok(QueryRequest {
                    query: query__.unwrap_or_default(),
                    input: input__.unwrap_or_default(),
                    options: options__,
                    policy_context: policy_context__,
                    identity_context: identity_context__,
                    resource_context: resource_context__,
                    policy_instance: policy_instance__,
                })
            }
        }
        deserializer.deserialize_struct("aserto.authorizer.v2.QueryRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for QueryResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.response.is_some() {
            len += 1;
        }
        if self.metrics.is_some() {
            len += 1;
        }
        if !self.trace.is_empty() {
            len += 1;
        }
        if !self.trace_summary.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("aserto.authorizer.v2.QueryResponse", len)?;
        if let Some(v) = self.response.as_ref() {
            struct_ser.serialize_field("response", v)?;
        }
        if let Some(v) = self.metrics.as_ref() {
            struct_ser.serialize_field("metrics", v)?;
        }
        if !self.trace.is_empty() {
            struct_ser.serialize_field("trace", &self.trace)?;
        }
        if !self.trace_summary.is_empty() {
            struct_ser.serialize_field("traceSummary", &self.trace_summary)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "response",
            "metrics",
            "trace",
            "trace_summary",
            "traceSummary",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Response,
            Metrics,
            Trace,
            TraceSummary,
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
                            "response" => Ok(GeneratedField::Response),
                            "metrics" => Ok(GeneratedField::Metrics),
                            "trace" => Ok(GeneratedField::Trace),
                            "traceSummary" | "trace_summary" => Ok(GeneratedField::TraceSummary),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct aserto.authorizer.v2.QueryResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<QueryResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut response__ = None;
                let mut metrics__ = None;
                let mut trace__ = None;
                let mut trace_summary__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Response => {
                            if response__.is_some() {
                                return Err(serde::de::Error::duplicate_field("response"));
                            }
                            response__ = map.next_value()?;
                        }
                        GeneratedField::Metrics => {
                            if metrics__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metrics"));
                            }
                            metrics__ = map.next_value()?;
                        }
                        GeneratedField::Trace => {
                            if trace__.is_some() {
                                return Err(serde::de::Error::duplicate_field("trace"));
                            }
                            trace__ = Some(map.next_value()?);
                        }
                        GeneratedField::TraceSummary => {
                            if trace_summary__.is_some() {
                                return Err(serde::de::Error::duplicate_field("traceSummary"));
                            }
                            trace_summary__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(QueryResponse {
                    response: response__,
                    metrics: metrics__,
                    trace: trace__.unwrap_or_default(),
                    trace_summary: trace_summary__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("aserto.authorizer.v2.QueryResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for TraceLevel {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unknown => "TRACE_LEVEL_UNKNOWN",
            Self::Off => "TRACE_LEVEL_OFF",
            Self::Full => "TRACE_LEVEL_FULL",
            Self::Notes => "TRACE_LEVEL_NOTES",
            Self::Fails => "TRACE_LEVEL_FAILS",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for TraceLevel {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "TRACE_LEVEL_UNKNOWN",
            "TRACE_LEVEL_OFF",
            "TRACE_LEVEL_FULL",
            "TRACE_LEVEL_NOTES",
            "TRACE_LEVEL_FAILS",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TraceLevel;

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
                    .and_then(TraceLevel::from_i32)
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
                    .and_then(TraceLevel::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "TRACE_LEVEL_UNKNOWN" => Ok(TraceLevel::Unknown),
                    "TRACE_LEVEL_OFF" => Ok(TraceLevel::Off),
                    "TRACE_LEVEL_FULL" => Ok(TraceLevel::Full),
                    "TRACE_LEVEL_NOTES" => Ok(TraceLevel::Notes),
                    "TRACE_LEVEL_FAILS" => Ok(TraceLevel::Fails),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}

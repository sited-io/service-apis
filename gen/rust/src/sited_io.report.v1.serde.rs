// @generated
impl serde::Serialize for CreateReportRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.report_type != 0 {
            len += 1;
        }
        if !self.title.is_empty() {
            len += 1;
        }
        if !self.content.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sited_io.report.v1.CreateReportRequest", len)?;
        if self.report_type != 0 {
            let v = ReportType::try_from(self.report_type)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.report_type)))?;
            struct_ser.serialize_field("reportType", &v)?;
        }
        if !self.title.is_empty() {
            struct_ser.serialize_field("title", &self.title)?;
        }
        if !self.content.is_empty() {
            struct_ser.serialize_field("content", &self.content)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateReportRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "report_type",
            "reportType",
            "title",
            "content",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ReportType,
            Title,
            Content,
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
                            "reportType" | "report_type" => Ok(GeneratedField::ReportType),
                            "title" => Ok(GeneratedField::Title),
                            "content" => Ok(GeneratedField::Content),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateReportRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.report.v1.CreateReportRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateReportRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut report_type__ = None;
                let mut title__ = None;
                let mut content__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ReportType => {
                            if report_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("reportType"));
                            }
                            report_type__ = Some(map_.next_value::<ReportType>()? as i32);
                        }
                        GeneratedField::Title => {
                            if title__.is_some() {
                                return Err(serde::de::Error::duplicate_field("title"));
                            }
                            title__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Content => {
                            if content__.is_some() {
                                return Err(serde::de::Error::duplicate_field("content"));
                            }
                            content__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(CreateReportRequest {
                    report_type: report_type__.unwrap_or_default(),
                    title: title__.unwrap_or_default(),
                    content: content__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sited_io.report.v1.CreateReportRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateReportResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.link.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sited_io.report.v1.CreateReportResponse", len)?;
        if let Some(v) = self.link.as_ref() {
            struct_ser.serialize_field("link", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateReportResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "link",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Link,
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
                            "link" => Ok(GeneratedField::Link),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateReportResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.report.v1.CreateReportResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateReportResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut link__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Link => {
                            if link__.is_some() {
                                return Err(serde::de::Error::duplicate_field("link"));
                            }
                            link__ = map_.next_value()?;
                        }
                    }
                }
                Ok(CreateReportResponse {
                    link: link__,
                })
            }
        }
        deserializer.deserialize_struct("sited_io.report.v1.CreateReportResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ReportType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "REPORT_TYPE_UNSPECIFIED",
            Self::Bug => "REPORT_TYPE_BUG",
            Self::FeatureRequest => "REPORT_TYPE_FEATURE_REQUEST",
            Self::Question => "REPORT_TYPE_QUESTION",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for ReportType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "REPORT_TYPE_UNSPECIFIED",
            "REPORT_TYPE_BUG",
            "REPORT_TYPE_FEATURE_REQUEST",
            "REPORT_TYPE_QUESTION",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ReportType;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "REPORT_TYPE_UNSPECIFIED" => Ok(ReportType::Unspecified),
                    "REPORT_TYPE_BUG" => Ok(ReportType::Bug),
                    "REPORT_TYPE_FEATURE_REQUEST" => Ok(ReportType::FeatureRequest),
                    "REPORT_TYPE_QUESTION" => Ok(ReportType::Question),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}

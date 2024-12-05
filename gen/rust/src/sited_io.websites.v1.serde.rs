// @generated
impl serde::Serialize for CheckDomainStatusRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.domain_id != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sited_io.websites.v1.CheckDomainStatusRequest", len)?;
        if self.domain_id != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("domainId", ToString::to_string(&self.domain_id).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CheckDomainStatusRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "domain_id",
            "domainId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            DomainId,
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
                            "domainId" | "domain_id" => Ok(GeneratedField::DomainId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CheckDomainStatusRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.websites.v1.CheckDomainStatusRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CheckDomainStatusRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut domain_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::DomainId => {
                            if domain_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("domainId"));
                            }
                            domain_id__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(CheckDomainStatusRequest {
                    domain_id: domain_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sited_io.websites.v1.CheckDomainStatusRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CheckDomainStatusResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.domain.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sited_io.websites.v1.CheckDomainStatusResponse", len)?;
        if let Some(v) = self.domain.as_ref() {
            struct_ser.serialize_field("domain", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CheckDomainStatusResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "domain",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Domain,
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
                            "domain" => Ok(GeneratedField::Domain),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CheckDomainStatusResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.websites.v1.CheckDomainStatusResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CheckDomainStatusResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut domain__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Domain => {
                            if domain__.is_some() {
                                return Err(serde::de::Error::duplicate_field("domain"));
                            }
                            domain__ = map_.next_value()?;
                        }
                    }
                }
                Ok(CheckDomainStatusResponse {
                    domain: domain__,
                })
            }
        }
        deserializer.deserialize_struct("sited_io.websites.v1.CheckDomainStatusResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Component {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.component_id.is_empty() {
            len += 1;
        }
        if self.component_type.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sited_io.websites.v1.Component", len)?;
        if !self.component_id.is_empty() {
            struct_ser.serialize_field("componentId", &self.component_id)?;
        }
        if let Some(v) = self.component_type.as_ref() {
            struct_ser.serialize_field("componentType", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Component {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "component_id",
            "componentId",
            "component_type",
            "componentType",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ComponentId,
            ComponentType,
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
                            "componentId" | "component_id" => Ok(GeneratedField::ComponentId),
                            "componentType" | "component_type" => Ok(GeneratedField::ComponentType),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Component;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.websites.v1.Component")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Component, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut component_id__ = None;
                let mut component_type__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ComponentId => {
                            if component_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("componentId"));
                            }
                            component_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ComponentType => {
                            if component_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("componentType"));
                            }
                            component_type__ = map_.next_value()?;
                        }
                    }
                }
                Ok(Component {
                    component_id: component_id__.unwrap_or_default(),
                    component_type: component_type__,
                })
            }
        }
        deserializer.deserialize_struct("sited_io.websites.v1.Component", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ComponentType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.inner.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sited_io.websites.v1.ComponentType", len)?;
        if let Some(v) = self.inner.as_ref() {
            match v {
                component_type::Inner::Header(v) => {
                    struct_ser.serialize_field("header", v)?;
                }
                component_type::Inner::Paragraph(v) => {
                    struct_ser.serialize_field("paragraph", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ComponentType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "header",
            "paragraph",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Header,
            Paragraph,
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
                            "header" => Ok(GeneratedField::Header),
                            "paragraph" => Ok(GeneratedField::Paragraph),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ComponentType;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.websites.v1.ComponentType")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ComponentType, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut inner__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Header => {
                            if inner__.is_some() {
                                return Err(serde::de::Error::duplicate_field("header"));
                            }
                            inner__ = map_.next_value::<::std::option::Option<_>>()?.map(component_type::Inner::Header)
;
                        }
                        GeneratedField::Paragraph => {
                            if inner__.is_some() {
                                return Err(serde::de::Error::duplicate_field("paragraph"));
                            }
                            inner__ = map_.next_value::<::std::option::Option<_>>()?.map(component_type::Inner::Paragraph)
;
                        }
                    }
                }
                Ok(ComponentType {
                    inner: inner__,
                })
            }
        }
        deserializer.deserialize_struct("sited_io.websites.v1.ComponentType", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateDomainRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.website_id.is_empty() {
            len += 1;
        }
        if !self.domain.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sited_io.websites.v1.CreateDomainRequest", len)?;
        if !self.website_id.is_empty() {
            struct_ser.serialize_field("websiteId", &self.website_id)?;
        }
        if !self.domain.is_empty() {
            struct_ser.serialize_field("domain", &self.domain)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateDomainRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "website_id",
            "websiteId",
            "domain",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            WebsiteId,
            Domain,
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
                            "websiteId" | "website_id" => Ok(GeneratedField::WebsiteId),
                            "domain" => Ok(GeneratedField::Domain),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateDomainRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.websites.v1.CreateDomainRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateDomainRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut website_id__ = None;
                let mut domain__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::WebsiteId => {
                            if website_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("websiteId"));
                            }
                            website_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Domain => {
                            if domain__.is_some() {
                                return Err(serde::de::Error::duplicate_field("domain"));
                            }
                            domain__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(CreateDomainRequest {
                    website_id: website_id__.unwrap_or_default(),
                    domain: domain__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sited_io.websites.v1.CreateDomainRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateDomainResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.domain.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sited_io.websites.v1.CreateDomainResponse", len)?;
        if let Some(v) = self.domain.as_ref() {
            struct_ser.serialize_field("domain", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateDomainResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "domain",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Domain,
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
                            "domain" => Ok(GeneratedField::Domain),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateDomainResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.websites.v1.CreateDomainResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateDomainResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut domain__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Domain => {
                            if domain__.is_some() {
                                return Err(serde::de::Error::duplicate_field("domain"));
                            }
                            domain__ = map_.next_value()?;
                        }
                    }
                }
                Ok(CreateDomainResponse {
                    domain: domain__,
                })
            }
        }
        deserializer.deserialize_struct("sited_io.websites.v1.CreateDomainResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreatePageRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.website_id.is_empty() {
            len += 1;
        }
        if self.page_type != 0 {
            len += 1;
        }
        if !self.content_id.is_empty() {
            len += 1;
        }
        if !self.title.is_empty() {
            len += 1;
        }
        if self.is_home_page {
            len += 1;
        }
        if self.path.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sited_io.websites.v1.CreatePageRequest", len)?;
        if !self.website_id.is_empty() {
            struct_ser.serialize_field("websiteId", &self.website_id)?;
        }
        if self.page_type != 0 {
            let v = PageType::try_from(self.page_type)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.page_type)))?;
            struct_ser.serialize_field("pageType", &v)?;
        }
        if !self.content_id.is_empty() {
            struct_ser.serialize_field("contentId", &self.content_id)?;
        }
        if !self.title.is_empty() {
            struct_ser.serialize_field("title", &self.title)?;
        }
        if self.is_home_page {
            struct_ser.serialize_field("isHomePage", &self.is_home_page)?;
        }
        if let Some(v) = self.path.as_ref() {
            struct_ser.serialize_field("path", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreatePageRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "website_id",
            "websiteId",
            "page_type",
            "pageType",
            "content_id",
            "contentId",
            "title",
            "is_home_page",
            "isHomePage",
            "path",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            WebsiteId,
            PageType,
            ContentId,
            Title,
            IsHomePage,
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
                            "websiteId" | "website_id" => Ok(GeneratedField::WebsiteId),
                            "pageType" | "page_type" => Ok(GeneratedField::PageType),
                            "contentId" | "content_id" => Ok(GeneratedField::ContentId),
                            "title" => Ok(GeneratedField::Title),
                            "isHomePage" | "is_home_page" => Ok(GeneratedField::IsHomePage),
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
            type Value = CreatePageRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.websites.v1.CreatePageRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreatePageRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut website_id__ = None;
                let mut page_type__ = None;
                let mut content_id__ = None;
                let mut title__ = None;
                let mut is_home_page__ = None;
                let mut path__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::WebsiteId => {
                            if website_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("websiteId"));
                            }
                            website_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::PageType => {
                            if page_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pageType"));
                            }
                            page_type__ = Some(map_.next_value::<PageType>()? as i32);
                        }
                        GeneratedField::ContentId => {
                            if content_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("contentId"));
                            }
                            content_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Title => {
                            if title__.is_some() {
                                return Err(serde::de::Error::duplicate_field("title"));
                            }
                            title__ = Some(map_.next_value()?);
                        }
                        GeneratedField::IsHomePage => {
                            if is_home_page__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isHomePage"));
                            }
                            is_home_page__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Path => {
                            if path__.is_some() {
                                return Err(serde::de::Error::duplicate_field("path"));
                            }
                            path__ = map_.next_value()?;
                        }
                    }
                }
                Ok(CreatePageRequest {
                    website_id: website_id__.unwrap_or_default(),
                    page_type: page_type__.unwrap_or_default(),
                    content_id: content_id__.unwrap_or_default(),
                    title: title__.unwrap_or_default(),
                    is_home_page: is_home_page__.unwrap_or_default(),
                    path: path__,
                })
            }
        }
        deserializer.deserialize_struct("sited_io.websites.v1.CreatePageRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreatePageResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.page.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sited_io.websites.v1.CreatePageResponse", len)?;
        if let Some(v) = self.page.as_ref() {
            struct_ser.serialize_field("page", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreatePageResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "page",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Page,
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
                            "page" => Ok(GeneratedField::Page),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreatePageResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.websites.v1.CreatePageResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreatePageResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut page__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Page => {
                            if page__.is_some() {
                                return Err(serde::de::Error::duplicate_field("page"));
                            }
                            page__ = map_.next_value()?;
                        }
                    }
                }
                Ok(CreatePageResponse {
                    page: page__,
                })
            }
        }
        deserializer.deserialize_struct("sited_io.websites.v1.CreatePageResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateWebsiteRequest {
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
        let mut struct_ser = serializer.serialize_struct("sited_io.websites.v1.CreateWebsiteRequest", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateWebsiteRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
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
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateWebsiteRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.websites.v1.CreateWebsiteRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateWebsiteRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(CreateWebsiteRequest {
                    name: name__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sited_io.websites.v1.CreateWebsiteRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateWebsiteResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.website.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sited_io.websites.v1.CreateWebsiteResponse", len)?;
        if let Some(v) = self.website.as_ref() {
            struct_ser.serialize_field("website", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateWebsiteResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "website",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Website,
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
                            "website" => Ok(GeneratedField::Website),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateWebsiteResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.websites.v1.CreateWebsiteResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateWebsiteResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut website__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Website => {
                            if website__.is_some() {
                                return Err(serde::de::Error::duplicate_field("website"));
                            }
                            website__ = map_.next_value()?;
                        }
                    }
                }
                Ok(CreateWebsiteResponse {
                    website: website__,
                })
            }
        }
        deserializer.deserialize_struct("sited_io.websites.v1.CreateWebsiteResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CustomizationResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.primary_color.is_some() {
            len += 1;
        }
        if self.secondary_color.is_some() {
            len += 1;
        }
        if self.logo_image_url.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sited_io.websites.v1.CustomizationResponse", len)?;
        if let Some(v) = self.primary_color.as_ref() {
            struct_ser.serialize_field("primaryColor", v)?;
        }
        if let Some(v) = self.secondary_color.as_ref() {
            struct_ser.serialize_field("secondaryColor", v)?;
        }
        if let Some(v) = self.logo_image_url.as_ref() {
            struct_ser.serialize_field("logoImageUrl", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CustomizationResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "primary_color",
            "primaryColor",
            "secondary_color",
            "secondaryColor",
            "logo_image_url",
            "logoImageUrl",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PrimaryColor,
            SecondaryColor,
            LogoImageUrl,
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
                            "primaryColor" | "primary_color" => Ok(GeneratedField::PrimaryColor),
                            "secondaryColor" | "secondary_color" => Ok(GeneratedField::SecondaryColor),
                            "logoImageUrl" | "logo_image_url" => Ok(GeneratedField::LogoImageUrl),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CustomizationResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.websites.v1.CustomizationResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CustomizationResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut primary_color__ = None;
                let mut secondary_color__ = None;
                let mut logo_image_url__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::PrimaryColor => {
                            if primary_color__.is_some() {
                                return Err(serde::de::Error::duplicate_field("primaryColor"));
                            }
                            primary_color__ = map_.next_value()?;
                        }
                        GeneratedField::SecondaryColor => {
                            if secondary_color__.is_some() {
                                return Err(serde::de::Error::duplicate_field("secondaryColor"));
                            }
                            secondary_color__ = map_.next_value()?;
                        }
                        GeneratedField::LogoImageUrl => {
                            if logo_image_url__.is_some() {
                                return Err(serde::de::Error::duplicate_field("logoImageUrl"));
                            }
                            logo_image_url__ = map_.next_value()?;
                        }
                    }
                }
                Ok(CustomizationResponse {
                    primary_color: primary_color__,
                    secondary_color: secondary_color__,
                    logo_image_url: logo_image_url__,
                })
            }
        }
        deserializer.deserialize_struct("sited_io.websites.v1.CustomizationResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeleteDomainRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.domain_id != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sited_io.websites.v1.DeleteDomainRequest", len)?;
        if self.domain_id != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("domainId", ToString::to_string(&self.domain_id).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeleteDomainRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "domain_id",
            "domainId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            DomainId,
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
                            "domainId" | "domain_id" => Ok(GeneratedField::DomainId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DeleteDomainRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.websites.v1.DeleteDomainRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DeleteDomainRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut domain_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::DomainId => {
                            if domain_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("domainId"));
                            }
                            domain_id__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(DeleteDomainRequest {
                    domain_id: domain_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sited_io.websites.v1.DeleteDomainRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeleteDomainResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("sited_io.websites.v1.DeleteDomainResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeleteDomainResponse {
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
            type Value = DeleteDomainResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.websites.v1.DeleteDomainResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DeleteDomainResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(DeleteDomainResponse {
                })
            }
        }
        deserializer.deserialize_struct("sited_io.websites.v1.DeleteDomainResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeletePageRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.page_id != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sited_io.websites.v1.DeletePageRequest", len)?;
        if self.page_id != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("pageId", ToString::to_string(&self.page_id).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeletePageRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "page_id",
            "pageId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PageId,
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
                            "pageId" | "page_id" => Ok(GeneratedField::PageId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DeletePageRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.websites.v1.DeletePageRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DeletePageRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut page_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::PageId => {
                            if page_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pageId"));
                            }
                            page_id__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(DeletePageRequest {
                    page_id: page_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sited_io.websites.v1.DeletePageRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeletePageResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("sited_io.websites.v1.DeletePageResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeletePageResponse {
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
            type Value = DeletePageResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.websites.v1.DeletePageResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DeletePageResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(DeletePageResponse {
                })
            }
        }
        deserializer.deserialize_struct("sited_io.websites.v1.DeletePageResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeleteWebsiteRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.website_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sited_io.websites.v1.DeleteWebsiteRequest", len)?;
        if !self.website_id.is_empty() {
            struct_ser.serialize_field("websiteId", &self.website_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeleteWebsiteRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "website_id",
            "websiteId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            WebsiteId,
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
                            "websiteId" | "website_id" => Ok(GeneratedField::WebsiteId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DeleteWebsiteRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.websites.v1.DeleteWebsiteRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DeleteWebsiteRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut website_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::WebsiteId => {
                            if website_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("websiteId"));
                            }
                            website_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(DeleteWebsiteRequest {
                    website_id: website_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sited_io.websites.v1.DeleteWebsiteRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeleteWebsiteResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("sited_io.websites.v1.DeleteWebsiteResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeleteWebsiteResponse {
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
            type Value = DeleteWebsiteResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.websites.v1.DeleteWebsiteResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DeleteWebsiteResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(DeleteWebsiteResponse {
                })
            }
        }
        deserializer.deserialize_struct("sited_io.websites.v1.DeleteWebsiteResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DomainResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.domain_id != 0 {
            len += 1;
        }
        if !self.domain.is_empty() {
            len += 1;
        }
        if self.status != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sited_io.websites.v1.DomainResponse", len)?;
        if self.domain_id != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("domainId", ToString::to_string(&self.domain_id).as_str())?;
        }
        if !self.domain.is_empty() {
            struct_ser.serialize_field("domain", &self.domain)?;
        }
        if self.status != 0 {
            let v = DomainStatus::try_from(self.status)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.status)))?;
            struct_ser.serialize_field("status", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DomainResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "domain_id",
            "domainId",
            "domain",
            "status",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            DomainId,
            Domain,
            Status,
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
                            "domainId" | "domain_id" => Ok(GeneratedField::DomainId),
                            "domain" => Ok(GeneratedField::Domain),
                            "status" => Ok(GeneratedField::Status),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DomainResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.websites.v1.DomainResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DomainResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut domain_id__ = None;
                let mut domain__ = None;
                let mut status__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::DomainId => {
                            if domain_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("domainId"));
                            }
                            domain_id__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Domain => {
                            if domain__.is_some() {
                                return Err(serde::de::Error::duplicate_field("domain"));
                            }
                            domain__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Status => {
                            if status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            status__ = Some(map_.next_value::<DomainStatus>()? as i32);
                        }
                    }
                }
                Ok(DomainResponse {
                    domain_id: domain_id__.unwrap_or_default(),
                    domain: domain__.unwrap_or_default(),
                    status: status__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sited_io.websites.v1.DomainResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DomainStatus {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "DOMAIN_STATUS_UNSPECIFIED",
            Self::Internal => "DOMAIN_STATUS_INTERNAL",
            Self::Pending => "DOMAIN_STATUS_PENDING",
            Self::Active => "DOMAIN_STATUS_ACTIVE",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for DomainStatus {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "DOMAIN_STATUS_UNSPECIFIED",
            "DOMAIN_STATUS_INTERNAL",
            "DOMAIN_STATUS_PENDING",
            "DOMAIN_STATUS_ACTIVE",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DomainStatus;

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
                    "DOMAIN_STATUS_UNSPECIFIED" => Ok(DomainStatus::Unspecified),
                    "DOMAIN_STATUS_INTERNAL" => Ok(DomainStatus::Internal),
                    "DOMAIN_STATUS_PENDING" => Ok(DomainStatus::Pending),
                    "DOMAIN_STATUS_ACTIVE" => Ok(DomainStatus::Active),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for GetPageRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.page_id.is_some() {
            len += 1;
        }
        if self.website_id.is_some() {
            len += 1;
        }
        if self.path.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sited_io.websites.v1.GetPageRequest", len)?;
        if let Some(v) = self.page_id.as_ref() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("pageId", ToString::to_string(&v).as_str())?;
        }
        if let Some(v) = self.website_id.as_ref() {
            struct_ser.serialize_field("websiteId", v)?;
        }
        if let Some(v) = self.path.as_ref() {
            struct_ser.serialize_field("path", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetPageRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "page_id",
            "pageId",
            "website_id",
            "websiteId",
            "path",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PageId,
            WebsiteId,
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
                            "pageId" | "page_id" => Ok(GeneratedField::PageId),
                            "websiteId" | "website_id" => Ok(GeneratedField::WebsiteId),
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
            type Value = GetPageRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.websites.v1.GetPageRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetPageRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut page_id__ = None;
                let mut website_id__ = None;
                let mut path__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::PageId => {
                            if page_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pageId"));
                            }
                            page_id__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::WebsiteId => {
                            if website_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("websiteId"));
                            }
                            website_id__ = map_.next_value()?;
                        }
                        GeneratedField::Path => {
                            if path__.is_some() {
                                return Err(serde::de::Error::duplicate_field("path"));
                            }
                            path__ = map_.next_value()?;
                        }
                    }
                }
                Ok(GetPageRequest {
                    page_id: page_id__,
                    website_id: website_id__,
                    path: path__,
                })
            }
        }
        deserializer.deserialize_struct("sited_io.websites.v1.GetPageRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetPageResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.page.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sited_io.websites.v1.GetPageResponse", len)?;
        if let Some(v) = self.page.as_ref() {
            struct_ser.serialize_field("page", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetPageResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "page",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Page,
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
                            "page" => Ok(GeneratedField::Page),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetPageResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.websites.v1.GetPageResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetPageResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut page__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Page => {
                            if page__.is_some() {
                                return Err(serde::de::Error::duplicate_field("page"));
                            }
                            page__ = map_.next_value()?;
                        }
                    }
                }
                Ok(GetPageResponse {
                    page: page__,
                })
            }
        }
        deserializer.deserialize_struct("sited_io.websites.v1.GetPageResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetStaticPageRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.page_id != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sited_io.websites.v1.GetStaticPageRequest", len)?;
        if self.page_id != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("pageId", ToString::to_string(&self.page_id).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetStaticPageRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "page_id",
            "pageId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PageId,
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
                            "pageId" | "page_id" => Ok(GeneratedField::PageId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetStaticPageRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.websites.v1.GetStaticPageRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetStaticPageRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut page_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::PageId => {
                            if page_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pageId"));
                            }
                            page_id__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(GetStaticPageRequest {
                    page_id: page_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sited_io.websites.v1.GetStaticPageRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetStaticPageResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.static_page.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sited_io.websites.v1.GetStaticPageResponse", len)?;
        if let Some(v) = self.static_page.as_ref() {
            struct_ser.serialize_field("staticPage", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetStaticPageResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "static_page",
            "staticPage",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            StaticPage,
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
                            "staticPage" | "static_page" => Ok(GeneratedField::StaticPage),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetStaticPageResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.websites.v1.GetStaticPageResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetStaticPageResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut static_page__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::StaticPage => {
                            if static_page__.is_some() {
                                return Err(serde::de::Error::duplicate_field("staticPage"));
                            }
                            static_page__ = map_.next_value()?;
                        }
                    }
                }
                Ok(GetStaticPageResponse {
                    static_page: static_page__,
                })
            }
        }
        deserializer.deserialize_struct("sited_io.websites.v1.GetStaticPageResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetWebsiteRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.website_id.is_some() {
            len += 1;
        }
        if self.domain.is_some() {
            len += 1;
        }
        if self.client_id.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sited_io.websites.v1.GetWebsiteRequest", len)?;
        if let Some(v) = self.website_id.as_ref() {
            struct_ser.serialize_field("websiteId", v)?;
        }
        if let Some(v) = self.domain.as_ref() {
            struct_ser.serialize_field("domain", v)?;
        }
        if let Some(v) = self.client_id.as_ref() {
            struct_ser.serialize_field("clientId", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetWebsiteRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "website_id",
            "websiteId",
            "domain",
            "client_id",
            "clientId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            WebsiteId,
            Domain,
            ClientId,
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
                            "websiteId" | "website_id" => Ok(GeneratedField::WebsiteId),
                            "domain" => Ok(GeneratedField::Domain),
                            "clientId" | "client_id" => Ok(GeneratedField::ClientId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetWebsiteRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.websites.v1.GetWebsiteRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetWebsiteRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut website_id__ = None;
                let mut domain__ = None;
                let mut client_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::WebsiteId => {
                            if website_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("websiteId"));
                            }
                            website_id__ = map_.next_value()?;
                        }
                        GeneratedField::Domain => {
                            if domain__.is_some() {
                                return Err(serde::de::Error::duplicate_field("domain"));
                            }
                            domain__ = map_.next_value()?;
                        }
                        GeneratedField::ClientId => {
                            if client_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("clientId"));
                            }
                            client_id__ = map_.next_value()?;
                        }
                    }
                }
                Ok(GetWebsiteRequest {
                    website_id: website_id__,
                    domain: domain__,
                    client_id: client_id__,
                })
            }
        }
        deserializer.deserialize_struct("sited_io.websites.v1.GetWebsiteRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetWebsiteResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.website.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sited_io.websites.v1.GetWebsiteResponse", len)?;
        if let Some(v) = self.website.as_ref() {
            struct_ser.serialize_field("website", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetWebsiteResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "website",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Website,
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
                            "website" => Ok(GeneratedField::Website),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetWebsiteResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.websites.v1.GetWebsiteResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetWebsiteResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut website__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Website => {
                            if website__.is_some() {
                                return Err(serde::de::Error::duplicate_field("website"));
                            }
                            website__ = map_.next_value()?;
                        }
                    }
                }
                Ok(GetWebsiteResponse {
                    website: website__,
                })
            }
        }
        deserializer.deserialize_struct("sited_io.websites.v1.GetWebsiteResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for HeaderComponent {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.level != 0 {
            len += 1;
        }
        if !self.content.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sited_io.websites.v1.HeaderComponent", len)?;
        if self.level != 0 {
            struct_ser.serialize_field("level", &self.level)?;
        }
        if !self.content.is_empty() {
            struct_ser.serialize_field("content", &self.content)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for HeaderComponent {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "level",
            "content",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Level,
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
                            "level" => Ok(GeneratedField::Level),
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
            type Value = HeaderComponent;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.websites.v1.HeaderComponent")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<HeaderComponent, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut level__ = None;
                let mut content__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Level => {
                            if level__.is_some() {
                                return Err(serde::de::Error::duplicate_field("level"));
                            }
                            level__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Content => {
                            if content__.is_some() {
                                return Err(serde::de::Error::duplicate_field("content"));
                            }
                            content__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(HeaderComponent {
                    level: level__.unwrap_or_default(),
                    content: content__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sited_io.websites.v1.HeaderComponent", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for InlineElement {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.element_type.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sited_io.websites.v1.InlineElement", len)?;
        if let Some(v) = self.element_type.as_ref() {
            match v {
                inline_element::ElementType::Text(v) => {
                    struct_ser.serialize_field("text", v)?;
                }
                inline_element::ElementType::Link(v) => {
                    struct_ser.serialize_field("link", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for InlineElement {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "text",
            "link",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Text,
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
                            "text" => Ok(GeneratedField::Text),
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
            type Value = InlineElement;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.websites.v1.InlineElement")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<InlineElement, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut element_type__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Text => {
                            if element_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("text"));
                            }
                            element_type__ = map_.next_value::<::std::option::Option<_>>()?.map(inline_element::ElementType::Text)
;
                        }
                        GeneratedField::Link => {
                            if element_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("link"));
                            }
                            element_type__ = map_.next_value::<::std::option::Option<_>>()?.map(inline_element::ElementType::Link)
;
                        }
                    }
                }
                Ok(InlineElement {
                    element_type: element_type__,
                })
            }
        }
        deserializer.deserialize_struct("sited_io.websites.v1.InlineElement", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for LinkElement {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.text.is_empty() {
            len += 1;
        }
        if !self.url.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sited_io.websites.v1.LinkElement", len)?;
        if !self.text.is_empty() {
            struct_ser.serialize_field("text", &self.text)?;
        }
        if !self.url.is_empty() {
            struct_ser.serialize_field("url", &self.url)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for LinkElement {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "text",
            "url",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Text,
            Url,
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
                            "text" => Ok(GeneratedField::Text),
                            "url" => Ok(GeneratedField::Url),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = LinkElement;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.websites.v1.LinkElement")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<LinkElement, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut text__ = None;
                let mut url__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Text => {
                            if text__.is_some() {
                                return Err(serde::de::Error::duplicate_field("text"));
                            }
                            text__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Url => {
                            if url__.is_some() {
                                return Err(serde::de::Error::duplicate_field("url"));
                            }
                            url__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(LinkElement {
                    text: text__.unwrap_or_default(),
                    url: url__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sited_io.websites.v1.LinkElement", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListPagesRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.website_id.is_some() {
            len += 1;
        }
        if self.pagination.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sited_io.websites.v1.ListPagesRequest", len)?;
        if let Some(v) = self.website_id.as_ref() {
            struct_ser.serialize_field("websiteId", v)?;
        }
        if let Some(v) = self.pagination.as_ref() {
            struct_ser.serialize_field("pagination", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListPagesRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "website_id",
            "websiteId",
            "pagination",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            WebsiteId,
            Pagination,
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
                            "websiteId" | "website_id" => Ok(GeneratedField::WebsiteId),
                            "pagination" => Ok(GeneratedField::Pagination),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListPagesRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.websites.v1.ListPagesRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListPagesRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut website_id__ = None;
                let mut pagination__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::WebsiteId => {
                            if website_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("websiteId"));
                            }
                            website_id__ = map_.next_value()?;
                        }
                        GeneratedField::Pagination => {
                            if pagination__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pagination"));
                            }
                            pagination__ = map_.next_value()?;
                        }
                    }
                }
                Ok(ListPagesRequest {
                    website_id: website_id__,
                    pagination: pagination__,
                })
            }
        }
        deserializer.deserialize_struct("sited_io.websites.v1.ListPagesRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListPagesResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.pages.is_empty() {
            len += 1;
        }
        if self.pagination.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sited_io.websites.v1.ListPagesResponse", len)?;
        if !self.pages.is_empty() {
            struct_ser.serialize_field("pages", &self.pages)?;
        }
        if let Some(v) = self.pagination.as_ref() {
            struct_ser.serialize_field("pagination", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListPagesResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "pages",
            "pagination",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Pages,
            Pagination,
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
                            "pages" => Ok(GeneratedField::Pages),
                            "pagination" => Ok(GeneratedField::Pagination),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListPagesResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.websites.v1.ListPagesResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListPagesResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut pages__ = None;
                let mut pagination__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Pages => {
                            if pages__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pages"));
                            }
                            pages__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Pagination => {
                            if pagination__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pagination"));
                            }
                            pagination__ = map_.next_value()?;
                        }
                    }
                }
                Ok(ListPagesResponse {
                    pages: pages__.unwrap_or_default(),
                    pagination: pagination__,
                })
            }
        }
        deserializer.deserialize_struct("sited_io.websites.v1.ListPagesResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListWebsitesRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.user_id.is_some() {
            len += 1;
        }
        if self.pagination.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sited_io.websites.v1.ListWebsitesRequest", len)?;
        if let Some(v) = self.user_id.as_ref() {
            struct_ser.serialize_field("userId", v)?;
        }
        if let Some(v) = self.pagination.as_ref() {
            struct_ser.serialize_field("pagination", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListWebsitesRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "user_id",
            "userId",
            "pagination",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            UserId,
            Pagination,
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
                            "userId" | "user_id" => Ok(GeneratedField::UserId),
                            "pagination" => Ok(GeneratedField::Pagination),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListWebsitesRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.websites.v1.ListWebsitesRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListWebsitesRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut user_id__ = None;
                let mut pagination__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::UserId => {
                            if user_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userId"));
                            }
                            user_id__ = map_.next_value()?;
                        }
                        GeneratedField::Pagination => {
                            if pagination__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pagination"));
                            }
                            pagination__ = map_.next_value()?;
                        }
                    }
                }
                Ok(ListWebsitesRequest {
                    user_id: user_id__,
                    pagination: pagination__,
                })
            }
        }
        deserializer.deserialize_struct("sited_io.websites.v1.ListWebsitesRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListWebsitesResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.websites.is_empty() {
            len += 1;
        }
        if self.pagination.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sited_io.websites.v1.ListWebsitesResponse", len)?;
        if !self.websites.is_empty() {
            struct_ser.serialize_field("websites", &self.websites)?;
        }
        if let Some(v) = self.pagination.as_ref() {
            struct_ser.serialize_field("pagination", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListWebsitesResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "websites",
            "pagination",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Websites,
            Pagination,
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
                            "websites" => Ok(GeneratedField::Websites),
                            "pagination" => Ok(GeneratedField::Pagination),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListWebsitesResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.websites.v1.ListWebsitesResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListWebsitesResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut websites__ = None;
                let mut pagination__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Websites => {
                            if websites__.is_some() {
                                return Err(serde::de::Error::duplicate_field("websites"));
                            }
                            websites__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Pagination => {
                            if pagination__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pagination"));
                            }
                            pagination__ = map_.next_value()?;
                        }
                    }
                }
                Ok(ListWebsitesResponse {
                    websites: websites__.unwrap_or_default(),
                    pagination: pagination__,
                })
            }
        }
        deserializer.deserialize_struct("sited_io.websites.v1.ListWebsitesResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PageResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.page_id != 0 {
            len += 1;
        }
        if self.page_type != 0 {
            len += 1;
        }
        if !self.content_id.is_empty() {
            len += 1;
        }
        if !self.title.is_empty() {
            len += 1;
        }
        if self.is_home_page {
            len += 1;
        }
        if !self.path.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sited_io.websites.v1.PageResponse", len)?;
        if self.page_id != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("pageId", ToString::to_string(&self.page_id).as_str())?;
        }
        if self.page_type != 0 {
            let v = PageType::try_from(self.page_type)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.page_type)))?;
            struct_ser.serialize_field("pageType", &v)?;
        }
        if !self.content_id.is_empty() {
            struct_ser.serialize_field("contentId", &self.content_id)?;
        }
        if !self.title.is_empty() {
            struct_ser.serialize_field("title", &self.title)?;
        }
        if self.is_home_page {
            struct_ser.serialize_field("isHomePage", &self.is_home_page)?;
        }
        if !self.path.is_empty() {
            struct_ser.serialize_field("path", &self.path)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PageResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "page_id",
            "pageId",
            "page_type",
            "pageType",
            "content_id",
            "contentId",
            "title",
            "is_home_page",
            "isHomePage",
            "path",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PageId,
            PageType,
            ContentId,
            Title,
            IsHomePage,
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
                            "pageId" | "page_id" => Ok(GeneratedField::PageId),
                            "pageType" | "page_type" => Ok(GeneratedField::PageType),
                            "contentId" | "content_id" => Ok(GeneratedField::ContentId),
                            "title" => Ok(GeneratedField::Title),
                            "isHomePage" | "is_home_page" => Ok(GeneratedField::IsHomePage),
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
            type Value = PageResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.websites.v1.PageResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<PageResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut page_id__ = None;
                let mut page_type__ = None;
                let mut content_id__ = None;
                let mut title__ = None;
                let mut is_home_page__ = None;
                let mut path__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::PageId => {
                            if page_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pageId"));
                            }
                            page_id__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::PageType => {
                            if page_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pageType"));
                            }
                            page_type__ = Some(map_.next_value::<PageType>()? as i32);
                        }
                        GeneratedField::ContentId => {
                            if content_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("contentId"));
                            }
                            content_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Title => {
                            if title__.is_some() {
                                return Err(serde::de::Error::duplicate_field("title"));
                            }
                            title__ = Some(map_.next_value()?);
                        }
                        GeneratedField::IsHomePage => {
                            if is_home_page__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isHomePage"));
                            }
                            is_home_page__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Path => {
                            if path__.is_some() {
                                return Err(serde::de::Error::duplicate_field("path"));
                            }
                            path__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(PageResponse {
                    page_id: page_id__.unwrap_or_default(),
                    page_type: page_type__.unwrap_or_default(),
                    content_id: content_id__.unwrap_or_default(),
                    title: title__.unwrap_or_default(),
                    is_home_page: is_home_page__.unwrap_or_default(),
                    path: path__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sited_io.websites.v1.PageResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PageType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "PAGE_TYPE_UNSPECIFIED",
            Self::Static => "PAGE_TYPE_STATIC",
            Self::Shop => "PAGE_TYPE_SHOP",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for PageType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "PAGE_TYPE_UNSPECIFIED",
            "PAGE_TYPE_STATIC",
            "PAGE_TYPE_SHOP",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PageType;

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
                    "PAGE_TYPE_UNSPECIFIED" => Ok(PageType::Unspecified),
                    "PAGE_TYPE_STATIC" => Ok(PageType::Static),
                    "PAGE_TYPE_SHOP" => Ok(PageType::Shop),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for ParagraphComponent {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.content.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sited_io.websites.v1.ParagraphComponent", len)?;
        if !self.content.is_empty() {
            struct_ser.serialize_field("content", &self.content)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ParagraphComponent {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "content",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
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
            type Value = ParagraphComponent;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.websites.v1.ParagraphComponent")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ParagraphComponent, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut content__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Content => {
                            if content__.is_some() {
                                return Err(serde::de::Error::duplicate_field("content"));
                            }
                            content__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ParagraphComponent {
                    content: content__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sited_io.websites.v1.ParagraphComponent", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PutLogoImageRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.website_id.is_empty() {
            len += 1;
        }
        if self.image.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sited_io.websites.v1.PutLogoImageRequest", len)?;
        if !self.website_id.is_empty() {
            struct_ser.serialize_field("websiteId", &self.website_id)?;
        }
        if let Some(v) = self.image.as_ref() {
            struct_ser.serialize_field("image", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PutLogoImageRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "website_id",
            "websiteId",
            "image",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            WebsiteId,
            Image,
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
                            "websiteId" | "website_id" => Ok(GeneratedField::WebsiteId),
                            "image" => Ok(GeneratedField::Image),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PutLogoImageRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.websites.v1.PutLogoImageRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<PutLogoImageRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut website_id__ = None;
                let mut image__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::WebsiteId => {
                            if website_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("websiteId"));
                            }
                            website_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Image => {
                            if image__.is_some() {
                                return Err(serde::de::Error::duplicate_field("image"));
                            }
                            image__ = map_.next_value()?;
                        }
                    }
                }
                Ok(PutLogoImageRequest {
                    website_id: website_id__.unwrap_or_default(),
                    image: image__,
                })
            }
        }
        deserializer.deserialize_struct("sited_io.websites.v1.PutLogoImageRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PutLogoImageResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("sited_io.websites.v1.PutLogoImageResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PutLogoImageResponse {
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
            type Value = PutLogoImageResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.websites.v1.PutLogoImageResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<PutLogoImageResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(PutLogoImageResponse {
                })
            }
        }
        deserializer.deserialize_struct("sited_io.websites.v1.PutLogoImageResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RemoveLogoImageRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.website_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sited_io.websites.v1.RemoveLogoImageRequest", len)?;
        if !self.website_id.is_empty() {
            struct_ser.serialize_field("websiteId", &self.website_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RemoveLogoImageRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "website_id",
            "websiteId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            WebsiteId,
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
                            "websiteId" | "website_id" => Ok(GeneratedField::WebsiteId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RemoveLogoImageRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.websites.v1.RemoveLogoImageRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RemoveLogoImageRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut website_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::WebsiteId => {
                            if website_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("websiteId"));
                            }
                            website_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(RemoveLogoImageRequest {
                    website_id: website_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sited_io.websites.v1.RemoveLogoImageRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RemoveLogoImageResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("sited_io.websites.v1.RemoveLogoImageResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RemoveLogoImageResponse {
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
            type Value = RemoveLogoImageResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.websites.v1.RemoveLogoImageResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RemoveLogoImageResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(RemoveLogoImageResponse {
                })
            }
        }
        deserializer.deserialize_struct("sited_io.websites.v1.RemoveLogoImageResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for StaticPageResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.page_id != 0 {
            len += 1;
        }
        if !self.website_id.is_empty() {
            len += 1;
        }
        if !self.user_id.is_empty() {
            len += 1;
        }
        if !self.components.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sited_io.websites.v1.StaticPageResponse", len)?;
        if self.page_id != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("pageId", ToString::to_string(&self.page_id).as_str())?;
        }
        if !self.website_id.is_empty() {
            struct_ser.serialize_field("websiteId", &self.website_id)?;
        }
        if !self.user_id.is_empty() {
            struct_ser.serialize_field("userId", &self.user_id)?;
        }
        if !self.components.is_empty() {
            struct_ser.serialize_field("components", &self.components)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for StaticPageResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "page_id",
            "pageId",
            "website_id",
            "websiteId",
            "user_id",
            "userId",
            "components",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PageId,
            WebsiteId,
            UserId,
            Components,
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
                            "pageId" | "page_id" => Ok(GeneratedField::PageId),
                            "websiteId" | "website_id" => Ok(GeneratedField::WebsiteId),
                            "userId" | "user_id" => Ok(GeneratedField::UserId),
                            "components" => Ok(GeneratedField::Components),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = StaticPageResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.websites.v1.StaticPageResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<StaticPageResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut page_id__ = None;
                let mut website_id__ = None;
                let mut user_id__ = None;
                let mut components__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::PageId => {
                            if page_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pageId"));
                            }
                            page_id__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::WebsiteId => {
                            if website_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("websiteId"));
                            }
                            website_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::UserId => {
                            if user_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userId"));
                            }
                            user_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Components => {
                            if components__.is_some() {
                                return Err(serde::de::Error::duplicate_field("components"));
                            }
                            components__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(StaticPageResponse {
                    page_id: page_id__.unwrap_or_default(),
                    website_id: website_id__.unwrap_or_default(),
                    user_id: user_id__.unwrap_or_default(),
                    components: components__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sited_io.websites.v1.StaticPageResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for TextElement {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.text.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sited_io.websites.v1.TextElement", len)?;
        if !self.text.is_empty() {
            struct_ser.serialize_field("text", &self.text)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TextElement {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "text",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Text,
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
                            "text" => Ok(GeneratedField::Text),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TextElement;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.websites.v1.TextElement")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<TextElement, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut text__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Text => {
                            if text__.is_some() {
                                return Err(serde::de::Error::duplicate_field("text"));
                            }
                            text__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(TextElement {
                    text: text__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sited_io.websites.v1.TextElement", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateCustomizationRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.website_id.is_empty() {
            len += 1;
        }
        if self.primary_color.is_some() {
            len += 1;
        }
        if self.secondary_color.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sited_io.websites.v1.UpdateCustomizationRequest", len)?;
        if !self.website_id.is_empty() {
            struct_ser.serialize_field("websiteId", &self.website_id)?;
        }
        if let Some(v) = self.primary_color.as_ref() {
            struct_ser.serialize_field("primaryColor", v)?;
        }
        if let Some(v) = self.secondary_color.as_ref() {
            struct_ser.serialize_field("secondaryColor", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateCustomizationRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "website_id",
            "websiteId",
            "primary_color",
            "primaryColor",
            "secondary_color",
            "secondaryColor",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            WebsiteId,
            PrimaryColor,
            SecondaryColor,
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
                            "websiteId" | "website_id" => Ok(GeneratedField::WebsiteId),
                            "primaryColor" | "primary_color" => Ok(GeneratedField::PrimaryColor),
                            "secondaryColor" | "secondary_color" => Ok(GeneratedField::SecondaryColor),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UpdateCustomizationRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.websites.v1.UpdateCustomizationRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateCustomizationRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut website_id__ = None;
                let mut primary_color__ = None;
                let mut secondary_color__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::WebsiteId => {
                            if website_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("websiteId"));
                            }
                            website_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::PrimaryColor => {
                            if primary_color__.is_some() {
                                return Err(serde::de::Error::duplicate_field("primaryColor"));
                            }
                            primary_color__ = map_.next_value()?;
                        }
                        GeneratedField::SecondaryColor => {
                            if secondary_color__.is_some() {
                                return Err(serde::de::Error::duplicate_field("secondaryColor"));
                            }
                            secondary_color__ = map_.next_value()?;
                        }
                    }
                }
                Ok(UpdateCustomizationRequest {
                    website_id: website_id__.unwrap_or_default(),
                    primary_color: primary_color__,
                    secondary_color: secondary_color__,
                })
            }
        }
        deserializer.deserialize_struct("sited_io.websites.v1.UpdateCustomizationRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateCustomizationResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.customization.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sited_io.websites.v1.UpdateCustomizationResponse", len)?;
        if let Some(v) = self.customization.as_ref() {
            struct_ser.serialize_field("customization", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateCustomizationResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "customization",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Customization,
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
                            "customization" => Ok(GeneratedField::Customization),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UpdateCustomizationResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.websites.v1.UpdateCustomizationResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateCustomizationResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut customization__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Customization => {
                            if customization__.is_some() {
                                return Err(serde::de::Error::duplicate_field("customization"));
                            }
                            customization__ = map_.next_value()?;
                        }
                    }
                }
                Ok(UpdateCustomizationResponse {
                    customization: customization__,
                })
            }
        }
        deserializer.deserialize_struct("sited_io.websites.v1.UpdateCustomizationResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdatePageRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.page_id != 0 {
            len += 1;
        }
        if self.page_type.is_some() {
            len += 1;
        }
        if self.content_id.is_some() {
            len += 1;
        }
        if self.title.is_some() {
            len += 1;
        }
        if self.is_home_page.is_some() {
            len += 1;
        }
        if self.path.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sited_io.websites.v1.UpdatePageRequest", len)?;
        if self.page_id != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("pageId", ToString::to_string(&self.page_id).as_str())?;
        }
        if let Some(v) = self.page_type.as_ref() {
            let v = PageType::try_from(*v)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", *v)))?;
            struct_ser.serialize_field("pageType", &v)?;
        }
        if let Some(v) = self.content_id.as_ref() {
            struct_ser.serialize_field("contentId", v)?;
        }
        if let Some(v) = self.title.as_ref() {
            struct_ser.serialize_field("title", v)?;
        }
        if let Some(v) = self.is_home_page.as_ref() {
            struct_ser.serialize_field("isHomePage", v)?;
        }
        if let Some(v) = self.path.as_ref() {
            struct_ser.serialize_field("path", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdatePageRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "page_id",
            "pageId",
            "page_type",
            "pageType",
            "content_id",
            "contentId",
            "title",
            "is_home_page",
            "isHomePage",
            "path",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PageId,
            PageType,
            ContentId,
            Title,
            IsHomePage,
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
                            "pageId" | "page_id" => Ok(GeneratedField::PageId),
                            "pageType" | "page_type" => Ok(GeneratedField::PageType),
                            "contentId" | "content_id" => Ok(GeneratedField::ContentId),
                            "title" => Ok(GeneratedField::Title),
                            "isHomePage" | "is_home_page" => Ok(GeneratedField::IsHomePage),
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
            type Value = UpdatePageRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.websites.v1.UpdatePageRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdatePageRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut page_id__ = None;
                let mut page_type__ = None;
                let mut content_id__ = None;
                let mut title__ = None;
                let mut is_home_page__ = None;
                let mut path__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::PageId => {
                            if page_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pageId"));
                            }
                            page_id__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::PageType => {
                            if page_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pageType"));
                            }
                            page_type__ = map_.next_value::<::std::option::Option<PageType>>()?.map(|x| x as i32);
                        }
                        GeneratedField::ContentId => {
                            if content_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("contentId"));
                            }
                            content_id__ = map_.next_value()?;
                        }
                        GeneratedField::Title => {
                            if title__.is_some() {
                                return Err(serde::de::Error::duplicate_field("title"));
                            }
                            title__ = map_.next_value()?;
                        }
                        GeneratedField::IsHomePage => {
                            if is_home_page__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isHomePage"));
                            }
                            is_home_page__ = map_.next_value()?;
                        }
                        GeneratedField::Path => {
                            if path__.is_some() {
                                return Err(serde::de::Error::duplicate_field("path"));
                            }
                            path__ = map_.next_value()?;
                        }
                    }
                }
                Ok(UpdatePageRequest {
                    page_id: page_id__.unwrap_or_default(),
                    page_type: page_type__,
                    content_id: content_id__,
                    title: title__,
                    is_home_page: is_home_page__,
                    path: path__,
                })
            }
        }
        deserializer.deserialize_struct("sited_io.websites.v1.UpdatePageRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdatePageResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.page.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sited_io.websites.v1.UpdatePageResponse", len)?;
        if let Some(v) = self.page.as_ref() {
            struct_ser.serialize_field("page", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdatePageResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "page",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Page,
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
                            "page" => Ok(GeneratedField::Page),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UpdatePageResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.websites.v1.UpdatePageResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdatePageResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut page__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Page => {
                            if page__.is_some() {
                                return Err(serde::de::Error::duplicate_field("page"));
                            }
                            page__ = map_.next_value()?;
                        }
                    }
                }
                Ok(UpdatePageResponse {
                    page: page__,
                })
            }
        }
        deserializer.deserialize_struct("sited_io.websites.v1.UpdatePageResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateStaticPageRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.page_id != 0 {
            len += 1;
        }
        if !self.components.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sited_io.websites.v1.UpdateStaticPageRequest", len)?;
        if self.page_id != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("pageId", ToString::to_string(&self.page_id).as_str())?;
        }
        if !self.components.is_empty() {
            struct_ser.serialize_field("components", &self.components)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateStaticPageRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "page_id",
            "pageId",
            "components",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PageId,
            Components,
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
                            "pageId" | "page_id" => Ok(GeneratedField::PageId),
                            "components" => Ok(GeneratedField::Components),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UpdateStaticPageRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.websites.v1.UpdateStaticPageRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateStaticPageRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut page_id__ = None;
                let mut components__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::PageId => {
                            if page_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pageId"));
                            }
                            page_id__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Components => {
                            if components__.is_some() {
                                return Err(serde::de::Error::duplicate_field("components"));
                            }
                            components__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(UpdateStaticPageRequest {
                    page_id: page_id__.unwrap_or_default(),
                    components: components__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sited_io.websites.v1.UpdateStaticPageRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateStaticPageResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.static_page.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sited_io.websites.v1.UpdateStaticPageResponse", len)?;
        if let Some(v) = self.static_page.as_ref() {
            struct_ser.serialize_field("staticPage", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateStaticPageResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "static_page",
            "staticPage",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            StaticPage,
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
                            "staticPage" | "static_page" => Ok(GeneratedField::StaticPage),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UpdateStaticPageResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.websites.v1.UpdateStaticPageResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateStaticPageResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut static_page__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::StaticPage => {
                            if static_page__.is_some() {
                                return Err(serde::de::Error::duplicate_field("staticPage"));
                            }
                            static_page__ = map_.next_value()?;
                        }
                    }
                }
                Ok(UpdateStaticPageResponse {
                    static_page: static_page__,
                })
            }
        }
        deserializer.deserialize_struct("sited_io.websites.v1.UpdateStaticPageResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateWebsiteRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.website_id.is_empty() {
            len += 1;
        }
        if self.name.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sited_io.websites.v1.UpdateWebsiteRequest", len)?;
        if !self.website_id.is_empty() {
            struct_ser.serialize_field("websiteId", &self.website_id)?;
        }
        if let Some(v) = self.name.as_ref() {
            struct_ser.serialize_field("name", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateWebsiteRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "website_id",
            "websiteId",
            "name",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            WebsiteId,
            Name,
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
                            "websiteId" | "website_id" => Ok(GeneratedField::WebsiteId),
                            "name" => Ok(GeneratedField::Name),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UpdateWebsiteRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.websites.v1.UpdateWebsiteRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateWebsiteRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut website_id__ = None;
                let mut name__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::WebsiteId => {
                            if website_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("websiteId"));
                            }
                            website_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = map_.next_value()?;
                        }
                    }
                }
                Ok(UpdateWebsiteRequest {
                    website_id: website_id__.unwrap_or_default(),
                    name: name__,
                })
            }
        }
        deserializer.deserialize_struct("sited_io.websites.v1.UpdateWebsiteRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateWebsiteResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.website.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sited_io.websites.v1.UpdateWebsiteResponse", len)?;
        if let Some(v) = self.website.as_ref() {
            struct_ser.serialize_field("website", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateWebsiteResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "website",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Website,
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
                            "website" => Ok(GeneratedField::Website),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UpdateWebsiteResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.websites.v1.UpdateWebsiteResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateWebsiteResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut website__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Website => {
                            if website__.is_some() {
                                return Err(serde::de::Error::duplicate_field("website"));
                            }
                            website__ = map_.next_value()?;
                        }
                    }
                }
                Ok(UpdateWebsiteResponse {
                    website: website__,
                })
            }
        }
        deserializer.deserialize_struct("sited_io.websites.v1.UpdateWebsiteResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for WebsiteResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.website_id.is_empty() {
            len += 1;
        }
        if !self.user_id.is_empty() {
            len += 1;
        }
        if self.created_at != 0 {
            len += 1;
        }
        if self.updated_at != 0 {
            len += 1;
        }
        if !self.name.is_empty() {
            len += 1;
        }
        if !self.client_id.is_empty() {
            len += 1;
        }
        if self.customization.is_some() {
            len += 1;
        }
        if !self.domains.is_empty() {
            len += 1;
        }
        if !self.pages.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sited_io.websites.v1.WebsiteResponse", len)?;
        if !self.website_id.is_empty() {
            struct_ser.serialize_field("websiteId", &self.website_id)?;
        }
        if !self.user_id.is_empty() {
            struct_ser.serialize_field("userId", &self.user_id)?;
        }
        if self.created_at != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("createdAt", ToString::to_string(&self.created_at).as_str())?;
        }
        if self.updated_at != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("updatedAt", ToString::to_string(&self.updated_at).as_str())?;
        }
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.client_id.is_empty() {
            struct_ser.serialize_field("clientId", &self.client_id)?;
        }
        if let Some(v) = self.customization.as_ref() {
            struct_ser.serialize_field("customization", v)?;
        }
        if !self.domains.is_empty() {
            struct_ser.serialize_field("domains", &self.domains)?;
        }
        if !self.pages.is_empty() {
            struct_ser.serialize_field("pages", &self.pages)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for WebsiteResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "website_id",
            "websiteId",
            "user_id",
            "userId",
            "created_at",
            "createdAt",
            "updated_at",
            "updatedAt",
            "name",
            "client_id",
            "clientId",
            "customization",
            "domains",
            "pages",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            WebsiteId,
            UserId,
            CreatedAt,
            UpdatedAt,
            Name,
            ClientId,
            Customization,
            Domains,
            Pages,
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
                            "websiteId" | "website_id" => Ok(GeneratedField::WebsiteId),
                            "userId" | "user_id" => Ok(GeneratedField::UserId),
                            "createdAt" | "created_at" => Ok(GeneratedField::CreatedAt),
                            "updatedAt" | "updated_at" => Ok(GeneratedField::UpdatedAt),
                            "name" => Ok(GeneratedField::Name),
                            "clientId" | "client_id" => Ok(GeneratedField::ClientId),
                            "customization" => Ok(GeneratedField::Customization),
                            "domains" => Ok(GeneratedField::Domains),
                            "pages" => Ok(GeneratedField::Pages),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = WebsiteResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.websites.v1.WebsiteResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<WebsiteResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut website_id__ = None;
                let mut user_id__ = None;
                let mut created_at__ = None;
                let mut updated_at__ = None;
                let mut name__ = None;
                let mut client_id__ = None;
                let mut customization__ = None;
                let mut domains__ = None;
                let mut pages__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::WebsiteId => {
                            if website_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("websiteId"));
                            }
                            website_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::UserId => {
                            if user_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userId"));
                            }
                            user_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::CreatedAt => {
                            if created_at__.is_some() {
                                return Err(serde::de::Error::duplicate_field("createdAt"));
                            }
                            created_at__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::UpdatedAt => {
                            if updated_at__.is_some() {
                                return Err(serde::de::Error::duplicate_field("updatedAt"));
                            }
                            updated_at__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ClientId => {
                            if client_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("clientId"));
                            }
                            client_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Customization => {
                            if customization__.is_some() {
                                return Err(serde::de::Error::duplicate_field("customization"));
                            }
                            customization__ = map_.next_value()?;
                        }
                        GeneratedField::Domains => {
                            if domains__.is_some() {
                                return Err(serde::de::Error::duplicate_field("domains"));
                            }
                            domains__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Pages => {
                            if pages__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pages"));
                            }
                            pages__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(WebsiteResponse {
                    website_id: website_id__.unwrap_or_default(),
                    user_id: user_id__.unwrap_or_default(),
                    created_at: created_at__.unwrap_or_default(),
                    updated_at: updated_at__.unwrap_or_default(),
                    name: name__.unwrap_or_default(),
                    client_id: client_id__.unwrap_or_default(),
                    customization: customization__,
                    domains: domains__.unwrap_or_default(),
                    pages: pages__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sited_io.websites.v1.WebsiteResponse", FIELDS, GeneratedVisitor)
    }
}

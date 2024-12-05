// @generated
impl serde::Serialize for AddDomainToShopRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.shop_id.is_empty() {
            len += 1;
        }
        if !self.domain.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sited_io.commerce.v1.AddDomainToShopRequest", len)?;
        if !self.shop_id.is_empty() {
            struct_ser.serialize_field("shopId", &self.shop_id)?;
        }
        if !self.domain.is_empty() {
            struct_ser.serialize_field("domain", &self.domain)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AddDomainToShopRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "shop_id",
            "shopId",
            "domain",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ShopId,
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
                            "shopId" | "shop_id" => Ok(GeneratedField::ShopId),
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
            type Value = AddDomainToShopRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.commerce.v1.AddDomainToShopRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AddDomainToShopRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut shop_id__ = None;
                let mut domain__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ShopId => {
                            if shop_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("shopId"));
                            }
                            shop_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Domain => {
                            if domain__.is_some() {
                                return Err(serde::de::Error::duplicate_field("domain"));
                            }
                            domain__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(AddDomainToShopRequest {
                    shop_id: shop_id__.unwrap_or_default(),
                    domain: domain__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sited_io.commerce.v1.AddDomainToShopRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for AddDomainToShopResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("sited_io.commerce.v1.AddDomainToShopResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AddDomainToShopResponse {
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
            type Value = AddDomainToShopResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.commerce.v1.AddDomainToShopResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AddDomainToShopResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(AddDomainToShopResponse {
                })
            }
        }
        deserializer.deserialize_struct("sited_io.commerce.v1.AddDomainToShopResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for AddImageToOfferRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.offer_id.is_empty() {
            len += 1;
        }
        if self.image.is_some() {
            len += 1;
        }
        if self.ordering != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sited_io.commerce.v1.AddImageToOfferRequest", len)?;
        if !self.offer_id.is_empty() {
            struct_ser.serialize_field("offerId", &self.offer_id)?;
        }
        if let Some(v) = self.image.as_ref() {
            struct_ser.serialize_field("image", v)?;
        }
        if self.ordering != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("ordering", ToString::to_string(&self.ordering).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AddImageToOfferRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "offer_id",
            "offerId",
            "image",
            "ordering",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            OfferId,
            Image,
            Ordering,
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
                            "offerId" | "offer_id" => Ok(GeneratedField::OfferId),
                            "image" => Ok(GeneratedField::Image),
                            "ordering" => Ok(GeneratedField::Ordering),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AddImageToOfferRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.commerce.v1.AddImageToOfferRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AddImageToOfferRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut offer_id__ = None;
                let mut image__ = None;
                let mut ordering__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::OfferId => {
                            if offer_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("offerId"));
                            }
                            offer_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Image => {
                            if image__.is_some() {
                                return Err(serde::de::Error::duplicate_field("image"));
                            }
                            image__ = map_.next_value()?;
                        }
                        GeneratedField::Ordering => {
                            if ordering__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ordering"));
                            }
                            ordering__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(AddImageToOfferRequest {
                    offer_id: offer_id__.unwrap_or_default(),
                    image: image__,
                    ordering: ordering__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sited_io.commerce.v1.AddImageToOfferRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for AddImageToOfferResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("sited_io.commerce.v1.AddImageToOfferResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AddImageToOfferResponse {
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
            type Value = AddImageToOfferResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.commerce.v1.AddImageToOfferResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AddImageToOfferResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(AddImageToOfferResponse {
                })
            }
        }
        deserializer.deserialize_struct("sited_io.commerce.v1.AddImageToOfferResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateOfferRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.shop_id.is_empty() {
            len += 1;
        }
        if !self.name.is_empty() {
            len += 1;
        }
        if self.description.is_some() {
            len += 1;
        }
        if self.r#type != 0 {
            len += 1;
        }
        if self.is_featured {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sited_io.commerce.v1.CreateOfferRequest", len)?;
        if !self.shop_id.is_empty() {
            struct_ser.serialize_field("shopId", &self.shop_id)?;
        }
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if let Some(v) = self.description.as_ref() {
            struct_ser.serialize_field("description", v)?;
        }
        if self.r#type != 0 {
            let v = OfferType::try_from(self.r#type)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.r#type)))?;
            struct_ser.serialize_field("type", &v)?;
        }
        if self.is_featured {
            struct_ser.serialize_field("isFeatured", &self.is_featured)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateOfferRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "shop_id",
            "shopId",
            "name",
            "description",
            "type",
            "is_featured",
            "isFeatured",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ShopId,
            Name,
            Description,
            Type,
            IsFeatured,
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
                            "shopId" | "shop_id" => Ok(GeneratedField::ShopId),
                            "name" => Ok(GeneratedField::Name),
                            "description" => Ok(GeneratedField::Description),
                            "type" => Ok(GeneratedField::Type),
                            "isFeatured" | "is_featured" => Ok(GeneratedField::IsFeatured),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateOfferRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.commerce.v1.CreateOfferRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateOfferRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut shop_id__ = None;
                let mut name__ = None;
                let mut description__ = None;
                let mut r#type__ = None;
                let mut is_featured__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ShopId => {
                            if shop_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("shopId"));
                            }
                            shop_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = map_.next_value()?;
                        }
                        GeneratedField::Type => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type__ = Some(map_.next_value::<OfferType>()? as i32);
                        }
                        GeneratedField::IsFeatured => {
                            if is_featured__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isFeatured"));
                            }
                            is_featured__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(CreateOfferRequest {
                    shop_id: shop_id__.unwrap_or_default(),
                    name: name__.unwrap_or_default(),
                    description: description__,
                    r#type: r#type__.unwrap_or_default(),
                    is_featured: is_featured__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sited_io.commerce.v1.CreateOfferRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateOfferResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.offer.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sited_io.commerce.v1.CreateOfferResponse", len)?;
        if let Some(v) = self.offer.as_ref() {
            struct_ser.serialize_field("offer", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateOfferResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "offer",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Offer,
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
                            "offer" => Ok(GeneratedField::Offer),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateOfferResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.commerce.v1.CreateOfferResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateOfferResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut offer__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Offer => {
                            if offer__.is_some() {
                                return Err(serde::de::Error::duplicate_field("offer"));
                            }
                            offer__ = map_.next_value()?;
                        }
                    }
                }
                Ok(CreateOfferResponse {
                    offer: offer__,
                })
            }
        }
        deserializer.deserialize_struct("sited_io.commerce.v1.CreateOfferResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateShopRequest {
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
        if !self.slug.is_empty() {
            len += 1;
        }
        if self.description.is_some() {
            len += 1;
        }
        if self.platform_fee_percent.is_some() {
            len += 1;
        }
        if self.minimum_platform_fee_cent.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sited_io.commerce.v1.CreateShopRequest", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.slug.is_empty() {
            struct_ser.serialize_field("slug", &self.slug)?;
        }
        if let Some(v) = self.description.as_ref() {
            struct_ser.serialize_field("description", v)?;
        }
        if let Some(v) = self.platform_fee_percent.as_ref() {
            struct_ser.serialize_field("platformFeePercent", v)?;
        }
        if let Some(v) = self.minimum_platform_fee_cent.as_ref() {
            struct_ser.serialize_field("minimumPlatformFeeCent", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateShopRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "slug",
            "description",
            "platform_fee_percent",
            "platformFeePercent",
            "minimum_platform_fee_cent",
            "minimumPlatformFeeCent",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            Slug,
            Description,
            PlatformFeePercent,
            MinimumPlatformFeeCent,
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
                            "slug" => Ok(GeneratedField::Slug),
                            "description" => Ok(GeneratedField::Description),
                            "platformFeePercent" | "platform_fee_percent" => Ok(GeneratedField::PlatformFeePercent),
                            "minimumPlatformFeeCent" | "minimum_platform_fee_cent" => Ok(GeneratedField::MinimumPlatformFeeCent),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateShopRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.commerce.v1.CreateShopRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateShopRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut slug__ = None;
                let mut description__ = None;
                let mut platform_fee_percent__ = None;
                let mut minimum_platform_fee_cent__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Slug => {
                            if slug__.is_some() {
                                return Err(serde::de::Error::duplicate_field("slug"));
                            }
                            slug__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = map_.next_value()?;
                        }
                        GeneratedField::PlatformFeePercent => {
                            if platform_fee_percent__.is_some() {
                                return Err(serde::de::Error::duplicate_field("platformFeePercent"));
                            }
                            platform_fee_percent__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::MinimumPlatformFeeCent => {
                            if minimum_platform_fee_cent__.is_some() {
                                return Err(serde::de::Error::duplicate_field("minimumPlatformFeeCent"));
                            }
                            minimum_platform_fee_cent__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                    }
                }
                Ok(CreateShopRequest {
                    name: name__.unwrap_or_default(),
                    slug: slug__.unwrap_or_default(),
                    description: description__,
                    platform_fee_percent: platform_fee_percent__,
                    minimum_platform_fee_cent: minimum_platform_fee_cent__,
                })
            }
        }
        deserializer.deserialize_struct("sited_io.commerce.v1.CreateShopRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateShopResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.shop.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sited_io.commerce.v1.CreateShopResponse", len)?;
        if let Some(v) = self.shop.as_ref() {
            struct_ser.serialize_field("shop", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateShopResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "shop",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Shop,
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
                            "shop" => Ok(GeneratedField::Shop),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateShopResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.commerce.v1.CreateShopResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateShopResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut shop__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Shop => {
                            if shop__.is_some() {
                                return Err(serde::de::Error::duplicate_field("shop"));
                            }
                            shop__ = map_.next_value()?;
                        }
                    }
                }
                Ok(CreateShopResponse {
                    shop: shop__,
                })
            }
        }
        deserializer.deserialize_struct("sited_io.commerce.v1.CreateShopResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Currency {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "CURRENCY_UNSPECIFIED",
            Self::Eur => "CURRENCY_EUR",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for Currency {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "CURRENCY_UNSPECIFIED",
            "CURRENCY_EUR",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Currency;

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
                    "CURRENCY_UNSPECIFIED" => Ok(Currency::Unspecified),
                    "CURRENCY_EUR" => Ok(Currency::Eur),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for DeleteOfferRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.offer_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sited_io.commerce.v1.DeleteOfferRequest", len)?;
        if !self.offer_id.is_empty() {
            struct_ser.serialize_field("offerId", &self.offer_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeleteOfferRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "offer_id",
            "offerId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            OfferId,
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
                            "offerId" | "offer_id" => Ok(GeneratedField::OfferId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DeleteOfferRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.commerce.v1.DeleteOfferRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DeleteOfferRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut offer_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::OfferId => {
                            if offer_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("offerId"));
                            }
                            offer_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(DeleteOfferRequest {
                    offer_id: offer_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sited_io.commerce.v1.DeleteOfferRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeleteOfferResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("sited_io.commerce.v1.DeleteOfferResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeleteOfferResponse {
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
            type Value = DeleteOfferResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.commerce.v1.DeleteOfferResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DeleteOfferResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(DeleteOfferResponse {
                })
            }
        }
        deserializer.deserialize_struct("sited_io.commerce.v1.DeleteOfferResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeleteShippingRateRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.shipping_rate_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sited_io.commerce.v1.DeleteShippingRateRequest", len)?;
        if !self.shipping_rate_id.is_empty() {
            struct_ser.serialize_field("shippingRateId", &self.shipping_rate_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeleteShippingRateRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "shipping_rate_id",
            "shippingRateId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ShippingRateId,
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
                            "shippingRateId" | "shipping_rate_id" => Ok(GeneratedField::ShippingRateId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DeleteShippingRateRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.commerce.v1.DeleteShippingRateRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DeleteShippingRateRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut shipping_rate_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ShippingRateId => {
                            if shipping_rate_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("shippingRateId"));
                            }
                            shipping_rate_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(DeleteShippingRateRequest {
                    shipping_rate_id: shipping_rate_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sited_io.commerce.v1.DeleteShippingRateRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeleteShippingRateResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("sited_io.commerce.v1.DeleteShippingRateResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeleteShippingRateResponse {
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
            type Value = DeleteShippingRateResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.commerce.v1.DeleteShippingRateResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DeleteShippingRateResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(DeleteShippingRateResponse {
                })
            }
        }
        deserializer.deserialize_struct("sited_io.commerce.v1.DeleteShippingRateResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeleteShopCustomizationRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.shop_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sited_io.commerce.v1.DeleteShopCustomizationRequest", len)?;
        if !self.shop_id.is_empty() {
            struct_ser.serialize_field("shopId", &self.shop_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeleteShopCustomizationRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "shop_id",
            "shopId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ShopId,
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
                            "shopId" | "shop_id" => Ok(GeneratedField::ShopId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DeleteShopCustomizationRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.commerce.v1.DeleteShopCustomizationRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DeleteShopCustomizationRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut shop_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ShopId => {
                            if shop_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("shopId"));
                            }
                            shop_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(DeleteShopCustomizationRequest {
                    shop_id: shop_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sited_io.commerce.v1.DeleteShopCustomizationRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeleteShopCustomizationResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("sited_io.commerce.v1.DeleteShopCustomizationResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeleteShopCustomizationResponse {
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
            type Value = DeleteShopCustomizationResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.commerce.v1.DeleteShopCustomizationResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DeleteShopCustomizationResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(DeleteShopCustomizationResponse {
                })
            }
        }
        deserializer.deserialize_struct("sited_io.commerce.v1.DeleteShopCustomizationResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeleteShopRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.shop_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sited_io.commerce.v1.DeleteShopRequest", len)?;
        if !self.shop_id.is_empty() {
            struct_ser.serialize_field("shopId", &self.shop_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeleteShopRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "shop_id",
            "shopId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ShopId,
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
                            "shopId" | "shop_id" => Ok(GeneratedField::ShopId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DeleteShopRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.commerce.v1.DeleteShopRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DeleteShopRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut shop_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ShopId => {
                            if shop_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("shopId"));
                            }
                            shop_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(DeleteShopRequest {
                    shop_id: shop_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sited_io.commerce.v1.DeleteShopRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeleteShopResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("sited_io.commerce.v1.DeleteShopResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeleteShopResponse {
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
            type Value = DeleteShopResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.commerce.v1.DeleteShopResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DeleteShopResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(DeleteShopResponse {
                })
            }
        }
        deserializer.deserialize_struct("sited_io.commerce.v1.DeleteShopResponse", FIELDS, GeneratedVisitor)
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
                    "DOMAIN_STATUS_PENDING" => Ok(DomainStatus::Pending),
                    "DOMAIN_STATUS_ACTIVE" => Ok(DomainStatus::Active),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for DomainStatusResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.shop_id.is_empty() {
            len += 1;
        }
        if !self.domain.is_empty() {
            len += 1;
        }
        if self.status != 0 {
            len += 1;
        }
        if self.client_id.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sited_io.commerce.v1.DomainStatusResponse", len)?;
        if !self.shop_id.is_empty() {
            struct_ser.serialize_field("shopId", &self.shop_id)?;
        }
        if !self.domain.is_empty() {
            struct_ser.serialize_field("domain", &self.domain)?;
        }
        if self.status != 0 {
            let v = DomainStatus::try_from(self.status)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.status)))?;
            struct_ser.serialize_field("status", &v)?;
        }
        if let Some(v) = self.client_id.as_ref() {
            struct_ser.serialize_field("clientId", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DomainStatusResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "shop_id",
            "shopId",
            "domain",
            "status",
            "client_id",
            "clientId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ShopId,
            Domain,
            Status,
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
                            "shopId" | "shop_id" => Ok(GeneratedField::ShopId),
                            "domain" => Ok(GeneratedField::Domain),
                            "status" => Ok(GeneratedField::Status),
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
            type Value = DomainStatusResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.commerce.v1.DomainStatusResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DomainStatusResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut shop_id__ = None;
                let mut domain__ = None;
                let mut status__ = None;
                let mut client_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ShopId => {
                            if shop_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("shopId"));
                            }
                            shop_id__ = Some(map_.next_value()?);
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
                        GeneratedField::ClientId => {
                            if client_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("clientId"));
                            }
                            client_id__ = map_.next_value()?;
                        }
                    }
                }
                Ok(DomainStatusResponse {
                    shop_id: shop_id__.unwrap_or_default(),
                    domain: domain__.unwrap_or_default(),
                    status: status__.unwrap_or_default(),
                    client_id: client_id__,
                })
            }
        }
        deserializer.deserialize_struct("sited_io.commerce.v1.DomainStatusResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetClientIdForDomainRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.domain.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sited_io.commerce.v1.GetClientIdForDomainRequest", len)?;
        if !self.domain.is_empty() {
            struct_ser.serialize_field("domain", &self.domain)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetClientIdForDomainRequest {
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
            type Value = GetClientIdForDomainRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.commerce.v1.GetClientIdForDomainRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetClientIdForDomainRequest, V::Error>
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
                            domain__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(GetClientIdForDomainRequest {
                    domain: domain__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sited_io.commerce.v1.GetClientIdForDomainRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetClientIdForDomainResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.client_id.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sited_io.commerce.v1.GetClientIdForDomainResponse", len)?;
        if let Some(v) = self.client_id.as_ref() {
            struct_ser.serialize_field("clientId", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetClientIdForDomainResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "client_id",
            "clientId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
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
            type Value = GetClientIdForDomainResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.commerce.v1.GetClientIdForDomainResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetClientIdForDomainResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut client_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ClientId => {
                            if client_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("clientId"));
                            }
                            client_id__ = map_.next_value()?;
                        }
                    }
                }
                Ok(GetClientIdForDomainResponse {
                    client_id: client_id__,
                })
            }
        }
        deserializer.deserialize_struct("sited_io.commerce.v1.GetClientIdForDomainResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetDomainStatusRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.shop_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sited_io.commerce.v1.GetDomainStatusRequest", len)?;
        if !self.shop_id.is_empty() {
            struct_ser.serialize_field("shopId", &self.shop_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetDomainStatusRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "shop_id",
            "shopId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ShopId,
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
                            "shopId" | "shop_id" => Ok(GeneratedField::ShopId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetDomainStatusRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.commerce.v1.GetDomainStatusRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetDomainStatusRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut shop_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ShopId => {
                            if shop_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("shopId"));
                            }
                            shop_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(GetDomainStatusRequest {
                    shop_id: shop_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sited_io.commerce.v1.GetDomainStatusRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetDomainStatusResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.domain_status.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sited_io.commerce.v1.GetDomainStatusResponse", len)?;
        if let Some(v) = self.domain_status.as_ref() {
            struct_ser.serialize_field("domainStatus", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetDomainStatusResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "domain_status",
            "domainStatus",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            DomainStatus,
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
                            "domainStatus" | "domain_status" => Ok(GeneratedField::DomainStatus),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetDomainStatusResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.commerce.v1.GetDomainStatusResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetDomainStatusResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut domain_status__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::DomainStatus => {
                            if domain_status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("domainStatus"));
                            }
                            domain_status__ = map_.next_value()?;
                        }
                    }
                }
                Ok(GetDomainStatusResponse {
                    domain_status: domain_status__,
                })
            }
        }
        deserializer.deserialize_struct("sited_io.commerce.v1.GetDomainStatusResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetMyOfferRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.offer_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sited_io.commerce.v1.GetMyOfferRequest", len)?;
        if !self.offer_id.is_empty() {
            struct_ser.serialize_field("offerId", &self.offer_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetMyOfferRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "offer_id",
            "offerId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            OfferId,
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
                            "offerId" | "offer_id" => Ok(GeneratedField::OfferId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetMyOfferRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.commerce.v1.GetMyOfferRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetMyOfferRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut offer_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::OfferId => {
                            if offer_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("offerId"));
                            }
                            offer_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(GetMyOfferRequest {
                    offer_id: offer_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sited_io.commerce.v1.GetMyOfferRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetMyOfferResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.offer.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sited_io.commerce.v1.GetMyOfferResponse", len)?;
        if let Some(v) = self.offer.as_ref() {
            struct_ser.serialize_field("offer", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetMyOfferResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "offer",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Offer,
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
                            "offer" => Ok(GeneratedField::Offer),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetMyOfferResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.commerce.v1.GetMyOfferResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetMyOfferResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut offer__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Offer => {
                            if offer__.is_some() {
                                return Err(serde::de::Error::duplicate_field("offer"));
                            }
                            offer__ = map_.next_value()?;
                        }
                    }
                }
                Ok(GetMyOfferResponse {
                    offer: offer__,
                })
            }
        }
        deserializer.deserialize_struct("sited_io.commerce.v1.GetMyOfferResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetOfferRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.offer_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sited_io.commerce.v1.GetOfferRequest", len)?;
        if !self.offer_id.is_empty() {
            struct_ser.serialize_field("offerId", &self.offer_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetOfferRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "offer_id",
            "offerId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            OfferId,
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
                            "offerId" | "offer_id" => Ok(GeneratedField::OfferId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetOfferRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.commerce.v1.GetOfferRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetOfferRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut offer_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::OfferId => {
                            if offer_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("offerId"));
                            }
                            offer_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(GetOfferRequest {
                    offer_id: offer_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sited_io.commerce.v1.GetOfferRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetOfferResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.offer.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sited_io.commerce.v1.GetOfferResponse", len)?;
        if let Some(v) = self.offer.as_ref() {
            struct_ser.serialize_field("offer", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetOfferResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "offer",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Offer,
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
                            "offer" => Ok(GeneratedField::Offer),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetOfferResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.commerce.v1.GetOfferResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetOfferResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut offer__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Offer => {
                            if offer__.is_some() {
                                return Err(serde::de::Error::duplicate_field("offer"));
                            }
                            offer__ = map_.next_value()?;
                        }
                    }
                }
                Ok(GetOfferResponse {
                    offer: offer__,
                })
            }
        }
        deserializer.deserialize_struct("sited_io.commerce.v1.GetOfferResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetShippingRateRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.offer_id.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sited_io.commerce.v1.GetShippingRateRequest", len)?;
        if let Some(v) = self.offer_id.as_ref() {
            struct_ser.serialize_field("offerId", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetShippingRateRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "offer_id",
            "offerId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            OfferId,
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
                            "offerId" | "offer_id" => Ok(GeneratedField::OfferId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetShippingRateRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.commerce.v1.GetShippingRateRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetShippingRateRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut offer_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::OfferId => {
                            if offer_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("offerId"));
                            }
                            offer_id__ = map_.next_value()?;
                        }
                    }
                }
                Ok(GetShippingRateRequest {
                    offer_id: offer_id__,
                })
            }
        }
        deserializer.deserialize_struct("sited_io.commerce.v1.GetShippingRateRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetShippingRateResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.shipping_rate.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sited_io.commerce.v1.GetShippingRateResponse", len)?;
        if let Some(v) = self.shipping_rate.as_ref() {
            struct_ser.serialize_field("shippingRate", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetShippingRateResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "shipping_rate",
            "shippingRate",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ShippingRate,
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
                            "shippingRate" | "shipping_rate" => Ok(GeneratedField::ShippingRate),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetShippingRateResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.commerce.v1.GetShippingRateResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetShippingRateResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut shipping_rate__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ShippingRate => {
                            if shipping_rate__.is_some() {
                                return Err(serde::de::Error::duplicate_field("shippingRate"));
                            }
                            shipping_rate__ = map_.next_value()?;
                        }
                    }
                }
                Ok(GetShippingRateResponse {
                    shipping_rate: shipping_rate__,
                })
            }
        }
        deserializer.deserialize_struct("sited_io.commerce.v1.GetShippingRateResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetShopCustomizationRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.shop_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sited_io.commerce.v1.GetShopCustomizationRequest", len)?;
        if !self.shop_id.is_empty() {
            struct_ser.serialize_field("shopId", &self.shop_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetShopCustomizationRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "shop_id",
            "shopId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ShopId,
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
                            "shopId" | "shop_id" => Ok(GeneratedField::ShopId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetShopCustomizationRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.commerce.v1.GetShopCustomizationRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetShopCustomizationRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut shop_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ShopId => {
                            if shop_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("shopId"));
                            }
                            shop_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(GetShopCustomizationRequest {
                    shop_id: shop_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sited_io.commerce.v1.GetShopCustomizationRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetShopCustomizationResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.shop_customization.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sited_io.commerce.v1.GetShopCustomizationResponse", len)?;
        if let Some(v) = self.shop_customization.as_ref() {
            struct_ser.serialize_field("shopCustomization", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetShopCustomizationResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "shop_customization",
            "shopCustomization",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ShopCustomization,
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
                            "shopCustomization" | "shop_customization" => Ok(GeneratedField::ShopCustomization),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetShopCustomizationResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.commerce.v1.GetShopCustomizationResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetShopCustomizationResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut shop_customization__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ShopCustomization => {
                            if shop_customization__.is_some() {
                                return Err(serde::de::Error::duplicate_field("shopCustomization"));
                            }
                            shop_customization__ = map_.next_value()?;
                        }
                    }
                }
                Ok(GetShopCustomizationResponse {
                    shop_customization: shop_customization__,
                })
            }
        }
        deserializer.deserialize_struct("sited_io.commerce.v1.GetShopCustomizationResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetShopRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.shop_id.is_some() {
            len += 1;
        }
        if self.extended.is_some() {
            len += 1;
        }
        if self.slug.is_some() {
            len += 1;
        }
        if self.domain.is_some() {
            len += 1;
        }
        if self.owner.is_some() {
            len += 1;
        }
        if self.website_id.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sited_io.commerce.v1.GetShopRequest", len)?;
        if let Some(v) = self.shop_id.as_ref() {
            struct_ser.serialize_field("shopId", v)?;
        }
        if let Some(v) = self.extended.as_ref() {
            struct_ser.serialize_field("extended", v)?;
        }
        if let Some(v) = self.slug.as_ref() {
            struct_ser.serialize_field("slug", v)?;
        }
        if let Some(v) = self.domain.as_ref() {
            struct_ser.serialize_field("domain", v)?;
        }
        if let Some(v) = self.owner.as_ref() {
            struct_ser.serialize_field("owner", v)?;
        }
        if let Some(v) = self.website_id.as_ref() {
            struct_ser.serialize_field("websiteId", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetShopRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "shop_id",
            "shopId",
            "extended",
            "slug",
            "domain",
            "owner",
            "website_id",
            "websiteId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ShopId,
            Extended,
            Slug,
            Domain,
            Owner,
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
                            "shopId" | "shop_id" => Ok(GeneratedField::ShopId),
                            "extended" => Ok(GeneratedField::Extended),
                            "slug" => Ok(GeneratedField::Slug),
                            "domain" => Ok(GeneratedField::Domain),
                            "owner" => Ok(GeneratedField::Owner),
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
            type Value = GetShopRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.commerce.v1.GetShopRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetShopRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut shop_id__ = None;
                let mut extended__ = None;
                let mut slug__ = None;
                let mut domain__ = None;
                let mut owner__ = None;
                let mut website_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ShopId => {
                            if shop_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("shopId"));
                            }
                            shop_id__ = map_.next_value()?;
                        }
                        GeneratedField::Extended => {
                            if extended__.is_some() {
                                return Err(serde::de::Error::duplicate_field("extended"));
                            }
                            extended__ = map_.next_value()?;
                        }
                        GeneratedField::Slug => {
                            if slug__.is_some() {
                                return Err(serde::de::Error::duplicate_field("slug"));
                            }
                            slug__ = map_.next_value()?;
                        }
                        GeneratedField::Domain => {
                            if domain__.is_some() {
                                return Err(serde::de::Error::duplicate_field("domain"));
                            }
                            domain__ = map_.next_value()?;
                        }
                        GeneratedField::Owner => {
                            if owner__.is_some() {
                                return Err(serde::de::Error::duplicate_field("owner"));
                            }
                            owner__ = map_.next_value()?;
                        }
                        GeneratedField::WebsiteId => {
                            if website_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("websiteId"));
                            }
                            website_id__ = map_.next_value()?;
                        }
                    }
                }
                Ok(GetShopRequest {
                    shop_id: shop_id__,
                    extended: extended__,
                    slug: slug__,
                    domain: domain__,
                    owner: owner__,
                    website_id: website_id__,
                })
            }
        }
        deserializer.deserialize_struct("sited_io.commerce.v1.GetShopRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetShopResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.shop.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sited_io.commerce.v1.GetShopResponse", len)?;
        if let Some(v) = self.shop.as_ref() {
            struct_ser.serialize_field("shop", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetShopResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "shop",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Shop,
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
                            "shop" => Ok(GeneratedField::Shop),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetShopResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.commerce.v1.GetShopResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetShopResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut shop__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Shop => {
                            if shop__.is_some() {
                                return Err(serde::de::Error::duplicate_field("shop"));
                            }
                            shop__ = map_.next_value()?;
                        }
                    }
                }
                Ok(GetShopResponse {
                    shop: shop__,
                })
            }
        }
        deserializer.deserialize_struct("sited_io.commerce.v1.GetShopResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListOffersRequest {
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
        if self.shop_id.is_some() {
            len += 1;
        }
        if self.pagination.is_some() {
            len += 1;
        }
        if self.order_by.is_some() {
            len += 1;
        }
        if self.filter.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sited_io.commerce.v1.ListOffersRequest", len)?;
        if let Some(v) = self.user_id.as_ref() {
            struct_ser.serialize_field("userId", v)?;
        }
        if let Some(v) = self.shop_id.as_ref() {
            struct_ser.serialize_field("shopId", v)?;
        }
        if let Some(v) = self.pagination.as_ref() {
            struct_ser.serialize_field("pagination", v)?;
        }
        if let Some(v) = self.order_by.as_ref() {
            struct_ser.serialize_field("orderBy", v)?;
        }
        if let Some(v) = self.filter.as_ref() {
            struct_ser.serialize_field("filter", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListOffersRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "user_id",
            "userId",
            "shop_id",
            "shopId",
            "pagination",
            "order_by",
            "orderBy",
            "filter",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            UserId,
            ShopId,
            Pagination,
            OrderBy,
            Filter,
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
                            "shopId" | "shop_id" => Ok(GeneratedField::ShopId),
                            "pagination" => Ok(GeneratedField::Pagination),
                            "orderBy" | "order_by" => Ok(GeneratedField::OrderBy),
                            "filter" => Ok(GeneratedField::Filter),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListOffersRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.commerce.v1.ListOffersRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListOffersRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut user_id__ = None;
                let mut shop_id__ = None;
                let mut pagination__ = None;
                let mut order_by__ = None;
                let mut filter__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::UserId => {
                            if user_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userId"));
                            }
                            user_id__ = map_.next_value()?;
                        }
                        GeneratedField::ShopId => {
                            if shop_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("shopId"));
                            }
                            shop_id__ = map_.next_value()?;
                        }
                        GeneratedField::Pagination => {
                            if pagination__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pagination"));
                            }
                            pagination__ = map_.next_value()?;
                        }
                        GeneratedField::OrderBy => {
                            if order_by__.is_some() {
                                return Err(serde::de::Error::duplicate_field("orderBy"));
                            }
                            order_by__ = map_.next_value()?;
                        }
                        GeneratedField::Filter => {
                            if filter__.is_some() {
                                return Err(serde::de::Error::duplicate_field("filter"));
                            }
                            filter__ = map_.next_value()?;
                        }
                    }
                }
                Ok(ListOffersRequest {
                    user_id: user_id__,
                    shop_id: shop_id__,
                    pagination: pagination__,
                    order_by: order_by__,
                    filter: filter__,
                })
            }
        }
        deserializer.deserialize_struct("sited_io.commerce.v1.ListOffersRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListOffersResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.offers.is_empty() {
            len += 1;
        }
        if self.pagination.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sited_io.commerce.v1.ListOffersResponse", len)?;
        if !self.offers.is_empty() {
            struct_ser.serialize_field("offers", &self.offers)?;
        }
        if let Some(v) = self.pagination.as_ref() {
            struct_ser.serialize_field("pagination", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListOffersResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "offers",
            "pagination",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Offers,
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
                            "offers" => Ok(GeneratedField::Offers),
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
            type Value = ListOffersResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.commerce.v1.ListOffersResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListOffersResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut offers__ = None;
                let mut pagination__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Offers => {
                            if offers__.is_some() {
                                return Err(serde::de::Error::duplicate_field("offers"));
                            }
                            offers__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Pagination => {
                            if pagination__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pagination"));
                            }
                            pagination__ = map_.next_value()?;
                        }
                    }
                }
                Ok(ListOffersResponse {
                    offers: offers__.unwrap_or_default(),
                    pagination: pagination__,
                })
            }
        }
        deserializer.deserialize_struct("sited_io.commerce.v1.ListOffersResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListShopsRequest {
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
        if self.order_by.is_some() {
            len += 1;
        }
        if self.filter.is_some() {
            len += 1;
        }
        if self.extended.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sited_io.commerce.v1.ListShopsRequest", len)?;
        if let Some(v) = self.user_id.as_ref() {
            struct_ser.serialize_field("userId", v)?;
        }
        if let Some(v) = self.pagination.as_ref() {
            struct_ser.serialize_field("pagination", v)?;
        }
        if let Some(v) = self.order_by.as_ref() {
            struct_ser.serialize_field("orderBy", v)?;
        }
        if let Some(v) = self.filter.as_ref() {
            struct_ser.serialize_field("filter", v)?;
        }
        if let Some(v) = self.extended.as_ref() {
            struct_ser.serialize_field("extended", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListShopsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "user_id",
            "userId",
            "pagination",
            "order_by",
            "orderBy",
            "filter",
            "extended",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            UserId,
            Pagination,
            OrderBy,
            Filter,
            Extended,
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
                            "orderBy" | "order_by" => Ok(GeneratedField::OrderBy),
                            "filter" => Ok(GeneratedField::Filter),
                            "extended" => Ok(GeneratedField::Extended),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListShopsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.commerce.v1.ListShopsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListShopsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut user_id__ = None;
                let mut pagination__ = None;
                let mut order_by__ = None;
                let mut filter__ = None;
                let mut extended__ = None;
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
                        GeneratedField::OrderBy => {
                            if order_by__.is_some() {
                                return Err(serde::de::Error::duplicate_field("orderBy"));
                            }
                            order_by__ = map_.next_value()?;
                        }
                        GeneratedField::Filter => {
                            if filter__.is_some() {
                                return Err(serde::de::Error::duplicate_field("filter"));
                            }
                            filter__ = map_.next_value()?;
                        }
                        GeneratedField::Extended => {
                            if extended__.is_some() {
                                return Err(serde::de::Error::duplicate_field("extended"));
                            }
                            extended__ = map_.next_value()?;
                        }
                    }
                }
                Ok(ListShopsRequest {
                    user_id: user_id__,
                    pagination: pagination__,
                    order_by: order_by__,
                    filter: filter__,
                    extended: extended__,
                })
            }
        }
        deserializer.deserialize_struct("sited_io.commerce.v1.ListShopsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListShopsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.shops.is_empty() {
            len += 1;
        }
        if self.pagination.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sited_io.commerce.v1.ListShopsResponse", len)?;
        if !self.shops.is_empty() {
            struct_ser.serialize_field("shops", &self.shops)?;
        }
        if let Some(v) = self.pagination.as_ref() {
            struct_ser.serialize_field("pagination", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListShopsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "shops",
            "pagination",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Shops,
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
                            "shops" => Ok(GeneratedField::Shops),
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
            type Value = ListShopsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.commerce.v1.ListShopsResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListShopsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut shops__ = None;
                let mut pagination__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Shops => {
                            if shops__.is_some() {
                                return Err(serde::de::Error::duplicate_field("shops"));
                            }
                            shops__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Pagination => {
                            if pagination__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pagination"));
                            }
                            pagination__ = map_.next_value()?;
                        }
                    }
                }
                Ok(ListShopsResponse {
                    shops: shops__.unwrap_or_default(),
                    pagination: pagination__,
                })
            }
        }
        deserializer.deserialize_struct("sited_io.commerce.v1.ListShopsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for OfferImageResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.offer_image_id.is_empty() {
            len += 1;
        }
        if !self.image_url.is_empty() {
            len += 1;
        }
        if self.ordering != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sited_io.commerce.v1.OfferImageResponse", len)?;
        if !self.offer_image_id.is_empty() {
            struct_ser.serialize_field("offerImageId", &self.offer_image_id)?;
        }
        if !self.image_url.is_empty() {
            struct_ser.serialize_field("imageUrl", &self.image_url)?;
        }
        if self.ordering != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("ordering", ToString::to_string(&self.ordering).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for OfferImageResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "offer_image_id",
            "offerImageId",
            "image_url",
            "imageUrl",
            "ordering",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            OfferImageId,
            ImageUrl,
            Ordering,
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
                            "offerImageId" | "offer_image_id" => Ok(GeneratedField::OfferImageId),
                            "imageUrl" | "image_url" => Ok(GeneratedField::ImageUrl),
                            "ordering" => Ok(GeneratedField::Ordering),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = OfferImageResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.commerce.v1.OfferImageResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<OfferImageResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut offer_image_id__ = None;
                let mut image_url__ = None;
                let mut ordering__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::OfferImageId => {
                            if offer_image_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("offerImageId"));
                            }
                            offer_image_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ImageUrl => {
                            if image_url__.is_some() {
                                return Err(serde::de::Error::duplicate_field("imageUrl"));
                            }
                            image_url__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Ordering => {
                            if ordering__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ordering"));
                            }
                            ordering__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(OfferImageResponse {
                    offer_image_id: offer_image_id__.unwrap_or_default(),
                    image_url: image_url__.unwrap_or_default(),
                    ordering: ordering__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sited_io.commerce.v1.OfferImageResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for OfferResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.offer_id.is_empty() {
            len += 1;
        }
        if !self.shop_id.is_empty() {
            len += 1;
        }
        if !self.shop_name.is_empty() {
            len += 1;
        }
        if !self.shop_slug.is_empty() {
            len += 1;
        }
        if self.shop_domain.is_some() {
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
        if !self.description.is_empty() {
            len += 1;
        }
        if self.is_active {
            len += 1;
        }
        if self.is_featured {
            len += 1;
        }
        if self.r#type != 0 {
            len += 1;
        }
        if !self.images.is_empty() {
            len += 1;
        }
        if self.price.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sited_io.commerce.v1.OfferResponse", len)?;
        if !self.offer_id.is_empty() {
            struct_ser.serialize_field("offerId", &self.offer_id)?;
        }
        if !self.shop_id.is_empty() {
            struct_ser.serialize_field("shopId", &self.shop_id)?;
        }
        if !self.shop_name.is_empty() {
            struct_ser.serialize_field("shopName", &self.shop_name)?;
        }
        if !self.shop_slug.is_empty() {
            struct_ser.serialize_field("shopSlug", &self.shop_slug)?;
        }
        if let Some(v) = self.shop_domain.as_ref() {
            struct_ser.serialize_field("shopDomain", v)?;
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
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
        }
        if self.is_active {
            struct_ser.serialize_field("isActive", &self.is_active)?;
        }
        if self.is_featured {
            struct_ser.serialize_field("isFeatured", &self.is_featured)?;
        }
        if self.r#type != 0 {
            let v = OfferType::try_from(self.r#type)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.r#type)))?;
            struct_ser.serialize_field("type", &v)?;
        }
        if !self.images.is_empty() {
            struct_ser.serialize_field("images", &self.images)?;
        }
        if let Some(v) = self.price.as_ref() {
            struct_ser.serialize_field("price", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for OfferResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "offer_id",
            "offerId",
            "shop_id",
            "shopId",
            "shop_name",
            "shopName",
            "shop_slug",
            "shopSlug",
            "shop_domain",
            "shopDomain",
            "user_id",
            "userId",
            "created_at",
            "createdAt",
            "updated_at",
            "updatedAt",
            "name",
            "description",
            "is_active",
            "isActive",
            "is_featured",
            "isFeatured",
            "type",
            "images",
            "price",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            OfferId,
            ShopId,
            ShopName,
            ShopSlug,
            ShopDomain,
            UserId,
            CreatedAt,
            UpdatedAt,
            Name,
            Description,
            IsActive,
            IsFeatured,
            Type,
            Images,
            Price,
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
                            "offerId" | "offer_id" => Ok(GeneratedField::OfferId),
                            "shopId" | "shop_id" => Ok(GeneratedField::ShopId),
                            "shopName" | "shop_name" => Ok(GeneratedField::ShopName),
                            "shopSlug" | "shop_slug" => Ok(GeneratedField::ShopSlug),
                            "shopDomain" | "shop_domain" => Ok(GeneratedField::ShopDomain),
                            "userId" | "user_id" => Ok(GeneratedField::UserId),
                            "createdAt" | "created_at" => Ok(GeneratedField::CreatedAt),
                            "updatedAt" | "updated_at" => Ok(GeneratedField::UpdatedAt),
                            "name" => Ok(GeneratedField::Name),
                            "description" => Ok(GeneratedField::Description),
                            "isActive" | "is_active" => Ok(GeneratedField::IsActive),
                            "isFeatured" | "is_featured" => Ok(GeneratedField::IsFeatured),
                            "type" => Ok(GeneratedField::Type),
                            "images" => Ok(GeneratedField::Images),
                            "price" => Ok(GeneratedField::Price),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = OfferResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.commerce.v1.OfferResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<OfferResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut offer_id__ = None;
                let mut shop_id__ = None;
                let mut shop_name__ = None;
                let mut shop_slug__ = None;
                let mut shop_domain__ = None;
                let mut user_id__ = None;
                let mut created_at__ = None;
                let mut updated_at__ = None;
                let mut name__ = None;
                let mut description__ = None;
                let mut is_active__ = None;
                let mut is_featured__ = None;
                let mut r#type__ = None;
                let mut images__ = None;
                let mut price__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::OfferId => {
                            if offer_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("offerId"));
                            }
                            offer_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ShopId => {
                            if shop_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("shopId"));
                            }
                            shop_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ShopName => {
                            if shop_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("shopName"));
                            }
                            shop_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ShopSlug => {
                            if shop_slug__.is_some() {
                                return Err(serde::de::Error::duplicate_field("shopSlug"));
                            }
                            shop_slug__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ShopDomain => {
                            if shop_domain__.is_some() {
                                return Err(serde::de::Error::duplicate_field("shopDomain"));
                            }
                            shop_domain__ = map_.next_value()?;
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
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = Some(map_.next_value()?);
                        }
                        GeneratedField::IsActive => {
                            if is_active__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isActive"));
                            }
                            is_active__ = Some(map_.next_value()?);
                        }
                        GeneratedField::IsFeatured => {
                            if is_featured__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isFeatured"));
                            }
                            is_featured__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Type => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type__ = Some(map_.next_value::<OfferType>()? as i32);
                        }
                        GeneratedField::Images => {
                            if images__.is_some() {
                                return Err(serde::de::Error::duplicate_field("images"));
                            }
                            images__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Price => {
                            if price__.is_some() {
                                return Err(serde::de::Error::duplicate_field("price"));
                            }
                            price__ = map_.next_value()?;
                        }
                    }
                }
                Ok(OfferResponse {
                    offer_id: offer_id__.unwrap_or_default(),
                    shop_id: shop_id__.unwrap_or_default(),
                    shop_name: shop_name__.unwrap_or_default(),
                    shop_slug: shop_slug__.unwrap_or_default(),
                    shop_domain: shop_domain__,
                    user_id: user_id__.unwrap_or_default(),
                    created_at: created_at__.unwrap_or_default(),
                    updated_at: updated_at__.unwrap_or_default(),
                    name: name__.unwrap_or_default(),
                    description: description__.unwrap_or_default(),
                    is_active: is_active__.unwrap_or_default(),
                    is_featured: is_featured__.unwrap_or_default(),
                    r#type: r#type__.unwrap_or_default(),
                    images: images__.unwrap_or_default(),
                    price: price__,
                })
            }
        }
        deserializer.deserialize_struct("sited_io.commerce.v1.OfferResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for OfferType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "OFFER_TYPE_UNSPECIFIED",
            Self::Physical => "OFFER_TYPE_PHYSICAL",
            Self::Digital => "OFFER_TYPE_DIGITAL",
            Self::Service => "OFFER_TYPE_SERVICE",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for OfferType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "OFFER_TYPE_UNSPECIFIED",
            "OFFER_TYPE_PHYSICAL",
            "OFFER_TYPE_DIGITAL",
            "OFFER_TYPE_SERVICE",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = OfferType;

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
                    "OFFER_TYPE_UNSPECIFIED" => Ok(OfferType::Unspecified),
                    "OFFER_TYPE_PHYSICAL" => Ok(OfferType::Physical),
                    "OFFER_TYPE_DIGITAL" => Ok(OfferType::Digital),
                    "OFFER_TYPE_SERVICE" => Ok(OfferType::Service),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for OffersFilter {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.field != 0 {
            len += 1;
        }
        if !self.query.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sited_io.commerce.v1.OffersFilter", len)?;
        if self.field != 0 {
            let v = OffersFilterField::try_from(self.field)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.field)))?;
            struct_ser.serialize_field("field", &v)?;
        }
        if !self.query.is_empty() {
            struct_ser.serialize_field("query", &self.query)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for OffersFilter {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "field",
            "query",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Field,
            Query,
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
                            "field" => Ok(GeneratedField::Field),
                            "query" => Ok(GeneratedField::Query),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = OffersFilter;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.commerce.v1.OffersFilter")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<OffersFilter, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut field__ = None;
                let mut query__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Field => {
                            if field__.is_some() {
                                return Err(serde::de::Error::duplicate_field("field"));
                            }
                            field__ = Some(map_.next_value::<OffersFilterField>()? as i32);
                        }
                        GeneratedField::Query => {
                            if query__.is_some() {
                                return Err(serde::de::Error::duplicate_field("query"));
                            }
                            query__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(OffersFilter {
                    field: field__.unwrap_or_default(),
                    query: query__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sited_io.commerce.v1.OffersFilter", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for OffersFilterField {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "OFFERS_FILTER_FIELD_UNSPECIFIED",
            Self::Name => "OFFERS_FILTER_FIELD_NAME",
            Self::Description => "OFFERS_FILTER_FIELD_DESCRIPTION",
            Self::NameAndDescription => "OFFERS_FILTER_FIELD_NAME_AND_DESCRIPTION",
            Self::Type => "OFFERS_FILTER_FIELD_TYPE",
            Self::IsFeatured => "OFFERS_FILTER_FIELD_IS_FEATURED",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for OffersFilterField {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "OFFERS_FILTER_FIELD_UNSPECIFIED",
            "OFFERS_FILTER_FIELD_NAME",
            "OFFERS_FILTER_FIELD_DESCRIPTION",
            "OFFERS_FILTER_FIELD_NAME_AND_DESCRIPTION",
            "OFFERS_FILTER_FIELD_TYPE",
            "OFFERS_FILTER_FIELD_IS_FEATURED",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = OffersFilterField;

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
                    "OFFERS_FILTER_FIELD_UNSPECIFIED" => Ok(OffersFilterField::Unspecified),
                    "OFFERS_FILTER_FIELD_NAME" => Ok(OffersFilterField::Name),
                    "OFFERS_FILTER_FIELD_DESCRIPTION" => Ok(OffersFilterField::Description),
                    "OFFERS_FILTER_FIELD_NAME_AND_DESCRIPTION" => Ok(OffersFilterField::NameAndDescription),
                    "OFFERS_FILTER_FIELD_TYPE" => Ok(OffersFilterField::Type),
                    "OFFERS_FILTER_FIELD_IS_FEATURED" => Ok(OffersFilterField::IsFeatured),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for OffersOrderBy {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.field != 0 {
            len += 1;
        }
        if self.direction != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sited_io.commerce.v1.OffersOrderBy", len)?;
        if self.field != 0 {
            let v = OffersOrderByField::try_from(self.field)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.field)))?;
            struct_ser.serialize_field("field", &v)?;
        }
        if self.direction != 0 {
            let v = super::super::types::v1::Direction::try_from(self.direction)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.direction)))?;
            struct_ser.serialize_field("direction", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for OffersOrderBy {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "field",
            "direction",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Field,
            Direction,
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
                            "field" => Ok(GeneratedField::Field),
                            "direction" => Ok(GeneratedField::Direction),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = OffersOrderBy;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.commerce.v1.OffersOrderBy")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<OffersOrderBy, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut field__ = None;
                let mut direction__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Field => {
                            if field__.is_some() {
                                return Err(serde::de::Error::duplicate_field("field"));
                            }
                            field__ = Some(map_.next_value::<OffersOrderByField>()? as i32);
                        }
                        GeneratedField::Direction => {
                            if direction__.is_some() {
                                return Err(serde::de::Error::duplicate_field("direction"));
                            }
                            direction__ = Some(map_.next_value::<super::super::types::v1::Direction>()? as i32);
                        }
                    }
                }
                Ok(OffersOrderBy {
                    field: field__.unwrap_or_default(),
                    direction: direction__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sited_io.commerce.v1.OffersOrderBy", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for OffersOrderByField {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "OFFERS_ORDER_BY_FIELD_UNSPECIFIED",
            Self::CreatedAt => "OFFERS_ORDER_BY_FIELD_CREATED_AT",
            Self::UpdatedAt => "OFFERS_ORDER_BY_FIELD_UPDATED_AT",
            Self::Name => "OFFERS_ORDER_BY_FIELD_NAME",
            Self::Random => "OFFERS_ORDER_BY_FIELD_RANDOM",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for OffersOrderByField {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "OFFERS_ORDER_BY_FIELD_UNSPECIFIED",
            "OFFERS_ORDER_BY_FIELD_CREATED_AT",
            "OFFERS_ORDER_BY_FIELD_UPDATED_AT",
            "OFFERS_ORDER_BY_FIELD_NAME",
            "OFFERS_ORDER_BY_FIELD_RANDOM",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = OffersOrderByField;

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
                    "OFFERS_ORDER_BY_FIELD_UNSPECIFIED" => Ok(OffersOrderByField::Unspecified),
                    "OFFERS_ORDER_BY_FIELD_CREATED_AT" => Ok(OffersOrderByField::CreatedAt),
                    "OFFERS_ORDER_BY_FIELD_UPDATED_AT" => Ok(OffersOrderByField::UpdatedAt),
                    "OFFERS_ORDER_BY_FIELD_NAME" => Ok(OffersOrderByField::Name),
                    "OFFERS_ORDER_BY_FIELD_RANDOM" => Ok(OffersOrderByField::Random),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for Price {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.currency != 0 {
            len += 1;
        }
        if self.price_type != 0 {
            len += 1;
        }
        if self.billing_scheme != 0 {
            len += 1;
        }
        if self.unit_amount != 0 {
            len += 1;
        }
        if self.recurring.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sited_io.commerce.v1.Price", len)?;
        if self.currency != 0 {
            let v = Currency::try_from(self.currency)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.currency)))?;
            struct_ser.serialize_field("currency", &v)?;
        }
        if self.price_type != 0 {
            let v = PriceType::try_from(self.price_type)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.price_type)))?;
            struct_ser.serialize_field("priceType", &v)?;
        }
        if self.billing_scheme != 0 {
            let v = PriceBillingScheme::try_from(self.billing_scheme)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.billing_scheme)))?;
            struct_ser.serialize_field("billingScheme", &v)?;
        }
        if self.unit_amount != 0 {
            struct_ser.serialize_field("unitAmount", &self.unit_amount)?;
        }
        if let Some(v) = self.recurring.as_ref() {
            struct_ser.serialize_field("recurring", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Price {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "currency",
            "price_type",
            "priceType",
            "billing_scheme",
            "billingScheme",
            "unit_amount",
            "unitAmount",
            "recurring",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Currency,
            PriceType,
            BillingScheme,
            UnitAmount,
            Recurring,
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
                            "currency" => Ok(GeneratedField::Currency),
                            "priceType" | "price_type" => Ok(GeneratedField::PriceType),
                            "billingScheme" | "billing_scheme" => Ok(GeneratedField::BillingScheme),
                            "unitAmount" | "unit_amount" => Ok(GeneratedField::UnitAmount),
                            "recurring" => Ok(GeneratedField::Recurring),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Price;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.commerce.v1.Price")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Price, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut currency__ = None;
                let mut price_type__ = None;
                let mut billing_scheme__ = None;
                let mut unit_amount__ = None;
                let mut recurring__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Currency => {
                            if currency__.is_some() {
                                return Err(serde::de::Error::duplicate_field("currency"));
                            }
                            currency__ = Some(map_.next_value::<Currency>()? as i32);
                        }
                        GeneratedField::PriceType => {
                            if price_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("priceType"));
                            }
                            price_type__ = Some(map_.next_value::<PriceType>()? as i32);
                        }
                        GeneratedField::BillingScheme => {
                            if billing_scheme__.is_some() {
                                return Err(serde::de::Error::duplicate_field("billingScheme"));
                            }
                            billing_scheme__ = Some(map_.next_value::<PriceBillingScheme>()? as i32);
                        }
                        GeneratedField::UnitAmount => {
                            if unit_amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("unitAmount"));
                            }
                            unit_amount__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Recurring => {
                            if recurring__.is_some() {
                                return Err(serde::de::Error::duplicate_field("recurring"));
                            }
                            recurring__ = map_.next_value()?;
                        }
                    }
                }
                Ok(Price {
                    currency: currency__.unwrap_or_default(),
                    price_type: price_type__.unwrap_or_default(),
                    billing_scheme: billing_scheme__.unwrap_or_default(),
                    unit_amount: unit_amount__.unwrap_or_default(),
                    recurring: recurring__,
                })
            }
        }
        deserializer.deserialize_struct("sited_io.commerce.v1.Price", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PriceBillingScheme {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "PRICE_BILLING_SCHEME_UNSPECIFIED",
            Self::PerUnit => "PRICE_BILLING_SCHEME_PER_UNIT",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for PriceBillingScheme {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "PRICE_BILLING_SCHEME_UNSPECIFIED",
            "PRICE_BILLING_SCHEME_PER_UNIT",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PriceBillingScheme;

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
                    "PRICE_BILLING_SCHEME_UNSPECIFIED" => Ok(PriceBillingScheme::Unspecified),
                    "PRICE_BILLING_SCHEME_PER_UNIT" => Ok(PriceBillingScheme::PerUnit),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for PriceType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "PRICE_TYPE_UNSPECIFIED",
            Self::OneTime => "PRICE_TYPE_ONE_TIME",
            Self::Recurring => "PRICE_TYPE_RECURRING",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for PriceType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "PRICE_TYPE_UNSPECIFIED",
            "PRICE_TYPE_ONE_TIME",
            "PRICE_TYPE_RECURRING",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PriceType;

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
                    "PRICE_TYPE_UNSPECIFIED" => Ok(PriceType::Unspecified),
                    "PRICE_TYPE_ONE_TIME" => Ok(PriceType::OneTime),
                    "PRICE_TYPE_RECURRING" => Ok(PriceType::Recurring),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for PutBannerImageToShopRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.shop_id.is_empty() {
            len += 1;
        }
        if self.image.is_some() {
            len += 1;
        }
        if self.image_dark.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sited_io.commerce.v1.PutBannerImageToShopRequest", len)?;
        if !self.shop_id.is_empty() {
            struct_ser.serialize_field("shopId", &self.shop_id)?;
        }
        if let Some(v) = self.image.as_ref() {
            struct_ser.serialize_field("image", v)?;
        }
        if let Some(v) = self.image_dark.as_ref() {
            struct_ser.serialize_field("imageDark", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PutBannerImageToShopRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "shop_id",
            "shopId",
            "image",
            "image_dark",
            "imageDark",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ShopId,
            Image,
            ImageDark,
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
                            "shopId" | "shop_id" => Ok(GeneratedField::ShopId),
                            "image" => Ok(GeneratedField::Image),
                            "imageDark" | "image_dark" => Ok(GeneratedField::ImageDark),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PutBannerImageToShopRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.commerce.v1.PutBannerImageToShopRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<PutBannerImageToShopRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut shop_id__ = None;
                let mut image__ = None;
                let mut image_dark__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ShopId => {
                            if shop_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("shopId"));
                            }
                            shop_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Image => {
                            if image__.is_some() {
                                return Err(serde::de::Error::duplicate_field("image"));
                            }
                            image__ = map_.next_value()?;
                        }
                        GeneratedField::ImageDark => {
                            if image_dark__.is_some() {
                                return Err(serde::de::Error::duplicate_field("imageDark"));
                            }
                            image_dark__ = map_.next_value()?;
                        }
                    }
                }
                Ok(PutBannerImageToShopRequest {
                    shop_id: shop_id__.unwrap_or_default(),
                    image: image__,
                    image_dark: image_dark__,
                })
            }
        }
        deserializer.deserialize_struct("sited_io.commerce.v1.PutBannerImageToShopRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PutBannerImageToShopResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("sited_io.commerce.v1.PutBannerImageToShopResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PutBannerImageToShopResponse {
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
            type Value = PutBannerImageToShopResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.commerce.v1.PutBannerImageToShopResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<PutBannerImageToShopResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(PutBannerImageToShopResponse {
                })
            }
        }
        deserializer.deserialize_struct("sited_io.commerce.v1.PutBannerImageToShopResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PutLogoImageToShopRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.shop_id.is_empty() {
            len += 1;
        }
        if self.image.is_some() {
            len += 1;
        }
        if self.image_dark.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sited_io.commerce.v1.PutLogoImageToShopRequest", len)?;
        if !self.shop_id.is_empty() {
            struct_ser.serialize_field("shopId", &self.shop_id)?;
        }
        if let Some(v) = self.image.as_ref() {
            struct_ser.serialize_field("image", v)?;
        }
        if let Some(v) = self.image_dark.as_ref() {
            struct_ser.serialize_field("imageDark", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PutLogoImageToShopRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "shop_id",
            "shopId",
            "image",
            "image_dark",
            "imageDark",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ShopId,
            Image,
            ImageDark,
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
                            "shopId" | "shop_id" => Ok(GeneratedField::ShopId),
                            "image" => Ok(GeneratedField::Image),
                            "imageDark" | "image_dark" => Ok(GeneratedField::ImageDark),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PutLogoImageToShopRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.commerce.v1.PutLogoImageToShopRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<PutLogoImageToShopRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut shop_id__ = None;
                let mut image__ = None;
                let mut image_dark__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ShopId => {
                            if shop_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("shopId"));
                            }
                            shop_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Image => {
                            if image__.is_some() {
                                return Err(serde::de::Error::duplicate_field("image"));
                            }
                            image__ = map_.next_value()?;
                        }
                        GeneratedField::ImageDark => {
                            if image_dark__.is_some() {
                                return Err(serde::de::Error::duplicate_field("imageDark"));
                            }
                            image_dark__ = map_.next_value()?;
                        }
                    }
                }
                Ok(PutLogoImageToShopRequest {
                    shop_id: shop_id__.unwrap_or_default(),
                    image: image__,
                    image_dark: image_dark__,
                })
            }
        }
        deserializer.deserialize_struct("sited_io.commerce.v1.PutLogoImageToShopRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PutLogoImageToShopResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("sited_io.commerce.v1.PutLogoImageToShopResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PutLogoImageToShopResponse {
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
            type Value = PutLogoImageToShopResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.commerce.v1.PutLogoImageToShopResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<PutLogoImageToShopResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(PutLogoImageToShopResponse {
                })
            }
        }
        deserializer.deserialize_struct("sited_io.commerce.v1.PutLogoImageToShopResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PutPriceToOfferRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.offer_id.is_empty() {
            len += 1;
        }
        if self.price.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sited_io.commerce.v1.PutPriceToOfferRequest", len)?;
        if !self.offer_id.is_empty() {
            struct_ser.serialize_field("offerId", &self.offer_id)?;
        }
        if let Some(v) = self.price.as_ref() {
            struct_ser.serialize_field("price", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PutPriceToOfferRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "offer_id",
            "offerId",
            "price",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            OfferId,
            Price,
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
                            "offerId" | "offer_id" => Ok(GeneratedField::OfferId),
                            "price" => Ok(GeneratedField::Price),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PutPriceToOfferRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.commerce.v1.PutPriceToOfferRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<PutPriceToOfferRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut offer_id__ = None;
                let mut price__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::OfferId => {
                            if offer_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("offerId"));
                            }
                            offer_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Price => {
                            if price__.is_some() {
                                return Err(serde::de::Error::duplicate_field("price"));
                            }
                            price__ = map_.next_value()?;
                        }
                    }
                }
                Ok(PutPriceToOfferRequest {
                    offer_id: offer_id__.unwrap_or_default(),
                    price: price__,
                })
            }
        }
        deserializer.deserialize_struct("sited_io.commerce.v1.PutPriceToOfferRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PutPriceToOfferResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("sited_io.commerce.v1.PutPriceToOfferResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PutPriceToOfferResponse {
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
            type Value = PutPriceToOfferResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.commerce.v1.PutPriceToOfferResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<PutPriceToOfferResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(PutPriceToOfferResponse {
                })
            }
        }
        deserializer.deserialize_struct("sited_io.commerce.v1.PutPriceToOfferResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PutShippingRateRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.offer_id.is_empty() {
            len += 1;
        }
        if self.amount != 0 {
            len += 1;
        }
        if self.currency != 0 {
            len += 1;
        }
        if self.all_countries {
            len += 1;
        }
        if !self.specific_countries.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sited_io.commerce.v1.PutShippingRateRequest", len)?;
        if !self.offer_id.is_empty() {
            struct_ser.serialize_field("offerId", &self.offer_id)?;
        }
        if self.amount != 0 {
            struct_ser.serialize_field("amount", &self.amount)?;
        }
        if self.currency != 0 {
            let v = Currency::try_from(self.currency)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.currency)))?;
            struct_ser.serialize_field("currency", &v)?;
        }
        if self.all_countries {
            struct_ser.serialize_field("allCountries", &self.all_countries)?;
        }
        if !self.specific_countries.is_empty() {
            let v = self.specific_countries.iter().cloned().map(|v| {
                ShippingCountry::try_from(v)
                    .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", v)))
                }).collect::<std::result::Result<Vec<_>, _>>()?;
            struct_ser.serialize_field("specificCountries", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PutShippingRateRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "offer_id",
            "offerId",
            "amount",
            "currency",
            "all_countries",
            "allCountries",
            "specific_countries",
            "specificCountries",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            OfferId,
            Amount,
            Currency,
            AllCountries,
            SpecificCountries,
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
                            "offerId" | "offer_id" => Ok(GeneratedField::OfferId),
                            "amount" => Ok(GeneratedField::Amount),
                            "currency" => Ok(GeneratedField::Currency),
                            "allCountries" | "all_countries" => Ok(GeneratedField::AllCountries),
                            "specificCountries" | "specific_countries" => Ok(GeneratedField::SpecificCountries),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PutShippingRateRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.commerce.v1.PutShippingRateRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<PutShippingRateRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut offer_id__ = None;
                let mut amount__ = None;
                let mut currency__ = None;
                let mut all_countries__ = None;
                let mut specific_countries__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::OfferId => {
                            if offer_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("offerId"));
                            }
                            offer_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Amount => {
                            if amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("amount"));
                            }
                            amount__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Currency => {
                            if currency__.is_some() {
                                return Err(serde::de::Error::duplicate_field("currency"));
                            }
                            currency__ = Some(map_.next_value::<Currency>()? as i32);
                        }
                        GeneratedField::AllCountries => {
                            if all_countries__.is_some() {
                                return Err(serde::de::Error::duplicate_field("allCountries"));
                            }
                            all_countries__ = Some(map_.next_value()?);
                        }
                        GeneratedField::SpecificCountries => {
                            if specific_countries__.is_some() {
                                return Err(serde::de::Error::duplicate_field("specificCountries"));
                            }
                            specific_countries__ = Some(map_.next_value::<Vec<ShippingCountry>>()?.into_iter().map(|x| x as i32).collect());
                        }
                    }
                }
                Ok(PutShippingRateRequest {
                    offer_id: offer_id__.unwrap_or_default(),
                    amount: amount__.unwrap_or_default(),
                    currency: currency__.unwrap_or_default(),
                    all_countries: all_countries__.unwrap_or_default(),
                    specific_countries: specific_countries__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sited_io.commerce.v1.PutShippingRateRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PutShippingRateResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("sited_io.commerce.v1.PutShippingRateResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PutShippingRateResponse {
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
            type Value = PutShippingRateResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.commerce.v1.PutShippingRateResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<PutShippingRateResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(PutShippingRateResponse {
                })
            }
        }
        deserializer.deserialize_struct("sited_io.commerce.v1.PutShippingRateResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PutShopCustomizationRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.shop_id.is_empty() {
            len += 1;
        }
        if self.primary_color.is_some() {
            len += 1;
        }
        if self.layout_type != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sited_io.commerce.v1.PutShopCustomizationRequest", len)?;
        if !self.shop_id.is_empty() {
            struct_ser.serialize_field("shopId", &self.shop_id)?;
        }
        if let Some(v) = self.primary_color.as_ref() {
            struct_ser.serialize_field("primaryColor", v)?;
        }
        if self.layout_type != 0 {
            let v = ShopLayoutType::try_from(self.layout_type)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.layout_type)))?;
            struct_ser.serialize_field("layoutType", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PutShopCustomizationRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "shop_id",
            "shopId",
            "primary_color",
            "primaryColor",
            "layout_type",
            "layoutType",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ShopId,
            PrimaryColor,
            LayoutType,
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
                            "shopId" | "shop_id" => Ok(GeneratedField::ShopId),
                            "primaryColor" | "primary_color" => Ok(GeneratedField::PrimaryColor),
                            "layoutType" | "layout_type" => Ok(GeneratedField::LayoutType),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PutShopCustomizationRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.commerce.v1.PutShopCustomizationRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<PutShopCustomizationRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut shop_id__ = None;
                let mut primary_color__ = None;
                let mut layout_type__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ShopId => {
                            if shop_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("shopId"));
                            }
                            shop_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::PrimaryColor => {
                            if primary_color__.is_some() {
                                return Err(serde::de::Error::duplicate_field("primaryColor"));
                            }
                            primary_color__ = map_.next_value()?;
                        }
                        GeneratedField::LayoutType => {
                            if layout_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("layoutType"));
                            }
                            layout_type__ = Some(map_.next_value::<ShopLayoutType>()? as i32);
                        }
                    }
                }
                Ok(PutShopCustomizationRequest {
                    shop_id: shop_id__.unwrap_or_default(),
                    primary_color: primary_color__,
                    layout_type: layout_type__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sited_io.commerce.v1.PutShopCustomizationRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PutShopCustomizationResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.shop_customization.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sited_io.commerce.v1.PutShopCustomizationResponse", len)?;
        if let Some(v) = self.shop_customization.as_ref() {
            struct_ser.serialize_field("shopCustomization", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PutShopCustomizationResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "shop_customization",
            "shopCustomization",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ShopCustomization,
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
                            "shopCustomization" | "shop_customization" => Ok(GeneratedField::ShopCustomization),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PutShopCustomizationResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.commerce.v1.PutShopCustomizationResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<PutShopCustomizationResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut shop_customization__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ShopCustomization => {
                            if shop_customization__.is_some() {
                                return Err(serde::de::Error::duplicate_field("shopCustomization"));
                            }
                            shop_customization__ = map_.next_value()?;
                        }
                    }
                }
                Ok(PutShopCustomizationResponse {
                    shop_customization: shop_customization__,
                })
            }
        }
        deserializer.deserialize_struct("sited_io.commerce.v1.PutShopCustomizationResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Recurring {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.interval != 0 {
            len += 1;
        }
        if self.interval_count != 0 {
            len += 1;
        }
        if self.trial_period_days.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sited_io.commerce.v1.Recurring", len)?;
        if self.interval != 0 {
            let v = RecurringInterval::try_from(self.interval)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.interval)))?;
            struct_ser.serialize_field("interval", &v)?;
        }
        if self.interval_count != 0 {
            struct_ser.serialize_field("intervalCount", &self.interval_count)?;
        }
        if let Some(v) = self.trial_period_days.as_ref() {
            struct_ser.serialize_field("trialPeriodDays", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Recurring {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "interval",
            "interval_count",
            "intervalCount",
            "trial_period_days",
            "trialPeriodDays",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Interval,
            IntervalCount,
            TrialPeriodDays,
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
                            "interval" => Ok(GeneratedField::Interval),
                            "intervalCount" | "interval_count" => Ok(GeneratedField::IntervalCount),
                            "trialPeriodDays" | "trial_period_days" => Ok(GeneratedField::TrialPeriodDays),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Recurring;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.commerce.v1.Recurring")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Recurring, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut interval__ = None;
                let mut interval_count__ = None;
                let mut trial_period_days__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Interval => {
                            if interval__.is_some() {
                                return Err(serde::de::Error::duplicate_field("interval"));
                            }
                            interval__ = Some(map_.next_value::<RecurringInterval>()? as i32);
                        }
                        GeneratedField::IntervalCount => {
                            if interval_count__.is_some() {
                                return Err(serde::de::Error::duplicate_field("intervalCount"));
                            }
                            interval_count__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::TrialPeriodDays => {
                            if trial_period_days__.is_some() {
                                return Err(serde::de::Error::duplicate_field("trialPeriodDays"));
                            }
                            trial_period_days__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                    }
                }
                Ok(Recurring {
                    interval: interval__.unwrap_or_default(),
                    interval_count: interval_count__.unwrap_or_default(),
                    trial_period_days: trial_period_days__,
                })
            }
        }
        deserializer.deserialize_struct("sited_io.commerce.v1.Recurring", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RecurringInterval {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "RECURRING_INTERVAL_UNSPECIFIED",
            Self::Day => "RECURRING_INTERVAL_DAY",
            Self::Week => "RECURRING_INTERVAL_WEEK",
            Self::Month => "RECURRING_INTERVAL_MONTH",
            Self::Year => "RECURRING_INTERVAL_YEAR",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for RecurringInterval {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "RECURRING_INTERVAL_UNSPECIFIED",
            "RECURRING_INTERVAL_DAY",
            "RECURRING_INTERVAL_WEEK",
            "RECURRING_INTERVAL_MONTH",
            "RECURRING_INTERVAL_YEAR",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RecurringInterval;

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
                    "RECURRING_INTERVAL_UNSPECIFIED" => Ok(RecurringInterval::Unspecified),
                    "RECURRING_INTERVAL_DAY" => Ok(RecurringInterval::Day),
                    "RECURRING_INTERVAL_WEEK" => Ok(RecurringInterval::Week),
                    "RECURRING_INTERVAL_MONTH" => Ok(RecurringInterval::Month),
                    "RECURRING_INTERVAL_YEAR" => Ok(RecurringInterval::Year),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for RemoveBannerImageFromShopRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.shop_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sited_io.commerce.v1.RemoveBannerImageFromShopRequest", len)?;
        if !self.shop_id.is_empty() {
            struct_ser.serialize_field("shopId", &self.shop_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RemoveBannerImageFromShopRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "shop_id",
            "shopId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ShopId,
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
                            "shopId" | "shop_id" => Ok(GeneratedField::ShopId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RemoveBannerImageFromShopRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.commerce.v1.RemoveBannerImageFromShopRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RemoveBannerImageFromShopRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut shop_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ShopId => {
                            if shop_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("shopId"));
                            }
                            shop_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(RemoveBannerImageFromShopRequest {
                    shop_id: shop_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sited_io.commerce.v1.RemoveBannerImageFromShopRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RemoveBannerImageFromShopResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("sited_io.commerce.v1.RemoveBannerImageFromShopResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RemoveBannerImageFromShopResponse {
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
            type Value = RemoveBannerImageFromShopResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.commerce.v1.RemoveBannerImageFromShopResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RemoveBannerImageFromShopResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(RemoveBannerImageFromShopResponse {
                })
            }
        }
        deserializer.deserialize_struct("sited_io.commerce.v1.RemoveBannerImageFromShopResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RemoveDomainFromShopRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.shop_id.is_empty() {
            len += 1;
        }
        if !self.domain.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sited_io.commerce.v1.RemoveDomainFromShopRequest", len)?;
        if !self.shop_id.is_empty() {
            struct_ser.serialize_field("shopId", &self.shop_id)?;
        }
        if !self.domain.is_empty() {
            struct_ser.serialize_field("domain", &self.domain)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RemoveDomainFromShopRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "shop_id",
            "shopId",
            "domain",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ShopId,
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
                            "shopId" | "shop_id" => Ok(GeneratedField::ShopId),
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
            type Value = RemoveDomainFromShopRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.commerce.v1.RemoveDomainFromShopRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RemoveDomainFromShopRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut shop_id__ = None;
                let mut domain__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ShopId => {
                            if shop_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("shopId"));
                            }
                            shop_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Domain => {
                            if domain__.is_some() {
                                return Err(serde::de::Error::duplicate_field("domain"));
                            }
                            domain__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(RemoveDomainFromShopRequest {
                    shop_id: shop_id__.unwrap_or_default(),
                    domain: domain__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sited_io.commerce.v1.RemoveDomainFromShopRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RemoveDomainFromShopResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("sited_io.commerce.v1.RemoveDomainFromShopResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RemoveDomainFromShopResponse {
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
            type Value = RemoveDomainFromShopResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.commerce.v1.RemoveDomainFromShopResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RemoveDomainFromShopResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(RemoveDomainFromShopResponse {
                })
            }
        }
        deserializer.deserialize_struct("sited_io.commerce.v1.RemoveDomainFromShopResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RemoveImageFromOfferRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.offer_image_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sited_io.commerce.v1.RemoveImageFromOfferRequest", len)?;
        if !self.offer_image_id.is_empty() {
            struct_ser.serialize_field("offerImageId", &self.offer_image_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RemoveImageFromOfferRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "offer_image_id",
            "offerImageId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            OfferImageId,
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
                            "offerImageId" | "offer_image_id" => Ok(GeneratedField::OfferImageId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RemoveImageFromOfferRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.commerce.v1.RemoveImageFromOfferRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RemoveImageFromOfferRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut offer_image_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::OfferImageId => {
                            if offer_image_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("offerImageId"));
                            }
                            offer_image_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(RemoveImageFromOfferRequest {
                    offer_image_id: offer_image_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sited_io.commerce.v1.RemoveImageFromOfferRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RemoveImageFromOfferResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("sited_io.commerce.v1.RemoveImageFromOfferResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RemoveImageFromOfferResponse {
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
            type Value = RemoveImageFromOfferResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.commerce.v1.RemoveImageFromOfferResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RemoveImageFromOfferResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(RemoveImageFromOfferResponse {
                })
            }
        }
        deserializer.deserialize_struct("sited_io.commerce.v1.RemoveImageFromOfferResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RemoveLogoImageFromShopRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.shop_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sited_io.commerce.v1.RemoveLogoImageFromShopRequest", len)?;
        if !self.shop_id.is_empty() {
            struct_ser.serialize_field("shopId", &self.shop_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RemoveLogoImageFromShopRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "shop_id",
            "shopId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ShopId,
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
                            "shopId" | "shop_id" => Ok(GeneratedField::ShopId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RemoveLogoImageFromShopRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.commerce.v1.RemoveLogoImageFromShopRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RemoveLogoImageFromShopRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut shop_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ShopId => {
                            if shop_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("shopId"));
                            }
                            shop_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(RemoveLogoImageFromShopRequest {
                    shop_id: shop_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sited_io.commerce.v1.RemoveLogoImageFromShopRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RemoveLogoImageFromShopResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("sited_io.commerce.v1.RemoveLogoImageFromShopResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RemoveLogoImageFromShopResponse {
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
            type Value = RemoveLogoImageFromShopResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.commerce.v1.RemoveLogoImageFromShopResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RemoveLogoImageFromShopResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(RemoveLogoImageFromShopResponse {
                })
            }
        }
        deserializer.deserialize_struct("sited_io.commerce.v1.RemoveLogoImageFromShopResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RemovePriceFromOfferRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.offer_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sited_io.commerce.v1.RemovePriceFromOfferRequest", len)?;
        if !self.offer_id.is_empty() {
            struct_ser.serialize_field("offerId", &self.offer_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RemovePriceFromOfferRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "offer_id",
            "offerId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            OfferId,
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
                            "offerId" | "offer_id" => Ok(GeneratedField::OfferId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RemovePriceFromOfferRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.commerce.v1.RemovePriceFromOfferRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RemovePriceFromOfferRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut offer_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::OfferId => {
                            if offer_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("offerId"));
                            }
                            offer_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(RemovePriceFromOfferRequest {
                    offer_id: offer_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sited_io.commerce.v1.RemovePriceFromOfferRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RemovePriceFromOfferResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("sited_io.commerce.v1.RemovePriceFromOfferResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RemovePriceFromOfferResponse {
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
            type Value = RemovePriceFromOfferResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.commerce.v1.RemovePriceFromOfferResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RemovePriceFromOfferResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(RemovePriceFromOfferResponse {
                })
            }
        }
        deserializer.deserialize_struct("sited_io.commerce.v1.RemovePriceFromOfferResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ShippingCountry {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "SHIPPING_COUNTRY_UNSPECIFIED",
            Self::Ac => "SHIPPING_COUNTRY_AC",
            Self::Ad => "SHIPPING_COUNTRY_AD",
            Self::Ae => "SHIPPING_COUNTRY_AE",
            Self::Af => "SHIPPING_COUNTRY_AF",
            Self::Ag => "SHIPPING_COUNTRY_AG",
            Self::Ai => "SHIPPING_COUNTRY_AI",
            Self::Al => "SHIPPING_COUNTRY_AL",
            Self::Am => "SHIPPING_COUNTRY_AM",
            Self::Ao => "SHIPPING_COUNTRY_AO",
            Self::Aq => "SHIPPING_COUNTRY_AQ",
            Self::Ar => "SHIPPING_COUNTRY_AR",
            Self::At => "SHIPPING_COUNTRY_AT",
            Self::Au => "SHIPPING_COUNTRY_AU",
            Self::Aw => "SHIPPING_COUNTRY_AW",
            Self::Ax => "SHIPPING_COUNTRY_AX",
            Self::Az => "SHIPPING_COUNTRY_AZ",
            Self::Ba => "SHIPPING_COUNTRY_BA",
            Self::Bb => "SHIPPING_COUNTRY_BB",
            Self::Bd => "SHIPPING_COUNTRY_BD",
            Self::Be => "SHIPPING_COUNTRY_BE",
            Self::Bf => "SHIPPING_COUNTRY_BF",
            Self::Bg => "SHIPPING_COUNTRY_BG",
            Self::Bh => "SHIPPING_COUNTRY_BH",
            Self::Bi => "SHIPPING_COUNTRY_BI",
            Self::Bj => "SHIPPING_COUNTRY_BJ",
            Self::Bl => "SHIPPING_COUNTRY_BL",
            Self::Bm => "SHIPPING_COUNTRY_BM",
            Self::Bn => "SHIPPING_COUNTRY_BN",
            Self::Bo => "SHIPPING_COUNTRY_BO",
            Self::Bq => "SHIPPING_COUNTRY_BQ",
            Self::Br => "SHIPPING_COUNTRY_BR",
            Self::Bs => "SHIPPING_COUNTRY_BS",
            Self::Bt => "SHIPPING_COUNTRY_BT",
            Self::Bv => "SHIPPING_COUNTRY_BV",
            Self::Bw => "SHIPPING_COUNTRY_BW",
            Self::By => "SHIPPING_COUNTRY_BY",
            Self::Bz => "SHIPPING_COUNTRY_BZ",
            Self::Ca => "SHIPPING_COUNTRY_CA",
            Self::Cd => "SHIPPING_COUNTRY_CD",
            Self::Cf => "SHIPPING_COUNTRY_CF",
            Self::Cg => "SHIPPING_COUNTRY_CG",
            Self::Ch => "SHIPPING_COUNTRY_CH",
            Self::Ci => "SHIPPING_COUNTRY_CI",
            Self::Ck => "SHIPPING_COUNTRY_CK",
            Self::Cl => "SHIPPING_COUNTRY_CL",
            Self::Cm => "SHIPPING_COUNTRY_CM",
            Self::Cn => "SHIPPING_COUNTRY_CN",
            Self::Co => "SHIPPING_COUNTRY_CO",
            Self::Cr => "SHIPPING_COUNTRY_CR",
            Self::Cv => "SHIPPING_COUNTRY_CV",
            Self::Cw => "SHIPPING_COUNTRY_CW",
            Self::Cy => "SHIPPING_COUNTRY_CY",
            Self::Cz => "SHIPPING_COUNTRY_CZ",
            Self::De => "SHIPPING_COUNTRY_DE",
            Self::Dj => "SHIPPING_COUNTRY_DJ",
            Self::Dk => "SHIPPING_COUNTRY_DK",
            Self::Dm => "SHIPPING_COUNTRY_DM",
            Self::Do => "SHIPPING_COUNTRY_DO",
            Self::Dz => "SHIPPING_COUNTRY_DZ",
            Self::Ec => "SHIPPING_COUNTRY_EC",
            Self::Ee => "SHIPPING_COUNTRY_EE",
            Self::Eg => "SHIPPING_COUNTRY_EG",
            Self::Eh => "SHIPPING_COUNTRY_EH",
            Self::Er => "SHIPPING_COUNTRY_ER",
            Self::Es => "SHIPPING_COUNTRY_ES",
            Self::Et => "SHIPPING_COUNTRY_ET",
            Self::Fi => "SHIPPING_COUNTRY_FI",
            Self::Fj => "SHIPPING_COUNTRY_FJ",
            Self::Fk => "SHIPPING_COUNTRY_FK",
            Self::Fo => "SHIPPING_COUNTRY_FO",
            Self::Fr => "SHIPPING_COUNTRY_FR",
            Self::Ga => "SHIPPING_COUNTRY_GA",
            Self::Gb => "SHIPPING_COUNTRY_GB",
            Self::Gd => "SHIPPING_COUNTRY_GD",
            Self::Ge => "SHIPPING_COUNTRY_GE",
            Self::Gf => "SHIPPING_COUNTRY_GF",
            Self::Gg => "SHIPPING_COUNTRY_GG",
            Self::Gh => "SHIPPING_COUNTRY_GH",
            Self::Gi => "SHIPPING_COUNTRY_GI",
            Self::Gl => "SHIPPING_COUNTRY_GL",
            Self::Gm => "SHIPPING_COUNTRY_GM",
            Self::Gn => "SHIPPING_COUNTRY_GN",
            Self::Gp => "SHIPPING_COUNTRY_GP",
            Self::Gq => "SHIPPING_COUNTRY_GQ",
            Self::Gr => "SHIPPING_COUNTRY_GR",
            Self::Gs => "SHIPPING_COUNTRY_GS",
            Self::Gt => "SHIPPING_COUNTRY_GT",
            Self::Gu => "SHIPPING_COUNTRY_GU",
            Self::Gw => "SHIPPING_COUNTRY_GW",
            Self::Gy => "SHIPPING_COUNTRY_GY",
            Self::Hk => "SHIPPING_COUNTRY_HK",
            Self::Hn => "SHIPPING_COUNTRY_HN",
            Self::Hr => "SHIPPING_COUNTRY_HR",
            Self::Ht => "SHIPPING_COUNTRY_HT",
            Self::Hu => "SHIPPING_COUNTRY_HU",
            Self::Id => "SHIPPING_COUNTRY_ID",
            Self::Ie => "SHIPPING_COUNTRY_IE",
            Self::Il => "SHIPPING_COUNTRY_IL",
            Self::Im => "SHIPPING_COUNTRY_IM",
            Self::In => "SHIPPING_COUNTRY_IN",
            Self::Io => "SHIPPING_COUNTRY_IO",
            Self::Iq => "SHIPPING_COUNTRY_IQ",
            Self::Is => "SHIPPING_COUNTRY_IS",
            Self::It => "SHIPPING_COUNTRY_IT",
            Self::Je => "SHIPPING_COUNTRY_JE",
            Self::Jm => "SHIPPING_COUNTRY_JM",
            Self::Jo => "SHIPPING_COUNTRY_JO",
            Self::Jp => "SHIPPING_COUNTRY_JP",
            Self::Ke => "SHIPPING_COUNTRY_KE",
            Self::Kg => "SHIPPING_COUNTRY_KG",
            Self::Kh => "SHIPPING_COUNTRY_KH",
            Self::Ki => "SHIPPING_COUNTRY_KI",
            Self::Km => "SHIPPING_COUNTRY_KM",
            Self::Kn => "SHIPPING_COUNTRY_KN",
            Self::Kr => "SHIPPING_COUNTRY_KR",
            Self::Kw => "SHIPPING_COUNTRY_KW",
            Self::Ky => "SHIPPING_COUNTRY_KY",
            Self::La => "SHIPPING_COUNTRY_LA",
            Self::Lb => "SHIPPING_COUNTRY_LB",
            Self::Lc => "SHIPPING_COUNTRY_LC",
            Self::Li => "SHIPPING_COUNTRY_LI",
            Self::Lk => "SHIPPING_COUNTRY_LK",
            Self::Lr => "SHIPPING_COUNTRY_LR",
            Self::Ls => "SHIPPING_COUNTRY_LS",
            Self::Lt => "SHIPPING_COUNTRY_LT",
            Self::Lu => "SHIPPING_COUNTRY_LU",
            Self::Lv => "SHIPPING_COUNTRY_LV",
            Self::Ly => "SHIPPING_COUNTRY_LY",
            Self::Ma => "SHIPPING_COUNTRY_MA",
            Self::Mc => "SHIPPING_COUNTRY_MC",
            Self::Md => "SHIPPING_COUNTRY_MD",
            Self::Me => "SHIPPING_COUNTRY_ME",
            Self::Mf => "SHIPPING_COUNTRY_MF",
            Self::Mg => "SHIPPING_COUNTRY_MG",
            Self::Mk => "SHIPPING_COUNTRY_MK",
            Self::Ml => "SHIPPING_COUNTRY_ML",
            Self::Mm => "SHIPPING_COUNTRY_MM",
            Self::Mn => "SHIPPING_COUNTRY_MN",
            Self::Mo => "SHIPPING_COUNTRY_MO",
            Self::Mq => "SHIPPING_COUNTRY_MQ",
            Self::Mr => "SHIPPING_COUNTRY_MR",
            Self::Ms => "SHIPPING_COUNTRY_MS",
            Self::Mt => "SHIPPING_COUNTRY_MT",
            Self::Mu => "SHIPPING_COUNTRY_MU",
            Self::Mv => "SHIPPING_COUNTRY_MV",
            Self::Mw => "SHIPPING_COUNTRY_MW",
            Self::Mx => "SHIPPING_COUNTRY_MX",
            Self::My => "SHIPPING_COUNTRY_MY",
            Self::Mz => "SHIPPING_COUNTRY_MZ",
            Self::Na => "SHIPPING_COUNTRY_NA",
            Self::Nc => "SHIPPING_COUNTRY_NC",
            Self::Ne => "SHIPPING_COUNTRY_NE",
            Self::Ng => "SHIPPING_COUNTRY_NG",
            Self::Ni => "SHIPPING_COUNTRY_NI",
            Self::Nl => "SHIPPING_COUNTRY_NL",
            Self::No => "SHIPPING_COUNTRY_NO",
            Self::Np => "SHIPPING_COUNTRY_NP",
            Self::Nr => "SHIPPING_COUNTRY_NR",
            Self::Nu => "SHIPPING_COUNTRY_NU",
            Self::Nz => "SHIPPING_COUNTRY_NZ",
            Self::Om => "SHIPPING_COUNTRY_OM",
            Self::Pa => "SHIPPING_COUNTRY_PA",
            Self::Pe => "SHIPPING_COUNTRY_PE",
            Self::Pf => "SHIPPING_COUNTRY_PF",
            Self::Pg => "SHIPPING_COUNTRY_PG",
            Self::Ph => "SHIPPING_COUNTRY_PH",
            Self::Pk => "SHIPPING_COUNTRY_PK",
            Self::Pl => "SHIPPING_COUNTRY_PL",
            Self::Pm => "SHIPPING_COUNTRY_PM",
            Self::Pn => "SHIPPING_COUNTRY_PN",
            Self::Pr => "SHIPPING_COUNTRY_PR",
            Self::Ps => "SHIPPING_COUNTRY_PS",
            Self::Pt => "SHIPPING_COUNTRY_PT",
            Self::Py => "SHIPPING_COUNTRY_PY",
            Self::Qa => "SHIPPING_COUNTRY_QA",
            Self::Re => "SHIPPING_COUNTRY_RE",
            Self::Ro => "SHIPPING_COUNTRY_RO",
            Self::Rs => "SHIPPING_COUNTRY_RS",
            Self::Ru => "SHIPPING_COUNTRY_RU",
            Self::Rw => "SHIPPING_COUNTRY_RW",
            Self::Sa => "SHIPPING_COUNTRY_SA",
            Self::Sb => "SHIPPING_COUNTRY_SB",
            Self::Sc => "SHIPPING_COUNTRY_SC",
            Self::Se => "SHIPPING_COUNTRY_SE",
            Self::Sg => "SHIPPING_COUNTRY_SG",
            Self::Sh => "SHIPPING_COUNTRY_SH",
            Self::Si => "SHIPPING_COUNTRY_SI",
            Self::Sj => "SHIPPING_COUNTRY_SJ",
            Self::Sk => "SHIPPING_COUNTRY_SK",
            Self::Sl => "SHIPPING_COUNTRY_SL",
            Self::Sm => "SHIPPING_COUNTRY_SM",
            Self::Sn => "SHIPPING_COUNTRY_SN",
            Self::So => "SHIPPING_COUNTRY_SO",
            Self::Sr => "SHIPPING_COUNTRY_SR",
            Self::Ss => "SHIPPING_COUNTRY_SS",
            Self::St => "SHIPPING_COUNTRY_ST",
            Self::Sv => "SHIPPING_COUNTRY_SV",
            Self::Sx => "SHIPPING_COUNTRY_SX",
            Self::Sz => "SHIPPING_COUNTRY_SZ",
            Self::Ta => "SHIPPING_COUNTRY_TA",
            Self::Tc => "SHIPPING_COUNTRY_TC",
            Self::Td => "SHIPPING_COUNTRY_TD",
            Self::Tf => "SHIPPING_COUNTRY_TF",
            Self::Tg => "SHIPPING_COUNTRY_TG",
            Self::Th => "SHIPPING_COUNTRY_TH",
            Self::Tj => "SHIPPING_COUNTRY_TJ",
            Self::Tk => "SHIPPING_COUNTRY_TK",
            Self::Tl => "SHIPPING_COUNTRY_TL",
            Self::Tm => "SHIPPING_COUNTRY_TM",
            Self::Tn => "SHIPPING_COUNTRY_TN",
            Self::To => "SHIPPING_COUNTRY_TO",
            Self::Tr => "SHIPPING_COUNTRY_TR",
            Self::Tt => "SHIPPING_COUNTRY_TT",
            Self::Tv => "SHIPPING_COUNTRY_TV",
            Self::Tw => "SHIPPING_COUNTRY_TW",
            Self::Tz => "SHIPPING_COUNTRY_TZ",
            Self::Ua => "SHIPPING_COUNTRY_UA",
            Self::Ug => "SHIPPING_COUNTRY_UG",
            Self::Us => "SHIPPING_COUNTRY_US",
            Self::Uy => "SHIPPING_COUNTRY_UY",
            Self::Uz => "SHIPPING_COUNTRY_UZ",
            Self::Va => "SHIPPING_COUNTRY_VA",
            Self::Vc => "SHIPPING_COUNTRY_VC",
            Self::Ve => "SHIPPING_COUNTRY_VE",
            Self::Vg => "SHIPPING_COUNTRY_VG",
            Self::Vn => "SHIPPING_COUNTRY_VN",
            Self::Vu => "SHIPPING_COUNTRY_VU",
            Self::Wf => "SHIPPING_COUNTRY_WF",
            Self::Ws => "SHIPPING_COUNTRY_WS",
            Self::Xk => "SHIPPING_COUNTRY_XK",
            Self::Ye => "SHIPPING_COUNTRY_YE",
            Self::Yt => "SHIPPING_COUNTRY_YT",
            Self::Za => "SHIPPING_COUNTRY_ZA",
            Self::Zm => "SHIPPING_COUNTRY_ZM",
            Self::Zw => "SHIPPING_COUNTRY_ZW",
            Self::Zz => "SHIPPING_COUNTRY_ZZ",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for ShippingCountry {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "SHIPPING_COUNTRY_UNSPECIFIED",
            "SHIPPING_COUNTRY_AC",
            "SHIPPING_COUNTRY_AD",
            "SHIPPING_COUNTRY_AE",
            "SHIPPING_COUNTRY_AF",
            "SHIPPING_COUNTRY_AG",
            "SHIPPING_COUNTRY_AI",
            "SHIPPING_COUNTRY_AL",
            "SHIPPING_COUNTRY_AM",
            "SHIPPING_COUNTRY_AO",
            "SHIPPING_COUNTRY_AQ",
            "SHIPPING_COUNTRY_AR",
            "SHIPPING_COUNTRY_AT",
            "SHIPPING_COUNTRY_AU",
            "SHIPPING_COUNTRY_AW",
            "SHIPPING_COUNTRY_AX",
            "SHIPPING_COUNTRY_AZ",
            "SHIPPING_COUNTRY_BA",
            "SHIPPING_COUNTRY_BB",
            "SHIPPING_COUNTRY_BD",
            "SHIPPING_COUNTRY_BE",
            "SHIPPING_COUNTRY_BF",
            "SHIPPING_COUNTRY_BG",
            "SHIPPING_COUNTRY_BH",
            "SHIPPING_COUNTRY_BI",
            "SHIPPING_COUNTRY_BJ",
            "SHIPPING_COUNTRY_BL",
            "SHIPPING_COUNTRY_BM",
            "SHIPPING_COUNTRY_BN",
            "SHIPPING_COUNTRY_BO",
            "SHIPPING_COUNTRY_BQ",
            "SHIPPING_COUNTRY_BR",
            "SHIPPING_COUNTRY_BS",
            "SHIPPING_COUNTRY_BT",
            "SHIPPING_COUNTRY_BV",
            "SHIPPING_COUNTRY_BW",
            "SHIPPING_COUNTRY_BY",
            "SHIPPING_COUNTRY_BZ",
            "SHIPPING_COUNTRY_CA",
            "SHIPPING_COUNTRY_CD",
            "SHIPPING_COUNTRY_CF",
            "SHIPPING_COUNTRY_CG",
            "SHIPPING_COUNTRY_CH",
            "SHIPPING_COUNTRY_CI",
            "SHIPPING_COUNTRY_CK",
            "SHIPPING_COUNTRY_CL",
            "SHIPPING_COUNTRY_CM",
            "SHIPPING_COUNTRY_CN",
            "SHIPPING_COUNTRY_CO",
            "SHIPPING_COUNTRY_CR",
            "SHIPPING_COUNTRY_CV",
            "SHIPPING_COUNTRY_CW",
            "SHIPPING_COUNTRY_CY",
            "SHIPPING_COUNTRY_CZ",
            "SHIPPING_COUNTRY_DE",
            "SHIPPING_COUNTRY_DJ",
            "SHIPPING_COUNTRY_DK",
            "SHIPPING_COUNTRY_DM",
            "SHIPPING_COUNTRY_DO",
            "SHIPPING_COUNTRY_DZ",
            "SHIPPING_COUNTRY_EC",
            "SHIPPING_COUNTRY_EE",
            "SHIPPING_COUNTRY_EG",
            "SHIPPING_COUNTRY_EH",
            "SHIPPING_COUNTRY_ER",
            "SHIPPING_COUNTRY_ES",
            "SHIPPING_COUNTRY_ET",
            "SHIPPING_COUNTRY_FI",
            "SHIPPING_COUNTRY_FJ",
            "SHIPPING_COUNTRY_FK",
            "SHIPPING_COUNTRY_FO",
            "SHIPPING_COUNTRY_FR",
            "SHIPPING_COUNTRY_GA",
            "SHIPPING_COUNTRY_GB",
            "SHIPPING_COUNTRY_GD",
            "SHIPPING_COUNTRY_GE",
            "SHIPPING_COUNTRY_GF",
            "SHIPPING_COUNTRY_GG",
            "SHIPPING_COUNTRY_GH",
            "SHIPPING_COUNTRY_GI",
            "SHIPPING_COUNTRY_GL",
            "SHIPPING_COUNTRY_GM",
            "SHIPPING_COUNTRY_GN",
            "SHIPPING_COUNTRY_GP",
            "SHIPPING_COUNTRY_GQ",
            "SHIPPING_COUNTRY_GR",
            "SHIPPING_COUNTRY_GS",
            "SHIPPING_COUNTRY_GT",
            "SHIPPING_COUNTRY_GU",
            "SHIPPING_COUNTRY_GW",
            "SHIPPING_COUNTRY_GY",
            "SHIPPING_COUNTRY_HK",
            "SHIPPING_COUNTRY_HN",
            "SHIPPING_COUNTRY_HR",
            "SHIPPING_COUNTRY_HT",
            "SHIPPING_COUNTRY_HU",
            "SHIPPING_COUNTRY_ID",
            "SHIPPING_COUNTRY_IE",
            "SHIPPING_COUNTRY_IL",
            "SHIPPING_COUNTRY_IM",
            "SHIPPING_COUNTRY_IN",
            "SHIPPING_COUNTRY_IO",
            "SHIPPING_COUNTRY_IQ",
            "SHIPPING_COUNTRY_IS",
            "SHIPPING_COUNTRY_IT",
            "SHIPPING_COUNTRY_JE",
            "SHIPPING_COUNTRY_JM",
            "SHIPPING_COUNTRY_JO",
            "SHIPPING_COUNTRY_JP",
            "SHIPPING_COUNTRY_KE",
            "SHIPPING_COUNTRY_KG",
            "SHIPPING_COUNTRY_KH",
            "SHIPPING_COUNTRY_KI",
            "SHIPPING_COUNTRY_KM",
            "SHIPPING_COUNTRY_KN",
            "SHIPPING_COUNTRY_KR",
            "SHIPPING_COUNTRY_KW",
            "SHIPPING_COUNTRY_KY",
            "SHIPPING_COUNTRY_LA",
            "SHIPPING_COUNTRY_LB",
            "SHIPPING_COUNTRY_LC",
            "SHIPPING_COUNTRY_LI",
            "SHIPPING_COUNTRY_LK",
            "SHIPPING_COUNTRY_LR",
            "SHIPPING_COUNTRY_LS",
            "SHIPPING_COUNTRY_LT",
            "SHIPPING_COUNTRY_LU",
            "SHIPPING_COUNTRY_LV",
            "SHIPPING_COUNTRY_LY",
            "SHIPPING_COUNTRY_MA",
            "SHIPPING_COUNTRY_MC",
            "SHIPPING_COUNTRY_MD",
            "SHIPPING_COUNTRY_ME",
            "SHIPPING_COUNTRY_MF",
            "SHIPPING_COUNTRY_MG",
            "SHIPPING_COUNTRY_MK",
            "SHIPPING_COUNTRY_ML",
            "SHIPPING_COUNTRY_MM",
            "SHIPPING_COUNTRY_MN",
            "SHIPPING_COUNTRY_MO",
            "SHIPPING_COUNTRY_MQ",
            "SHIPPING_COUNTRY_MR",
            "SHIPPING_COUNTRY_MS",
            "SHIPPING_COUNTRY_MT",
            "SHIPPING_COUNTRY_MU",
            "SHIPPING_COUNTRY_MV",
            "SHIPPING_COUNTRY_MW",
            "SHIPPING_COUNTRY_MX",
            "SHIPPING_COUNTRY_MY",
            "SHIPPING_COUNTRY_MZ",
            "SHIPPING_COUNTRY_NA",
            "SHIPPING_COUNTRY_NC",
            "SHIPPING_COUNTRY_NE",
            "SHIPPING_COUNTRY_NG",
            "SHIPPING_COUNTRY_NI",
            "SHIPPING_COUNTRY_NL",
            "SHIPPING_COUNTRY_NO",
            "SHIPPING_COUNTRY_NP",
            "SHIPPING_COUNTRY_NR",
            "SHIPPING_COUNTRY_NU",
            "SHIPPING_COUNTRY_NZ",
            "SHIPPING_COUNTRY_OM",
            "SHIPPING_COUNTRY_PA",
            "SHIPPING_COUNTRY_PE",
            "SHIPPING_COUNTRY_PF",
            "SHIPPING_COUNTRY_PG",
            "SHIPPING_COUNTRY_PH",
            "SHIPPING_COUNTRY_PK",
            "SHIPPING_COUNTRY_PL",
            "SHIPPING_COUNTRY_PM",
            "SHIPPING_COUNTRY_PN",
            "SHIPPING_COUNTRY_PR",
            "SHIPPING_COUNTRY_PS",
            "SHIPPING_COUNTRY_PT",
            "SHIPPING_COUNTRY_PY",
            "SHIPPING_COUNTRY_QA",
            "SHIPPING_COUNTRY_RE",
            "SHIPPING_COUNTRY_RO",
            "SHIPPING_COUNTRY_RS",
            "SHIPPING_COUNTRY_RU",
            "SHIPPING_COUNTRY_RW",
            "SHIPPING_COUNTRY_SA",
            "SHIPPING_COUNTRY_SB",
            "SHIPPING_COUNTRY_SC",
            "SHIPPING_COUNTRY_SE",
            "SHIPPING_COUNTRY_SG",
            "SHIPPING_COUNTRY_SH",
            "SHIPPING_COUNTRY_SI",
            "SHIPPING_COUNTRY_SJ",
            "SHIPPING_COUNTRY_SK",
            "SHIPPING_COUNTRY_SL",
            "SHIPPING_COUNTRY_SM",
            "SHIPPING_COUNTRY_SN",
            "SHIPPING_COUNTRY_SO",
            "SHIPPING_COUNTRY_SR",
            "SHIPPING_COUNTRY_SS",
            "SHIPPING_COUNTRY_ST",
            "SHIPPING_COUNTRY_SV",
            "SHIPPING_COUNTRY_SX",
            "SHIPPING_COUNTRY_SZ",
            "SHIPPING_COUNTRY_TA",
            "SHIPPING_COUNTRY_TC",
            "SHIPPING_COUNTRY_TD",
            "SHIPPING_COUNTRY_TF",
            "SHIPPING_COUNTRY_TG",
            "SHIPPING_COUNTRY_TH",
            "SHIPPING_COUNTRY_TJ",
            "SHIPPING_COUNTRY_TK",
            "SHIPPING_COUNTRY_TL",
            "SHIPPING_COUNTRY_TM",
            "SHIPPING_COUNTRY_TN",
            "SHIPPING_COUNTRY_TO",
            "SHIPPING_COUNTRY_TR",
            "SHIPPING_COUNTRY_TT",
            "SHIPPING_COUNTRY_TV",
            "SHIPPING_COUNTRY_TW",
            "SHIPPING_COUNTRY_TZ",
            "SHIPPING_COUNTRY_UA",
            "SHIPPING_COUNTRY_UG",
            "SHIPPING_COUNTRY_US",
            "SHIPPING_COUNTRY_UY",
            "SHIPPING_COUNTRY_UZ",
            "SHIPPING_COUNTRY_VA",
            "SHIPPING_COUNTRY_VC",
            "SHIPPING_COUNTRY_VE",
            "SHIPPING_COUNTRY_VG",
            "SHIPPING_COUNTRY_VN",
            "SHIPPING_COUNTRY_VU",
            "SHIPPING_COUNTRY_WF",
            "SHIPPING_COUNTRY_WS",
            "SHIPPING_COUNTRY_XK",
            "SHIPPING_COUNTRY_YE",
            "SHIPPING_COUNTRY_YT",
            "SHIPPING_COUNTRY_ZA",
            "SHIPPING_COUNTRY_ZM",
            "SHIPPING_COUNTRY_ZW",
            "SHIPPING_COUNTRY_ZZ",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ShippingCountry;

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
                    "SHIPPING_COUNTRY_UNSPECIFIED" => Ok(ShippingCountry::Unspecified),
                    "SHIPPING_COUNTRY_AC" => Ok(ShippingCountry::Ac),
                    "SHIPPING_COUNTRY_AD" => Ok(ShippingCountry::Ad),
                    "SHIPPING_COUNTRY_AE" => Ok(ShippingCountry::Ae),
                    "SHIPPING_COUNTRY_AF" => Ok(ShippingCountry::Af),
                    "SHIPPING_COUNTRY_AG" => Ok(ShippingCountry::Ag),
                    "SHIPPING_COUNTRY_AI" => Ok(ShippingCountry::Ai),
                    "SHIPPING_COUNTRY_AL" => Ok(ShippingCountry::Al),
                    "SHIPPING_COUNTRY_AM" => Ok(ShippingCountry::Am),
                    "SHIPPING_COUNTRY_AO" => Ok(ShippingCountry::Ao),
                    "SHIPPING_COUNTRY_AQ" => Ok(ShippingCountry::Aq),
                    "SHIPPING_COUNTRY_AR" => Ok(ShippingCountry::Ar),
                    "SHIPPING_COUNTRY_AT" => Ok(ShippingCountry::At),
                    "SHIPPING_COUNTRY_AU" => Ok(ShippingCountry::Au),
                    "SHIPPING_COUNTRY_AW" => Ok(ShippingCountry::Aw),
                    "SHIPPING_COUNTRY_AX" => Ok(ShippingCountry::Ax),
                    "SHIPPING_COUNTRY_AZ" => Ok(ShippingCountry::Az),
                    "SHIPPING_COUNTRY_BA" => Ok(ShippingCountry::Ba),
                    "SHIPPING_COUNTRY_BB" => Ok(ShippingCountry::Bb),
                    "SHIPPING_COUNTRY_BD" => Ok(ShippingCountry::Bd),
                    "SHIPPING_COUNTRY_BE" => Ok(ShippingCountry::Be),
                    "SHIPPING_COUNTRY_BF" => Ok(ShippingCountry::Bf),
                    "SHIPPING_COUNTRY_BG" => Ok(ShippingCountry::Bg),
                    "SHIPPING_COUNTRY_BH" => Ok(ShippingCountry::Bh),
                    "SHIPPING_COUNTRY_BI" => Ok(ShippingCountry::Bi),
                    "SHIPPING_COUNTRY_BJ" => Ok(ShippingCountry::Bj),
                    "SHIPPING_COUNTRY_BL" => Ok(ShippingCountry::Bl),
                    "SHIPPING_COUNTRY_BM" => Ok(ShippingCountry::Bm),
                    "SHIPPING_COUNTRY_BN" => Ok(ShippingCountry::Bn),
                    "SHIPPING_COUNTRY_BO" => Ok(ShippingCountry::Bo),
                    "SHIPPING_COUNTRY_BQ" => Ok(ShippingCountry::Bq),
                    "SHIPPING_COUNTRY_BR" => Ok(ShippingCountry::Br),
                    "SHIPPING_COUNTRY_BS" => Ok(ShippingCountry::Bs),
                    "SHIPPING_COUNTRY_BT" => Ok(ShippingCountry::Bt),
                    "SHIPPING_COUNTRY_BV" => Ok(ShippingCountry::Bv),
                    "SHIPPING_COUNTRY_BW" => Ok(ShippingCountry::Bw),
                    "SHIPPING_COUNTRY_BY" => Ok(ShippingCountry::By),
                    "SHIPPING_COUNTRY_BZ" => Ok(ShippingCountry::Bz),
                    "SHIPPING_COUNTRY_CA" => Ok(ShippingCountry::Ca),
                    "SHIPPING_COUNTRY_CD" => Ok(ShippingCountry::Cd),
                    "SHIPPING_COUNTRY_CF" => Ok(ShippingCountry::Cf),
                    "SHIPPING_COUNTRY_CG" => Ok(ShippingCountry::Cg),
                    "SHIPPING_COUNTRY_CH" => Ok(ShippingCountry::Ch),
                    "SHIPPING_COUNTRY_CI" => Ok(ShippingCountry::Ci),
                    "SHIPPING_COUNTRY_CK" => Ok(ShippingCountry::Ck),
                    "SHIPPING_COUNTRY_CL" => Ok(ShippingCountry::Cl),
                    "SHIPPING_COUNTRY_CM" => Ok(ShippingCountry::Cm),
                    "SHIPPING_COUNTRY_CN" => Ok(ShippingCountry::Cn),
                    "SHIPPING_COUNTRY_CO" => Ok(ShippingCountry::Co),
                    "SHIPPING_COUNTRY_CR" => Ok(ShippingCountry::Cr),
                    "SHIPPING_COUNTRY_CV" => Ok(ShippingCountry::Cv),
                    "SHIPPING_COUNTRY_CW" => Ok(ShippingCountry::Cw),
                    "SHIPPING_COUNTRY_CY" => Ok(ShippingCountry::Cy),
                    "SHIPPING_COUNTRY_CZ" => Ok(ShippingCountry::Cz),
                    "SHIPPING_COUNTRY_DE" => Ok(ShippingCountry::De),
                    "SHIPPING_COUNTRY_DJ" => Ok(ShippingCountry::Dj),
                    "SHIPPING_COUNTRY_DK" => Ok(ShippingCountry::Dk),
                    "SHIPPING_COUNTRY_DM" => Ok(ShippingCountry::Dm),
                    "SHIPPING_COUNTRY_DO" => Ok(ShippingCountry::Do),
                    "SHIPPING_COUNTRY_DZ" => Ok(ShippingCountry::Dz),
                    "SHIPPING_COUNTRY_EC" => Ok(ShippingCountry::Ec),
                    "SHIPPING_COUNTRY_EE" => Ok(ShippingCountry::Ee),
                    "SHIPPING_COUNTRY_EG" => Ok(ShippingCountry::Eg),
                    "SHIPPING_COUNTRY_EH" => Ok(ShippingCountry::Eh),
                    "SHIPPING_COUNTRY_ER" => Ok(ShippingCountry::Er),
                    "SHIPPING_COUNTRY_ES" => Ok(ShippingCountry::Es),
                    "SHIPPING_COUNTRY_ET" => Ok(ShippingCountry::Et),
                    "SHIPPING_COUNTRY_FI" => Ok(ShippingCountry::Fi),
                    "SHIPPING_COUNTRY_FJ" => Ok(ShippingCountry::Fj),
                    "SHIPPING_COUNTRY_FK" => Ok(ShippingCountry::Fk),
                    "SHIPPING_COUNTRY_FO" => Ok(ShippingCountry::Fo),
                    "SHIPPING_COUNTRY_FR" => Ok(ShippingCountry::Fr),
                    "SHIPPING_COUNTRY_GA" => Ok(ShippingCountry::Ga),
                    "SHIPPING_COUNTRY_GB" => Ok(ShippingCountry::Gb),
                    "SHIPPING_COUNTRY_GD" => Ok(ShippingCountry::Gd),
                    "SHIPPING_COUNTRY_GE" => Ok(ShippingCountry::Ge),
                    "SHIPPING_COUNTRY_GF" => Ok(ShippingCountry::Gf),
                    "SHIPPING_COUNTRY_GG" => Ok(ShippingCountry::Gg),
                    "SHIPPING_COUNTRY_GH" => Ok(ShippingCountry::Gh),
                    "SHIPPING_COUNTRY_GI" => Ok(ShippingCountry::Gi),
                    "SHIPPING_COUNTRY_GL" => Ok(ShippingCountry::Gl),
                    "SHIPPING_COUNTRY_GM" => Ok(ShippingCountry::Gm),
                    "SHIPPING_COUNTRY_GN" => Ok(ShippingCountry::Gn),
                    "SHIPPING_COUNTRY_GP" => Ok(ShippingCountry::Gp),
                    "SHIPPING_COUNTRY_GQ" => Ok(ShippingCountry::Gq),
                    "SHIPPING_COUNTRY_GR" => Ok(ShippingCountry::Gr),
                    "SHIPPING_COUNTRY_GS" => Ok(ShippingCountry::Gs),
                    "SHIPPING_COUNTRY_GT" => Ok(ShippingCountry::Gt),
                    "SHIPPING_COUNTRY_GU" => Ok(ShippingCountry::Gu),
                    "SHIPPING_COUNTRY_GW" => Ok(ShippingCountry::Gw),
                    "SHIPPING_COUNTRY_GY" => Ok(ShippingCountry::Gy),
                    "SHIPPING_COUNTRY_HK" => Ok(ShippingCountry::Hk),
                    "SHIPPING_COUNTRY_HN" => Ok(ShippingCountry::Hn),
                    "SHIPPING_COUNTRY_HR" => Ok(ShippingCountry::Hr),
                    "SHIPPING_COUNTRY_HT" => Ok(ShippingCountry::Ht),
                    "SHIPPING_COUNTRY_HU" => Ok(ShippingCountry::Hu),
                    "SHIPPING_COUNTRY_ID" => Ok(ShippingCountry::Id),
                    "SHIPPING_COUNTRY_IE" => Ok(ShippingCountry::Ie),
                    "SHIPPING_COUNTRY_IL" => Ok(ShippingCountry::Il),
                    "SHIPPING_COUNTRY_IM" => Ok(ShippingCountry::Im),
                    "SHIPPING_COUNTRY_IN" => Ok(ShippingCountry::In),
                    "SHIPPING_COUNTRY_IO" => Ok(ShippingCountry::Io),
                    "SHIPPING_COUNTRY_IQ" => Ok(ShippingCountry::Iq),
                    "SHIPPING_COUNTRY_IS" => Ok(ShippingCountry::Is),
                    "SHIPPING_COUNTRY_IT" => Ok(ShippingCountry::It),
                    "SHIPPING_COUNTRY_JE" => Ok(ShippingCountry::Je),
                    "SHIPPING_COUNTRY_JM" => Ok(ShippingCountry::Jm),
                    "SHIPPING_COUNTRY_JO" => Ok(ShippingCountry::Jo),
                    "SHIPPING_COUNTRY_JP" => Ok(ShippingCountry::Jp),
                    "SHIPPING_COUNTRY_KE" => Ok(ShippingCountry::Ke),
                    "SHIPPING_COUNTRY_KG" => Ok(ShippingCountry::Kg),
                    "SHIPPING_COUNTRY_KH" => Ok(ShippingCountry::Kh),
                    "SHIPPING_COUNTRY_KI" => Ok(ShippingCountry::Ki),
                    "SHIPPING_COUNTRY_KM" => Ok(ShippingCountry::Km),
                    "SHIPPING_COUNTRY_KN" => Ok(ShippingCountry::Kn),
                    "SHIPPING_COUNTRY_KR" => Ok(ShippingCountry::Kr),
                    "SHIPPING_COUNTRY_KW" => Ok(ShippingCountry::Kw),
                    "SHIPPING_COUNTRY_KY" => Ok(ShippingCountry::Ky),
                    "SHIPPING_COUNTRY_LA" => Ok(ShippingCountry::La),
                    "SHIPPING_COUNTRY_LB" => Ok(ShippingCountry::Lb),
                    "SHIPPING_COUNTRY_LC" => Ok(ShippingCountry::Lc),
                    "SHIPPING_COUNTRY_LI" => Ok(ShippingCountry::Li),
                    "SHIPPING_COUNTRY_LK" => Ok(ShippingCountry::Lk),
                    "SHIPPING_COUNTRY_LR" => Ok(ShippingCountry::Lr),
                    "SHIPPING_COUNTRY_LS" => Ok(ShippingCountry::Ls),
                    "SHIPPING_COUNTRY_LT" => Ok(ShippingCountry::Lt),
                    "SHIPPING_COUNTRY_LU" => Ok(ShippingCountry::Lu),
                    "SHIPPING_COUNTRY_LV" => Ok(ShippingCountry::Lv),
                    "SHIPPING_COUNTRY_LY" => Ok(ShippingCountry::Ly),
                    "SHIPPING_COUNTRY_MA" => Ok(ShippingCountry::Ma),
                    "SHIPPING_COUNTRY_MC" => Ok(ShippingCountry::Mc),
                    "SHIPPING_COUNTRY_MD" => Ok(ShippingCountry::Md),
                    "SHIPPING_COUNTRY_ME" => Ok(ShippingCountry::Me),
                    "SHIPPING_COUNTRY_MF" => Ok(ShippingCountry::Mf),
                    "SHIPPING_COUNTRY_MG" => Ok(ShippingCountry::Mg),
                    "SHIPPING_COUNTRY_MK" => Ok(ShippingCountry::Mk),
                    "SHIPPING_COUNTRY_ML" => Ok(ShippingCountry::Ml),
                    "SHIPPING_COUNTRY_MM" => Ok(ShippingCountry::Mm),
                    "SHIPPING_COUNTRY_MN" => Ok(ShippingCountry::Mn),
                    "SHIPPING_COUNTRY_MO" => Ok(ShippingCountry::Mo),
                    "SHIPPING_COUNTRY_MQ" => Ok(ShippingCountry::Mq),
                    "SHIPPING_COUNTRY_MR" => Ok(ShippingCountry::Mr),
                    "SHIPPING_COUNTRY_MS" => Ok(ShippingCountry::Ms),
                    "SHIPPING_COUNTRY_MT" => Ok(ShippingCountry::Mt),
                    "SHIPPING_COUNTRY_MU" => Ok(ShippingCountry::Mu),
                    "SHIPPING_COUNTRY_MV" => Ok(ShippingCountry::Mv),
                    "SHIPPING_COUNTRY_MW" => Ok(ShippingCountry::Mw),
                    "SHIPPING_COUNTRY_MX" => Ok(ShippingCountry::Mx),
                    "SHIPPING_COUNTRY_MY" => Ok(ShippingCountry::My),
                    "SHIPPING_COUNTRY_MZ" => Ok(ShippingCountry::Mz),
                    "SHIPPING_COUNTRY_NA" => Ok(ShippingCountry::Na),
                    "SHIPPING_COUNTRY_NC" => Ok(ShippingCountry::Nc),
                    "SHIPPING_COUNTRY_NE" => Ok(ShippingCountry::Ne),
                    "SHIPPING_COUNTRY_NG" => Ok(ShippingCountry::Ng),
                    "SHIPPING_COUNTRY_NI" => Ok(ShippingCountry::Ni),
                    "SHIPPING_COUNTRY_NL" => Ok(ShippingCountry::Nl),
                    "SHIPPING_COUNTRY_NO" => Ok(ShippingCountry::No),
                    "SHIPPING_COUNTRY_NP" => Ok(ShippingCountry::Np),
                    "SHIPPING_COUNTRY_NR" => Ok(ShippingCountry::Nr),
                    "SHIPPING_COUNTRY_NU" => Ok(ShippingCountry::Nu),
                    "SHIPPING_COUNTRY_NZ" => Ok(ShippingCountry::Nz),
                    "SHIPPING_COUNTRY_OM" => Ok(ShippingCountry::Om),
                    "SHIPPING_COUNTRY_PA" => Ok(ShippingCountry::Pa),
                    "SHIPPING_COUNTRY_PE" => Ok(ShippingCountry::Pe),
                    "SHIPPING_COUNTRY_PF" => Ok(ShippingCountry::Pf),
                    "SHIPPING_COUNTRY_PG" => Ok(ShippingCountry::Pg),
                    "SHIPPING_COUNTRY_PH" => Ok(ShippingCountry::Ph),
                    "SHIPPING_COUNTRY_PK" => Ok(ShippingCountry::Pk),
                    "SHIPPING_COUNTRY_PL" => Ok(ShippingCountry::Pl),
                    "SHIPPING_COUNTRY_PM" => Ok(ShippingCountry::Pm),
                    "SHIPPING_COUNTRY_PN" => Ok(ShippingCountry::Pn),
                    "SHIPPING_COUNTRY_PR" => Ok(ShippingCountry::Pr),
                    "SHIPPING_COUNTRY_PS" => Ok(ShippingCountry::Ps),
                    "SHIPPING_COUNTRY_PT" => Ok(ShippingCountry::Pt),
                    "SHIPPING_COUNTRY_PY" => Ok(ShippingCountry::Py),
                    "SHIPPING_COUNTRY_QA" => Ok(ShippingCountry::Qa),
                    "SHIPPING_COUNTRY_RE" => Ok(ShippingCountry::Re),
                    "SHIPPING_COUNTRY_RO" => Ok(ShippingCountry::Ro),
                    "SHIPPING_COUNTRY_RS" => Ok(ShippingCountry::Rs),
                    "SHIPPING_COUNTRY_RU" => Ok(ShippingCountry::Ru),
                    "SHIPPING_COUNTRY_RW" => Ok(ShippingCountry::Rw),
                    "SHIPPING_COUNTRY_SA" => Ok(ShippingCountry::Sa),
                    "SHIPPING_COUNTRY_SB" => Ok(ShippingCountry::Sb),
                    "SHIPPING_COUNTRY_SC" => Ok(ShippingCountry::Sc),
                    "SHIPPING_COUNTRY_SE" => Ok(ShippingCountry::Se),
                    "SHIPPING_COUNTRY_SG" => Ok(ShippingCountry::Sg),
                    "SHIPPING_COUNTRY_SH" => Ok(ShippingCountry::Sh),
                    "SHIPPING_COUNTRY_SI" => Ok(ShippingCountry::Si),
                    "SHIPPING_COUNTRY_SJ" => Ok(ShippingCountry::Sj),
                    "SHIPPING_COUNTRY_SK" => Ok(ShippingCountry::Sk),
                    "SHIPPING_COUNTRY_SL" => Ok(ShippingCountry::Sl),
                    "SHIPPING_COUNTRY_SM" => Ok(ShippingCountry::Sm),
                    "SHIPPING_COUNTRY_SN" => Ok(ShippingCountry::Sn),
                    "SHIPPING_COUNTRY_SO" => Ok(ShippingCountry::So),
                    "SHIPPING_COUNTRY_SR" => Ok(ShippingCountry::Sr),
                    "SHIPPING_COUNTRY_SS" => Ok(ShippingCountry::Ss),
                    "SHIPPING_COUNTRY_ST" => Ok(ShippingCountry::St),
                    "SHIPPING_COUNTRY_SV" => Ok(ShippingCountry::Sv),
                    "SHIPPING_COUNTRY_SX" => Ok(ShippingCountry::Sx),
                    "SHIPPING_COUNTRY_SZ" => Ok(ShippingCountry::Sz),
                    "SHIPPING_COUNTRY_TA" => Ok(ShippingCountry::Ta),
                    "SHIPPING_COUNTRY_TC" => Ok(ShippingCountry::Tc),
                    "SHIPPING_COUNTRY_TD" => Ok(ShippingCountry::Td),
                    "SHIPPING_COUNTRY_TF" => Ok(ShippingCountry::Tf),
                    "SHIPPING_COUNTRY_TG" => Ok(ShippingCountry::Tg),
                    "SHIPPING_COUNTRY_TH" => Ok(ShippingCountry::Th),
                    "SHIPPING_COUNTRY_TJ" => Ok(ShippingCountry::Tj),
                    "SHIPPING_COUNTRY_TK" => Ok(ShippingCountry::Tk),
                    "SHIPPING_COUNTRY_TL" => Ok(ShippingCountry::Tl),
                    "SHIPPING_COUNTRY_TM" => Ok(ShippingCountry::Tm),
                    "SHIPPING_COUNTRY_TN" => Ok(ShippingCountry::Tn),
                    "SHIPPING_COUNTRY_TO" => Ok(ShippingCountry::To),
                    "SHIPPING_COUNTRY_TR" => Ok(ShippingCountry::Tr),
                    "SHIPPING_COUNTRY_TT" => Ok(ShippingCountry::Tt),
                    "SHIPPING_COUNTRY_TV" => Ok(ShippingCountry::Tv),
                    "SHIPPING_COUNTRY_TW" => Ok(ShippingCountry::Tw),
                    "SHIPPING_COUNTRY_TZ" => Ok(ShippingCountry::Tz),
                    "SHIPPING_COUNTRY_UA" => Ok(ShippingCountry::Ua),
                    "SHIPPING_COUNTRY_UG" => Ok(ShippingCountry::Ug),
                    "SHIPPING_COUNTRY_US" => Ok(ShippingCountry::Us),
                    "SHIPPING_COUNTRY_UY" => Ok(ShippingCountry::Uy),
                    "SHIPPING_COUNTRY_UZ" => Ok(ShippingCountry::Uz),
                    "SHIPPING_COUNTRY_VA" => Ok(ShippingCountry::Va),
                    "SHIPPING_COUNTRY_VC" => Ok(ShippingCountry::Vc),
                    "SHIPPING_COUNTRY_VE" => Ok(ShippingCountry::Ve),
                    "SHIPPING_COUNTRY_VG" => Ok(ShippingCountry::Vg),
                    "SHIPPING_COUNTRY_VN" => Ok(ShippingCountry::Vn),
                    "SHIPPING_COUNTRY_VU" => Ok(ShippingCountry::Vu),
                    "SHIPPING_COUNTRY_WF" => Ok(ShippingCountry::Wf),
                    "SHIPPING_COUNTRY_WS" => Ok(ShippingCountry::Ws),
                    "SHIPPING_COUNTRY_XK" => Ok(ShippingCountry::Xk),
                    "SHIPPING_COUNTRY_YE" => Ok(ShippingCountry::Ye),
                    "SHIPPING_COUNTRY_YT" => Ok(ShippingCountry::Yt),
                    "SHIPPING_COUNTRY_ZA" => Ok(ShippingCountry::Za),
                    "SHIPPING_COUNTRY_ZM" => Ok(ShippingCountry::Zm),
                    "SHIPPING_COUNTRY_ZW" => Ok(ShippingCountry::Zw),
                    "SHIPPING_COUNTRY_ZZ" => Ok(ShippingCountry::Zz),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for ShippingRateResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.shipping_rate_id.is_empty() {
            len += 1;
        }
        if !self.offer_id.is_empty() {
            len += 1;
        }
        if !self.user_id.is_empty() {
            len += 1;
        }
        if self.amount != 0 {
            len += 1;
        }
        if self.currency != 0 {
            len += 1;
        }
        if self.all_countries {
            len += 1;
        }
        if !self.specific_countries.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sited_io.commerce.v1.ShippingRateResponse", len)?;
        if !self.shipping_rate_id.is_empty() {
            struct_ser.serialize_field("shippingRateId", &self.shipping_rate_id)?;
        }
        if !self.offer_id.is_empty() {
            struct_ser.serialize_field("offerId", &self.offer_id)?;
        }
        if !self.user_id.is_empty() {
            struct_ser.serialize_field("userId", &self.user_id)?;
        }
        if self.amount != 0 {
            struct_ser.serialize_field("amount", &self.amount)?;
        }
        if self.currency != 0 {
            let v = Currency::try_from(self.currency)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.currency)))?;
            struct_ser.serialize_field("currency", &v)?;
        }
        if self.all_countries {
            struct_ser.serialize_field("allCountries", &self.all_countries)?;
        }
        if !self.specific_countries.is_empty() {
            let v = self.specific_countries.iter().cloned().map(|v| {
                ShippingCountry::try_from(v)
                    .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", v)))
                }).collect::<std::result::Result<Vec<_>, _>>()?;
            struct_ser.serialize_field("specificCountries", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ShippingRateResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "shipping_rate_id",
            "shippingRateId",
            "offer_id",
            "offerId",
            "user_id",
            "userId",
            "amount",
            "currency",
            "all_countries",
            "allCountries",
            "specific_countries",
            "specificCountries",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ShippingRateId,
            OfferId,
            UserId,
            Amount,
            Currency,
            AllCountries,
            SpecificCountries,
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
                            "shippingRateId" | "shipping_rate_id" => Ok(GeneratedField::ShippingRateId),
                            "offerId" | "offer_id" => Ok(GeneratedField::OfferId),
                            "userId" | "user_id" => Ok(GeneratedField::UserId),
                            "amount" => Ok(GeneratedField::Amount),
                            "currency" => Ok(GeneratedField::Currency),
                            "allCountries" | "all_countries" => Ok(GeneratedField::AllCountries),
                            "specificCountries" | "specific_countries" => Ok(GeneratedField::SpecificCountries),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ShippingRateResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.commerce.v1.ShippingRateResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ShippingRateResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut shipping_rate_id__ = None;
                let mut offer_id__ = None;
                let mut user_id__ = None;
                let mut amount__ = None;
                let mut currency__ = None;
                let mut all_countries__ = None;
                let mut specific_countries__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ShippingRateId => {
                            if shipping_rate_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("shippingRateId"));
                            }
                            shipping_rate_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::OfferId => {
                            if offer_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("offerId"));
                            }
                            offer_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::UserId => {
                            if user_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userId"));
                            }
                            user_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Amount => {
                            if amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("amount"));
                            }
                            amount__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Currency => {
                            if currency__.is_some() {
                                return Err(serde::de::Error::duplicate_field("currency"));
                            }
                            currency__ = Some(map_.next_value::<Currency>()? as i32);
                        }
                        GeneratedField::AllCountries => {
                            if all_countries__.is_some() {
                                return Err(serde::de::Error::duplicate_field("allCountries"));
                            }
                            all_countries__ = Some(map_.next_value()?);
                        }
                        GeneratedField::SpecificCountries => {
                            if specific_countries__.is_some() {
                                return Err(serde::de::Error::duplicate_field("specificCountries"));
                            }
                            specific_countries__ = Some(map_.next_value::<Vec<ShippingCountry>>()?.into_iter().map(|x| x as i32).collect());
                        }
                    }
                }
                Ok(ShippingRateResponse {
                    shipping_rate_id: shipping_rate_id__.unwrap_or_default(),
                    offer_id: offer_id__.unwrap_or_default(),
                    user_id: user_id__.unwrap_or_default(),
                    amount: amount__.unwrap_or_default(),
                    currency: currency__.unwrap_or_default(),
                    all_countries: all_countries__.unwrap_or_default(),
                    specific_countries: specific_countries__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sited_io.commerce.v1.ShippingRateResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ShopCustomizationResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.shop_id.is_empty() {
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
        if self.logo_image_light_url.is_some() {
            len += 1;
        }
        if self.logo_image_dark_url.is_some() {
            len += 1;
        }
        if self.banner_image_light_url.is_some() {
            len += 1;
        }
        if self.banner_image_dark_url.is_some() {
            len += 1;
        }
        if self.primary_color.is_some() {
            len += 1;
        }
        if self.layout_type != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sited_io.commerce.v1.ShopCustomizationResponse", len)?;
        if !self.shop_id.is_empty() {
            struct_ser.serialize_field("shopId", &self.shop_id)?;
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
        if let Some(v) = self.logo_image_light_url.as_ref() {
            struct_ser.serialize_field("logoImageLightUrl", v)?;
        }
        if let Some(v) = self.logo_image_dark_url.as_ref() {
            struct_ser.serialize_field("logoImageDarkUrl", v)?;
        }
        if let Some(v) = self.banner_image_light_url.as_ref() {
            struct_ser.serialize_field("bannerImageLightUrl", v)?;
        }
        if let Some(v) = self.banner_image_dark_url.as_ref() {
            struct_ser.serialize_field("bannerImageDarkUrl", v)?;
        }
        if let Some(v) = self.primary_color.as_ref() {
            struct_ser.serialize_field("primaryColor", v)?;
        }
        if self.layout_type != 0 {
            let v = ShopLayoutType::try_from(self.layout_type)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.layout_type)))?;
            struct_ser.serialize_field("layoutType", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ShopCustomizationResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "shop_id",
            "shopId",
            "user_id",
            "userId",
            "created_at",
            "createdAt",
            "updated_at",
            "updatedAt",
            "logo_image_light_url",
            "logoImageLightUrl",
            "logo_image_dark_url",
            "logoImageDarkUrl",
            "banner_image_light_url",
            "bannerImageLightUrl",
            "banner_image_dark_url",
            "bannerImageDarkUrl",
            "primary_color",
            "primaryColor",
            "layout_type",
            "layoutType",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ShopId,
            UserId,
            CreatedAt,
            UpdatedAt,
            LogoImageLightUrl,
            LogoImageDarkUrl,
            BannerImageLightUrl,
            BannerImageDarkUrl,
            PrimaryColor,
            LayoutType,
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
                            "shopId" | "shop_id" => Ok(GeneratedField::ShopId),
                            "userId" | "user_id" => Ok(GeneratedField::UserId),
                            "createdAt" | "created_at" => Ok(GeneratedField::CreatedAt),
                            "updatedAt" | "updated_at" => Ok(GeneratedField::UpdatedAt),
                            "logoImageLightUrl" | "logo_image_light_url" => Ok(GeneratedField::LogoImageLightUrl),
                            "logoImageDarkUrl" | "logo_image_dark_url" => Ok(GeneratedField::LogoImageDarkUrl),
                            "bannerImageLightUrl" | "banner_image_light_url" => Ok(GeneratedField::BannerImageLightUrl),
                            "bannerImageDarkUrl" | "banner_image_dark_url" => Ok(GeneratedField::BannerImageDarkUrl),
                            "primaryColor" | "primary_color" => Ok(GeneratedField::PrimaryColor),
                            "layoutType" | "layout_type" => Ok(GeneratedField::LayoutType),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ShopCustomizationResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.commerce.v1.ShopCustomizationResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ShopCustomizationResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut shop_id__ = None;
                let mut user_id__ = None;
                let mut created_at__ = None;
                let mut updated_at__ = None;
                let mut logo_image_light_url__ = None;
                let mut logo_image_dark_url__ = None;
                let mut banner_image_light_url__ = None;
                let mut banner_image_dark_url__ = None;
                let mut primary_color__ = None;
                let mut layout_type__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ShopId => {
                            if shop_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("shopId"));
                            }
                            shop_id__ = Some(map_.next_value()?);
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
                        GeneratedField::LogoImageLightUrl => {
                            if logo_image_light_url__.is_some() {
                                return Err(serde::de::Error::duplicate_field("logoImageLightUrl"));
                            }
                            logo_image_light_url__ = map_.next_value()?;
                        }
                        GeneratedField::LogoImageDarkUrl => {
                            if logo_image_dark_url__.is_some() {
                                return Err(serde::de::Error::duplicate_field("logoImageDarkUrl"));
                            }
                            logo_image_dark_url__ = map_.next_value()?;
                        }
                        GeneratedField::BannerImageLightUrl => {
                            if banner_image_light_url__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bannerImageLightUrl"));
                            }
                            banner_image_light_url__ = map_.next_value()?;
                        }
                        GeneratedField::BannerImageDarkUrl => {
                            if banner_image_dark_url__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bannerImageDarkUrl"));
                            }
                            banner_image_dark_url__ = map_.next_value()?;
                        }
                        GeneratedField::PrimaryColor => {
                            if primary_color__.is_some() {
                                return Err(serde::de::Error::duplicate_field("primaryColor"));
                            }
                            primary_color__ = map_.next_value()?;
                        }
                        GeneratedField::LayoutType => {
                            if layout_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("layoutType"));
                            }
                            layout_type__ = Some(map_.next_value::<ShopLayoutType>()? as i32);
                        }
                    }
                }
                Ok(ShopCustomizationResponse {
                    shop_id: shop_id__.unwrap_or_default(),
                    user_id: user_id__.unwrap_or_default(),
                    created_at: created_at__.unwrap_or_default(),
                    updated_at: updated_at__.unwrap_or_default(),
                    logo_image_light_url: logo_image_light_url__,
                    logo_image_dark_url: logo_image_dark_url__,
                    banner_image_light_url: banner_image_light_url__,
                    banner_image_dark_url: banner_image_dark_url__,
                    primary_color: primary_color__,
                    layout_type: layout_type__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sited_io.commerce.v1.ShopCustomizationResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ShopLayoutType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "SHOP_LAYOUT_TYPE_UNSPECIFIED",
            Self::Fead => "SHOP_LAYOUT_TYPE_FEAD",
            Self::OfferList => "SHOP_LAYOUT_TYPE_OFFER_LIST",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for ShopLayoutType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "SHOP_LAYOUT_TYPE_UNSPECIFIED",
            "SHOP_LAYOUT_TYPE_FEAD",
            "SHOP_LAYOUT_TYPE_OFFER_LIST",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ShopLayoutType;

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
                    "SHOP_LAYOUT_TYPE_UNSPECIFIED" => Ok(ShopLayoutType::Unspecified),
                    "SHOP_LAYOUT_TYPE_FEAD" => Ok(ShopLayoutType::Fead),
                    "SHOP_LAYOUT_TYPE_OFFER_LIST" => Ok(ShopLayoutType::OfferList),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for ShopResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.shop_id.is_empty() {
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
        if !self.slug.is_empty() {
            len += 1;
        }
        if self.domain.is_some() {
            len += 1;
        }
        if self.description.is_some() {
            len += 1;
        }
        if self.platform_fee_percent != 0 {
            len += 1;
        }
        if self.minimum_platform_fee_cent != 0 {
            len += 1;
        }
        if self.customization.is_some() {
            len += 1;
        }
        if self.is_active {
            len += 1;
        }
        if self.contact_email_address.is_some() {
            len += 1;
        }
        if self.client_id.is_some() {
            len += 1;
        }
        if !self.website_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sited_io.commerce.v1.ShopResponse", len)?;
        if !self.shop_id.is_empty() {
            struct_ser.serialize_field("shopId", &self.shop_id)?;
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
        if !self.slug.is_empty() {
            struct_ser.serialize_field("slug", &self.slug)?;
        }
        if let Some(v) = self.domain.as_ref() {
            struct_ser.serialize_field("domain", v)?;
        }
        if let Some(v) = self.description.as_ref() {
            struct_ser.serialize_field("description", v)?;
        }
        if self.platform_fee_percent != 0 {
            struct_ser.serialize_field("platformFeePercent", &self.platform_fee_percent)?;
        }
        if self.minimum_platform_fee_cent != 0 {
            struct_ser.serialize_field("minimumPlatformFeeCent", &self.minimum_platform_fee_cent)?;
        }
        if let Some(v) = self.customization.as_ref() {
            struct_ser.serialize_field("customization", v)?;
        }
        if self.is_active {
            struct_ser.serialize_field("isActive", &self.is_active)?;
        }
        if let Some(v) = self.contact_email_address.as_ref() {
            struct_ser.serialize_field("contactEmailAddress", v)?;
        }
        if let Some(v) = self.client_id.as_ref() {
            struct_ser.serialize_field("clientId", v)?;
        }
        if !self.website_id.is_empty() {
            struct_ser.serialize_field("websiteId", &self.website_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ShopResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "shop_id",
            "shopId",
            "user_id",
            "userId",
            "created_at",
            "createdAt",
            "updated_at",
            "updatedAt",
            "name",
            "slug",
            "domain",
            "description",
            "platform_fee_percent",
            "platformFeePercent",
            "minimum_platform_fee_cent",
            "minimumPlatformFeeCent",
            "customization",
            "is_active",
            "isActive",
            "contact_email_address",
            "contactEmailAddress",
            "client_id",
            "clientId",
            "website_id",
            "websiteId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ShopId,
            UserId,
            CreatedAt,
            UpdatedAt,
            Name,
            Slug,
            Domain,
            Description,
            PlatformFeePercent,
            MinimumPlatformFeeCent,
            Customization,
            IsActive,
            ContactEmailAddress,
            ClientId,
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
                            "shopId" | "shop_id" => Ok(GeneratedField::ShopId),
                            "userId" | "user_id" => Ok(GeneratedField::UserId),
                            "createdAt" | "created_at" => Ok(GeneratedField::CreatedAt),
                            "updatedAt" | "updated_at" => Ok(GeneratedField::UpdatedAt),
                            "name" => Ok(GeneratedField::Name),
                            "slug" => Ok(GeneratedField::Slug),
                            "domain" => Ok(GeneratedField::Domain),
                            "description" => Ok(GeneratedField::Description),
                            "platformFeePercent" | "platform_fee_percent" => Ok(GeneratedField::PlatformFeePercent),
                            "minimumPlatformFeeCent" | "minimum_platform_fee_cent" => Ok(GeneratedField::MinimumPlatformFeeCent),
                            "customization" => Ok(GeneratedField::Customization),
                            "isActive" | "is_active" => Ok(GeneratedField::IsActive),
                            "contactEmailAddress" | "contact_email_address" => Ok(GeneratedField::ContactEmailAddress),
                            "clientId" | "client_id" => Ok(GeneratedField::ClientId),
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
            type Value = ShopResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.commerce.v1.ShopResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ShopResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut shop_id__ = None;
                let mut user_id__ = None;
                let mut created_at__ = None;
                let mut updated_at__ = None;
                let mut name__ = None;
                let mut slug__ = None;
                let mut domain__ = None;
                let mut description__ = None;
                let mut platform_fee_percent__ = None;
                let mut minimum_platform_fee_cent__ = None;
                let mut customization__ = None;
                let mut is_active__ = None;
                let mut contact_email_address__ = None;
                let mut client_id__ = None;
                let mut website_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ShopId => {
                            if shop_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("shopId"));
                            }
                            shop_id__ = Some(map_.next_value()?);
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
                        GeneratedField::Slug => {
                            if slug__.is_some() {
                                return Err(serde::de::Error::duplicate_field("slug"));
                            }
                            slug__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Domain => {
                            if domain__.is_some() {
                                return Err(serde::de::Error::duplicate_field("domain"));
                            }
                            domain__ = map_.next_value()?;
                        }
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = map_.next_value()?;
                        }
                        GeneratedField::PlatformFeePercent => {
                            if platform_fee_percent__.is_some() {
                                return Err(serde::de::Error::duplicate_field("platformFeePercent"));
                            }
                            platform_fee_percent__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::MinimumPlatformFeeCent => {
                            if minimum_platform_fee_cent__.is_some() {
                                return Err(serde::de::Error::duplicate_field("minimumPlatformFeeCent"));
                            }
                            minimum_platform_fee_cent__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Customization => {
                            if customization__.is_some() {
                                return Err(serde::de::Error::duplicate_field("customization"));
                            }
                            customization__ = map_.next_value()?;
                        }
                        GeneratedField::IsActive => {
                            if is_active__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isActive"));
                            }
                            is_active__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ContactEmailAddress => {
                            if contact_email_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("contactEmailAddress"));
                            }
                            contact_email_address__ = map_.next_value()?;
                        }
                        GeneratedField::ClientId => {
                            if client_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("clientId"));
                            }
                            client_id__ = map_.next_value()?;
                        }
                        GeneratedField::WebsiteId => {
                            if website_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("websiteId"));
                            }
                            website_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ShopResponse {
                    shop_id: shop_id__.unwrap_or_default(),
                    user_id: user_id__.unwrap_or_default(),
                    created_at: created_at__.unwrap_or_default(),
                    updated_at: updated_at__.unwrap_or_default(),
                    name: name__.unwrap_or_default(),
                    slug: slug__.unwrap_or_default(),
                    domain: domain__,
                    description: description__,
                    platform_fee_percent: platform_fee_percent__.unwrap_or_default(),
                    minimum_platform_fee_cent: minimum_platform_fee_cent__.unwrap_or_default(),
                    customization: customization__,
                    is_active: is_active__.unwrap_or_default(),
                    contact_email_address: contact_email_address__,
                    client_id: client_id__,
                    website_id: website_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sited_io.commerce.v1.ShopResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ShopsFilter {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.field != 0 {
            len += 1;
        }
        if !self.query.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sited_io.commerce.v1.ShopsFilter", len)?;
        if self.field != 0 {
            let v = ShopsFilterField::try_from(self.field)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.field)))?;
            struct_ser.serialize_field("field", &v)?;
        }
        if !self.query.is_empty() {
            struct_ser.serialize_field("query", &self.query)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ShopsFilter {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "field",
            "query",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Field,
            Query,
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
                            "field" => Ok(GeneratedField::Field),
                            "query" => Ok(GeneratedField::Query),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ShopsFilter;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.commerce.v1.ShopsFilter")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ShopsFilter, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut field__ = None;
                let mut query__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Field => {
                            if field__.is_some() {
                                return Err(serde::de::Error::duplicate_field("field"));
                            }
                            field__ = Some(map_.next_value::<ShopsFilterField>()? as i32);
                        }
                        GeneratedField::Query => {
                            if query__.is_some() {
                                return Err(serde::de::Error::duplicate_field("query"));
                            }
                            query__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ShopsFilter {
                    field: field__.unwrap_or_default(),
                    query: query__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sited_io.commerce.v1.ShopsFilter", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ShopsFilterField {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "SHOPS_FILTER_FIELD_UNSPECIFIED",
            Self::Name => "SHOPS_FILTER_FIELD_NAME",
            Self::Description => "SHOPS_FILTER_FIELD_DESCRIPTION",
            Self::NameAndDescription => "SHOPS_FILTER_FIELD_NAME_AND_DESCRIPTION",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for ShopsFilterField {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "SHOPS_FILTER_FIELD_UNSPECIFIED",
            "SHOPS_FILTER_FIELD_NAME",
            "SHOPS_FILTER_FIELD_DESCRIPTION",
            "SHOPS_FILTER_FIELD_NAME_AND_DESCRIPTION",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ShopsFilterField;

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
                    "SHOPS_FILTER_FIELD_UNSPECIFIED" => Ok(ShopsFilterField::Unspecified),
                    "SHOPS_FILTER_FIELD_NAME" => Ok(ShopsFilterField::Name),
                    "SHOPS_FILTER_FIELD_DESCRIPTION" => Ok(ShopsFilterField::Description),
                    "SHOPS_FILTER_FIELD_NAME_AND_DESCRIPTION" => Ok(ShopsFilterField::NameAndDescription),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for ShopsOrderBy {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.field != 0 {
            len += 1;
        }
        if self.direction != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sited_io.commerce.v1.ShopsOrderBy", len)?;
        if self.field != 0 {
            let v = ShopsOrderByField::try_from(self.field)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.field)))?;
            struct_ser.serialize_field("field", &v)?;
        }
        if self.direction != 0 {
            let v = super::super::types::v1::Direction::try_from(self.direction)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.direction)))?;
            struct_ser.serialize_field("direction", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ShopsOrderBy {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "field",
            "direction",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Field,
            Direction,
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
                            "field" => Ok(GeneratedField::Field),
                            "direction" => Ok(GeneratedField::Direction),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ShopsOrderBy;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.commerce.v1.ShopsOrderBy")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ShopsOrderBy, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut field__ = None;
                let mut direction__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Field => {
                            if field__.is_some() {
                                return Err(serde::de::Error::duplicate_field("field"));
                            }
                            field__ = Some(map_.next_value::<ShopsOrderByField>()? as i32);
                        }
                        GeneratedField::Direction => {
                            if direction__.is_some() {
                                return Err(serde::de::Error::duplicate_field("direction"));
                            }
                            direction__ = Some(map_.next_value::<super::super::types::v1::Direction>()? as i32);
                        }
                    }
                }
                Ok(ShopsOrderBy {
                    field: field__.unwrap_or_default(),
                    direction: direction__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sited_io.commerce.v1.ShopsOrderBy", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ShopsOrderByField {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "SHOPS_ORDER_BY_FIELD_UNSPECIFIED",
            Self::CreatedAt => "SHOPS_ORDER_BY_FIELD_CREATED_AT",
            Self::UpdatedAt => "SHOPS_ORDER_BY_FIELD_UPDATED_AT",
            Self::Name => "SHOPS_ORDER_BY_FIELD_NAME",
            Self::Random => "SHOPS_ORDER_BY_FIELD_RANDOM",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for ShopsOrderByField {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "SHOPS_ORDER_BY_FIELD_UNSPECIFIED",
            "SHOPS_ORDER_BY_FIELD_CREATED_AT",
            "SHOPS_ORDER_BY_FIELD_UPDATED_AT",
            "SHOPS_ORDER_BY_FIELD_NAME",
            "SHOPS_ORDER_BY_FIELD_RANDOM",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ShopsOrderByField;

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
                    "SHOPS_ORDER_BY_FIELD_UNSPECIFIED" => Ok(ShopsOrderByField::Unspecified),
                    "SHOPS_ORDER_BY_FIELD_CREATED_AT" => Ok(ShopsOrderByField::CreatedAt),
                    "SHOPS_ORDER_BY_FIELD_UPDATED_AT" => Ok(ShopsOrderByField::UpdatedAt),
                    "SHOPS_ORDER_BY_FIELD_NAME" => Ok(ShopsOrderByField::Name),
                    "SHOPS_ORDER_BY_FIELD_RANDOM" => Ok(ShopsOrderByField::Random),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateDomainStatusRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.shop_id.is_empty() {
            len += 1;
        }
        if !self.domain.is_empty() {
            len += 1;
        }
        if self.status != 0 {
            len += 1;
        }
        if !self.client_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sited_io.commerce.v1.UpdateDomainStatusRequest", len)?;
        if !self.shop_id.is_empty() {
            struct_ser.serialize_field("shopId", &self.shop_id)?;
        }
        if !self.domain.is_empty() {
            struct_ser.serialize_field("domain", &self.domain)?;
        }
        if self.status != 0 {
            let v = DomainStatus::try_from(self.status)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.status)))?;
            struct_ser.serialize_field("status", &v)?;
        }
        if !self.client_id.is_empty() {
            struct_ser.serialize_field("clientId", &self.client_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateDomainStatusRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "shop_id",
            "shopId",
            "domain",
            "status",
            "client_id",
            "clientId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ShopId,
            Domain,
            Status,
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
                            "shopId" | "shop_id" => Ok(GeneratedField::ShopId),
                            "domain" => Ok(GeneratedField::Domain),
                            "status" => Ok(GeneratedField::Status),
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
            type Value = UpdateDomainStatusRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.commerce.v1.UpdateDomainStatusRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateDomainStatusRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut shop_id__ = None;
                let mut domain__ = None;
                let mut status__ = None;
                let mut client_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ShopId => {
                            if shop_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("shopId"));
                            }
                            shop_id__ = Some(map_.next_value()?);
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
                        GeneratedField::ClientId => {
                            if client_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("clientId"));
                            }
                            client_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(UpdateDomainStatusRequest {
                    shop_id: shop_id__.unwrap_or_default(),
                    domain: domain__.unwrap_or_default(),
                    status: status__.unwrap_or_default(),
                    client_id: client_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sited_io.commerce.v1.UpdateDomainStatusRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateDomainStatusResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("sited_io.commerce.v1.UpdateDomainStatusResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateDomainStatusResponse {
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
            type Value = UpdateDomainStatusResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.commerce.v1.UpdateDomainStatusResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateDomainStatusResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(UpdateDomainStatusResponse {
                })
            }
        }
        deserializer.deserialize_struct("sited_io.commerce.v1.UpdateDomainStatusResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateOfferRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.offer_id.is_empty() {
            len += 1;
        }
        if self.name.is_some() {
            len += 1;
        }
        if self.description.is_some() {
            len += 1;
        }
        if self.is_active.is_some() {
            len += 1;
        }
        if self.r#type.is_some() {
            len += 1;
        }
        if self.is_featured.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sited_io.commerce.v1.UpdateOfferRequest", len)?;
        if !self.offer_id.is_empty() {
            struct_ser.serialize_field("offerId", &self.offer_id)?;
        }
        if let Some(v) = self.name.as_ref() {
            struct_ser.serialize_field("name", v)?;
        }
        if let Some(v) = self.description.as_ref() {
            struct_ser.serialize_field("description", v)?;
        }
        if let Some(v) = self.is_active.as_ref() {
            struct_ser.serialize_field("isActive", v)?;
        }
        if let Some(v) = self.r#type.as_ref() {
            let v = OfferType::try_from(*v)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", *v)))?;
            struct_ser.serialize_field("type", &v)?;
        }
        if let Some(v) = self.is_featured.as_ref() {
            struct_ser.serialize_field("isFeatured", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateOfferRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "offer_id",
            "offerId",
            "name",
            "description",
            "is_active",
            "isActive",
            "type",
            "is_featured",
            "isFeatured",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            OfferId,
            Name,
            Description,
            IsActive,
            Type,
            IsFeatured,
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
                            "offerId" | "offer_id" => Ok(GeneratedField::OfferId),
                            "name" => Ok(GeneratedField::Name),
                            "description" => Ok(GeneratedField::Description),
                            "isActive" | "is_active" => Ok(GeneratedField::IsActive),
                            "type" => Ok(GeneratedField::Type),
                            "isFeatured" | "is_featured" => Ok(GeneratedField::IsFeatured),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UpdateOfferRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.commerce.v1.UpdateOfferRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateOfferRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut offer_id__ = None;
                let mut name__ = None;
                let mut description__ = None;
                let mut is_active__ = None;
                let mut r#type__ = None;
                let mut is_featured__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::OfferId => {
                            if offer_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("offerId"));
                            }
                            offer_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = map_.next_value()?;
                        }
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = map_.next_value()?;
                        }
                        GeneratedField::IsActive => {
                            if is_active__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isActive"));
                            }
                            is_active__ = map_.next_value()?;
                        }
                        GeneratedField::Type => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type__ = map_.next_value::<::std::option::Option<OfferType>>()?.map(|x| x as i32);
                        }
                        GeneratedField::IsFeatured => {
                            if is_featured__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isFeatured"));
                            }
                            is_featured__ = map_.next_value()?;
                        }
                    }
                }
                Ok(UpdateOfferRequest {
                    offer_id: offer_id__.unwrap_or_default(),
                    name: name__,
                    description: description__,
                    is_active: is_active__,
                    r#type: r#type__,
                    is_featured: is_featured__,
                })
            }
        }
        deserializer.deserialize_struct("sited_io.commerce.v1.UpdateOfferRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateOfferResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.offer.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sited_io.commerce.v1.UpdateOfferResponse", len)?;
        if let Some(v) = self.offer.as_ref() {
            struct_ser.serialize_field("offer", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateOfferResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "offer",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Offer,
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
                            "offer" => Ok(GeneratedField::Offer),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UpdateOfferResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.commerce.v1.UpdateOfferResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateOfferResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut offer__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Offer => {
                            if offer__.is_some() {
                                return Err(serde::de::Error::duplicate_field("offer"));
                            }
                            offer__ = map_.next_value()?;
                        }
                    }
                }
                Ok(UpdateOfferResponse {
                    offer: offer__,
                })
            }
        }
        deserializer.deserialize_struct("sited_io.commerce.v1.UpdateOfferResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateShopRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.shop_id.is_empty() {
            len += 1;
        }
        if self.name.is_some() {
            len += 1;
        }
        if self.slug.is_some() {
            len += 1;
        }
        if self.description.is_some() {
            len += 1;
        }
        if self.platform_fee_percent.is_some() {
            len += 1;
        }
        if self.minimum_platform_fee_cent.is_some() {
            len += 1;
        }
        if self.is_active.is_some() {
            len += 1;
        }
        if self.contact_email_address.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sited_io.commerce.v1.UpdateShopRequest", len)?;
        if !self.shop_id.is_empty() {
            struct_ser.serialize_field("shopId", &self.shop_id)?;
        }
        if let Some(v) = self.name.as_ref() {
            struct_ser.serialize_field("name", v)?;
        }
        if let Some(v) = self.slug.as_ref() {
            struct_ser.serialize_field("slug", v)?;
        }
        if let Some(v) = self.description.as_ref() {
            struct_ser.serialize_field("description", v)?;
        }
        if let Some(v) = self.platform_fee_percent.as_ref() {
            struct_ser.serialize_field("platformFeePercent", v)?;
        }
        if let Some(v) = self.minimum_platform_fee_cent.as_ref() {
            struct_ser.serialize_field("minimumPlatformFeeCent", v)?;
        }
        if let Some(v) = self.is_active.as_ref() {
            struct_ser.serialize_field("isActive", v)?;
        }
        if let Some(v) = self.contact_email_address.as_ref() {
            struct_ser.serialize_field("contactEmailAddress", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateShopRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "shop_id",
            "shopId",
            "name",
            "slug",
            "description",
            "platform_fee_percent",
            "platformFeePercent",
            "minimum_platform_fee_cent",
            "minimumPlatformFeeCent",
            "is_active",
            "isActive",
            "contact_email_address",
            "contactEmailAddress",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ShopId,
            Name,
            Slug,
            Description,
            PlatformFeePercent,
            MinimumPlatformFeeCent,
            IsActive,
            ContactEmailAddress,
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
                            "shopId" | "shop_id" => Ok(GeneratedField::ShopId),
                            "name" => Ok(GeneratedField::Name),
                            "slug" => Ok(GeneratedField::Slug),
                            "description" => Ok(GeneratedField::Description),
                            "platformFeePercent" | "platform_fee_percent" => Ok(GeneratedField::PlatformFeePercent),
                            "minimumPlatformFeeCent" | "minimum_platform_fee_cent" => Ok(GeneratedField::MinimumPlatformFeeCent),
                            "isActive" | "is_active" => Ok(GeneratedField::IsActive),
                            "contactEmailAddress" | "contact_email_address" => Ok(GeneratedField::ContactEmailAddress),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UpdateShopRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.commerce.v1.UpdateShopRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateShopRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut shop_id__ = None;
                let mut name__ = None;
                let mut slug__ = None;
                let mut description__ = None;
                let mut platform_fee_percent__ = None;
                let mut minimum_platform_fee_cent__ = None;
                let mut is_active__ = None;
                let mut contact_email_address__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ShopId => {
                            if shop_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("shopId"));
                            }
                            shop_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = map_.next_value()?;
                        }
                        GeneratedField::Slug => {
                            if slug__.is_some() {
                                return Err(serde::de::Error::duplicate_field("slug"));
                            }
                            slug__ = map_.next_value()?;
                        }
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = map_.next_value()?;
                        }
                        GeneratedField::PlatformFeePercent => {
                            if platform_fee_percent__.is_some() {
                                return Err(serde::de::Error::duplicate_field("platformFeePercent"));
                            }
                            platform_fee_percent__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::MinimumPlatformFeeCent => {
                            if minimum_platform_fee_cent__.is_some() {
                                return Err(serde::de::Error::duplicate_field("minimumPlatformFeeCent"));
                            }
                            minimum_platform_fee_cent__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::IsActive => {
                            if is_active__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isActive"));
                            }
                            is_active__ = map_.next_value()?;
                        }
                        GeneratedField::ContactEmailAddress => {
                            if contact_email_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("contactEmailAddress"));
                            }
                            contact_email_address__ = map_.next_value()?;
                        }
                    }
                }
                Ok(UpdateShopRequest {
                    shop_id: shop_id__.unwrap_or_default(),
                    name: name__,
                    slug: slug__,
                    description: description__,
                    platform_fee_percent: platform_fee_percent__,
                    minimum_platform_fee_cent: minimum_platform_fee_cent__,
                    is_active: is_active__,
                    contact_email_address: contact_email_address__,
                })
            }
        }
        deserializer.deserialize_struct("sited_io.commerce.v1.UpdateShopRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateShopResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.shop.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sited_io.commerce.v1.UpdateShopResponse", len)?;
        if let Some(v) = self.shop.as_ref() {
            struct_ser.serialize_field("shop", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateShopResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "shop",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Shop,
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
                            "shop" => Ok(GeneratedField::Shop),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UpdateShopResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.commerce.v1.UpdateShopResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateShopResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut shop__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Shop => {
                            if shop__.is_some() {
                                return Err(serde::de::Error::duplicate_field("shop"));
                            }
                            shop__ = map_.next_value()?;
                        }
                    }
                }
                Ok(UpdateShopResponse {
                    shop: shop__,
                })
            }
        }
        deserializer.deserialize_struct("sited_io.commerce.v1.UpdateShopResponse", FIELDS, GeneratedVisitor)
    }
}

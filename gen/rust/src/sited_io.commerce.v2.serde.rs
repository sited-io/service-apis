// @generated
impl serde::Serialize for AddFileToOfferRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.file_name.is_empty() {
            len += 1;
        }
        if !self.offer_id.is_empty() {
            len += 1;
        }
        if !self.content.is_empty() {
            len += 1;
        }
        if self.content_type.is_some() {
            len += 1;
        }
        if self.ordering.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sited_io.commerce.v2.AddFileToOfferRequest", len)?;
        if !self.file_name.is_empty() {
            struct_ser.serialize_field("fileName", &self.file_name)?;
        }
        if !self.offer_id.is_empty() {
            struct_ser.serialize_field("offerId", &self.offer_id)?;
        }
        if !self.content.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("content", pbjson::private::base64::encode(&self.content).as_str())?;
        }
        if let Some(v) = self.content_type.as_ref() {
            struct_ser.serialize_field("contentType", v)?;
        }
        if let Some(v) = self.ordering.as_ref() {
            struct_ser.serialize_field("ordering", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AddFileToOfferRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "file_name",
            "fileName",
            "offer_id",
            "offerId",
            "content",
            "content_type",
            "contentType",
            "ordering",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            FileName,
            OfferId,
            Content,
            ContentType,
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
                            "fileName" | "file_name" => Ok(GeneratedField::FileName),
                            "offerId" | "offer_id" => Ok(GeneratedField::OfferId),
                            "content" => Ok(GeneratedField::Content),
                            "contentType" | "content_type" => Ok(GeneratedField::ContentType),
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
            type Value = AddFileToOfferRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.commerce.v2.AddFileToOfferRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AddFileToOfferRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut file_name__ = None;
                let mut offer_id__ = None;
                let mut content__ = None;
                let mut content_type__ = None;
                let mut ordering__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::FileName => {
                            if file_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fileName"));
                            }
                            file_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::OfferId => {
                            if offer_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("offerId"));
                            }
                            offer_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Content => {
                            if content__.is_some() {
                                return Err(serde::de::Error::duplicate_field("content"));
                            }
                            content__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::ContentType => {
                            if content_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("contentType"));
                            }
                            content_type__ = map_.next_value()?;
                        }
                        GeneratedField::Ordering => {
                            if ordering__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ordering"));
                            }
                            ordering__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                    }
                }
                Ok(AddFileToOfferRequest {
                    file_name: file_name__.unwrap_or_default(),
                    offer_id: offer_id__.unwrap_or_default(),
                    content: content__.unwrap_or_default(),
                    content_type: content_type__,
                    ordering: ordering__,
                })
            }
        }
        deserializer.deserialize_struct("sited_io.commerce.v2.AddFileToOfferRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for AddFileToOfferResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("sited_io.commerce.v2.AddFileToOfferResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AddFileToOfferResponse {
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
            type Value = AddFileToOfferResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.commerce.v2.AddFileToOfferResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AddFileToOfferResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(AddFileToOfferResponse {
                })
            }
        }
        deserializer.deserialize_struct("sited_io.commerce.v2.AddFileToOfferResponse", FIELDS, GeneratedVisitor)
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
        if !self.data.is_empty() {
            len += 1;
        }
        if self.ordering != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sited_io.commerce.v2.AddImageToOfferRequest", len)?;
        if !self.offer_id.is_empty() {
            struct_ser.serialize_field("offerId", &self.offer_id)?;
        }
        if !self.data.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("data", pbjson::private::base64::encode(&self.data).as_str())?;
        }
        if self.ordering != 0 {
            struct_ser.serialize_field("ordering", &self.ordering)?;
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
            "data",
            "ordering",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            OfferId,
            Data,
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
                            "data" => Ok(GeneratedField::Data),
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
                formatter.write_str("struct sited_io.commerce.v2.AddImageToOfferRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AddImageToOfferRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut offer_id__ = None;
                let mut data__ = None;
                let mut ordering__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::OfferId => {
                            if offer_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("offerId"));
                            }
                            offer_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Data => {
                            if data__.is_some() {
                                return Err(serde::de::Error::duplicate_field("data"));
                            }
                            data__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
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
                    data: data__.unwrap_or_default(),
                    ordering: ordering__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sited_io.commerce.v2.AddImageToOfferRequest", FIELDS, GeneratedVisitor)
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
        let struct_ser = serializer.serialize_struct("sited_io.commerce.v2.AddImageToOfferResponse", len)?;
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
                formatter.write_str("struct sited_io.commerce.v2.AddImageToOfferResponse")
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
        deserializer.deserialize_struct("sited_io.commerce.v2.AddImageToOfferResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for AddOfferToShopRequest {
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
        let mut struct_ser = serializer.serialize_struct("sited_io.commerce.v2.AddOfferToShopRequest", len)?;
        if !self.offer_id.is_empty() {
            struct_ser.serialize_field("offerId", &self.offer_id)?;
        }
        if !self.shop_id.is_empty() {
            struct_ser.serialize_field("shopId", &self.shop_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AddOfferToShopRequest {
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
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            OfferId,
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
                            "offerId" | "offer_id" => Ok(GeneratedField::OfferId),
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
            type Value = AddOfferToShopRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.commerce.v2.AddOfferToShopRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AddOfferToShopRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut offer_id__ = None;
                let mut shop_id__ = None;
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
                    }
                }
                Ok(AddOfferToShopRequest {
                    offer_id: offer_id__.unwrap_or_default(),
                    shop_id: shop_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sited_io.commerce.v2.AddOfferToShopRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for AddOfferToShopResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("sited_io.commerce.v2.AddOfferToShopResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AddOfferToShopResponse {
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
            type Value = AddOfferToShopResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.commerce.v2.AddOfferToShopResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AddOfferToShopResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(AddOfferToShopResponse {
                })
            }
        }
        deserializer.deserialize_struct("sited_io.commerce.v2.AddOfferToShopResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for BuyOfferRequest {
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
        if self.payment_method.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sited_io.commerce.v2.BuyOfferRequest", len)?;
        if !self.offer_id.is_empty() {
            struct_ser.serialize_field("offerId", &self.offer_id)?;
        }
        if !self.shop_id.is_empty() {
            struct_ser.serialize_field("shopId", &self.shop_id)?;
        }
        if let Some(v) = self.payment_method.as_ref() {
            match v {
                buy_offer_request::PaymentMethod::Stripe(v) => {
                    struct_ser.serialize_field("stripe", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for BuyOfferRequest {
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
            "stripe",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            OfferId,
            ShopId,
            Stripe,
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
                            "stripe" => Ok(GeneratedField::Stripe),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = BuyOfferRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.commerce.v2.BuyOfferRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<BuyOfferRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut offer_id__ = None;
                let mut shop_id__ = None;
                let mut payment_method__ = None;
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
                        GeneratedField::Stripe => {
                            if payment_method__.is_some() {
                                return Err(serde::de::Error::duplicate_field("stripe"));
                            }
                            payment_method__ = map_.next_value::<::std::option::Option<_>>()?.map(buy_offer_request::PaymentMethod::Stripe)
;
                        }
                    }
                }
                Ok(BuyOfferRequest {
                    offer_id: offer_id__.unwrap_or_default(),
                    shop_id: shop_id__.unwrap_or_default(),
                    payment_method: payment_method__,
                })
            }
        }
        deserializer.deserialize_struct("sited_io.commerce.v2.BuyOfferRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for buy_offer_request::Stripe {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.success_url.is_empty() {
            len += 1;
        }
        if !self.cancel_url.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sited_io.commerce.v2.BuyOfferRequest.Stripe", len)?;
        if !self.success_url.is_empty() {
            struct_ser.serialize_field("successUrl", &self.success_url)?;
        }
        if !self.cancel_url.is_empty() {
            struct_ser.serialize_field("cancelUrl", &self.cancel_url)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for buy_offer_request::Stripe {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "success_url",
            "successUrl",
            "cancel_url",
            "cancelUrl",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            SuccessUrl,
            CancelUrl,
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
                            "successUrl" | "success_url" => Ok(GeneratedField::SuccessUrl),
                            "cancelUrl" | "cancel_url" => Ok(GeneratedField::CancelUrl),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = buy_offer_request::Stripe;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.commerce.v2.BuyOfferRequest.Stripe")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<buy_offer_request::Stripe, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut success_url__ = None;
                let mut cancel_url__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::SuccessUrl => {
                            if success_url__.is_some() {
                                return Err(serde::de::Error::duplicate_field("successUrl"));
                            }
                            success_url__ = Some(map_.next_value()?);
                        }
                        GeneratedField::CancelUrl => {
                            if cancel_url__.is_some() {
                                return Err(serde::de::Error::duplicate_field("cancelUrl"));
                            }
                            cancel_url__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(buy_offer_request::Stripe {
                    success_url: success_url__.unwrap_or_default(),
                    cancel_url: cancel_url__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sited_io.commerce.v2.BuyOfferRequest.Stripe", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for BuyOfferResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.payment_method.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sited_io.commerce.v2.BuyOfferResponse", len)?;
        if let Some(v) = self.payment_method.as_ref() {
            match v {
                buy_offer_response::PaymentMethod::Stripe(v) => {
                    struct_ser.serialize_field("stripe", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for BuyOfferResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "stripe",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Stripe,
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
                            "stripe" => Ok(GeneratedField::Stripe),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = BuyOfferResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.commerce.v2.BuyOfferResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<BuyOfferResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut payment_method__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Stripe => {
                            if payment_method__.is_some() {
                                return Err(serde::de::Error::duplicate_field("stripe"));
                            }
                            payment_method__ = map_.next_value::<::std::option::Option<_>>()?.map(buy_offer_response::PaymentMethod::Stripe)
;
                        }
                    }
                }
                Ok(BuyOfferResponse {
                    payment_method: payment_method__,
                })
            }
        }
        deserializer.deserialize_struct("sited_io.commerce.v2.BuyOfferResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for buy_offer_response::Stripe {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.link.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sited_io.commerce.v2.BuyOfferResponse.Stripe", len)?;
        if !self.link.is_empty() {
            struct_ser.serialize_field("link", &self.link)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for buy_offer_response::Stripe {
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
            type Value = buy_offer_response::Stripe;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.commerce.v2.BuyOfferResponse.Stripe")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<buy_offer_response::Stripe, V::Error>
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
                            link__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(buy_offer_response::Stripe {
                    link: link__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sited_io.commerce.v2.BuyOfferResponse.Stripe", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CancelSubscriptionRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.order_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sited_io.commerce.v2.CancelSubscriptionRequest", len)?;
        if !self.order_id.is_empty() {
            struct_ser.serialize_field("orderId", &self.order_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CancelSubscriptionRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "order_id",
            "orderId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            OrderId,
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
                            "orderId" | "order_id" => Ok(GeneratedField::OrderId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CancelSubscriptionRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.commerce.v2.CancelSubscriptionRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CancelSubscriptionRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut order_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::OrderId => {
                            if order_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("orderId"));
                            }
                            order_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(CancelSubscriptionRequest {
                    order_id: order_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sited_io.commerce.v2.CancelSubscriptionRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CancelSubscriptionResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("sited_io.commerce.v2.CancelSubscriptionResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CancelSubscriptionResponse {
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
            type Value = CancelSubscriptionResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.commerce.v2.CancelSubscriptionResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CancelSubscriptionResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(CancelSubscriptionResponse {
                })
            }
        }
        deserializer.deserialize_struct("sited_io.commerce.v2.CancelSubscriptionResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CompleteMultipartUploadRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.offer_file_id.is_empty() {
            len += 1;
        }
        if !self.upload_id.is_empty() {
            len += 1;
        }
        if !self.parts.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sited_io.commerce.v2.CompleteMultipartUploadRequest", len)?;
        if !self.offer_file_id.is_empty() {
            struct_ser.serialize_field("offerFileId", &self.offer_file_id)?;
        }
        if !self.upload_id.is_empty() {
            struct_ser.serialize_field("uploadId", &self.upload_id)?;
        }
        if !self.parts.is_empty() {
            struct_ser.serialize_field("parts", &self.parts)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CompleteMultipartUploadRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "offer_file_id",
            "offerFileId",
            "upload_id",
            "uploadId",
            "parts",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            OfferFileId,
            UploadId,
            Parts,
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
                            "offerFileId" | "offer_file_id" => Ok(GeneratedField::OfferFileId),
                            "uploadId" | "upload_id" => Ok(GeneratedField::UploadId),
                            "parts" => Ok(GeneratedField::Parts),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CompleteMultipartUploadRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.commerce.v2.CompleteMultipartUploadRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CompleteMultipartUploadRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut offer_file_id__ = None;
                let mut upload_id__ = None;
                let mut parts__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::OfferFileId => {
                            if offer_file_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("offerFileId"));
                            }
                            offer_file_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::UploadId => {
                            if upload_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("uploadId"));
                            }
                            upload_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Parts => {
                            if parts__.is_some() {
                                return Err(serde::de::Error::duplicate_field("parts"));
                            }
                            parts__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(CompleteMultipartUploadRequest {
                    offer_file_id: offer_file_id__.unwrap_or_default(),
                    upload_id: upload_id__.unwrap_or_default(),
                    parts: parts__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sited_io.commerce.v2.CompleteMultipartUploadRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CompleteMultipartUploadResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("sited_io.commerce.v2.CompleteMultipartUploadResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CompleteMultipartUploadResponse {
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
            type Value = CompleteMultipartUploadResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.commerce.v2.CompleteMultipartUploadResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CompleteMultipartUploadResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(CompleteMultipartUploadResponse {
                })
            }
        }
        deserializer.deserialize_struct("sited_io.commerce.v2.CompleteMultipartUploadResponse", FIELDS, GeneratedVisitor)
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
        if self.details.is_some() {
            len += 1;
        }
        if self.offer_type.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sited_io.commerce.v2.CreateOfferRequest", len)?;
        if let Some(v) = self.details.as_ref() {
            struct_ser.serialize_field("details", v)?;
        }
        if let Some(v) = self.offer_type.as_ref() {
            struct_ser.serialize_field("offerType", v)?;
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
            "details",
            "offer_type",
            "offerType",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Details,
            OfferType,
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
                            "details" => Ok(GeneratedField::Details),
                            "offerType" | "offer_type" => Ok(GeneratedField::OfferType),
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
                formatter.write_str("struct sited_io.commerce.v2.CreateOfferRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateOfferRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut details__ = None;
                let mut offer_type__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Details => {
                            if details__.is_some() {
                                return Err(serde::de::Error::duplicate_field("details"));
                            }
                            details__ = map_.next_value()?;
                        }
                        GeneratedField::OfferType => {
                            if offer_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("offerType"));
                            }
                            offer_type__ = map_.next_value()?;
                        }
                    }
                }
                Ok(CreateOfferRequest {
                    details: details__,
                    offer_type: offer_type__,
                })
            }
        }
        deserializer.deserialize_struct("sited_io.commerce.v2.CreateOfferRequest", FIELDS, GeneratedVisitor)
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
        let mut struct_ser = serializer.serialize_struct("sited_io.commerce.v2.CreateOfferResponse", len)?;
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
                formatter.write_str("struct sited_io.commerce.v2.CreateOfferResponse")
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
        deserializer.deserialize_struct("sited_io.commerce.v2.CreateOfferResponse", FIELDS, GeneratedVisitor)
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
        if !self.website_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sited_io.commerce.v2.CreateShopRequest", len)?;
        if !self.website_id.is_empty() {
            struct_ser.serialize_field("websiteId", &self.website_id)?;
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
            type Value = CreateShopRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.commerce.v2.CreateShopRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateShopRequest, V::Error>
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
                Ok(CreateShopRequest {
                    website_id: website_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sited_io.commerce.v2.CreateShopRequest", FIELDS, GeneratedVisitor)
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
        let mut struct_ser = serializer.serialize_struct("sited_io.commerce.v2.CreateShopResponse", len)?;
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
                formatter.write_str("struct sited_io.commerce.v2.CreateShopResponse")
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
        deserializer.deserialize_struct("sited_io.commerce.v2.CreateShopResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateStripeAccountRequest {
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
        if !self.refresh_url.is_empty() {
            len += 1;
        }
        if !self.return_url.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sited_io.commerce.v2.CreateStripeAccountRequest", len)?;
        if !self.website_id.is_empty() {
            struct_ser.serialize_field("websiteId", &self.website_id)?;
        }
        if !self.refresh_url.is_empty() {
            struct_ser.serialize_field("refreshUrl", &self.refresh_url)?;
        }
        if !self.return_url.is_empty() {
            struct_ser.serialize_field("returnUrl", &self.return_url)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateStripeAccountRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "website_id",
            "websiteId",
            "refresh_url",
            "refreshUrl",
            "return_url",
            "returnUrl",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            WebsiteId,
            RefreshUrl,
            ReturnUrl,
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
                            "refreshUrl" | "refresh_url" => Ok(GeneratedField::RefreshUrl),
                            "returnUrl" | "return_url" => Ok(GeneratedField::ReturnUrl),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateStripeAccountRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.commerce.v2.CreateStripeAccountRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateStripeAccountRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut website_id__ = None;
                let mut refresh_url__ = None;
                let mut return_url__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::WebsiteId => {
                            if website_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("websiteId"));
                            }
                            website_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::RefreshUrl => {
                            if refresh_url__.is_some() {
                                return Err(serde::de::Error::duplicate_field("refreshUrl"));
                            }
                            refresh_url__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ReturnUrl => {
                            if return_url__.is_some() {
                                return Err(serde::de::Error::duplicate_field("returnUrl"));
                            }
                            return_url__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(CreateStripeAccountRequest {
                    website_id: website_id__.unwrap_or_default(),
                    refresh_url: refresh_url__.unwrap_or_default(),
                    return_url: return_url__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sited_io.commerce.v2.CreateStripeAccountRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateStripeAccountResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.stripe_account.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sited_io.commerce.v2.CreateStripeAccountResponse", len)?;
        if let Some(v) = self.stripe_account.as_ref() {
            struct_ser.serialize_field("stripeAccount", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateStripeAccountResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "stripe_account",
            "stripeAccount",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            StripeAccount,
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
                            "stripeAccount" | "stripe_account" => Ok(GeneratedField::StripeAccount),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateStripeAccountResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.commerce.v2.CreateStripeAccountResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateStripeAccountResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut stripe_account__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::StripeAccount => {
                            if stripe_account__.is_some() {
                                return Err(serde::de::Error::duplicate_field("stripeAccount"));
                            }
                            stripe_account__ = map_.next_value()?;
                        }
                    }
                }
                Ok(CreateStripeAccountResponse {
                    stripe_account: stripe_account__,
                })
            }
        }
        deserializer.deserialize_struct("sited_io.commerce.v2.CreateStripeAccountResponse", FIELDS, GeneratedVisitor)
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
        let mut struct_ser = serializer.serialize_struct("sited_io.commerce.v2.DeleteOfferRequest", len)?;
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
                formatter.write_str("struct sited_io.commerce.v2.DeleteOfferRequest")
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
        deserializer.deserialize_struct("sited_io.commerce.v2.DeleteOfferRequest", FIELDS, GeneratedVisitor)
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
        let struct_ser = serializer.serialize_struct("sited_io.commerce.v2.DeleteOfferResponse", len)?;
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
                formatter.write_str("struct sited_io.commerce.v2.DeleteOfferResponse")
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
        deserializer.deserialize_struct("sited_io.commerce.v2.DeleteOfferResponse", FIELDS, GeneratedVisitor)
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
        let mut struct_ser = serializer.serialize_struct("sited_io.commerce.v2.DeleteShopRequest", len)?;
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
                formatter.write_str("struct sited_io.commerce.v2.DeleteShopRequest")
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
        deserializer.deserialize_struct("sited_io.commerce.v2.DeleteShopRequest", FIELDS, GeneratedVisitor)
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
        let struct_ser = serializer.serialize_struct("sited_io.commerce.v2.DeleteShopResponse", len)?;
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
                formatter.write_str("struct sited_io.commerce.v2.DeleteShopResponse")
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
        deserializer.deserialize_struct("sited_io.commerce.v2.DeleteShopResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DownloadFileRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.offer_file_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sited_io.commerce.v2.DownloadFileRequest", len)?;
        if !self.offer_file_id.is_empty() {
            struct_ser.serialize_field("offerFileId", &self.offer_file_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DownloadFileRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "offer_file_id",
            "offerFileId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            OfferFileId,
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
                            "offerFileId" | "offer_file_id" => Ok(GeneratedField::OfferFileId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DownloadFileRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.commerce.v2.DownloadFileRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DownloadFileRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut offer_file_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::OfferFileId => {
                            if offer_file_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("offerFileId"));
                            }
                            offer_file_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(DownloadFileRequest {
                    offer_file_id: offer_file_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sited_io.commerce.v2.DownloadFileRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DownloadFileResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.download_url.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sited_io.commerce.v2.DownloadFileResponse", len)?;
        if !self.download_url.is_empty() {
            struct_ser.serialize_field("downloadUrl", &self.download_url)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DownloadFileResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "download_url",
            "downloadUrl",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            DownloadUrl,
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
                            "downloadUrl" | "download_url" => Ok(GeneratedField::DownloadUrl),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DownloadFileResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.commerce.v2.DownloadFileResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DownloadFileResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut download_url__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::DownloadUrl => {
                            if download_url__.is_some() {
                                return Err(serde::de::Error::duplicate_field("downloadUrl"));
                            }
                            download_url__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(DownloadFileResponse {
                    download_url: download_url__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sited_io.commerce.v2.DownloadFileResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for FileFilter {
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
        let mut struct_ser = serializer.serialize_struct("sited_io.commerce.v2.FileFilter", len)?;
        if self.field != 0 {
            let v = FileFilterField::try_from(self.field)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.field)))?;
            struct_ser.serialize_field("field", &v)?;
        }
        if !self.query.is_empty() {
            struct_ser.serialize_field("query", &self.query)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for FileFilter {
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
            type Value = FileFilter;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.commerce.v2.FileFilter")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<FileFilter, V::Error>
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
                            field__ = Some(map_.next_value::<FileFilterField>()? as i32);
                        }
                        GeneratedField::Query => {
                            if query__.is_some() {
                                return Err(serde::de::Error::duplicate_field("query"));
                            }
                            query__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(FileFilter {
                    field: field__.unwrap_or_default(),
                    query: query__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sited_io.commerce.v2.FileFilter", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for FileFilterField {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "FILE_FILTER_FIELD_UNSPECIFIED",
            Self::Filename => "FILE_FILTER_FIELD_FILENAME",
            Self::OfferId => "FILE_FILTER_FIELD_OFFER_ID",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for FileFilterField {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "FILE_FILTER_FIELD_UNSPECIFIED",
            "FILE_FILTER_FIELD_FILENAME",
            "FILE_FILTER_FIELD_OFFER_ID",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = FileFilterField;

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
                    "FILE_FILTER_FIELD_UNSPECIFIED" => Ok(FileFilterField::Unspecified),
                    "FILE_FILTER_FIELD_FILENAME" => Ok(FileFilterField::Filename),
                    "FILE_FILTER_FIELD_OFFER_ID" => Ok(FileFilterField::OfferId),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for FileOrderBy {
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
        let mut struct_ser = serializer.serialize_struct("sited_io.commerce.v2.FileOrderBy", len)?;
        if self.field != 0 {
            let v = FileOrderByField::try_from(self.field)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.field)))?;
            struct_ser.serialize_field("field", &v)?;
        }
        if self.direction != 0 {
            let v = super::super::types::query::v1::Direction::try_from(self.direction)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.direction)))?;
            struct_ser.serialize_field("direction", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for FileOrderBy {
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
            type Value = FileOrderBy;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.commerce.v2.FileOrderBy")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<FileOrderBy, V::Error>
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
                            field__ = Some(map_.next_value::<FileOrderByField>()? as i32);
                        }
                        GeneratedField::Direction => {
                            if direction__.is_some() {
                                return Err(serde::de::Error::duplicate_field("direction"));
                            }
                            direction__ = Some(map_.next_value::<super::super::types::query::v1::Direction>()? as i32);
                        }
                    }
                }
                Ok(FileOrderBy {
                    field: field__.unwrap_or_default(),
                    direction: direction__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sited_io.commerce.v2.FileOrderBy", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for FileOrderByField {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "FILE_ORDER_BY_FIELD_UNSPECIFIED",
            Self::CreatedAt => "FILE_ORDER_BY_FIELD_CREATED_AT",
            Self::UpdatedAt => "FILE_ORDER_BY_FIELD_UPDATED_AT",
            Self::Ordering => "FILE_ORDER_BY_FIELD_ORDERING",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for FileOrderByField {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "FILE_ORDER_BY_FIELD_UNSPECIFIED",
            "FILE_ORDER_BY_FIELD_CREATED_AT",
            "FILE_ORDER_BY_FIELD_UPDATED_AT",
            "FILE_ORDER_BY_FIELD_ORDERING",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = FileOrderByField;

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
                    "FILE_ORDER_BY_FIELD_UNSPECIFIED" => Ok(FileOrderByField::Unspecified),
                    "FILE_ORDER_BY_FIELD_CREATED_AT" => Ok(FileOrderByField::CreatedAt),
                    "FILE_ORDER_BY_FIELD_UPDATED_AT" => Ok(FileOrderByField::UpdatedAt),
                    "FILE_ORDER_BY_FIELD_ORDERING" => Ok(FileOrderByField::Ordering),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
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
        let mut struct_ser = serializer.serialize_struct("sited_io.commerce.v2.GetOfferRequest", len)?;
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
                formatter.write_str("struct sited_io.commerce.v2.GetOfferRequest")
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
        deserializer.deserialize_struct("sited_io.commerce.v2.GetOfferRequest", FIELDS, GeneratedVisitor)
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
        let mut struct_ser = serializer.serialize_struct("sited_io.commerce.v2.GetOfferResponse", len)?;
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
                formatter.write_str("struct sited_io.commerce.v2.GetOfferResponse")
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
        deserializer.deserialize_struct("sited_io.commerce.v2.GetOfferResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetOrderRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.order_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sited_io.commerce.v2.GetOrderRequest", len)?;
        if !self.order_id.is_empty() {
            struct_ser.serialize_field("orderId", &self.order_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetOrderRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "order_id",
            "orderId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            OrderId,
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
                            "orderId" | "order_id" => Ok(GeneratedField::OrderId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetOrderRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.commerce.v2.GetOrderRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetOrderRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut order_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::OrderId => {
                            if order_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("orderId"));
                            }
                            order_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(GetOrderRequest {
                    order_id: order_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sited_io.commerce.v2.GetOrderRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetOrderResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.order.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sited_io.commerce.v2.GetOrderResponse", len)?;
        if let Some(v) = self.order.as_ref() {
            struct_ser.serialize_field("order", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetOrderResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "order",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Order,
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
                            "order" => Ok(GeneratedField::Order),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetOrderResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.commerce.v2.GetOrderResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetOrderResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut order__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Order => {
                            if order__.is_some() {
                                return Err(serde::de::Error::duplicate_field("order"));
                            }
                            order__ = map_.next_value()?;
                        }
                    }
                }
                Ok(GetOrderResponse {
                    order: order__,
                })
            }
        }
        deserializer.deserialize_struct("sited_io.commerce.v2.GetOrderResponse", FIELDS, GeneratedVisitor)
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
        if !self.shop_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sited_io.commerce.v2.GetShopRequest", len)?;
        if !self.shop_id.is_empty() {
            struct_ser.serialize_field("shopId", &self.shop_id)?;
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
            type Value = GetShopRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.commerce.v2.GetShopRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetShopRequest, V::Error>
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
                Ok(GetShopRequest {
                    shop_id: shop_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sited_io.commerce.v2.GetShopRequest", FIELDS, GeneratedVisitor)
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
        let mut struct_ser = serializer.serialize_struct("sited_io.commerce.v2.GetShopResponse", len)?;
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
                formatter.write_str("struct sited_io.commerce.v2.GetShopResponse")
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
        deserializer.deserialize_struct("sited_io.commerce.v2.GetShopResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetStripeAccountRequest {
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
        let mut struct_ser = serializer.serialize_struct("sited_io.commerce.v2.GetStripeAccountRequest", len)?;
        if !self.website_id.is_empty() {
            struct_ser.serialize_field("websiteId", &self.website_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetStripeAccountRequest {
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
            type Value = GetStripeAccountRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.commerce.v2.GetStripeAccountRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetStripeAccountRequest, V::Error>
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
                Ok(GetStripeAccountRequest {
                    website_id: website_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sited_io.commerce.v2.GetStripeAccountRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetStripeAccountResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.stripe_account.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sited_io.commerce.v2.GetStripeAccountResponse", len)?;
        if let Some(v) = self.stripe_account.as_ref() {
            struct_ser.serialize_field("stripeAccount", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetStripeAccountResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "stripe_account",
            "stripeAccount",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            StripeAccount,
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
                            "stripeAccount" | "stripe_account" => Ok(GeneratedField::StripeAccount),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetStripeAccountResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.commerce.v2.GetStripeAccountResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetStripeAccountResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut stripe_account__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::StripeAccount => {
                            if stripe_account__.is_some() {
                                return Err(serde::de::Error::duplicate_field("stripeAccount"));
                            }
                            stripe_account__ = map_.next_value()?;
                        }
                    }
                }
                Ok(GetStripeAccountResponse {
                    stripe_account: stripe_account__,
                })
            }
        }
        deserializer.deserialize_struct("sited_io.commerce.v2.GetStripeAccountResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for InitiateMultipartUploadRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.file_name.is_empty() {
            len += 1;
        }
        if !self.offer_id.is_empty() {
            len += 1;
        }
        if self.total_size_bytes != 0 {
            len += 1;
        }
        if self.content_type.is_some() {
            len += 1;
        }
        if self.ordering.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sited_io.commerce.v2.InitiateMultipartUploadRequest", len)?;
        if !self.file_name.is_empty() {
            struct_ser.serialize_field("fileName", &self.file_name)?;
        }
        if !self.offer_id.is_empty() {
            struct_ser.serialize_field("offerId", &self.offer_id)?;
        }
        if self.total_size_bytes != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("totalSizeBytes", ToString::to_string(&self.total_size_bytes).as_str())?;
        }
        if let Some(v) = self.content_type.as_ref() {
            struct_ser.serialize_field("contentType", v)?;
        }
        if let Some(v) = self.ordering.as_ref() {
            struct_ser.serialize_field("ordering", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for InitiateMultipartUploadRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "file_name",
            "fileName",
            "offer_id",
            "offerId",
            "total_size_bytes",
            "totalSizeBytes",
            "content_type",
            "contentType",
            "ordering",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            FileName,
            OfferId,
            TotalSizeBytes,
            ContentType,
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
                            "fileName" | "file_name" => Ok(GeneratedField::FileName),
                            "offerId" | "offer_id" => Ok(GeneratedField::OfferId),
                            "totalSizeBytes" | "total_size_bytes" => Ok(GeneratedField::TotalSizeBytes),
                            "contentType" | "content_type" => Ok(GeneratedField::ContentType),
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
            type Value = InitiateMultipartUploadRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.commerce.v2.InitiateMultipartUploadRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<InitiateMultipartUploadRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut file_name__ = None;
                let mut offer_id__ = None;
                let mut total_size_bytes__ = None;
                let mut content_type__ = None;
                let mut ordering__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::FileName => {
                            if file_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fileName"));
                            }
                            file_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::OfferId => {
                            if offer_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("offerId"));
                            }
                            offer_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::TotalSizeBytes => {
                            if total_size_bytes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("totalSizeBytes"));
                            }
                            total_size_bytes__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::ContentType => {
                            if content_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("contentType"));
                            }
                            content_type__ = map_.next_value()?;
                        }
                        GeneratedField::Ordering => {
                            if ordering__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ordering"));
                            }
                            ordering__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                    }
                }
                Ok(InitiateMultipartUploadRequest {
                    file_name: file_name__.unwrap_or_default(),
                    offer_id: offer_id__.unwrap_or_default(),
                    total_size_bytes: total_size_bytes__.unwrap_or_default(),
                    content_type: content_type__,
                    ordering: ordering__,
                })
            }
        }
        deserializer.deserialize_struct("sited_io.commerce.v2.InitiateMultipartUploadRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for InitiateMultipartUploadResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.offer_file_id.is_empty() {
            len += 1;
        }
        if !self.key.is_empty() {
            len += 1;
        }
        if !self.upload_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sited_io.commerce.v2.InitiateMultipartUploadResponse", len)?;
        if !self.offer_file_id.is_empty() {
            struct_ser.serialize_field("offerFileId", &self.offer_file_id)?;
        }
        if !self.key.is_empty() {
            struct_ser.serialize_field("key", &self.key)?;
        }
        if !self.upload_id.is_empty() {
            struct_ser.serialize_field("uploadId", &self.upload_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for InitiateMultipartUploadResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "offer_file_id",
            "offerFileId",
            "key",
            "upload_id",
            "uploadId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            OfferFileId,
            Key,
            UploadId,
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
                            "offerFileId" | "offer_file_id" => Ok(GeneratedField::OfferFileId),
                            "key" => Ok(GeneratedField::Key),
                            "uploadId" | "upload_id" => Ok(GeneratedField::UploadId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = InitiateMultipartUploadResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.commerce.v2.InitiateMultipartUploadResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<InitiateMultipartUploadResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut offer_file_id__ = None;
                let mut key__ = None;
                let mut upload_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::OfferFileId => {
                            if offer_file_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("offerFileId"));
                            }
                            offer_file_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Key => {
                            if key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("key"));
                            }
                            key__ = Some(map_.next_value()?);
                        }
                        GeneratedField::UploadId => {
                            if upload_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("uploadId"));
                            }
                            upload_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(InitiateMultipartUploadResponse {
                    offer_file_id: offer_file_id__.unwrap_or_default(),
                    key: key__.unwrap_or_default(),
                    upload_id: upload_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sited_io.commerce.v2.InitiateMultipartUploadResponse", FIELDS, GeneratedVisitor)
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
        if self.owner.is_some() {
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
        let mut struct_ser = serializer.serialize_struct("sited_io.commerce.v2.ListOffersRequest", len)?;
        if let Some(v) = self.owner.as_ref() {
            struct_ser.serialize_field("owner", v)?;
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
            "owner",
            "shop_id",
            "shopId",
            "pagination",
            "order_by",
            "orderBy",
            "filter",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Owner,
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
                            "owner" => Ok(GeneratedField::Owner),
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
                formatter.write_str("struct sited_io.commerce.v2.ListOffersRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListOffersRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut owner__ = None;
                let mut shop_id__ = None;
                let mut pagination__ = None;
                let mut order_by__ = None;
                let mut filter__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Owner => {
                            if owner__.is_some() {
                                return Err(serde::de::Error::duplicate_field("owner"));
                            }
                            owner__ = map_.next_value()?;
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
                    owner: owner__,
                    shop_id: shop_id__,
                    pagination: pagination__,
                    order_by: order_by__,
                    filter: filter__,
                })
            }
        }
        deserializer.deserialize_struct("sited_io.commerce.v2.ListOffersRequest", FIELDS, GeneratedVisitor)
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
        let mut struct_ser = serializer.serialize_struct("sited_io.commerce.v2.ListOffersResponse", len)?;
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
                formatter.write_str("struct sited_io.commerce.v2.ListOffersResponse")
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
        deserializer.deserialize_struct("sited_io.commerce.v2.ListOffersResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListOrdersRequest {
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
        let mut struct_ser = serializer.serialize_struct("sited_io.commerce.v2.ListOrdersRequest", len)?;
        if let Some(v) = self.offer_id.as_ref() {
            struct_ser.serialize_field("offerId", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListOrdersRequest {
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
            type Value = ListOrdersRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.commerce.v2.ListOrdersRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListOrdersRequest, V::Error>
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
                Ok(ListOrdersRequest {
                    offer_id: offer_id__,
                })
            }
        }
        deserializer.deserialize_struct("sited_io.commerce.v2.ListOrdersRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListOrdersResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.orders.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sited_io.commerce.v2.ListOrdersResponse", len)?;
        if !self.orders.is_empty() {
            struct_ser.serialize_field("orders", &self.orders)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListOrdersResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "orders",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Orders,
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
                            "orders" => Ok(GeneratedField::Orders),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListOrdersResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.commerce.v2.ListOrdersResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListOrdersResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut orders__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Orders => {
                            if orders__.is_some() {
                                return Err(serde::de::Error::duplicate_field("orders"));
                            }
                            orders__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ListOrdersResponse {
                    orders: orders__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sited_io.commerce.v2.ListOrdersResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MultipartPart {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.part_number != 0 {
            len += 1;
        }
        if !self.etag.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sited_io.commerce.v2.MultipartPart", len)?;
        if self.part_number != 0 {
            struct_ser.serialize_field("partNumber", &self.part_number)?;
        }
        if !self.etag.is_empty() {
            struct_ser.serialize_field("etag", &self.etag)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MultipartPart {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "part_number",
            "partNumber",
            "etag",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PartNumber,
            Etag,
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
                            "partNumber" | "part_number" => Ok(GeneratedField::PartNumber),
                            "etag" => Ok(GeneratedField::Etag),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MultipartPart;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.commerce.v2.MultipartPart")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MultipartPart, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut part_number__ = None;
                let mut etag__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::PartNumber => {
                            if part_number__.is_some() {
                                return Err(serde::de::Error::duplicate_field("partNumber"));
                            }
                            part_number__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Etag => {
                            if etag__.is_some() {
                                return Err(serde::de::Error::duplicate_field("etag"));
                            }
                            etag__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(MultipartPart {
                    part_number: part_number__.unwrap_or_default(),
                    etag: etag__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sited_io.commerce.v2.MultipartPart", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Offer {
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
        if !self.owner.is_empty() {
            len += 1;
        }
        if self.created_at != 0 {
            len += 1;
        }
        if self.updated_at != 0 {
            len += 1;
        }
        if self.details.is_some() {
            len += 1;
        }
        if self.offer_type.is_some() {
            len += 1;
        }
        if self.price.is_some() {
            len += 1;
        }
        if self.shipping_rate.is_some() {
            len += 1;
        }
        if !self.images.is_empty() {
            len += 1;
        }
        if !self.files.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sited_io.commerce.v2.Offer", len)?;
        if !self.offer_id.is_empty() {
            struct_ser.serialize_field("offerId", &self.offer_id)?;
        }
        if !self.owner.is_empty() {
            struct_ser.serialize_field("owner", &self.owner)?;
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
        if let Some(v) = self.details.as_ref() {
            struct_ser.serialize_field("details", v)?;
        }
        if let Some(v) = self.offer_type.as_ref() {
            struct_ser.serialize_field("offerType", v)?;
        }
        if let Some(v) = self.price.as_ref() {
            struct_ser.serialize_field("price", v)?;
        }
        if let Some(v) = self.shipping_rate.as_ref() {
            struct_ser.serialize_field("shippingRate", v)?;
        }
        if !self.images.is_empty() {
            struct_ser.serialize_field("images", &self.images)?;
        }
        if !self.files.is_empty() {
            struct_ser.serialize_field("files", &self.files)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Offer {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "offer_id",
            "offerId",
            "owner",
            "created_at",
            "createdAt",
            "updated_at",
            "updatedAt",
            "details",
            "offer_type",
            "offerType",
            "price",
            "shipping_rate",
            "shippingRate",
            "images",
            "files",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            OfferId,
            Owner,
            CreatedAt,
            UpdatedAt,
            Details,
            OfferType,
            Price,
            ShippingRate,
            Images,
            Files,
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
                            "owner" => Ok(GeneratedField::Owner),
                            "createdAt" | "created_at" => Ok(GeneratedField::CreatedAt),
                            "updatedAt" | "updated_at" => Ok(GeneratedField::UpdatedAt),
                            "details" => Ok(GeneratedField::Details),
                            "offerType" | "offer_type" => Ok(GeneratedField::OfferType),
                            "price" => Ok(GeneratedField::Price),
                            "shippingRate" | "shipping_rate" => Ok(GeneratedField::ShippingRate),
                            "images" => Ok(GeneratedField::Images),
                            "files" => Ok(GeneratedField::Files),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Offer;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.commerce.v2.Offer")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Offer, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut offer_id__ = None;
                let mut owner__ = None;
                let mut created_at__ = None;
                let mut updated_at__ = None;
                let mut details__ = None;
                let mut offer_type__ = None;
                let mut price__ = None;
                let mut shipping_rate__ = None;
                let mut images__ = None;
                let mut files__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::OfferId => {
                            if offer_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("offerId"));
                            }
                            offer_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Owner => {
                            if owner__.is_some() {
                                return Err(serde::de::Error::duplicate_field("owner"));
                            }
                            owner__ = Some(map_.next_value()?);
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
                        GeneratedField::Details => {
                            if details__.is_some() {
                                return Err(serde::de::Error::duplicate_field("details"));
                            }
                            details__ = map_.next_value()?;
                        }
                        GeneratedField::OfferType => {
                            if offer_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("offerType"));
                            }
                            offer_type__ = map_.next_value()?;
                        }
                        GeneratedField::Price => {
                            if price__.is_some() {
                                return Err(serde::de::Error::duplicate_field("price"));
                            }
                            price__ = map_.next_value()?;
                        }
                        GeneratedField::ShippingRate => {
                            if shipping_rate__.is_some() {
                                return Err(serde::de::Error::duplicate_field("shippingRate"));
                            }
                            shipping_rate__ = map_.next_value()?;
                        }
                        GeneratedField::Images => {
                            if images__.is_some() {
                                return Err(serde::de::Error::duplicate_field("images"));
                            }
                            images__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Files => {
                            if files__.is_some() {
                                return Err(serde::de::Error::duplicate_field("files"));
                            }
                            files__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(Offer {
                    offer_id: offer_id__.unwrap_or_default(),
                    owner: owner__.unwrap_or_default(),
                    created_at: created_at__.unwrap_or_default(),
                    updated_at: updated_at__.unwrap_or_default(),
                    details: details__,
                    offer_type: offer_type__,
                    price: price__,
                    shipping_rate: shipping_rate__,
                    images: images__.unwrap_or_default(),
                    files: files__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sited_io.commerce.v2.Offer", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for offer::Details {
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
        if self.description.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sited_io.commerce.v2.Offer.Details", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if let Some(v) = self.description.as_ref() {
            struct_ser.serialize_field("description", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for offer::Details {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "description",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            Description,
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
                            "description" => Ok(GeneratedField::Description),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = offer::Details;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.commerce.v2.Offer.Details")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<offer::Details, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut description__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
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
                    }
                }
                Ok(offer::Details {
                    name: name__.unwrap_or_default(),
                    description: description__,
                })
            }
        }
        deserializer.deserialize_struct("sited_io.commerce.v2.Offer.Details", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for OfferFile {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.offer_file_id.is_empty() {
            len += 1;
        }
        if !self.offer_id.is_empty() {
            len += 1;
        }
        if !self.owner.is_empty() {
            len += 1;
        }
        if !self.file_name.is_empty() {
            len += 1;
        }
        if self.content_type.is_some() {
            len += 1;
        }
        if self.total_size_bytes != 0 {
            len += 1;
        }
        if self.uploaded_size_bytes != 0 {
            len += 1;
        }
        if self.ordering != 0 {
            len += 1;
        }
        if !self.file_url.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sited_io.commerce.v2.OfferFile", len)?;
        if !self.offer_file_id.is_empty() {
            struct_ser.serialize_field("offerFileId", &self.offer_file_id)?;
        }
        if !self.offer_id.is_empty() {
            struct_ser.serialize_field("offerId", &self.offer_id)?;
        }
        if !self.owner.is_empty() {
            struct_ser.serialize_field("owner", &self.owner)?;
        }
        if !self.file_name.is_empty() {
            struct_ser.serialize_field("fileName", &self.file_name)?;
        }
        if let Some(v) = self.content_type.as_ref() {
            struct_ser.serialize_field("contentType", v)?;
        }
        if self.total_size_bytes != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("totalSizeBytes", ToString::to_string(&self.total_size_bytes).as_str())?;
        }
        if self.uploaded_size_bytes != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("uploadedSizeBytes", ToString::to_string(&self.uploaded_size_bytes).as_str())?;
        }
        if self.ordering != 0 {
            struct_ser.serialize_field("ordering", &self.ordering)?;
        }
        if !self.file_url.is_empty() {
            struct_ser.serialize_field("fileUrl", &self.file_url)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for OfferFile {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "offer_file_id",
            "offerFileId",
            "offer_id",
            "offerId",
            "owner",
            "file_name",
            "fileName",
            "content_type",
            "contentType",
            "total_size_bytes",
            "totalSizeBytes",
            "uploaded_size_bytes",
            "uploadedSizeBytes",
            "ordering",
            "file_url",
            "fileUrl",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            OfferFileId,
            OfferId,
            Owner,
            FileName,
            ContentType,
            TotalSizeBytes,
            UploadedSizeBytes,
            Ordering,
            FileUrl,
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
                            "offerFileId" | "offer_file_id" => Ok(GeneratedField::OfferFileId),
                            "offerId" | "offer_id" => Ok(GeneratedField::OfferId),
                            "owner" => Ok(GeneratedField::Owner),
                            "fileName" | "file_name" => Ok(GeneratedField::FileName),
                            "contentType" | "content_type" => Ok(GeneratedField::ContentType),
                            "totalSizeBytes" | "total_size_bytes" => Ok(GeneratedField::TotalSizeBytes),
                            "uploadedSizeBytes" | "uploaded_size_bytes" => Ok(GeneratedField::UploadedSizeBytes),
                            "ordering" => Ok(GeneratedField::Ordering),
                            "fileUrl" | "file_url" => Ok(GeneratedField::FileUrl),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = OfferFile;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.commerce.v2.OfferFile")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<OfferFile, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut offer_file_id__ = None;
                let mut offer_id__ = None;
                let mut owner__ = None;
                let mut file_name__ = None;
                let mut content_type__ = None;
                let mut total_size_bytes__ = None;
                let mut uploaded_size_bytes__ = None;
                let mut ordering__ = None;
                let mut file_url__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::OfferFileId => {
                            if offer_file_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("offerFileId"));
                            }
                            offer_file_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::OfferId => {
                            if offer_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("offerId"));
                            }
                            offer_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Owner => {
                            if owner__.is_some() {
                                return Err(serde::de::Error::duplicate_field("owner"));
                            }
                            owner__ = Some(map_.next_value()?);
                        }
                        GeneratedField::FileName => {
                            if file_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fileName"));
                            }
                            file_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ContentType => {
                            if content_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("contentType"));
                            }
                            content_type__ = map_.next_value()?;
                        }
                        GeneratedField::TotalSizeBytes => {
                            if total_size_bytes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("totalSizeBytes"));
                            }
                            total_size_bytes__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::UploadedSizeBytes => {
                            if uploaded_size_bytes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("uploadedSizeBytes"));
                            }
                            uploaded_size_bytes__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Ordering => {
                            if ordering__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ordering"));
                            }
                            ordering__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::FileUrl => {
                            if file_url__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fileUrl"));
                            }
                            file_url__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(OfferFile {
                    offer_file_id: offer_file_id__.unwrap_or_default(),
                    offer_id: offer_id__.unwrap_or_default(),
                    owner: owner__.unwrap_or_default(),
                    file_name: file_name__.unwrap_or_default(),
                    content_type: content_type__,
                    total_size_bytes: total_size_bytes__.unwrap_or_default(),
                    uploaded_size_bytes: uploaded_size_bytes__.unwrap_or_default(),
                    ordering: ordering__.unwrap_or_default(),
                    file_url: file_url__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sited_io.commerce.v2.OfferFile", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for OfferImage {
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
        if !self.owner.is_empty() {
            len += 1;
        }
        if !self.image_url.is_empty() {
            len += 1;
        }
        if self.ordering != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sited_io.commerce.v2.OfferImage", len)?;
        if !self.offer_image_id.is_empty() {
            struct_ser.serialize_field("offerImageId", &self.offer_image_id)?;
        }
        if !self.owner.is_empty() {
            struct_ser.serialize_field("owner", &self.owner)?;
        }
        if !self.image_url.is_empty() {
            struct_ser.serialize_field("imageUrl", &self.image_url)?;
        }
        if self.ordering != 0 {
            struct_ser.serialize_field("ordering", &self.ordering)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for OfferImage {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "offer_image_id",
            "offerImageId",
            "owner",
            "image_url",
            "imageUrl",
            "ordering",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            OfferImageId,
            Owner,
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
                            "owner" => Ok(GeneratedField::Owner),
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
            type Value = OfferImage;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.commerce.v2.OfferImage")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<OfferImage, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut offer_image_id__ = None;
                let mut owner__ = None;
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
                        GeneratedField::Owner => {
                            if owner__.is_some() {
                                return Err(serde::de::Error::duplicate_field("owner"));
                            }
                            owner__ = Some(map_.next_value()?);
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
                Ok(OfferImage {
                    offer_image_id: offer_image_id__.unwrap_or_default(),
                    owner: owner__.unwrap_or_default(),
                    image_url: image_url__.unwrap_or_default(),
                    ordering: ordering__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sited_io.commerce.v2.OfferImage", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for OfferPrice {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.unit_amount != 0 {
            len += 1;
        }
        if self.currency != 0 {
            len += 1;
        }
        if self.price_type.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sited_io.commerce.v2.OfferPrice", len)?;
        if self.unit_amount != 0 {
            struct_ser.serialize_field("unitAmount", &self.unit_amount)?;
        }
        if self.currency != 0 {
            let v = super::super::types::currency::v1::CurrencyCode::try_from(self.currency)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.currency)))?;
            struct_ser.serialize_field("currency", &v)?;
        }
        if let Some(v) = self.price_type.as_ref() {
            struct_ser.serialize_field("priceType", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for OfferPrice {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "unit_amount",
            "unitAmount",
            "currency",
            "price_type",
            "priceType",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            UnitAmount,
            Currency,
            PriceType,
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
                            "unitAmount" | "unit_amount" => Ok(GeneratedField::UnitAmount),
                            "currency" => Ok(GeneratedField::Currency),
                            "priceType" | "price_type" => Ok(GeneratedField::PriceType),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = OfferPrice;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.commerce.v2.OfferPrice")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<OfferPrice, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut unit_amount__ = None;
                let mut currency__ = None;
                let mut price_type__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::UnitAmount => {
                            if unit_amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("unitAmount"));
                            }
                            unit_amount__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Currency => {
                            if currency__.is_some() {
                                return Err(serde::de::Error::duplicate_field("currency"));
                            }
                            currency__ = Some(map_.next_value::<super::super::types::currency::v1::CurrencyCode>()? as i32);
                        }
                        GeneratedField::PriceType => {
                            if price_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("priceType"));
                            }
                            price_type__ = map_.next_value()?;
                        }
                    }
                }
                Ok(OfferPrice {
                    unit_amount: unit_amount__.unwrap_or_default(),
                    currency: currency__.unwrap_or_default(),
                    price_type: price_type__,
                })
            }
        }
        deserializer.deserialize_struct("sited_io.commerce.v2.OfferPrice", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for OfferType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.offer_type_kind.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sited_io.commerce.v2.OfferType", len)?;
        if let Some(v) = self.offer_type_kind.as_ref() {
            match v {
                offer_type::OfferTypeKind::Physical(v) => {
                    struct_ser.serialize_field("physical", v)?;
                }
                offer_type::OfferTypeKind::Digital(v) => {
                    struct_ser.serialize_field("digital", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for OfferType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "physical",
            "digital",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Physical,
            Digital,
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
                            "physical" => Ok(GeneratedField::Physical),
                            "digital" => Ok(GeneratedField::Digital),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = OfferType;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.commerce.v2.OfferType")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<OfferType, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut offer_type_kind__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Physical => {
                            if offer_type_kind__.is_some() {
                                return Err(serde::de::Error::duplicate_field("physical"));
                            }
                            offer_type_kind__ = map_.next_value::<::std::option::Option<_>>()?.map(offer_type::OfferTypeKind::Physical)
;
                        }
                        GeneratedField::Digital => {
                            if offer_type_kind__.is_some() {
                                return Err(serde::de::Error::duplicate_field("digital"));
                            }
                            offer_type_kind__ = map_.next_value::<::std::option::Option<_>>()?.map(offer_type::OfferTypeKind::Digital)
;
                        }
                    }
                }
                Ok(OfferType {
                    offer_type_kind: offer_type_kind__,
                })
            }
        }
        deserializer.deserialize_struct("sited_io.commerce.v2.OfferType", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for offer_type::Digital {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("sited_io.commerce.v2.OfferType.Digital", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for offer_type::Digital {
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
            type Value = offer_type::Digital;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.commerce.v2.OfferType.Digital")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<offer_type::Digital, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(offer_type::Digital {
                })
            }
        }
        deserializer.deserialize_struct("sited_io.commerce.v2.OfferType.Digital", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for offer_type::Physical {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("sited_io.commerce.v2.OfferType.Physical", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for offer_type::Physical {
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
            type Value = offer_type::Physical;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.commerce.v2.OfferType.Physical")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<offer_type::Physical, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(offer_type::Physical {
                })
            }
        }
        deserializer.deserialize_struct("sited_io.commerce.v2.OfferType.Physical", FIELDS, GeneratedVisitor)
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
        let mut struct_ser = serializer.serialize_struct("sited_io.commerce.v2.OffersFilter", len)?;
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
                formatter.write_str("struct sited_io.commerce.v2.OffersFilter")
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
        deserializer.deserialize_struct("sited_io.commerce.v2.OffersFilter", FIELDS, GeneratedVisitor)
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
        let mut struct_ser = serializer.serialize_struct("sited_io.commerce.v2.OffersOrderBy", len)?;
        if self.field != 0 {
            let v = OffersOrderByField::try_from(self.field)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.field)))?;
            struct_ser.serialize_field("field", &v)?;
        }
        if self.direction != 0 {
            let v = super::super::types::query::v1::Direction::try_from(self.direction)
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
                formatter.write_str("struct sited_io.commerce.v2.OffersOrderBy")
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
                            direction__ = Some(map_.next_value::<super::super::types::query::v1::Direction>()? as i32);
                        }
                    }
                }
                Ok(OffersOrderBy {
                    field: field__.unwrap_or_default(),
                    direction: direction__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sited_io.commerce.v2.OffersOrderBy", FIELDS, GeneratedVisitor)
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
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for Order {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.order_id.is_empty() {
            len += 1;
        }
        if !self.offer_id.is_empty() {
            len += 1;
        }
        if self.buyer_user_id.is_some() {
            len += 1;
        }
        if self.created_at != 0 {
            len += 1;
        }
        if self.updated_at != 0 {
            len += 1;
        }
        if self.order_type.is_some() {
            len += 1;
        }
        if self.payment_method.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sited_io.commerce.v2.Order", len)?;
        if !self.order_id.is_empty() {
            struct_ser.serialize_field("orderId", &self.order_id)?;
        }
        if !self.offer_id.is_empty() {
            struct_ser.serialize_field("offerId", &self.offer_id)?;
        }
        if let Some(v) = self.buyer_user_id.as_ref() {
            struct_ser.serialize_field("buyerUserId", v)?;
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
        if let Some(v) = self.order_type.as_ref() {
            struct_ser.serialize_field("orderType", v)?;
        }
        if let Some(v) = self.payment_method.as_ref() {
            struct_ser.serialize_field("paymentMethod", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Order {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "order_id",
            "orderId",
            "offer_id",
            "offerId",
            "buyer_user_id",
            "buyerUserId",
            "created_at",
            "createdAt",
            "updated_at",
            "updatedAt",
            "order_type",
            "orderType",
            "payment_method",
            "paymentMethod",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            OrderId,
            OfferId,
            BuyerUserId,
            CreatedAt,
            UpdatedAt,
            OrderType,
            PaymentMethod,
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
                            "orderId" | "order_id" => Ok(GeneratedField::OrderId),
                            "offerId" | "offer_id" => Ok(GeneratedField::OfferId),
                            "buyerUserId" | "buyer_user_id" => Ok(GeneratedField::BuyerUserId),
                            "createdAt" | "created_at" => Ok(GeneratedField::CreatedAt),
                            "updatedAt" | "updated_at" => Ok(GeneratedField::UpdatedAt),
                            "orderType" | "order_type" => Ok(GeneratedField::OrderType),
                            "paymentMethod" | "payment_method" => Ok(GeneratedField::PaymentMethod),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Order;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.commerce.v2.Order")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Order, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut order_id__ = None;
                let mut offer_id__ = None;
                let mut buyer_user_id__ = None;
                let mut created_at__ = None;
                let mut updated_at__ = None;
                let mut order_type__ = None;
                let mut payment_method__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::OrderId => {
                            if order_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("orderId"));
                            }
                            order_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::OfferId => {
                            if offer_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("offerId"));
                            }
                            offer_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::BuyerUserId => {
                            if buyer_user_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("buyerUserId"));
                            }
                            buyer_user_id__ = map_.next_value()?;
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
                        GeneratedField::OrderType => {
                            if order_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("orderType"));
                            }
                            order_type__ = map_.next_value()?;
                        }
                        GeneratedField::PaymentMethod => {
                            if payment_method__.is_some() {
                                return Err(serde::de::Error::duplicate_field("paymentMethod"));
                            }
                            payment_method__ = map_.next_value()?;
                        }
                    }
                }
                Ok(Order {
                    order_id: order_id__.unwrap_or_default(),
                    offer_id: offer_id__.unwrap_or_default(),
                    buyer_user_id: buyer_user_id__,
                    created_at: created_at__.unwrap_or_default(),
                    updated_at: updated_at__.unwrap_or_default(),
                    order_type: order_type__,
                    payment_method: payment_method__,
                })
            }
        }
        deserializer.deserialize_struct("sited_io.commerce.v2.Order", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for OrderType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.order_type_kind.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sited_io.commerce.v2.OrderType", len)?;
        if let Some(v) = self.order_type_kind.as_ref() {
            match v {
                order_type::OrderTypeKind::OneOff(v) => {
                    struct_ser.serialize_field("oneOff", v)?;
                }
                order_type::OrderTypeKind::Subscription(v) => {
                    struct_ser.serialize_field("subscription", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for OrderType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "one_off",
            "oneOff",
            "subscription",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            OneOff,
            Subscription,
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
                            "oneOff" | "one_off" => Ok(GeneratedField::OneOff),
                            "subscription" => Ok(GeneratedField::Subscription),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = OrderType;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.commerce.v2.OrderType")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<OrderType, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut order_type_kind__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::OneOff => {
                            if order_type_kind__.is_some() {
                                return Err(serde::de::Error::duplicate_field("oneOff"));
                            }
                            order_type_kind__ = map_.next_value::<::std::option::Option<_>>()?.map(order_type::OrderTypeKind::OneOff)
;
                        }
                        GeneratedField::Subscription => {
                            if order_type_kind__.is_some() {
                                return Err(serde::de::Error::duplicate_field("subscription"));
                            }
                            order_type_kind__ = map_.next_value::<::std::option::Option<_>>()?.map(order_type::OrderTypeKind::Subscription)
;
                        }
                    }
                }
                Ok(OrderType {
                    order_type_kind: order_type_kind__,
                })
            }
        }
        deserializer.deserialize_struct("sited_io.commerce.v2.OrderType", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for order_type::OneOff {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.payed_at.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sited_io.commerce.v2.OrderType.OneOff", len)?;
        if let Some(v) = self.payed_at.as_ref() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("payedAt", ToString::to_string(&v).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for order_type::OneOff {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "payed_at",
            "payedAt",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PayedAt,
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
                            "payedAt" | "payed_at" => Ok(GeneratedField::PayedAt),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = order_type::OneOff;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.commerce.v2.OrderType.OneOff")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<order_type::OneOff, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut payed_at__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::PayedAt => {
                            if payed_at__.is_some() {
                                return Err(serde::de::Error::duplicate_field("payedAt"));
                            }
                            payed_at__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                    }
                }
                Ok(order_type::OneOff {
                    payed_at: payed_at__,
                })
            }
        }
        deserializer.deserialize_struct("sited_io.commerce.v2.OrderType.OneOff", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for order_type::Subscription {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.current_period_start != 0 {
            len += 1;
        }
        if self.current_period_end != 0 {
            len += 1;
        }
        if !self.status.is_empty() {
            len += 1;
        }
        if self.payed_at.is_some() {
            len += 1;
        }
        if self.payed_until.is_some() {
            len += 1;
        }
        if self.canceled_at.is_some() {
            len += 1;
        }
        if self.cancel_at.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sited_io.commerce.v2.OrderType.Subscription", len)?;
        if self.current_period_start != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("currentPeriodStart", ToString::to_string(&self.current_period_start).as_str())?;
        }
        if self.current_period_end != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("currentPeriodEnd", ToString::to_string(&self.current_period_end).as_str())?;
        }
        if !self.status.is_empty() {
            struct_ser.serialize_field("status", &self.status)?;
        }
        if let Some(v) = self.payed_at.as_ref() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("payedAt", ToString::to_string(&v).as_str())?;
        }
        if let Some(v) = self.payed_until.as_ref() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("payedUntil", ToString::to_string(&v).as_str())?;
        }
        if let Some(v) = self.canceled_at.as_ref() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("canceledAt", ToString::to_string(&v).as_str())?;
        }
        if let Some(v) = self.cancel_at.as_ref() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("cancelAt", ToString::to_string(&v).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for order_type::Subscription {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "current_period_start",
            "currentPeriodStart",
            "current_period_end",
            "currentPeriodEnd",
            "status",
            "payed_at",
            "payedAt",
            "payed_until",
            "payedUntil",
            "canceled_at",
            "canceledAt",
            "cancel_at",
            "cancelAt",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            CurrentPeriodStart,
            CurrentPeriodEnd,
            Status,
            PayedAt,
            PayedUntil,
            CanceledAt,
            CancelAt,
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
                            "currentPeriodStart" | "current_period_start" => Ok(GeneratedField::CurrentPeriodStart),
                            "currentPeriodEnd" | "current_period_end" => Ok(GeneratedField::CurrentPeriodEnd),
                            "status" => Ok(GeneratedField::Status),
                            "payedAt" | "payed_at" => Ok(GeneratedField::PayedAt),
                            "payedUntil" | "payed_until" => Ok(GeneratedField::PayedUntil),
                            "canceledAt" | "canceled_at" => Ok(GeneratedField::CanceledAt),
                            "cancelAt" | "cancel_at" => Ok(GeneratedField::CancelAt),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = order_type::Subscription;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.commerce.v2.OrderType.Subscription")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<order_type::Subscription, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut current_period_start__ = None;
                let mut current_period_end__ = None;
                let mut status__ = None;
                let mut payed_at__ = None;
                let mut payed_until__ = None;
                let mut canceled_at__ = None;
                let mut cancel_at__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::CurrentPeriodStart => {
                            if current_period_start__.is_some() {
                                return Err(serde::de::Error::duplicate_field("currentPeriodStart"));
                            }
                            current_period_start__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::CurrentPeriodEnd => {
                            if current_period_end__.is_some() {
                                return Err(serde::de::Error::duplicate_field("currentPeriodEnd"));
                            }
                            current_period_end__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Status => {
                            if status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            status__ = Some(map_.next_value()?);
                        }
                        GeneratedField::PayedAt => {
                            if payed_at__.is_some() {
                                return Err(serde::de::Error::duplicate_field("payedAt"));
                            }
                            payed_at__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::PayedUntil => {
                            if payed_until__.is_some() {
                                return Err(serde::de::Error::duplicate_field("payedUntil"));
                            }
                            payed_until__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::CanceledAt => {
                            if canceled_at__.is_some() {
                                return Err(serde::de::Error::duplicate_field("canceledAt"));
                            }
                            canceled_at__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::CancelAt => {
                            if cancel_at__.is_some() {
                                return Err(serde::de::Error::duplicate_field("cancelAt"));
                            }
                            cancel_at__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                    }
                }
                Ok(order_type::Subscription {
                    current_period_start: current_period_start__.unwrap_or_default(),
                    current_period_end: current_period_end__.unwrap_or_default(),
                    status: status__.unwrap_or_default(),
                    payed_at: payed_at__,
                    payed_until: payed_until__,
                    canceled_at: canceled_at__,
                    cancel_at: cancel_at__,
                })
            }
        }
        deserializer.deserialize_struct("sited_io.commerce.v2.OrderType.Subscription", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Payment {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.order_id.is_empty() {
            len += 1;
        }
        if !self.offer_id.is_empty() {
            len += 1;
        }
        if self.buyer_user_id.is_some() {
            len += 1;
        }
        if self.order_type.is_some() {
            len += 1;
        }
        if self.payment_method.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sited_io.commerce.v2.Payment", len)?;
        if !self.order_id.is_empty() {
            struct_ser.serialize_field("orderId", &self.order_id)?;
        }
        if !self.offer_id.is_empty() {
            struct_ser.serialize_field("offerId", &self.offer_id)?;
        }
        if let Some(v) = self.buyer_user_id.as_ref() {
            struct_ser.serialize_field("buyerUserId", v)?;
        }
        if let Some(v) = self.order_type.as_ref() {
            struct_ser.serialize_field("orderType", v)?;
        }
        if let Some(v) = self.payment_method.as_ref() {
            struct_ser.serialize_field("paymentMethod", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Payment {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "order_id",
            "orderId",
            "offer_id",
            "offerId",
            "buyer_user_id",
            "buyerUserId",
            "order_type",
            "orderType",
            "payment_method",
            "paymentMethod",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            OrderId,
            OfferId,
            BuyerUserId,
            OrderType,
            PaymentMethod,
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
                            "orderId" | "order_id" => Ok(GeneratedField::OrderId),
                            "offerId" | "offer_id" => Ok(GeneratedField::OfferId),
                            "buyerUserId" | "buyer_user_id" => Ok(GeneratedField::BuyerUserId),
                            "orderType" | "order_type" => Ok(GeneratedField::OrderType),
                            "paymentMethod" | "payment_method" => Ok(GeneratedField::PaymentMethod),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Payment;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.commerce.v2.Payment")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Payment, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut order_id__ = None;
                let mut offer_id__ = None;
                let mut buyer_user_id__ = None;
                let mut order_type__ = None;
                let mut payment_method__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::OrderId => {
                            if order_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("orderId"));
                            }
                            order_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::OfferId => {
                            if offer_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("offerId"));
                            }
                            offer_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::BuyerUserId => {
                            if buyer_user_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("buyerUserId"));
                            }
                            buyer_user_id__ = map_.next_value()?;
                        }
                        GeneratedField::OrderType => {
                            if order_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("orderType"));
                            }
                            order_type__ = map_.next_value()?;
                        }
                        GeneratedField::PaymentMethod => {
                            if payment_method__.is_some() {
                                return Err(serde::de::Error::duplicate_field("paymentMethod"));
                            }
                            payment_method__ = map_.next_value()?;
                        }
                    }
                }
                Ok(Payment {
                    order_id: order_id__.unwrap_or_default(),
                    offer_id: offer_id__.unwrap_or_default(),
                    buyer_user_id: buyer_user_id__,
                    order_type: order_type__,
                    payment_method: payment_method__,
                })
            }
        }
        deserializer.deserialize_struct("sited_io.commerce.v2.Payment", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PaymentMethod {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.payment_method_kind.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sited_io.commerce.v2.PaymentMethod", len)?;
        if let Some(v) = self.payment_method_kind.as_ref() {
            match v {
                payment_method::PaymentMethodKind::Stripe(v) => {
                    struct_ser.serialize_field("stripe", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PaymentMethod {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "stripe",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Stripe,
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
                            "stripe" => Ok(GeneratedField::Stripe),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PaymentMethod;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.commerce.v2.PaymentMethod")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<PaymentMethod, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut payment_method_kind__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Stripe => {
                            if payment_method_kind__.is_some() {
                                return Err(serde::de::Error::duplicate_field("stripe"));
                            }
                            payment_method_kind__ = map_.next_value::<::std::option::Option<_>>()?.map(payment_method::PaymentMethodKind::Stripe)
;
                        }
                    }
                }
                Ok(PaymentMethod {
                    payment_method_kind: payment_method_kind__,
                })
            }
        }
        deserializer.deserialize_struct("sited_io.commerce.v2.PaymentMethod", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for payment_method::Stripe {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.subscription_id.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sited_io.commerce.v2.PaymentMethod.Stripe", len)?;
        if let Some(v) = self.subscription_id.as_ref() {
            struct_ser.serialize_field("subscriptionId", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for payment_method::Stripe {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "subscription_id",
            "subscriptionId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            SubscriptionId,
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
                            "subscriptionId" | "subscription_id" => Ok(GeneratedField::SubscriptionId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = payment_method::Stripe;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.commerce.v2.PaymentMethod.Stripe")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<payment_method::Stripe, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut subscription_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::SubscriptionId => {
                            if subscription_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("subscriptionId"));
                            }
                            subscription_id__ = map_.next_value()?;
                        }
                    }
                }
                Ok(payment_method::Stripe {
                    subscription_id: subscription_id__,
                })
            }
        }
        deserializer.deserialize_struct("sited_io.commerce.v2.PaymentMethod.Stripe", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PriceType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.price_type_kind.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sited_io.commerce.v2.PriceType", len)?;
        if let Some(v) = self.price_type_kind.as_ref() {
            match v {
                price_type::PriceTypeKind::OneTime(v) => {
                    struct_ser.serialize_field("oneTime", v)?;
                }
                price_type::PriceTypeKind::Recurring(v) => {
                    struct_ser.serialize_field("recurring", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PriceType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "one_time",
            "oneTime",
            "recurring",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            OneTime,
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
                            "oneTime" | "one_time" => Ok(GeneratedField::OneTime),
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
            type Value = PriceType;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.commerce.v2.PriceType")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<PriceType, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut price_type_kind__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::OneTime => {
                            if price_type_kind__.is_some() {
                                return Err(serde::de::Error::duplicate_field("oneTime"));
                            }
                            price_type_kind__ = map_.next_value::<::std::option::Option<_>>()?.map(price_type::PriceTypeKind::OneTime)
;
                        }
                        GeneratedField::Recurring => {
                            if price_type_kind__.is_some() {
                                return Err(serde::de::Error::duplicate_field("recurring"));
                            }
                            price_type_kind__ = map_.next_value::<::std::option::Option<_>>()?.map(price_type::PriceTypeKind::Recurring)
;
                        }
                    }
                }
                Ok(PriceType {
                    price_type_kind: price_type_kind__,
                })
            }
        }
        deserializer.deserialize_struct("sited_io.commerce.v2.PriceType", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for price_type::OneTime {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("sited_io.commerce.v2.PriceType.OneTime", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for price_type::OneTime {
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
            type Value = price_type::OneTime;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.commerce.v2.PriceType.OneTime")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<price_type::OneTime, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(price_type::OneTime {
                })
            }
        }
        deserializer.deserialize_struct("sited_io.commerce.v2.PriceType.OneTime", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for price_type::Recurring {
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
        let mut struct_ser = serializer.serialize_struct("sited_io.commerce.v2.PriceType.Recurring", len)?;
        if self.interval != 0 {
            let v = price_type::recurring::Interval::try_from(self.interval)
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
impl<'de> serde::Deserialize<'de> for price_type::Recurring {
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
            type Value = price_type::Recurring;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.commerce.v2.PriceType.Recurring")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<price_type::Recurring, V::Error>
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
                            interval__ = Some(map_.next_value::<price_type::recurring::Interval>()? as i32);
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
                Ok(price_type::Recurring {
                    interval: interval__.unwrap_or_default(),
                    interval_count: interval_count__.unwrap_or_default(),
                    trial_period_days: trial_period_days__,
                })
            }
        }
        deserializer.deserialize_struct("sited_io.commerce.v2.PriceType.Recurring", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for price_type::recurring::Interval {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "INTERVAL_UNSPECIFIED",
            Self::Day => "INTERVAL_DAY",
            Self::Week => "INTERVAL_WEEK",
            Self::Month => "INTERVAL_MONTH",
            Self::Year => "INTERVAL_YEAR",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for price_type::recurring::Interval {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "INTERVAL_UNSPECIFIED",
            "INTERVAL_DAY",
            "INTERVAL_WEEK",
            "INTERVAL_MONTH",
            "INTERVAL_YEAR",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = price_type::recurring::Interval;

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
                    "INTERVAL_UNSPECIFIED" => Ok(price_type::recurring::Interval::Unspecified),
                    "INTERVAL_DAY" => Ok(price_type::recurring::Interval::Day),
                    "INTERVAL_WEEK" => Ok(price_type::recurring::Interval::Week),
                    "INTERVAL_MONTH" => Ok(price_type::recurring::Interval::Month),
                    "INTERVAL_YEAR" => Ok(price_type::recurring::Interval::Year),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for PutMultipartChunkRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.offer_file_id.is_empty() {
            len += 1;
        }
        if !self.upload_id.is_empty() {
            len += 1;
        }
        if self.part_number != 0 {
            len += 1;
        }
        if !self.chunk.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sited_io.commerce.v2.PutMultipartChunkRequest", len)?;
        if !self.offer_file_id.is_empty() {
            struct_ser.serialize_field("offerFileId", &self.offer_file_id)?;
        }
        if !self.upload_id.is_empty() {
            struct_ser.serialize_field("uploadId", &self.upload_id)?;
        }
        if self.part_number != 0 {
            struct_ser.serialize_field("partNumber", &self.part_number)?;
        }
        if !self.chunk.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("chunk", pbjson::private::base64::encode(&self.chunk).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PutMultipartChunkRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "offer_file_id",
            "offerFileId",
            "upload_id",
            "uploadId",
            "part_number",
            "partNumber",
            "chunk",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            OfferFileId,
            UploadId,
            PartNumber,
            Chunk,
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
                            "offerFileId" | "offer_file_id" => Ok(GeneratedField::OfferFileId),
                            "uploadId" | "upload_id" => Ok(GeneratedField::UploadId),
                            "partNumber" | "part_number" => Ok(GeneratedField::PartNumber),
                            "chunk" => Ok(GeneratedField::Chunk),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PutMultipartChunkRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.commerce.v2.PutMultipartChunkRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<PutMultipartChunkRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut offer_file_id__ = None;
                let mut upload_id__ = None;
                let mut part_number__ = None;
                let mut chunk__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::OfferFileId => {
                            if offer_file_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("offerFileId"));
                            }
                            offer_file_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::UploadId => {
                            if upload_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("uploadId"));
                            }
                            upload_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::PartNumber => {
                            if part_number__.is_some() {
                                return Err(serde::de::Error::duplicate_field("partNumber"));
                            }
                            part_number__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Chunk => {
                            if chunk__.is_some() {
                                return Err(serde::de::Error::duplicate_field("chunk"));
                            }
                            chunk__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(PutMultipartChunkRequest {
                    offer_file_id: offer_file_id__.unwrap_or_default(),
                    upload_id: upload_id__.unwrap_or_default(),
                    part_number: part_number__.unwrap_or_default(),
                    chunk: chunk__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sited_io.commerce.v2.PutMultipartChunkRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PutMultipartChunkResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.part.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sited_io.commerce.v2.PutMultipartChunkResponse", len)?;
        if let Some(v) = self.part.as_ref() {
            struct_ser.serialize_field("part", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PutMultipartChunkResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "part",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Part,
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
                            "part" => Ok(GeneratedField::Part),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PutMultipartChunkResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.commerce.v2.PutMultipartChunkResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<PutMultipartChunkResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut part__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Part => {
                            if part__.is_some() {
                                return Err(serde::de::Error::duplicate_field("part"));
                            }
                            part__ = map_.next_value()?;
                        }
                    }
                }
                Ok(PutMultipartChunkResponse {
                    part: part__,
                })
            }
        }
        deserializer.deserialize_struct("sited_io.commerce.v2.PutMultipartChunkResponse", FIELDS, GeneratedVisitor)
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
        if self.unit_amount != 0 {
            len += 1;
        }
        if self.currency != 0 {
            len += 1;
        }
        if self.price_type.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sited_io.commerce.v2.PutPriceToOfferRequest", len)?;
        if !self.offer_id.is_empty() {
            struct_ser.serialize_field("offerId", &self.offer_id)?;
        }
        if self.unit_amount != 0 {
            struct_ser.serialize_field("unitAmount", &self.unit_amount)?;
        }
        if self.currency != 0 {
            let v = super::super::types::currency::v1::CurrencyCode::try_from(self.currency)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.currency)))?;
            struct_ser.serialize_field("currency", &v)?;
        }
        if let Some(v) = self.price_type.as_ref() {
            struct_ser.serialize_field("priceType", v)?;
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
            "unit_amount",
            "unitAmount",
            "currency",
            "price_type",
            "priceType",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            OfferId,
            UnitAmount,
            Currency,
            PriceType,
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
                            "unitAmount" | "unit_amount" => Ok(GeneratedField::UnitAmount),
                            "currency" => Ok(GeneratedField::Currency),
                            "priceType" | "price_type" => Ok(GeneratedField::PriceType),
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
                formatter.write_str("struct sited_io.commerce.v2.PutPriceToOfferRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<PutPriceToOfferRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut offer_id__ = None;
                let mut unit_amount__ = None;
                let mut currency__ = None;
                let mut price_type__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::OfferId => {
                            if offer_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("offerId"));
                            }
                            offer_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::UnitAmount => {
                            if unit_amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("unitAmount"));
                            }
                            unit_amount__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Currency => {
                            if currency__.is_some() {
                                return Err(serde::de::Error::duplicate_field("currency"));
                            }
                            currency__ = Some(map_.next_value::<super::super::types::currency::v1::CurrencyCode>()? as i32);
                        }
                        GeneratedField::PriceType => {
                            if price_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("priceType"));
                            }
                            price_type__ = map_.next_value()?;
                        }
                    }
                }
                Ok(PutPriceToOfferRequest {
                    offer_id: offer_id__.unwrap_or_default(),
                    unit_amount: unit_amount__.unwrap_or_default(),
                    currency: currency__.unwrap_or_default(),
                    price_type: price_type__,
                })
            }
        }
        deserializer.deserialize_struct("sited_io.commerce.v2.PutPriceToOfferRequest", FIELDS, GeneratedVisitor)
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
        let struct_ser = serializer.serialize_struct("sited_io.commerce.v2.PutPriceToOfferResponse", len)?;
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
                formatter.write_str("struct sited_io.commerce.v2.PutPriceToOfferResponse")
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
        deserializer.deserialize_struct("sited_io.commerce.v2.PutPriceToOfferResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PutShippingRateToOfferRequest {
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
        if self.unit_amount != 0 {
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
        let mut struct_ser = serializer.serialize_struct("sited_io.commerce.v2.PutShippingRateToOfferRequest", len)?;
        if !self.offer_id.is_empty() {
            struct_ser.serialize_field("offerId", &self.offer_id)?;
        }
        if self.unit_amount != 0 {
            struct_ser.serialize_field("unitAmount", &self.unit_amount)?;
        }
        if self.currency != 0 {
            let v = super::super::types::currency::v1::CurrencyCode::try_from(self.currency)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.currency)))?;
            struct_ser.serialize_field("currency", &v)?;
        }
        if self.all_countries {
            struct_ser.serialize_field("allCountries", &self.all_countries)?;
        }
        if !self.specific_countries.is_empty() {
            let v = self.specific_countries.iter().cloned().map(|v| {
                super::super::types::country::v1::CountryCode::try_from(v)
                    .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", v)))
                }).collect::<std::result::Result<Vec<_>, _>>()?;
            struct_ser.serialize_field("specificCountries", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PutShippingRateToOfferRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "offer_id",
            "offerId",
            "unit_amount",
            "unitAmount",
            "currency",
            "all_countries",
            "allCountries",
            "specific_countries",
            "specificCountries",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            OfferId,
            UnitAmount,
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
                            "unitAmount" | "unit_amount" => Ok(GeneratedField::UnitAmount),
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
            type Value = PutShippingRateToOfferRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.commerce.v2.PutShippingRateToOfferRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<PutShippingRateToOfferRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut offer_id__ = None;
                let mut unit_amount__ = None;
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
                        GeneratedField::UnitAmount => {
                            if unit_amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("unitAmount"));
                            }
                            unit_amount__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Currency => {
                            if currency__.is_some() {
                                return Err(serde::de::Error::duplicate_field("currency"));
                            }
                            currency__ = Some(map_.next_value::<super::super::types::currency::v1::CurrencyCode>()? as i32);
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
                            specific_countries__ = Some(map_.next_value::<Vec<super::super::types::country::v1::CountryCode>>()?.into_iter().map(|x| x as i32).collect());
                        }
                    }
                }
                Ok(PutShippingRateToOfferRequest {
                    offer_id: offer_id__.unwrap_or_default(),
                    unit_amount: unit_amount__.unwrap_or_default(),
                    currency: currency__.unwrap_or_default(),
                    all_countries: all_countries__.unwrap_or_default(),
                    specific_countries: specific_countries__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sited_io.commerce.v2.PutShippingRateToOfferRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PutShippingRateToOfferResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("sited_io.commerce.v2.PutShippingRateToOfferResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PutShippingRateToOfferResponse {
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
            type Value = PutShippingRateToOfferResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.commerce.v2.PutShippingRateToOfferResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<PutShippingRateToOfferResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(PutShippingRateToOfferResponse {
                })
            }
        }
        deserializer.deserialize_struct("sited_io.commerce.v2.PutShippingRateToOfferResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RemoveFileFromOfferRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.offer_file_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sited_io.commerce.v2.RemoveFileFromOfferRequest", len)?;
        if !self.offer_file_id.is_empty() {
            struct_ser.serialize_field("offerFileId", &self.offer_file_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RemoveFileFromOfferRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "offer_file_id",
            "offerFileId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            OfferFileId,
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
                            "offerFileId" | "offer_file_id" => Ok(GeneratedField::OfferFileId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RemoveFileFromOfferRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.commerce.v2.RemoveFileFromOfferRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RemoveFileFromOfferRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut offer_file_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::OfferFileId => {
                            if offer_file_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("offerFileId"));
                            }
                            offer_file_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(RemoveFileFromOfferRequest {
                    offer_file_id: offer_file_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sited_io.commerce.v2.RemoveFileFromOfferRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RemoveFileFromOfferResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("sited_io.commerce.v2.RemoveFileFromOfferResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RemoveFileFromOfferResponse {
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
            type Value = RemoveFileFromOfferResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.commerce.v2.RemoveFileFromOfferResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RemoveFileFromOfferResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(RemoveFileFromOfferResponse {
                })
            }
        }
        deserializer.deserialize_struct("sited_io.commerce.v2.RemoveFileFromOfferResponse", FIELDS, GeneratedVisitor)
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
        if !self.offer_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sited_io.commerce.v2.RemoveImageFromOfferRequest", len)?;
        if !self.offer_image_id.is_empty() {
            struct_ser.serialize_field("offerImageId", &self.offer_image_id)?;
        }
        if !self.offer_id.is_empty() {
            struct_ser.serialize_field("offerId", &self.offer_id)?;
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
            "offer_id",
            "offerId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            OfferImageId,
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
                            "offerImageId" | "offer_image_id" => Ok(GeneratedField::OfferImageId),
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
            type Value = RemoveImageFromOfferRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.commerce.v2.RemoveImageFromOfferRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RemoveImageFromOfferRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut offer_image_id__ = None;
                let mut offer_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::OfferImageId => {
                            if offer_image_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("offerImageId"));
                            }
                            offer_image_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::OfferId => {
                            if offer_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("offerId"));
                            }
                            offer_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(RemoveImageFromOfferRequest {
                    offer_image_id: offer_image_id__.unwrap_or_default(),
                    offer_id: offer_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sited_io.commerce.v2.RemoveImageFromOfferRequest", FIELDS, GeneratedVisitor)
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
        let struct_ser = serializer.serialize_struct("sited_io.commerce.v2.RemoveImageFromOfferResponse", len)?;
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
                formatter.write_str("struct sited_io.commerce.v2.RemoveImageFromOfferResponse")
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
        deserializer.deserialize_struct("sited_io.commerce.v2.RemoveImageFromOfferResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RemoveOfferFromShopRequest {
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
        let mut struct_ser = serializer.serialize_struct("sited_io.commerce.v2.RemoveOfferFromShopRequest", len)?;
        if !self.offer_id.is_empty() {
            struct_ser.serialize_field("offerId", &self.offer_id)?;
        }
        if !self.shop_id.is_empty() {
            struct_ser.serialize_field("shopId", &self.shop_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RemoveOfferFromShopRequest {
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
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            OfferId,
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
                            "offerId" | "offer_id" => Ok(GeneratedField::OfferId),
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
            type Value = RemoveOfferFromShopRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.commerce.v2.RemoveOfferFromShopRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RemoveOfferFromShopRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut offer_id__ = None;
                let mut shop_id__ = None;
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
                    }
                }
                Ok(RemoveOfferFromShopRequest {
                    offer_id: offer_id__.unwrap_or_default(),
                    shop_id: shop_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sited_io.commerce.v2.RemoveOfferFromShopRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RemoveOfferFromShopResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("sited_io.commerce.v2.RemoveOfferFromShopResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RemoveOfferFromShopResponse {
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
            type Value = RemoveOfferFromShopResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.commerce.v2.RemoveOfferFromShopResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RemoveOfferFromShopResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(RemoveOfferFromShopResponse {
                })
            }
        }
        deserializer.deserialize_struct("sited_io.commerce.v2.RemoveOfferFromShopResponse", FIELDS, GeneratedVisitor)
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
        let mut struct_ser = serializer.serialize_struct("sited_io.commerce.v2.RemovePriceFromOfferRequest", len)?;
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
                formatter.write_str("struct sited_io.commerce.v2.RemovePriceFromOfferRequest")
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
        deserializer.deserialize_struct("sited_io.commerce.v2.RemovePriceFromOfferRequest", FIELDS, GeneratedVisitor)
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
        let struct_ser = serializer.serialize_struct("sited_io.commerce.v2.RemovePriceFromOfferResponse", len)?;
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
                formatter.write_str("struct sited_io.commerce.v2.RemovePriceFromOfferResponse")
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
        deserializer.deserialize_struct("sited_io.commerce.v2.RemovePriceFromOfferResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RemoveShippingRateFromOfferRequest {
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
        let mut struct_ser = serializer.serialize_struct("sited_io.commerce.v2.RemoveShippingRateFromOfferRequest", len)?;
        if !self.offer_id.is_empty() {
            struct_ser.serialize_field("offerId", &self.offer_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RemoveShippingRateFromOfferRequest {
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
            type Value = RemoveShippingRateFromOfferRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.commerce.v2.RemoveShippingRateFromOfferRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RemoveShippingRateFromOfferRequest, V::Error>
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
                Ok(RemoveShippingRateFromOfferRequest {
                    offer_id: offer_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sited_io.commerce.v2.RemoveShippingRateFromOfferRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RemoveShippingRateFromOfferResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("sited_io.commerce.v2.RemoveShippingRateFromOfferResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RemoveShippingRateFromOfferResponse {
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
            type Value = RemoveShippingRateFromOfferResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.commerce.v2.RemoveShippingRateFromOfferResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RemoveShippingRateFromOfferResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(RemoveShippingRateFromOfferResponse {
                })
            }
        }
        deserializer.deserialize_struct("sited_io.commerce.v2.RemoveShippingRateFromOfferResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ResumeSubscriptionRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.order_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sited_io.commerce.v2.ResumeSubscriptionRequest", len)?;
        if !self.order_id.is_empty() {
            struct_ser.serialize_field("orderId", &self.order_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ResumeSubscriptionRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "order_id",
            "orderId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            OrderId,
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
                            "orderId" | "order_id" => Ok(GeneratedField::OrderId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ResumeSubscriptionRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.commerce.v2.ResumeSubscriptionRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ResumeSubscriptionRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut order_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::OrderId => {
                            if order_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("orderId"));
                            }
                            order_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ResumeSubscriptionRequest {
                    order_id: order_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sited_io.commerce.v2.ResumeSubscriptionRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ResumeSubscriptionResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("sited_io.commerce.v2.ResumeSubscriptionResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ResumeSubscriptionResponse {
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
            type Value = ResumeSubscriptionResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.commerce.v2.ResumeSubscriptionResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ResumeSubscriptionResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(ResumeSubscriptionResponse {
                })
            }
        }
        deserializer.deserialize_struct("sited_io.commerce.v2.ResumeSubscriptionResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ShippingRate {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.unit_amount != 0 {
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
        let mut struct_ser = serializer.serialize_struct("sited_io.commerce.v2.ShippingRate", len)?;
        if self.unit_amount != 0 {
            struct_ser.serialize_field("unitAmount", &self.unit_amount)?;
        }
        if self.currency != 0 {
            let v = super::super::types::currency::v1::CurrencyCode::try_from(self.currency)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.currency)))?;
            struct_ser.serialize_field("currency", &v)?;
        }
        if self.all_countries {
            struct_ser.serialize_field("allCountries", &self.all_countries)?;
        }
        if !self.specific_countries.is_empty() {
            let v = self.specific_countries.iter().cloned().map(|v| {
                super::super::types::country::v1::CountryCode::try_from(v)
                    .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", v)))
                }).collect::<std::result::Result<Vec<_>, _>>()?;
            struct_ser.serialize_field("specificCountries", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ShippingRate {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "unit_amount",
            "unitAmount",
            "currency",
            "all_countries",
            "allCountries",
            "specific_countries",
            "specificCountries",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            UnitAmount,
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
                            "unitAmount" | "unit_amount" => Ok(GeneratedField::UnitAmount),
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
            type Value = ShippingRate;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.commerce.v2.ShippingRate")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ShippingRate, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut unit_amount__ = None;
                let mut currency__ = None;
                let mut all_countries__ = None;
                let mut specific_countries__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::UnitAmount => {
                            if unit_amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("unitAmount"));
                            }
                            unit_amount__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Currency => {
                            if currency__.is_some() {
                                return Err(serde::de::Error::duplicate_field("currency"));
                            }
                            currency__ = Some(map_.next_value::<super::super::types::currency::v1::CurrencyCode>()? as i32);
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
                            specific_countries__ = Some(map_.next_value::<Vec<super::super::types::country::v1::CountryCode>>()?.into_iter().map(|x| x as i32).collect());
                        }
                    }
                }
                Ok(ShippingRate {
                    unit_amount: unit_amount__.unwrap_or_default(),
                    currency: currency__.unwrap_or_default(),
                    all_countries: all_countries__.unwrap_or_default(),
                    specific_countries: specific_countries__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sited_io.commerce.v2.ShippingRate", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Shop {
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
        if !self.owner.is_empty() {
            len += 1;
        }
        if !self.website_id.is_empty() {
            len += 1;
        }
        if !self.offers.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sited_io.commerce.v2.Shop", len)?;
        if !self.shop_id.is_empty() {
            struct_ser.serialize_field("shopId", &self.shop_id)?;
        }
        if !self.owner.is_empty() {
            struct_ser.serialize_field("owner", &self.owner)?;
        }
        if !self.website_id.is_empty() {
            struct_ser.serialize_field("websiteId", &self.website_id)?;
        }
        if !self.offers.is_empty() {
            struct_ser.serialize_field("offers", &self.offers)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Shop {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "shop_id",
            "shopId",
            "owner",
            "website_id",
            "websiteId",
            "offers",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ShopId,
            Owner,
            WebsiteId,
            Offers,
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
                            "owner" => Ok(GeneratedField::Owner),
                            "websiteId" | "website_id" => Ok(GeneratedField::WebsiteId),
                            "offers" => Ok(GeneratedField::Offers),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Shop;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.commerce.v2.Shop")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Shop, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut shop_id__ = None;
                let mut owner__ = None;
                let mut website_id__ = None;
                let mut offers__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ShopId => {
                            if shop_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("shopId"));
                            }
                            shop_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Owner => {
                            if owner__.is_some() {
                                return Err(serde::de::Error::duplicate_field("owner"));
                            }
                            owner__ = Some(map_.next_value()?);
                        }
                        GeneratedField::WebsiteId => {
                            if website_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("websiteId"));
                            }
                            website_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Offers => {
                            if offers__.is_some() {
                                return Err(serde::de::Error::duplicate_field("offers"));
                            }
                            offers__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(Shop {
                    shop_id: shop_id__.unwrap_or_default(),
                    owner: owner__.unwrap_or_default(),
                    website_id: website_id__.unwrap_or_default(),
                    offers: offers__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sited_io.commerce.v2.Shop", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for StripeAccount {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.stripe_account_id.is_empty() {
            len += 1;
        }
        if self.status.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sited_io.commerce.v2.StripeAccount", len)?;
        if !self.stripe_account_id.is_empty() {
            struct_ser.serialize_field("stripeAccountId", &self.stripe_account_id)?;
        }
        if let Some(v) = self.status.as_ref() {
            match v {
                stripe_account::Status::Pending(v) => {
                    struct_ser.serialize_field("pending", v)?;
                }
                stripe_account::Status::Configured(v) => {
                    struct_ser.serialize_field("configured", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for StripeAccount {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "stripe_account_id",
            "stripeAccountId",
            "pending",
            "configured",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            StripeAccountId,
            Pending,
            Configured,
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
                            "stripeAccountId" | "stripe_account_id" => Ok(GeneratedField::StripeAccountId),
                            "pending" => Ok(GeneratedField::Pending),
                            "configured" => Ok(GeneratedField::Configured),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = StripeAccount;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.commerce.v2.StripeAccount")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<StripeAccount, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut stripe_account_id__ = None;
                let mut status__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::StripeAccountId => {
                            if stripe_account_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("stripeAccountId"));
                            }
                            stripe_account_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Pending => {
                            if status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pending"));
                            }
                            status__ = map_.next_value::<::std::option::Option<_>>()?.map(stripe_account::Status::Pending)
;
                        }
                        GeneratedField::Configured => {
                            if status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("configured"));
                            }
                            status__ = map_.next_value::<::std::option::Option<_>>()?.map(stripe_account::Status::Configured)
;
                        }
                    }
                }
                Ok(StripeAccount {
                    stripe_account_id: stripe_account_id__.unwrap_or_default(),
                    status: status__,
                })
            }
        }
        deserializer.deserialize_struct("sited_io.commerce.v2.StripeAccount", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for stripe_account::Configured {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.charges_enabled {
            len += 1;
        }
        if self.details_submitted {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sited_io.commerce.v2.StripeAccount.Configured", len)?;
        if self.charges_enabled {
            struct_ser.serialize_field("chargesEnabled", &self.charges_enabled)?;
        }
        if self.details_submitted {
            struct_ser.serialize_field("detailsSubmitted", &self.details_submitted)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for stripe_account::Configured {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "charges_enabled",
            "chargesEnabled",
            "details_submitted",
            "detailsSubmitted",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ChargesEnabled,
            DetailsSubmitted,
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
                            "chargesEnabled" | "charges_enabled" => Ok(GeneratedField::ChargesEnabled),
                            "detailsSubmitted" | "details_submitted" => Ok(GeneratedField::DetailsSubmitted),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = stripe_account::Configured;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.commerce.v2.StripeAccount.Configured")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<stripe_account::Configured, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut charges_enabled__ = None;
                let mut details_submitted__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ChargesEnabled => {
                            if charges_enabled__.is_some() {
                                return Err(serde::de::Error::duplicate_field("chargesEnabled"));
                            }
                            charges_enabled__ = Some(map_.next_value()?);
                        }
                        GeneratedField::DetailsSubmitted => {
                            if details_submitted__.is_some() {
                                return Err(serde::de::Error::duplicate_field("detailsSubmitted"));
                            }
                            details_submitted__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(stripe_account::Configured {
                    charges_enabled: charges_enabled__.unwrap_or_default(),
                    details_submitted: details_submitted__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sited_io.commerce.v2.StripeAccount.Configured", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for stripe_account::Pending {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.link.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sited_io.commerce.v2.StripeAccount.Pending", len)?;
        if !self.link.is_empty() {
            struct_ser.serialize_field("link", &self.link)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for stripe_account::Pending {
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
            type Value = stripe_account::Pending;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.commerce.v2.StripeAccount.Pending")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<stripe_account::Pending, V::Error>
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
                            link__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(stripe_account::Pending {
                    link: link__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sited_io.commerce.v2.StripeAccount.Pending", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateFileOrderingRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.offer_file_id.is_empty() {
            len += 1;
        }
        if self.ordering != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sited_io.commerce.v2.UpdateFileOrderingRequest", len)?;
        if !self.offer_file_id.is_empty() {
            struct_ser.serialize_field("offerFileId", &self.offer_file_id)?;
        }
        if self.ordering != 0 {
            struct_ser.serialize_field("ordering", &self.ordering)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateFileOrderingRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "offer_file_id",
            "offerFileId",
            "ordering",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            OfferFileId,
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
                            "offerFileId" | "offer_file_id" => Ok(GeneratedField::OfferFileId),
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
            type Value = UpdateFileOrderingRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.commerce.v2.UpdateFileOrderingRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateFileOrderingRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut offer_file_id__ = None;
                let mut ordering__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::OfferFileId => {
                            if offer_file_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("offerFileId"));
                            }
                            offer_file_id__ = Some(map_.next_value()?);
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
                Ok(UpdateFileOrderingRequest {
                    offer_file_id: offer_file_id__.unwrap_or_default(),
                    ordering: ordering__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sited_io.commerce.v2.UpdateFileOrderingRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateFileOrderingResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("sited_io.commerce.v2.UpdateFileOrderingResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateFileOrderingResponse {
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
            type Value = UpdateFileOrderingResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.commerce.v2.UpdateFileOrderingResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateFileOrderingResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(UpdateFileOrderingResponse {
                })
            }
        }
        deserializer.deserialize_struct("sited_io.commerce.v2.UpdateFileOrderingResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateImageOrderingRequest {
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
        if self.ordering != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sited_io.commerce.v2.UpdateImageOrderingRequest", len)?;
        if !self.offer_image_id.is_empty() {
            struct_ser.serialize_field("offerImageId", &self.offer_image_id)?;
        }
        if self.ordering != 0 {
            struct_ser.serialize_field("ordering", &self.ordering)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateImageOrderingRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "offer_image_id",
            "offerImageId",
            "ordering",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            OfferImageId,
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
            type Value = UpdateImageOrderingRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.commerce.v2.UpdateImageOrderingRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateImageOrderingRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut offer_image_id__ = None;
                let mut ordering__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::OfferImageId => {
                            if offer_image_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("offerImageId"));
                            }
                            offer_image_id__ = Some(map_.next_value()?);
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
                Ok(UpdateImageOrderingRequest {
                    offer_image_id: offer_image_id__.unwrap_or_default(),
                    ordering: ordering__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sited_io.commerce.v2.UpdateImageOrderingRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateImageOrderingResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("sited_io.commerce.v2.UpdateImageOrderingResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateImageOrderingResponse {
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
            type Value = UpdateImageOrderingResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.commerce.v2.UpdateImageOrderingResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateImageOrderingResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(UpdateImageOrderingResponse {
                })
            }
        }
        deserializer.deserialize_struct("sited_io.commerce.v2.UpdateImageOrderingResponse", FIELDS, GeneratedVisitor)
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
        if self.details.is_some() {
            len += 1;
        }
        if self.offer_type.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sited_io.commerce.v2.UpdateOfferRequest", len)?;
        if !self.offer_id.is_empty() {
            struct_ser.serialize_field("offerId", &self.offer_id)?;
        }
        if let Some(v) = self.details.as_ref() {
            struct_ser.serialize_field("details", v)?;
        }
        if let Some(v) = self.offer_type.as_ref() {
            struct_ser.serialize_field("offerType", v)?;
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
            "details",
            "offer_type",
            "offerType",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            OfferId,
            Details,
            OfferType,
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
                            "details" => Ok(GeneratedField::Details),
                            "offerType" | "offer_type" => Ok(GeneratedField::OfferType),
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
                formatter.write_str("struct sited_io.commerce.v2.UpdateOfferRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateOfferRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut offer_id__ = None;
                let mut details__ = None;
                let mut offer_type__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::OfferId => {
                            if offer_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("offerId"));
                            }
                            offer_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Details => {
                            if details__.is_some() {
                                return Err(serde::de::Error::duplicate_field("details"));
                            }
                            details__ = map_.next_value()?;
                        }
                        GeneratedField::OfferType => {
                            if offer_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("offerType"));
                            }
                            offer_type__ = map_.next_value()?;
                        }
                    }
                }
                Ok(UpdateOfferRequest {
                    offer_id: offer_id__.unwrap_or_default(),
                    details: details__,
                    offer_type: offer_type__,
                })
            }
        }
        deserializer.deserialize_struct("sited_io.commerce.v2.UpdateOfferRequest", FIELDS, GeneratedVisitor)
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
        let mut struct_ser = serializer.serialize_struct("sited_io.commerce.v2.UpdateOfferResponse", len)?;
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
                formatter.write_str("struct sited_io.commerce.v2.UpdateOfferResponse")
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
        deserializer.deserialize_struct("sited_io.commerce.v2.UpdateOfferResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UserQuota {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.user_id.is_empty() {
            len += 1;
        }
        if self.max_allowed_size_bytes != 0 {
            len += 1;
        }
        if self.uploaded_size_bytes != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sited_io.commerce.v2.UserQuota", len)?;
        if !self.user_id.is_empty() {
            struct_ser.serialize_field("userId", &self.user_id)?;
        }
        if self.max_allowed_size_bytes != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("maxAllowedSizeBytes", ToString::to_string(&self.max_allowed_size_bytes).as_str())?;
        }
        if self.uploaded_size_bytes != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("uploadedSizeBytes", ToString::to_string(&self.uploaded_size_bytes).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UserQuota {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "user_id",
            "userId",
            "max_allowed_size_bytes",
            "maxAllowedSizeBytes",
            "uploaded_size_bytes",
            "uploadedSizeBytes",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            UserId,
            MaxAllowedSizeBytes,
            UploadedSizeBytes,
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
                            "maxAllowedSizeBytes" | "max_allowed_size_bytes" => Ok(GeneratedField::MaxAllowedSizeBytes),
                            "uploadedSizeBytes" | "uploaded_size_bytes" => Ok(GeneratedField::UploadedSizeBytes),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UserQuota;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.commerce.v2.UserQuota")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UserQuota, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut user_id__ = None;
                let mut max_allowed_size_bytes__ = None;
                let mut uploaded_size_bytes__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::UserId => {
                            if user_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userId"));
                            }
                            user_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::MaxAllowedSizeBytes => {
                            if max_allowed_size_bytes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxAllowedSizeBytes"));
                            }
                            max_allowed_size_bytes__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::UploadedSizeBytes => {
                            if uploaded_size_bytes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("uploadedSizeBytes"));
                            }
                            uploaded_size_bytes__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(UserQuota {
                    user_id: user_id__.unwrap_or_default(),
                    max_allowed_size_bytes: max_allowed_size_bytes__.unwrap_or_default(),
                    uploaded_size_bytes: uploaded_size_bytes__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sited_io.commerce.v2.UserQuota", FIELDS, GeneratedVisitor)
    }
}

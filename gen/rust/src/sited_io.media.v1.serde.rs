// @generated
impl serde::Serialize for AddMediaToOfferRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.media_id.is_empty() {
            len += 1;
        }
        if !self.offer_id.is_empty() {
            len += 1;
        }
        if self.ordering.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sited_io.media.v1.AddMediaToOfferRequest", len)?;
        if !self.media_id.is_empty() {
            struct_ser.serialize_field("mediaId", &self.media_id)?;
        }
        if !self.offer_id.is_empty() {
            struct_ser.serialize_field("offerId", &self.offer_id)?;
        }
        if let Some(v) = self.ordering.as_ref() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("ordering", ToString::to_string(&v).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AddMediaToOfferRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "media_id",
            "mediaId",
            "offer_id",
            "offerId",
            "ordering",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            MediaId,
            OfferId,
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
                            "mediaId" | "media_id" => Ok(GeneratedField::MediaId),
                            "offerId" | "offer_id" => Ok(GeneratedField::OfferId),
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
            type Value = AddMediaToOfferRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.media.v1.AddMediaToOfferRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AddMediaToOfferRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut media_id__ = None;
                let mut offer_id__ = None;
                let mut ordering__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::MediaId => {
                            if media_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("mediaId"));
                            }
                            media_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::OfferId => {
                            if offer_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("offerId"));
                            }
                            offer_id__ = Some(map_.next_value()?);
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
                Ok(AddMediaToOfferRequest {
                    media_id: media_id__.unwrap_or_default(),
                    offer_id: offer_id__.unwrap_or_default(),
                    ordering: ordering__,
                })
            }
        }
        deserializer.deserialize_struct("sited_io.media.v1.AddMediaToOfferRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for AddMediaToOfferResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("sited_io.media.v1.AddMediaToOfferResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AddMediaToOfferResponse {
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
            type Value = AddMediaToOfferResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.media.v1.AddMediaToOfferResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AddMediaToOfferResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(AddMediaToOfferResponse {
                })
            }
        }
        deserializer.deserialize_struct("sited_io.media.v1.AddMediaToOfferResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CancelMediaSubscriptionRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.media_subscription_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sited_io.media.v1.CancelMediaSubscriptionRequest", len)?;
        if !self.media_subscription_id.is_empty() {
            struct_ser.serialize_field("mediaSubscriptionId", &self.media_subscription_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CancelMediaSubscriptionRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "media_subscription_id",
            "mediaSubscriptionId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            MediaSubscriptionId,
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
                            "mediaSubscriptionId" | "media_subscription_id" => Ok(GeneratedField::MediaSubscriptionId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CancelMediaSubscriptionRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.media.v1.CancelMediaSubscriptionRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CancelMediaSubscriptionRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut media_subscription_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::MediaSubscriptionId => {
                            if media_subscription_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("mediaSubscriptionId"));
                            }
                            media_subscription_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(CancelMediaSubscriptionRequest {
                    media_subscription_id: media_subscription_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sited_io.media.v1.CancelMediaSubscriptionRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CancelMediaSubscriptionResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("sited_io.media.v1.CancelMediaSubscriptionResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CancelMediaSubscriptionResponse {
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
            type Value = CancelMediaSubscriptionResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.media.v1.CancelMediaSubscriptionResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CancelMediaSubscriptionResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(CancelMediaSubscriptionResponse {
                })
            }
        }
        deserializer.deserialize_struct("sited_io.media.v1.CancelMediaSubscriptionResponse", FIELDS, GeneratedVisitor)
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
        if !self.media_id.is_empty() {
            len += 1;
        }
        if !self.upload_id.is_empty() {
            len += 1;
        }
        if !self.parts.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sited_io.media.v1.CompleteMultipartUploadRequest", len)?;
        if !self.media_id.is_empty() {
            struct_ser.serialize_field("mediaId", &self.media_id)?;
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
            "media_id",
            "mediaId",
            "upload_id",
            "uploadId",
            "parts",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            MediaId,
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
                            "mediaId" | "media_id" => Ok(GeneratedField::MediaId),
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
                formatter.write_str("struct sited_io.media.v1.CompleteMultipartUploadRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CompleteMultipartUploadRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut media_id__ = None;
                let mut upload_id__ = None;
                let mut parts__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::MediaId => {
                            if media_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("mediaId"));
                            }
                            media_id__ = Some(map_.next_value()?);
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
                    media_id: media_id__.unwrap_or_default(),
                    upload_id: upload_id__.unwrap_or_default(),
                    parts: parts__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sited_io.media.v1.CompleteMultipartUploadRequest", FIELDS, GeneratedVisitor)
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
        let struct_ser = serializer.serialize_struct("sited_io.media.v1.CompleteMultipartUploadResponse", len)?;
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
                formatter.write_str("struct sited_io.media.v1.CompleteMultipartUploadResponse")
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
        deserializer.deserialize_struct("sited_io.media.v1.CompleteMultipartUploadResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateMediaRequest {
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
        if self.file.is_some() {
            len += 1;
        }
        if !self.file_name.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sited_io.media.v1.CreateMediaRequest", len)?;
        if !self.shop_id.is_empty() {
            struct_ser.serialize_field("shopId", &self.shop_id)?;
        }
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if let Some(v) = self.file.as_ref() {
            struct_ser.serialize_field("file", v)?;
        }
        if !self.file_name.is_empty() {
            struct_ser.serialize_field("fileName", &self.file_name)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateMediaRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "shop_id",
            "shopId",
            "name",
            "file",
            "file_name",
            "fileName",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ShopId,
            Name,
            File,
            FileName,
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
                            "file" => Ok(GeneratedField::File),
                            "fileName" | "file_name" => Ok(GeneratedField::FileName),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateMediaRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.media.v1.CreateMediaRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateMediaRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut shop_id__ = None;
                let mut name__ = None;
                let mut file__ = None;
                let mut file_name__ = None;
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
                        GeneratedField::File => {
                            if file__.is_some() {
                                return Err(serde::de::Error::duplicate_field("file"));
                            }
                            file__ = map_.next_value()?;
                        }
                        GeneratedField::FileName => {
                            if file_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fileName"));
                            }
                            file_name__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(CreateMediaRequest {
                    shop_id: shop_id__.unwrap_or_default(),
                    name: name__.unwrap_or_default(),
                    file: file__,
                    file_name: file_name__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sited_io.media.v1.CreateMediaRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateMediaResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.media.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sited_io.media.v1.CreateMediaResponse", len)?;
        if let Some(v) = self.media.as_ref() {
            struct_ser.serialize_field("media", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateMediaResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "media",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Media,
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
                            "media" => Ok(GeneratedField::Media),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateMediaResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.media.v1.CreateMediaResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateMediaResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut media__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Media => {
                            if media__.is_some() {
                                return Err(serde::de::Error::duplicate_field("media"));
                            }
                            media__ = map_.next_value()?;
                        }
                    }
                }
                Ok(CreateMediaResponse {
                    media: media__,
                })
            }
        }
        deserializer.deserialize_struct("sited_io.media.v1.CreateMediaResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeleteMediaRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.media_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sited_io.media.v1.DeleteMediaRequest", len)?;
        if !self.media_id.is_empty() {
            struct_ser.serialize_field("mediaId", &self.media_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeleteMediaRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "media_id",
            "mediaId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            MediaId,
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
                            "mediaId" | "media_id" => Ok(GeneratedField::MediaId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DeleteMediaRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.media.v1.DeleteMediaRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DeleteMediaRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut media_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::MediaId => {
                            if media_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("mediaId"));
                            }
                            media_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(DeleteMediaRequest {
                    media_id: media_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sited_io.media.v1.DeleteMediaRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeleteMediaResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("sited_io.media.v1.DeleteMediaResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeleteMediaResponse {
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
            type Value = DeleteMediaResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.media.v1.DeleteMediaResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DeleteMediaResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(DeleteMediaResponse {
                })
            }
        }
        deserializer.deserialize_struct("sited_io.media.v1.DeleteMediaResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DownloadMediaRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.media_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sited_io.media.v1.DownloadMediaRequest", len)?;
        if !self.media_id.is_empty() {
            struct_ser.serialize_field("mediaId", &self.media_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DownloadMediaRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "media_id",
            "mediaId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            MediaId,
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
                            "mediaId" | "media_id" => Ok(GeneratedField::MediaId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DownloadMediaRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.media.v1.DownloadMediaRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DownloadMediaRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut media_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::MediaId => {
                            if media_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("mediaId"));
                            }
                            media_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(DownloadMediaRequest {
                    media_id: media_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sited_io.media.v1.DownloadMediaRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DownloadMediaResponse {
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
        let mut struct_ser = serializer.serialize_struct("sited_io.media.v1.DownloadMediaResponse", len)?;
        if !self.download_url.is_empty() {
            struct_ser.serialize_field("downloadUrl", &self.download_url)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DownloadMediaResponse {
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
            type Value = DownloadMediaResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.media.v1.DownloadMediaResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DownloadMediaResponse, V::Error>
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
                Ok(DownloadMediaResponse {
                    download_url: download_url__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sited_io.media.v1.DownloadMediaResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetMediaRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.media_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sited_io.media.v1.GetMediaRequest", len)?;
        if !self.media_id.is_empty() {
            struct_ser.serialize_field("mediaId", &self.media_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetMediaRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "media_id",
            "mediaId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            MediaId,
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
                            "mediaId" | "media_id" => Ok(GeneratedField::MediaId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetMediaRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.media.v1.GetMediaRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetMediaRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut media_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::MediaId => {
                            if media_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("mediaId"));
                            }
                            media_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(GetMediaRequest {
                    media_id: media_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sited_io.media.v1.GetMediaRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetMediaResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.media.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sited_io.media.v1.GetMediaResponse", len)?;
        if let Some(v) = self.media.as_ref() {
            struct_ser.serialize_field("media", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetMediaResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "media",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Media,
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
                            "media" => Ok(GeneratedField::Media),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetMediaResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.media.v1.GetMediaResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetMediaResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut media__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Media => {
                            if media__.is_some() {
                                return Err(serde::de::Error::duplicate_field("media"));
                            }
                            media__ = map_.next_value()?;
                        }
                    }
                }
                Ok(GetMediaResponse {
                    media: media__,
                })
            }
        }
        deserializer.deserialize_struct("sited_io.media.v1.GetMediaResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetMediaSubscriptionRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.media_subscription_id.is_some() {
            len += 1;
        }
        if self.offer_id.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sited_io.media.v1.GetMediaSubscriptionRequest", len)?;
        if let Some(v) = self.media_subscription_id.as_ref() {
            struct_ser.serialize_field("mediaSubscriptionId", v)?;
        }
        if let Some(v) = self.offer_id.as_ref() {
            struct_ser.serialize_field("offerId", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetMediaSubscriptionRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "media_subscription_id",
            "mediaSubscriptionId",
            "offer_id",
            "offerId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            MediaSubscriptionId,
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
                            "mediaSubscriptionId" | "media_subscription_id" => Ok(GeneratedField::MediaSubscriptionId),
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
            type Value = GetMediaSubscriptionRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.media.v1.GetMediaSubscriptionRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetMediaSubscriptionRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut media_subscription_id__ = None;
                let mut offer_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::MediaSubscriptionId => {
                            if media_subscription_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("mediaSubscriptionId"));
                            }
                            media_subscription_id__ = map_.next_value()?;
                        }
                        GeneratedField::OfferId => {
                            if offer_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("offerId"));
                            }
                            offer_id__ = map_.next_value()?;
                        }
                    }
                }
                Ok(GetMediaSubscriptionRequest {
                    media_subscription_id: media_subscription_id__,
                    offer_id: offer_id__,
                })
            }
        }
        deserializer.deserialize_struct("sited_io.media.v1.GetMediaSubscriptionRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetMediaSubscriptionResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.media_subscription.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sited_io.media.v1.GetMediaSubscriptionResponse", len)?;
        if let Some(v) = self.media_subscription.as_ref() {
            struct_ser.serialize_field("mediaSubscription", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetMediaSubscriptionResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "media_subscription",
            "mediaSubscription",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            MediaSubscription,
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
                            "mediaSubscription" | "media_subscription" => Ok(GeneratedField::MediaSubscription),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetMediaSubscriptionResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.media.v1.GetMediaSubscriptionResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetMediaSubscriptionResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut media_subscription__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::MediaSubscription => {
                            if media_subscription__.is_some() {
                                return Err(serde::de::Error::duplicate_field("mediaSubscription"));
                            }
                            media_subscription__ = map_.next_value()?;
                        }
                    }
                }
                Ok(GetMediaSubscriptionResponse {
                    media_subscription: media_subscription__,
                })
            }
        }
        deserializer.deserialize_struct("sited_io.media.v1.GetMediaSubscriptionResponse", FIELDS, GeneratedVisitor)
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
        if !self.media_id.is_empty() {
            len += 1;
        }
        if !self.content_type.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sited_io.media.v1.InitiateMultipartUploadRequest", len)?;
        if !self.media_id.is_empty() {
            struct_ser.serialize_field("mediaId", &self.media_id)?;
        }
        if !self.content_type.is_empty() {
            struct_ser.serialize_field("contentType", &self.content_type)?;
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
            "media_id",
            "mediaId",
            "content_type",
            "contentType",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            MediaId,
            ContentType,
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
                            "mediaId" | "media_id" => Ok(GeneratedField::MediaId),
                            "contentType" | "content_type" => Ok(GeneratedField::ContentType),
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
                formatter.write_str("struct sited_io.media.v1.InitiateMultipartUploadRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<InitiateMultipartUploadRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut media_id__ = None;
                let mut content_type__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::MediaId => {
                            if media_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("mediaId"));
                            }
                            media_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ContentType => {
                            if content_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("contentType"));
                            }
                            content_type__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(InitiateMultipartUploadRequest {
                    media_id: media_id__.unwrap_or_default(),
                    content_type: content_type__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sited_io.media.v1.InitiateMultipartUploadRequest", FIELDS, GeneratedVisitor)
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
        if !self.key.is_empty() {
            len += 1;
        }
        if !self.upload_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sited_io.media.v1.InitiateMultipartUploadResponse", len)?;
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
            "key",
            "upload_id",
            "uploadId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
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
                formatter.write_str("struct sited_io.media.v1.InitiateMultipartUploadResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<InitiateMultipartUploadResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut key__ = None;
                let mut upload_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
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
                    key: key__.unwrap_or_default(),
                    upload_id: upload_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sited_io.media.v1.InitiateMultipartUploadResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListAccessibleMediaRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.pagination.is_some() {
            len += 1;
        }
        if self.order_by.is_some() {
            len += 1;
        }
        if self.filter.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sited_io.media.v1.ListAccessibleMediaRequest", len)?;
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
impl<'de> serde::Deserialize<'de> for ListAccessibleMediaRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "pagination",
            "order_by",
            "orderBy",
            "filter",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
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
            type Value = ListAccessibleMediaRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.media.v1.ListAccessibleMediaRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListAccessibleMediaRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut pagination__ = None;
                let mut order_by__ = None;
                let mut filter__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
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
                Ok(ListAccessibleMediaRequest {
                    pagination: pagination__,
                    order_by: order_by__,
                    filter: filter__,
                })
            }
        }
        deserializer.deserialize_struct("sited_io.media.v1.ListAccessibleMediaRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListAccessibleMediaResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.medias.is_empty() {
            len += 1;
        }
        if self.pagination.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sited_io.media.v1.ListAccessibleMediaResponse", len)?;
        if !self.medias.is_empty() {
            struct_ser.serialize_field("medias", &self.medias)?;
        }
        if let Some(v) = self.pagination.as_ref() {
            struct_ser.serialize_field("pagination", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListAccessibleMediaResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "medias",
            "pagination",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Medias,
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
                            "medias" => Ok(GeneratedField::Medias),
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
            type Value = ListAccessibleMediaResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.media.v1.ListAccessibleMediaResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListAccessibleMediaResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut medias__ = None;
                let mut pagination__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Medias => {
                            if medias__.is_some() {
                                return Err(serde::de::Error::duplicate_field("medias"));
                            }
                            medias__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Pagination => {
                            if pagination__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pagination"));
                            }
                            pagination__ = map_.next_value()?;
                        }
                    }
                }
                Ok(ListAccessibleMediaResponse {
                    medias: medias__.unwrap_or_default(),
                    pagination: pagination__,
                })
            }
        }
        deserializer.deserialize_struct("sited_io.media.v1.ListAccessibleMediaResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListMediaRequest {
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
        if self.pagination.is_some() {
            len += 1;
        }
        if self.order_by.is_some() {
            len += 1;
        }
        if self.filter.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sited_io.media.v1.ListMediaRequest", len)?;
        if !self.shop_id.is_empty() {
            struct_ser.serialize_field("shopId", &self.shop_id)?;
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
impl<'de> serde::Deserialize<'de> for ListMediaRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "shop_id",
            "shopId",
            "pagination",
            "order_by",
            "orderBy",
            "filter",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
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
            type Value = ListMediaRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.media.v1.ListMediaRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListMediaRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut shop_id__ = None;
                let mut pagination__ = None;
                let mut order_by__ = None;
                let mut filter__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ShopId => {
                            if shop_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("shopId"));
                            }
                            shop_id__ = Some(map_.next_value()?);
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
                Ok(ListMediaRequest {
                    shop_id: shop_id__.unwrap_or_default(),
                    pagination: pagination__,
                    order_by: order_by__,
                    filter: filter__,
                })
            }
        }
        deserializer.deserialize_struct("sited_io.media.v1.ListMediaRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListMediaResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.medias.is_empty() {
            len += 1;
        }
        if self.pagination.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sited_io.media.v1.ListMediaResponse", len)?;
        if !self.medias.is_empty() {
            struct_ser.serialize_field("medias", &self.medias)?;
        }
        if let Some(v) = self.pagination.as_ref() {
            struct_ser.serialize_field("pagination", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListMediaResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "medias",
            "pagination",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Medias,
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
                            "medias" => Ok(GeneratedField::Medias),
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
            type Value = ListMediaResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.media.v1.ListMediaResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListMediaResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut medias__ = None;
                let mut pagination__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Medias => {
                            if medias__.is_some() {
                                return Err(serde::de::Error::duplicate_field("medias"));
                            }
                            medias__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Pagination => {
                            if pagination__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pagination"));
                            }
                            pagination__ = map_.next_value()?;
                        }
                    }
                }
                Ok(ListMediaResponse {
                    medias: medias__.unwrap_or_default(),
                    pagination: pagination__,
                })
            }
        }
        deserializer.deserialize_struct("sited_io.media.v1.ListMediaResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListMediaSubscriptionsRequest {
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
        if self.pagination.is_some() {
            len += 1;
        }
        if self.is_accessible.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sited_io.media.v1.ListMediaSubscriptionsRequest", len)?;
        if let Some(v) = self.shop_id.as_ref() {
            struct_ser.serialize_field("shopId", v)?;
        }
        if let Some(v) = self.pagination.as_ref() {
            struct_ser.serialize_field("pagination", v)?;
        }
        if let Some(v) = self.is_accessible.as_ref() {
            struct_ser.serialize_field("isAccessible", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListMediaSubscriptionsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "shop_id",
            "shopId",
            "pagination",
            "is_accessible",
            "isAccessible",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ShopId,
            Pagination,
            IsAccessible,
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
                            "pagination" => Ok(GeneratedField::Pagination),
                            "isAccessible" | "is_accessible" => Ok(GeneratedField::IsAccessible),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListMediaSubscriptionsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.media.v1.ListMediaSubscriptionsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListMediaSubscriptionsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut shop_id__ = None;
                let mut pagination__ = None;
                let mut is_accessible__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
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
                        GeneratedField::IsAccessible => {
                            if is_accessible__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isAccessible"));
                            }
                            is_accessible__ = map_.next_value()?;
                        }
                    }
                }
                Ok(ListMediaSubscriptionsRequest {
                    shop_id: shop_id__,
                    pagination: pagination__,
                    is_accessible: is_accessible__,
                })
            }
        }
        deserializer.deserialize_struct("sited_io.media.v1.ListMediaSubscriptionsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListMediaSubscriptionsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.media_subscriptions.is_empty() {
            len += 1;
        }
        if self.pagination.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sited_io.media.v1.ListMediaSubscriptionsResponse", len)?;
        if !self.media_subscriptions.is_empty() {
            struct_ser.serialize_field("mediaSubscriptions", &self.media_subscriptions)?;
        }
        if let Some(v) = self.pagination.as_ref() {
            struct_ser.serialize_field("pagination", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListMediaSubscriptionsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "media_subscriptions",
            "mediaSubscriptions",
            "pagination",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            MediaSubscriptions,
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
                            "mediaSubscriptions" | "media_subscriptions" => Ok(GeneratedField::MediaSubscriptions),
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
            type Value = ListMediaSubscriptionsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.media.v1.ListMediaSubscriptionsResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListMediaSubscriptionsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut media_subscriptions__ = None;
                let mut pagination__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::MediaSubscriptions => {
                            if media_subscriptions__.is_some() {
                                return Err(serde::de::Error::duplicate_field("mediaSubscriptions"));
                            }
                            media_subscriptions__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Pagination => {
                            if pagination__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pagination"));
                            }
                            pagination__ = map_.next_value()?;
                        }
                    }
                }
                Ok(ListMediaSubscriptionsResponse {
                    media_subscriptions: media_subscriptions__.unwrap_or_default(),
                    pagination: pagination__,
                })
            }
        }
        deserializer.deserialize_struct("sited_io.media.v1.ListMediaSubscriptionsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MediaFilter {
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
        let mut struct_ser = serializer.serialize_struct("sited_io.media.v1.MediaFilter", len)?;
        if self.field != 0 {
            let v = MediaFilterField::try_from(self.field)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.field)))?;
            struct_ser.serialize_field("field", &v)?;
        }
        if !self.query.is_empty() {
            struct_ser.serialize_field("query", &self.query)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MediaFilter {
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
            type Value = MediaFilter;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.media.v1.MediaFilter")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MediaFilter, V::Error>
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
                            field__ = Some(map_.next_value::<MediaFilterField>()? as i32);
                        }
                        GeneratedField::Query => {
                            if query__.is_some() {
                                return Err(serde::de::Error::duplicate_field("query"));
                            }
                            query__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(MediaFilter {
                    field: field__.unwrap_or_default(),
                    query: query__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sited_io.media.v1.MediaFilter", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MediaFilterField {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "MEDIA_FILTER_FIELD_UNSPECIFIED",
            Self::Name => "MEDIA_FILTER_FIELD_NAME",
            Self::OfferId => "MEDIA_FILTER_FIELD_OFFER_ID",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for MediaFilterField {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "MEDIA_FILTER_FIELD_UNSPECIFIED",
            "MEDIA_FILTER_FIELD_NAME",
            "MEDIA_FILTER_FIELD_OFFER_ID",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MediaFilterField;

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
                    "MEDIA_FILTER_FIELD_UNSPECIFIED" => Ok(MediaFilterField::Unspecified),
                    "MEDIA_FILTER_FIELD_NAME" => Ok(MediaFilterField::Name),
                    "MEDIA_FILTER_FIELD_OFFER_ID" => Ok(MediaFilterField::OfferId),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for MediaOrderBy {
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
        let mut struct_ser = serializer.serialize_struct("sited_io.media.v1.MediaOrderBy", len)?;
        if self.field != 0 {
            let v = MediaOrderByField::try_from(self.field)
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
impl<'de> serde::Deserialize<'de> for MediaOrderBy {
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
            type Value = MediaOrderBy;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.media.v1.MediaOrderBy")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MediaOrderBy, V::Error>
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
                            field__ = Some(map_.next_value::<MediaOrderByField>()? as i32);
                        }
                        GeneratedField::Direction => {
                            if direction__.is_some() {
                                return Err(serde::de::Error::duplicate_field("direction"));
                            }
                            direction__ = Some(map_.next_value::<super::super::types::v1::Direction>()? as i32);
                        }
                    }
                }
                Ok(MediaOrderBy {
                    field: field__.unwrap_or_default(),
                    direction: direction__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sited_io.media.v1.MediaOrderBy", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MediaOrderByField {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "MEDIA_ORDER_BY_FIELD_UNSPECIFIED",
            Self::CreatedAt => "MEDIA_ORDER_BY_FIELD_CREATED_AT",
            Self::UpdatedAt => "MEDIA_ORDER_BY_FIELD_UPDATED_AT",
            Self::Ordering => "MEDIA_ORDER_BY_FIELD_ORDERING",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for MediaOrderByField {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "MEDIA_ORDER_BY_FIELD_UNSPECIFIED",
            "MEDIA_ORDER_BY_FIELD_CREATED_AT",
            "MEDIA_ORDER_BY_FIELD_UPDATED_AT",
            "MEDIA_ORDER_BY_FIELD_ORDERING",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MediaOrderByField;

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
                    "MEDIA_ORDER_BY_FIELD_UNSPECIFIED" => Ok(MediaOrderByField::Unspecified),
                    "MEDIA_ORDER_BY_FIELD_CREATED_AT" => Ok(MediaOrderByField::CreatedAt),
                    "MEDIA_ORDER_BY_FIELD_UPDATED_AT" => Ok(MediaOrderByField::UpdatedAt),
                    "MEDIA_ORDER_BY_FIELD_ORDERING" => Ok(MediaOrderByField::Ordering),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for MediaResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.media_id.is_empty() {
            len += 1;
        }
        if !self.offer_ids.is_empty() {
            len += 1;
        }
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
        if !self.file_name.is_empty() {
            len += 1;
        }
        if self.ordering != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sited_io.media.v1.MediaResponse", len)?;
        if !self.media_id.is_empty() {
            struct_ser.serialize_field("mediaId", &self.media_id)?;
        }
        if !self.offer_ids.is_empty() {
            struct_ser.serialize_field("offerIds", &self.offer_ids)?;
        }
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
        if !self.file_name.is_empty() {
            struct_ser.serialize_field("fileName", &self.file_name)?;
        }
        if self.ordering != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("ordering", ToString::to_string(&self.ordering).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MediaResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "media_id",
            "mediaId",
            "offer_ids",
            "offerIds",
            "shop_id",
            "shopId",
            "user_id",
            "userId",
            "created_at",
            "createdAt",
            "updated_at",
            "updatedAt",
            "name",
            "file_name",
            "fileName",
            "ordering",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            MediaId,
            OfferIds,
            ShopId,
            UserId,
            CreatedAt,
            UpdatedAt,
            Name,
            FileName,
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
                            "mediaId" | "media_id" => Ok(GeneratedField::MediaId),
                            "offerIds" | "offer_ids" => Ok(GeneratedField::OfferIds),
                            "shopId" | "shop_id" => Ok(GeneratedField::ShopId),
                            "userId" | "user_id" => Ok(GeneratedField::UserId),
                            "createdAt" | "created_at" => Ok(GeneratedField::CreatedAt),
                            "updatedAt" | "updated_at" => Ok(GeneratedField::UpdatedAt),
                            "name" => Ok(GeneratedField::Name),
                            "fileName" | "file_name" => Ok(GeneratedField::FileName),
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
            type Value = MediaResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.media.v1.MediaResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MediaResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut media_id__ = None;
                let mut offer_ids__ = None;
                let mut shop_id__ = None;
                let mut user_id__ = None;
                let mut created_at__ = None;
                let mut updated_at__ = None;
                let mut name__ = None;
                let mut file_name__ = None;
                let mut ordering__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::MediaId => {
                            if media_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("mediaId"));
                            }
                            media_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::OfferIds => {
                            if offer_ids__.is_some() {
                                return Err(serde::de::Error::duplicate_field("offerIds"));
                            }
                            offer_ids__ = Some(map_.next_value()?);
                        }
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
                        GeneratedField::FileName => {
                            if file_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fileName"));
                            }
                            file_name__ = Some(map_.next_value()?);
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
                Ok(MediaResponse {
                    media_id: media_id__.unwrap_or_default(),
                    offer_ids: offer_ids__.unwrap_or_default(),
                    shop_id: shop_id__.unwrap_or_default(),
                    user_id: user_id__.unwrap_or_default(),
                    created_at: created_at__.unwrap_or_default(),
                    updated_at: updated_at__.unwrap_or_default(),
                    name: name__.unwrap_or_default(),
                    file_name: file_name__.unwrap_or_default(),
                    ordering: ordering__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sited_io.media.v1.MediaResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MediaSubscriptionResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.media_subscription_id.is_empty() {
            len += 1;
        }
        if !self.buyer_user_id.is_empty() {
            len += 1;
        }
        if !self.shop_id.is_empty() {
            len += 1;
        }
        if !self.offer_id.is_empty() {
            len += 1;
        }
        if self.current_period_start != 0 {
            len += 1;
        }
        if self.current_period_end != 0 {
            len += 1;
        }
        if !self.subscription_status.is_empty() {
            len += 1;
        }
        if self.payed_at != 0 {
            len += 1;
        }
        if self.payed_until != 0 {
            len += 1;
        }
        if self.stripe_subscription_id.is_some() {
            len += 1;
        }
        if self.canceled_at.is_some() {
            len += 1;
        }
        if self.cancel_at.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sited_io.media.v1.MediaSubscriptionResponse", len)?;
        if !self.media_subscription_id.is_empty() {
            struct_ser.serialize_field("mediaSubscriptionId", &self.media_subscription_id)?;
        }
        if !self.buyer_user_id.is_empty() {
            struct_ser.serialize_field("buyerUserId", &self.buyer_user_id)?;
        }
        if !self.shop_id.is_empty() {
            struct_ser.serialize_field("shopId", &self.shop_id)?;
        }
        if !self.offer_id.is_empty() {
            struct_ser.serialize_field("offerId", &self.offer_id)?;
        }
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
        if !self.subscription_status.is_empty() {
            struct_ser.serialize_field("subscriptionStatus", &self.subscription_status)?;
        }
        if self.payed_at != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("payedAt", ToString::to_string(&self.payed_at).as_str())?;
        }
        if self.payed_until != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("payedUntil", ToString::to_string(&self.payed_until).as_str())?;
        }
        if let Some(v) = self.stripe_subscription_id.as_ref() {
            struct_ser.serialize_field("stripeSubscriptionId", v)?;
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
impl<'de> serde::Deserialize<'de> for MediaSubscriptionResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "media_subscription_id",
            "mediaSubscriptionId",
            "buyer_user_id",
            "buyerUserId",
            "shop_id",
            "shopId",
            "offer_id",
            "offerId",
            "current_period_start",
            "currentPeriodStart",
            "current_period_end",
            "currentPeriodEnd",
            "subscription_status",
            "subscriptionStatus",
            "payed_at",
            "payedAt",
            "payed_until",
            "payedUntil",
            "stripe_subscription_id",
            "stripeSubscriptionId",
            "canceled_at",
            "canceledAt",
            "cancel_at",
            "cancelAt",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            MediaSubscriptionId,
            BuyerUserId,
            ShopId,
            OfferId,
            CurrentPeriodStart,
            CurrentPeriodEnd,
            SubscriptionStatus,
            PayedAt,
            PayedUntil,
            StripeSubscriptionId,
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
                            "mediaSubscriptionId" | "media_subscription_id" => Ok(GeneratedField::MediaSubscriptionId),
                            "buyerUserId" | "buyer_user_id" => Ok(GeneratedField::BuyerUserId),
                            "shopId" | "shop_id" => Ok(GeneratedField::ShopId),
                            "offerId" | "offer_id" => Ok(GeneratedField::OfferId),
                            "currentPeriodStart" | "current_period_start" => Ok(GeneratedField::CurrentPeriodStart),
                            "currentPeriodEnd" | "current_period_end" => Ok(GeneratedField::CurrentPeriodEnd),
                            "subscriptionStatus" | "subscription_status" => Ok(GeneratedField::SubscriptionStatus),
                            "payedAt" | "payed_at" => Ok(GeneratedField::PayedAt),
                            "payedUntil" | "payed_until" => Ok(GeneratedField::PayedUntil),
                            "stripeSubscriptionId" | "stripe_subscription_id" => Ok(GeneratedField::StripeSubscriptionId),
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
            type Value = MediaSubscriptionResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.media.v1.MediaSubscriptionResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MediaSubscriptionResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut media_subscription_id__ = None;
                let mut buyer_user_id__ = None;
                let mut shop_id__ = None;
                let mut offer_id__ = None;
                let mut current_period_start__ = None;
                let mut current_period_end__ = None;
                let mut subscription_status__ = None;
                let mut payed_at__ = None;
                let mut payed_until__ = None;
                let mut stripe_subscription_id__ = None;
                let mut canceled_at__ = None;
                let mut cancel_at__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::MediaSubscriptionId => {
                            if media_subscription_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("mediaSubscriptionId"));
                            }
                            media_subscription_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::BuyerUserId => {
                            if buyer_user_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("buyerUserId"));
                            }
                            buyer_user_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ShopId => {
                            if shop_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("shopId"));
                            }
                            shop_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::OfferId => {
                            if offer_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("offerId"));
                            }
                            offer_id__ = Some(map_.next_value()?);
                        }
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
                        GeneratedField::SubscriptionStatus => {
                            if subscription_status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("subscriptionStatus"));
                            }
                            subscription_status__ = Some(map_.next_value()?);
                        }
                        GeneratedField::PayedAt => {
                            if payed_at__.is_some() {
                                return Err(serde::de::Error::duplicate_field("payedAt"));
                            }
                            payed_at__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::PayedUntil => {
                            if payed_until__.is_some() {
                                return Err(serde::de::Error::duplicate_field("payedUntil"));
                            }
                            payed_until__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::StripeSubscriptionId => {
                            if stripe_subscription_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("stripeSubscriptionId"));
                            }
                            stripe_subscription_id__ = map_.next_value()?;
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
                Ok(MediaSubscriptionResponse {
                    media_subscription_id: media_subscription_id__.unwrap_or_default(),
                    buyer_user_id: buyer_user_id__.unwrap_or_default(),
                    shop_id: shop_id__.unwrap_or_default(),
                    offer_id: offer_id__.unwrap_or_default(),
                    current_period_start: current_period_start__.unwrap_or_default(),
                    current_period_end: current_period_end__.unwrap_or_default(),
                    subscription_status: subscription_status__.unwrap_or_default(),
                    payed_at: payed_at__.unwrap_or_default(),
                    payed_until: payed_until__.unwrap_or_default(),
                    stripe_subscription_id: stripe_subscription_id__,
                    canceled_at: canceled_at__,
                    cancel_at: cancel_at__,
                })
            }
        }
        deserializer.deserialize_struct("sited_io.media.v1.MediaSubscriptionResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MediaUpload {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.content_type.is_empty() {
            len += 1;
        }
        if !self.data.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sited_io.media.v1.MediaUpload", len)?;
        if !self.content_type.is_empty() {
            struct_ser.serialize_field("contentType", &self.content_type)?;
        }
        if !self.data.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("data", pbjson::private::base64::encode(&self.data).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MediaUpload {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "content_type",
            "contentType",
            "data",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ContentType,
            Data,
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
                            "contentType" | "content_type" => Ok(GeneratedField::ContentType),
                            "data" => Ok(GeneratedField::Data),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MediaUpload;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.media.v1.MediaUpload")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MediaUpload, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut content_type__ = None;
                let mut data__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ContentType => {
                            if content_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("contentType"));
                            }
                            content_type__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Data => {
                            if data__.is_some() {
                                return Err(serde::de::Error::duplicate_field("data"));
                            }
                            data__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(MediaUpload {
                    content_type: content_type__.unwrap_or_default(),
                    data: data__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sited_io.media.v1.MediaUpload", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Part {
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
        let mut struct_ser = serializer.serialize_struct("sited_io.media.v1.Part", len)?;
        if self.part_number != 0 {
            struct_ser.serialize_field("partNumber", &self.part_number)?;
        }
        if !self.etag.is_empty() {
            struct_ser.serialize_field("etag", &self.etag)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Part {
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
            type Value = Part;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.media.v1.Part")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Part, V::Error>
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
                Ok(Part {
                    part_number: part_number__.unwrap_or_default(),
                    etag: etag__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sited_io.media.v1.Part", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PutMediaSubscriptionRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.media_subscription_id.is_empty() {
            len += 1;
        }
        if !self.buyer_user_id.is_empty() {
            len += 1;
        }
        if !self.offer_id.is_empty() {
            len += 1;
        }
        if self.current_period_start != 0 {
            len += 1;
        }
        if self.current_period_end != 0 {
            len += 1;
        }
        if !self.subscription_status.is_empty() {
            len += 1;
        }
        if self.payed_at != 0 {
            len += 1;
        }
        if self.payed_until != 0 {
            len += 1;
        }
        if !self.shop_id.is_empty() {
            len += 1;
        }
        if self.stripe_subscription_id.is_some() {
            len += 1;
        }
        if self.canceled_at.is_some() {
            len += 1;
        }
        if self.cancel_at.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sited_io.media.v1.PutMediaSubscriptionRequest", len)?;
        if !self.media_subscription_id.is_empty() {
            struct_ser.serialize_field("mediaSubscriptionId", &self.media_subscription_id)?;
        }
        if !self.buyer_user_id.is_empty() {
            struct_ser.serialize_field("buyerUserId", &self.buyer_user_id)?;
        }
        if !self.offer_id.is_empty() {
            struct_ser.serialize_field("offerId", &self.offer_id)?;
        }
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
        if !self.subscription_status.is_empty() {
            struct_ser.serialize_field("subscriptionStatus", &self.subscription_status)?;
        }
        if self.payed_at != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("payedAt", ToString::to_string(&self.payed_at).as_str())?;
        }
        if self.payed_until != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("payedUntil", ToString::to_string(&self.payed_until).as_str())?;
        }
        if !self.shop_id.is_empty() {
            struct_ser.serialize_field("shopId", &self.shop_id)?;
        }
        if let Some(v) = self.stripe_subscription_id.as_ref() {
            struct_ser.serialize_field("stripeSubscriptionId", v)?;
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
impl<'de> serde::Deserialize<'de> for PutMediaSubscriptionRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "media_subscription_id",
            "mediaSubscriptionId",
            "buyer_user_id",
            "buyerUserId",
            "offer_id",
            "offerId",
            "current_period_start",
            "currentPeriodStart",
            "current_period_end",
            "currentPeriodEnd",
            "subscription_status",
            "subscriptionStatus",
            "payed_at",
            "payedAt",
            "payed_until",
            "payedUntil",
            "shop_id",
            "shopId",
            "stripe_subscription_id",
            "stripeSubscriptionId",
            "canceled_at",
            "canceledAt",
            "cancel_at",
            "cancelAt",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            MediaSubscriptionId,
            BuyerUserId,
            OfferId,
            CurrentPeriodStart,
            CurrentPeriodEnd,
            SubscriptionStatus,
            PayedAt,
            PayedUntil,
            ShopId,
            StripeSubscriptionId,
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
                            "mediaSubscriptionId" | "media_subscription_id" => Ok(GeneratedField::MediaSubscriptionId),
                            "buyerUserId" | "buyer_user_id" => Ok(GeneratedField::BuyerUserId),
                            "offerId" | "offer_id" => Ok(GeneratedField::OfferId),
                            "currentPeriodStart" | "current_period_start" => Ok(GeneratedField::CurrentPeriodStart),
                            "currentPeriodEnd" | "current_period_end" => Ok(GeneratedField::CurrentPeriodEnd),
                            "subscriptionStatus" | "subscription_status" => Ok(GeneratedField::SubscriptionStatus),
                            "payedAt" | "payed_at" => Ok(GeneratedField::PayedAt),
                            "payedUntil" | "payed_until" => Ok(GeneratedField::PayedUntil),
                            "shopId" | "shop_id" => Ok(GeneratedField::ShopId),
                            "stripeSubscriptionId" | "stripe_subscription_id" => Ok(GeneratedField::StripeSubscriptionId),
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
            type Value = PutMediaSubscriptionRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.media.v1.PutMediaSubscriptionRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<PutMediaSubscriptionRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut media_subscription_id__ = None;
                let mut buyer_user_id__ = None;
                let mut offer_id__ = None;
                let mut current_period_start__ = None;
                let mut current_period_end__ = None;
                let mut subscription_status__ = None;
                let mut payed_at__ = None;
                let mut payed_until__ = None;
                let mut shop_id__ = None;
                let mut stripe_subscription_id__ = None;
                let mut canceled_at__ = None;
                let mut cancel_at__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::MediaSubscriptionId => {
                            if media_subscription_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("mediaSubscriptionId"));
                            }
                            media_subscription_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::BuyerUserId => {
                            if buyer_user_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("buyerUserId"));
                            }
                            buyer_user_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::OfferId => {
                            if offer_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("offerId"));
                            }
                            offer_id__ = Some(map_.next_value()?);
                        }
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
                        GeneratedField::SubscriptionStatus => {
                            if subscription_status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("subscriptionStatus"));
                            }
                            subscription_status__ = Some(map_.next_value()?);
                        }
                        GeneratedField::PayedAt => {
                            if payed_at__.is_some() {
                                return Err(serde::de::Error::duplicate_field("payedAt"));
                            }
                            payed_at__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::PayedUntil => {
                            if payed_until__.is_some() {
                                return Err(serde::de::Error::duplicate_field("payedUntil"));
                            }
                            payed_until__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::ShopId => {
                            if shop_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("shopId"));
                            }
                            shop_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::StripeSubscriptionId => {
                            if stripe_subscription_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("stripeSubscriptionId"));
                            }
                            stripe_subscription_id__ = map_.next_value()?;
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
                Ok(PutMediaSubscriptionRequest {
                    media_subscription_id: media_subscription_id__.unwrap_or_default(),
                    buyer_user_id: buyer_user_id__.unwrap_or_default(),
                    offer_id: offer_id__.unwrap_or_default(),
                    current_period_start: current_period_start__.unwrap_or_default(),
                    current_period_end: current_period_end__.unwrap_or_default(),
                    subscription_status: subscription_status__.unwrap_or_default(),
                    payed_at: payed_at__.unwrap_or_default(),
                    payed_until: payed_until__.unwrap_or_default(),
                    shop_id: shop_id__.unwrap_or_default(),
                    stripe_subscription_id: stripe_subscription_id__,
                    canceled_at: canceled_at__,
                    cancel_at: cancel_at__,
                })
            }
        }
        deserializer.deserialize_struct("sited_io.media.v1.PutMediaSubscriptionRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PutMediaSubscriptionResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("sited_io.media.v1.PutMediaSubscriptionResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PutMediaSubscriptionResponse {
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
            type Value = PutMediaSubscriptionResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.media.v1.PutMediaSubscriptionResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<PutMediaSubscriptionResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(PutMediaSubscriptionResponse {
                })
            }
        }
        deserializer.deserialize_struct("sited_io.media.v1.PutMediaSubscriptionResponse", FIELDS, GeneratedVisitor)
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
        if !self.media_id.is_empty() {
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
        let mut struct_ser = serializer.serialize_struct("sited_io.media.v1.PutMultipartChunkRequest", len)?;
        if !self.media_id.is_empty() {
            struct_ser.serialize_field("mediaId", &self.media_id)?;
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
            "media_id",
            "mediaId",
            "upload_id",
            "uploadId",
            "part_number",
            "partNumber",
            "chunk",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            MediaId,
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
                            "mediaId" | "media_id" => Ok(GeneratedField::MediaId),
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
                formatter.write_str("struct sited_io.media.v1.PutMultipartChunkRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<PutMultipartChunkRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut media_id__ = None;
                let mut upload_id__ = None;
                let mut part_number__ = None;
                let mut chunk__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::MediaId => {
                            if media_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("mediaId"));
                            }
                            media_id__ = Some(map_.next_value()?);
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
                    media_id: media_id__.unwrap_or_default(),
                    upload_id: upload_id__.unwrap_or_default(),
                    part_number: part_number__.unwrap_or_default(),
                    chunk: chunk__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sited_io.media.v1.PutMultipartChunkRequest", FIELDS, GeneratedVisitor)
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
        let mut struct_ser = serializer.serialize_struct("sited_io.media.v1.PutMultipartChunkResponse", len)?;
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
                formatter.write_str("struct sited_io.media.v1.PutMultipartChunkResponse")
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
        deserializer.deserialize_struct("sited_io.media.v1.PutMultipartChunkResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RemoveMediaFromOfferRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.media_id.is_empty() {
            len += 1;
        }
        if !self.offer_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sited_io.media.v1.RemoveMediaFromOfferRequest", len)?;
        if !self.media_id.is_empty() {
            struct_ser.serialize_field("mediaId", &self.media_id)?;
        }
        if !self.offer_id.is_empty() {
            struct_ser.serialize_field("offerId", &self.offer_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RemoveMediaFromOfferRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "media_id",
            "mediaId",
            "offer_id",
            "offerId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            MediaId,
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
                            "mediaId" | "media_id" => Ok(GeneratedField::MediaId),
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
            type Value = RemoveMediaFromOfferRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.media.v1.RemoveMediaFromOfferRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RemoveMediaFromOfferRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut media_id__ = None;
                let mut offer_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::MediaId => {
                            if media_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("mediaId"));
                            }
                            media_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::OfferId => {
                            if offer_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("offerId"));
                            }
                            offer_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(RemoveMediaFromOfferRequest {
                    media_id: media_id__.unwrap_or_default(),
                    offer_id: offer_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sited_io.media.v1.RemoveMediaFromOfferRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RemoveMediaFromOfferResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("sited_io.media.v1.RemoveMediaFromOfferResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RemoveMediaFromOfferResponse {
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
            type Value = RemoveMediaFromOfferResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.media.v1.RemoveMediaFromOfferResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RemoveMediaFromOfferResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(RemoveMediaFromOfferResponse {
                })
            }
        }
        deserializer.deserialize_struct("sited_io.media.v1.RemoveMediaFromOfferResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ResumeMediaSubscriptionRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.media_subscription_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sited_io.media.v1.ResumeMediaSubscriptionRequest", len)?;
        if !self.media_subscription_id.is_empty() {
            struct_ser.serialize_field("mediaSubscriptionId", &self.media_subscription_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ResumeMediaSubscriptionRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "media_subscription_id",
            "mediaSubscriptionId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            MediaSubscriptionId,
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
                            "mediaSubscriptionId" | "media_subscription_id" => Ok(GeneratedField::MediaSubscriptionId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ResumeMediaSubscriptionRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.media.v1.ResumeMediaSubscriptionRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ResumeMediaSubscriptionRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut media_subscription_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::MediaSubscriptionId => {
                            if media_subscription_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("mediaSubscriptionId"));
                            }
                            media_subscription_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ResumeMediaSubscriptionRequest {
                    media_subscription_id: media_subscription_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sited_io.media.v1.ResumeMediaSubscriptionRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ResumeMediaSubscriptionResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("sited_io.media.v1.ResumeMediaSubscriptionResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ResumeMediaSubscriptionResponse {
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
            type Value = ResumeMediaSubscriptionResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.media.v1.ResumeMediaSubscriptionResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ResumeMediaSubscriptionResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(ResumeMediaSubscriptionResponse {
                })
            }
        }
        deserializer.deserialize_struct("sited_io.media.v1.ResumeMediaSubscriptionResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateMediaOfferOrderingRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.media_id.is_empty() {
            len += 1;
        }
        if !self.offer_id.is_empty() {
            len += 1;
        }
        if self.ordering != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sited_io.media.v1.UpdateMediaOfferOrderingRequest", len)?;
        if !self.media_id.is_empty() {
            struct_ser.serialize_field("mediaId", &self.media_id)?;
        }
        if !self.offer_id.is_empty() {
            struct_ser.serialize_field("offerId", &self.offer_id)?;
        }
        if self.ordering != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("ordering", ToString::to_string(&self.ordering).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateMediaOfferOrderingRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "media_id",
            "mediaId",
            "offer_id",
            "offerId",
            "ordering",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            MediaId,
            OfferId,
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
                            "mediaId" | "media_id" => Ok(GeneratedField::MediaId),
                            "offerId" | "offer_id" => Ok(GeneratedField::OfferId),
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
            type Value = UpdateMediaOfferOrderingRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.media.v1.UpdateMediaOfferOrderingRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateMediaOfferOrderingRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut media_id__ = None;
                let mut offer_id__ = None;
                let mut ordering__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::MediaId => {
                            if media_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("mediaId"));
                            }
                            media_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::OfferId => {
                            if offer_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("offerId"));
                            }
                            offer_id__ = Some(map_.next_value()?);
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
                Ok(UpdateMediaOfferOrderingRequest {
                    media_id: media_id__.unwrap_or_default(),
                    offer_id: offer_id__.unwrap_or_default(),
                    ordering: ordering__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sited_io.media.v1.UpdateMediaOfferOrderingRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateMediaOfferOrderingResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("sited_io.media.v1.UpdateMediaOfferOrderingResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateMediaOfferOrderingResponse {
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
            type Value = UpdateMediaOfferOrderingResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.media.v1.UpdateMediaOfferOrderingResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateMediaOfferOrderingResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(UpdateMediaOfferOrderingResponse {
                })
            }
        }
        deserializer.deserialize_struct("sited_io.media.v1.UpdateMediaOfferOrderingResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateMediaRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.media_id.is_empty() {
            len += 1;
        }
        if self.name.is_some() {
            len += 1;
        }
        if self.file.is_some() {
            len += 1;
        }
        if self.file_name.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sited_io.media.v1.UpdateMediaRequest", len)?;
        if !self.media_id.is_empty() {
            struct_ser.serialize_field("mediaId", &self.media_id)?;
        }
        if let Some(v) = self.name.as_ref() {
            struct_ser.serialize_field("name", v)?;
        }
        if let Some(v) = self.file.as_ref() {
            struct_ser.serialize_field("file", v)?;
        }
        if let Some(v) = self.file_name.as_ref() {
            struct_ser.serialize_field("fileName", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateMediaRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "media_id",
            "mediaId",
            "name",
            "file",
            "file_name",
            "fileName",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            MediaId,
            Name,
            File,
            FileName,
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
                            "mediaId" | "media_id" => Ok(GeneratedField::MediaId),
                            "name" => Ok(GeneratedField::Name),
                            "file" => Ok(GeneratedField::File),
                            "fileName" | "file_name" => Ok(GeneratedField::FileName),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UpdateMediaRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.media.v1.UpdateMediaRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateMediaRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut media_id__ = None;
                let mut name__ = None;
                let mut file__ = None;
                let mut file_name__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::MediaId => {
                            if media_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("mediaId"));
                            }
                            media_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = map_.next_value()?;
                        }
                        GeneratedField::File => {
                            if file__.is_some() {
                                return Err(serde::de::Error::duplicate_field("file"));
                            }
                            file__ = map_.next_value()?;
                        }
                        GeneratedField::FileName => {
                            if file_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fileName"));
                            }
                            file_name__ = map_.next_value()?;
                        }
                    }
                }
                Ok(UpdateMediaRequest {
                    media_id: media_id__.unwrap_or_default(),
                    name: name__,
                    file: file__,
                    file_name: file_name__,
                })
            }
        }
        deserializer.deserialize_struct("sited_io.media.v1.UpdateMediaRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateMediaResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.media.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sited_io.media.v1.UpdateMediaResponse", len)?;
        if let Some(v) = self.media.as_ref() {
            struct_ser.serialize_field("media", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateMediaResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "media",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Media,
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
                            "media" => Ok(GeneratedField::Media),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UpdateMediaResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.media.v1.UpdateMediaResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateMediaResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut media__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Media => {
                            if media__.is_some() {
                                return Err(serde::de::Error::duplicate_field("media"));
                            }
                            media__ = map_.next_value()?;
                        }
                    }
                }
                Ok(UpdateMediaResponse {
                    media: media__,
                })
            }
        }
        deserializer.deserialize_struct("sited_io.media.v1.UpdateMediaResponse", FIELDS, GeneratedVisitor)
    }
}

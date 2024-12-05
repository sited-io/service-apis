// @generated
impl serde::Serialize for CancelSubscriptionRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.stripe_subscription_id.is_empty() {
            len += 1;
        }
        if !self.shop_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sited_io.payment.v1.CancelSubscriptionRequest", len)?;
        if !self.stripe_subscription_id.is_empty() {
            struct_ser.serialize_field("stripeSubscriptionId", &self.stripe_subscription_id)?;
        }
        if !self.shop_id.is_empty() {
            struct_ser.serialize_field("shopId", &self.shop_id)?;
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
            "stripe_subscription_id",
            "stripeSubscriptionId",
            "shop_id",
            "shopId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            StripeSubscriptionId,
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
                            "stripeSubscriptionId" | "stripe_subscription_id" => Ok(GeneratedField::StripeSubscriptionId),
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
            type Value = CancelSubscriptionRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.payment.v1.CancelSubscriptionRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CancelSubscriptionRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut stripe_subscription_id__ = None;
                let mut shop_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::StripeSubscriptionId => {
                            if stripe_subscription_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("stripeSubscriptionId"));
                            }
                            stripe_subscription_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ShopId => {
                            if shop_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("shopId"));
                            }
                            shop_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(CancelSubscriptionRequest {
                    stripe_subscription_id: stripe_subscription_id__.unwrap_or_default(),
                    shop_id: shop_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sited_io.payment.v1.CancelSubscriptionRequest", FIELDS, GeneratedVisitor)
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
        let struct_ser = serializer.serialize_struct("sited_io.payment.v1.CancelSubscriptionResponse", len)?;
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
                formatter.write_str("struct sited_io.payment.v1.CancelSubscriptionResponse")
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
        deserializer.deserialize_struct("sited_io.payment.v1.CancelSubscriptionResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateAccountLinkRequest {
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
        if !self.refresh_url.is_empty() {
            len += 1;
        }
        if !self.return_url.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sited_io.payment.v1.CreateAccountLinkRequest", len)?;
        if !self.shop_id.is_empty() {
            struct_ser.serialize_field("shopId", &self.shop_id)?;
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
impl<'de> serde::Deserialize<'de> for CreateAccountLinkRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "shop_id",
            "shopId",
            "refresh_url",
            "refreshUrl",
            "return_url",
            "returnUrl",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ShopId,
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
                            "shopId" | "shop_id" => Ok(GeneratedField::ShopId),
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
            type Value = CreateAccountLinkRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.payment.v1.CreateAccountLinkRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateAccountLinkRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut shop_id__ = None;
                let mut refresh_url__ = None;
                let mut return_url__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ShopId => {
                            if shop_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("shopId"));
                            }
                            shop_id__ = Some(map_.next_value()?);
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
                Ok(CreateAccountLinkRequest {
                    shop_id: shop_id__.unwrap_or_default(),
                    refresh_url: refresh_url__.unwrap_or_default(),
                    return_url: return_url__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sited_io.payment.v1.CreateAccountLinkRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateAccountLinkResponse {
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
        let mut struct_ser = serializer.serialize_struct("sited_io.payment.v1.CreateAccountLinkResponse", len)?;
        if !self.link.is_empty() {
            struct_ser.serialize_field("link", &self.link)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateAccountLinkResponse {
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
            type Value = CreateAccountLinkResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.payment.v1.CreateAccountLinkResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateAccountLinkResponse, V::Error>
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
                Ok(CreateAccountLinkResponse {
                    link: link__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sited_io.payment.v1.CreateAccountLinkResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateAccountRequest {
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
        let mut struct_ser = serializer.serialize_struct("sited_io.payment.v1.CreateAccountRequest", len)?;
        if !self.shop_id.is_empty() {
            struct_ser.serialize_field("shopId", &self.shop_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateAccountRequest {
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
            type Value = CreateAccountRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.payment.v1.CreateAccountRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateAccountRequest, V::Error>
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
                Ok(CreateAccountRequest {
                    shop_id: shop_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sited_io.payment.v1.CreateAccountRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateAccountResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.account.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sited_io.payment.v1.CreateAccountResponse", len)?;
        if let Some(v) = self.account.as_ref() {
            struct_ser.serialize_field("account", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateAccountResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "account",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Account,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
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
                            "account" => Ok(GeneratedField::Account),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateAccountResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.payment.v1.CreateAccountResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateAccountResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut account__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Account => {
                            if account__.is_some() {
                                return Err(serde::de::Error::duplicate_field("account"));
                            }
                            account__ = map_.next_value()?;
                        }
                    }
                }
                Ok(CreateAccountResponse {
                    account: account__,
                })
            }
        }
        deserializer.deserialize_struct("sited_io.payment.v1.CreateAccountResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateCheckoutSessionRequest {
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
        if !self.success_url.is_empty() {
            len += 1;
        }
        if !self.cancel_url.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sited_io.payment.v1.CreateCheckoutSessionRequest", len)?;
        if !self.offer_id.is_empty() {
            struct_ser.serialize_field("offerId", &self.offer_id)?;
        }
        if !self.success_url.is_empty() {
            struct_ser.serialize_field("successUrl", &self.success_url)?;
        }
        if !self.cancel_url.is_empty() {
            struct_ser.serialize_field("cancelUrl", &self.cancel_url)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateCheckoutSessionRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "offer_id",
            "offerId",
            "success_url",
            "successUrl",
            "cancel_url",
            "cancelUrl",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            OfferId,
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
                            "offerId" | "offer_id" => Ok(GeneratedField::OfferId),
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
            type Value = CreateCheckoutSessionRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.payment.v1.CreateCheckoutSessionRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateCheckoutSessionRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut offer_id__ = None;
                let mut success_url__ = None;
                let mut cancel_url__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::OfferId => {
                            if offer_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("offerId"));
                            }
                            offer_id__ = Some(map_.next_value()?);
                        }
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
                Ok(CreateCheckoutSessionRequest {
                    offer_id: offer_id__.unwrap_or_default(),
                    success_url: success_url__.unwrap_or_default(),
                    cancel_url: cancel_url__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sited_io.payment.v1.CreateCheckoutSessionRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateCheckoutSessionResponse {
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
        let mut struct_ser = serializer.serialize_struct("sited_io.payment.v1.CreateCheckoutSessionResponse", len)?;
        if !self.link.is_empty() {
            struct_ser.serialize_field("link", &self.link)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateCheckoutSessionResponse {
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
            type Value = CreateCheckoutSessionResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.payment.v1.CreateCheckoutSessionResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateCheckoutSessionResponse, V::Error>
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
                Ok(CreateCheckoutSessionResponse {
                    link: link__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sited_io.payment.v1.CreateCheckoutSessionResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetAccountDetailsRequest {
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
        let mut struct_ser = serializer.serialize_struct("sited_io.payment.v1.GetAccountDetailsRequest", len)?;
        if !self.shop_id.is_empty() {
            struct_ser.serialize_field("shopId", &self.shop_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetAccountDetailsRequest {
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
            type Value = GetAccountDetailsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.payment.v1.GetAccountDetailsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetAccountDetailsRequest, V::Error>
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
                Ok(GetAccountDetailsRequest {
                    shop_id: shop_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sited_io.payment.v1.GetAccountDetailsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetAccountDetailsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.account.is_some() {
            len += 1;
        }
        if self.details.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sited_io.payment.v1.GetAccountDetailsResponse", len)?;
        if let Some(v) = self.account.as_ref() {
            struct_ser.serialize_field("account", v)?;
        }
        if let Some(v) = self.details.as_ref() {
            struct_ser.serialize_field("details", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetAccountDetailsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "account",
            "details",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Account,
            Details,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
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
                            "account" => Ok(GeneratedField::Account),
                            "details" => Ok(GeneratedField::Details),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetAccountDetailsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.payment.v1.GetAccountDetailsResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetAccountDetailsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut account__ = None;
                let mut details__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Account => {
                            if account__.is_some() {
                                return Err(serde::de::Error::duplicate_field("account"));
                            }
                            account__ = map_.next_value()?;
                        }
                        GeneratedField::Details => {
                            if details__.is_some() {
                                return Err(serde::de::Error::duplicate_field("details"));
                            }
                            details__ = map_.next_value()?;
                        }
                    }
                }
                Ok(GetAccountDetailsResponse {
                    account: account__,
                    details: details__,
                })
            }
        }
        deserializer.deserialize_struct("sited_io.payment.v1.GetAccountDetailsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetAccountRequest {
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
        let mut struct_ser = serializer.serialize_struct("sited_io.payment.v1.GetAccountRequest", len)?;
        if !self.shop_id.is_empty() {
            struct_ser.serialize_field("shopId", &self.shop_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetAccountRequest {
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
            type Value = GetAccountRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.payment.v1.GetAccountRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetAccountRequest, V::Error>
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
                Ok(GetAccountRequest {
                    shop_id: shop_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sited_io.payment.v1.GetAccountRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetAccountResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.account.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sited_io.payment.v1.GetAccountResponse", len)?;
        if let Some(v) = self.account.as_ref() {
            struct_ser.serialize_field("account", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetAccountResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "account",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Account,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
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
                            "account" => Ok(GeneratedField::Account),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetAccountResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.payment.v1.GetAccountResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetAccountResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut account__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Account => {
                            if account__.is_some() {
                                return Err(serde::de::Error::duplicate_field("account"));
                            }
                            account__ = map_.next_value()?;
                        }
                    }
                }
                Ok(GetAccountResponse {
                    account: account__,
                })
            }
        }
        deserializer.deserialize_struct("sited_io.payment.v1.GetAccountResponse", FIELDS, GeneratedVisitor)
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
        if !self.stripe_subscription_id.is_empty() {
            len += 1;
        }
        if !self.shop_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sited_io.payment.v1.ResumeSubscriptionRequest", len)?;
        if !self.stripe_subscription_id.is_empty() {
            struct_ser.serialize_field("stripeSubscriptionId", &self.stripe_subscription_id)?;
        }
        if !self.shop_id.is_empty() {
            struct_ser.serialize_field("shopId", &self.shop_id)?;
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
            "stripe_subscription_id",
            "stripeSubscriptionId",
            "shop_id",
            "shopId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            StripeSubscriptionId,
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
                            "stripeSubscriptionId" | "stripe_subscription_id" => Ok(GeneratedField::StripeSubscriptionId),
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
            type Value = ResumeSubscriptionRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.payment.v1.ResumeSubscriptionRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ResumeSubscriptionRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut stripe_subscription_id__ = None;
                let mut shop_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::StripeSubscriptionId => {
                            if stripe_subscription_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("stripeSubscriptionId"));
                            }
                            stripe_subscription_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ShopId => {
                            if shop_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("shopId"));
                            }
                            shop_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ResumeSubscriptionRequest {
                    stripe_subscription_id: stripe_subscription_id__.unwrap_or_default(),
                    shop_id: shop_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sited_io.payment.v1.ResumeSubscriptionRequest", FIELDS, GeneratedVisitor)
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
        let struct_ser = serializer.serialize_struct("sited_io.payment.v1.ResumeSubscriptionResponse", len)?;
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
                formatter.write_str("struct sited_io.payment.v1.ResumeSubscriptionResponse")
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
        deserializer.deserialize_struct("sited_io.payment.v1.ResumeSubscriptionResponse", FIELDS, GeneratedVisitor)
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
        if !self.shop_id.is_empty() {
            len += 1;
        }
        if !self.stripe_account_id.is_empty() {
            len += 1;
        }
        if self.enabled {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sited_io.payment.v1.StripeAccount", len)?;
        if !self.shop_id.is_empty() {
            struct_ser.serialize_field("shopId", &self.shop_id)?;
        }
        if !self.stripe_account_id.is_empty() {
            struct_ser.serialize_field("stripeAccountId", &self.stripe_account_id)?;
        }
        if self.enabled {
            struct_ser.serialize_field("enabled", &self.enabled)?;
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
            "shop_id",
            "shopId",
            "stripe_account_id",
            "stripeAccountId",
            "enabled",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ShopId,
            StripeAccountId,
            Enabled,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
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
                            "stripeAccountId" | "stripe_account_id" => Ok(GeneratedField::StripeAccountId),
                            "enabled" => Ok(GeneratedField::Enabled),
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
                formatter.write_str("struct sited_io.payment.v1.StripeAccount")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<StripeAccount, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut shop_id__ = None;
                let mut stripe_account_id__ = None;
                let mut enabled__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ShopId => {
                            if shop_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("shopId"));
                            }
                            shop_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::StripeAccountId => {
                            if stripe_account_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("stripeAccountId"));
                            }
                            stripe_account_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Enabled => {
                            if enabled__.is_some() {
                                return Err(serde::de::Error::duplicate_field("enabled"));
                            }
                            enabled__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(StripeAccount {
                    shop_id: shop_id__.unwrap_or_default(),
                    stripe_account_id: stripe_account_id__.unwrap_or_default(),
                    enabled: enabled__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sited_io.payment.v1.StripeAccount", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for StripeAccountDetails {
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
        let mut struct_ser = serializer.serialize_struct("sited_io.payment.v1.StripeAccountDetails", len)?;
        if self.charges_enabled {
            struct_ser.serialize_field("chargesEnabled", &self.charges_enabled)?;
        }
        if self.details_submitted {
            struct_ser.serialize_field("detailsSubmitted", &self.details_submitted)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for StripeAccountDetails {
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
            type Value = StripeAccountDetails;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sited_io.payment.v1.StripeAccountDetails")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<StripeAccountDetails, V::Error>
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
                Ok(StripeAccountDetails {
                    charges_enabled: charges_enabled__.unwrap_or_default(),
                    details_submitted: details_submitted__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sited_io.payment.v1.StripeAccountDetails", FIELDS, GeneratedVisitor)
    }
}

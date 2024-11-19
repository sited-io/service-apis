// @generated
pub mod sited_io {
    pub mod commerce {
        #[cfg(feature = "sited_io-commerce-v1")]
        // @@protoc_insertion_point(attribute:sited_io.commerce.v1)
        pub mod v1 {
            include!("sited_io.commerce.v1.rs");
            // @@protoc_insertion_point(sited_io.commerce.v1)
        }
        #[cfg(feature = "sited_io-commerce-v2")]
        // @@protoc_insertion_point(attribute:sited_io.commerce.v2)
        pub mod v2 {
            include!("sited_io.commerce.v2.rs");
            // @@protoc_insertion_point(sited_io.commerce.v2)
        }
    }
    pub mod media {
        #[cfg(feature = "sited_io-media-v1")]
        // @@protoc_insertion_point(attribute:sited_io.media.v1)
        pub mod v1 {
            include!("sited_io.media.v1.rs");
            // @@protoc_insertion_point(sited_io.media.v1)
        }
    }
    pub mod payment {
        #[cfg(feature = "sited_io-payment-v1")]
        // @@protoc_insertion_point(attribute:sited_io.payment.v1)
        pub mod v1 {
            include!("sited_io.payment.v1.rs");
            // @@protoc_insertion_point(sited_io.payment.v1)
        }
    }
    pub mod report {
        #[cfg(feature = "sited_io-report-v1")]
        // @@protoc_insertion_point(attribute:sited_io.report.v1)
        pub mod v1 {
            include!("sited_io.report.v1.rs");
            // @@protoc_insertion_point(sited_io.report.v1)
        }
    }
    pub mod types {
        pub mod country {
            #[cfg(feature = "sited_io-types-country-v1")]
            // @@protoc_insertion_point(attribute:sited_io.types.country.v1)
            pub mod v1 {
                include!("sited_io.types.country.v1.rs");
                // @@protoc_insertion_point(sited_io.types.country.v1)
            }
        }
        pub mod currency {
            #[cfg(feature = "sited_io-types-currency-v1")]
            // @@protoc_insertion_point(attribute:sited_io.types.currency.v1)
            pub mod v1 {
                include!("sited_io.types.currency.v1.rs");
                // @@protoc_insertion_point(sited_io.types.currency.v1)
            }
        }
        pub mod query {
            #[cfg(feature = "sited_io-types-query-v1")]
            // @@protoc_insertion_point(attribute:sited_io.types.query.v1)
            pub mod v1 {
                include!("sited_io.types.query.v1.rs");
                // @@protoc_insertion_point(sited_io.types.query.v1)
            }
        }
        #[cfg(feature = "sited_io-types-v1")]
        // @@protoc_insertion_point(attribute:sited_io.types.v1)
        pub mod v1 {
            include!("sited_io.types.v1.rs");
            // @@protoc_insertion_point(sited_io.types.v1)
        }
    }
    pub mod websites {
        #[cfg(feature = "sited_io-websites-v1")]
        // @@protoc_insertion_point(attribute:sited_io.websites.v1)
        pub mod v1 {
            include!("sited_io.websites.v1.rs");
            // @@protoc_insertion_point(sited_io.websites.v1)
        }
    }
}
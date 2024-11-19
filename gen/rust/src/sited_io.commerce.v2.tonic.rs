// @generated
/// Generated client implementations.
pub mod commerce_service_client {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct CommerceServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl CommerceServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> CommerceServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + std::marker::Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + std::marker::Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> CommerceServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + std::marker::Send + std::marker::Sync,
        {
            CommerceServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        pub async fn create_offer(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateOfferRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateOfferResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/sited_io.commerce.v2.CommerceService/CreateOffer",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sited_io.commerce.v2.CommerceService",
                        "CreateOffer",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_offer(
            &mut self,
            request: impl tonic::IntoRequest<super::GetOfferRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetOfferResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/sited_io.commerce.v2.CommerceService/GetOffer",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("sited_io.commerce.v2.CommerceService", "GetOffer"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_offers(
            &mut self,
            request: impl tonic::IntoRequest<super::ListOffersRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListOffersResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/sited_io.commerce.v2.CommerceService/ListOffers",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("sited_io.commerce.v2.CommerceService", "ListOffers"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_offer(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateOfferRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateOfferResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/sited_io.commerce.v2.CommerceService/UpdateOffer",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sited_io.commerce.v2.CommerceService",
                        "UpdateOffer",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn delete_offer(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteOfferRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeleteOfferResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/sited_io.commerce.v2.CommerceService/DeleteOffer",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sited_io.commerce.v2.CommerceService",
                        "DeleteOffer",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn put_price_to_offer(
            &mut self,
            request: impl tonic::IntoRequest<super::PutPriceToOfferRequest>,
        ) -> std::result::Result<
            tonic::Response<super::PutPriceToOfferResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/sited_io.commerce.v2.CommerceService/PutPriceToOffer",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sited_io.commerce.v2.CommerceService",
                        "PutPriceToOffer",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn remove_price_from_offer(
            &mut self,
            request: impl tonic::IntoRequest<super::RemovePriceFromOfferRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RemovePriceFromOfferResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/sited_io.commerce.v2.CommerceService/RemovePriceFromOffer",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sited_io.commerce.v2.CommerceService",
                        "RemovePriceFromOffer",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn put_shipping_rate_to_offer(
            &mut self,
            request: impl tonic::IntoRequest<super::PutShippingRateToOfferRequest>,
        ) -> std::result::Result<
            tonic::Response<super::PutShippingRateToOfferResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/sited_io.commerce.v2.CommerceService/PutShippingRateToOffer",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sited_io.commerce.v2.CommerceService",
                        "PutShippingRateToOffer",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn remove_shipping_rate_from_offer(
            &mut self,
            request: impl tonic::IntoRequest<super::RemoveShippingRateFromOfferRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RemoveShippingRateFromOfferResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/sited_io.commerce.v2.CommerceService/RemoveShippingRateFromOffer",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sited_io.commerce.v2.CommerceService",
                        "RemoveShippingRateFromOffer",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn add_image_to_offer(
            &mut self,
            request: impl tonic::IntoRequest<super::AddImageToOfferRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AddImageToOfferResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/sited_io.commerce.v2.CommerceService/AddImageToOffer",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sited_io.commerce.v2.CommerceService",
                        "AddImageToOffer",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_image_ordering(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateImageOrderingRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateImageOrderingResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/sited_io.commerce.v2.CommerceService/UpdateImageOrdering",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sited_io.commerce.v2.CommerceService",
                        "UpdateImageOrdering",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn remove_image_from_offer(
            &mut self,
            request: impl tonic::IntoRequest<super::RemoveImageFromOfferRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RemoveImageFromOfferResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/sited_io.commerce.v2.CommerceService/RemoveImageFromOffer",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sited_io.commerce.v2.CommerceService",
                        "RemoveImageFromOffer",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn add_file_to_offer(
            &mut self,
            request: impl tonic::IntoRequest<super::AddFileToOfferRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AddFileToOfferResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/sited_io.commerce.v2.CommerceService/AddFileToOffer",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sited_io.commerce.v2.CommerceService",
                        "AddFileToOffer",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn initiate_multipart_upload(
            &mut self,
            request: impl tonic::IntoRequest<super::InitiateMultipartUploadRequest>,
        ) -> std::result::Result<
            tonic::Response<super::InitiateMultipartUploadResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/sited_io.commerce.v2.CommerceService/InitiateMultipartUpload",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sited_io.commerce.v2.CommerceService",
                        "InitiateMultipartUpload",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn put_multipart_chunk(
            &mut self,
            request: impl tonic::IntoRequest<super::PutMultipartChunkRequest>,
        ) -> std::result::Result<
            tonic::Response<super::PutMultipartChunkResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/sited_io.commerce.v2.CommerceService/PutMultipartChunk",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sited_io.commerce.v2.CommerceService",
                        "PutMultipartChunk",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn complete_multipart_upload(
            &mut self,
            request: impl tonic::IntoRequest<super::CompleteMultipartUploadRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CompleteMultipartUploadResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/sited_io.commerce.v2.CommerceService/CompleteMultipartUpload",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sited_io.commerce.v2.CommerceService",
                        "CompleteMultipartUpload",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn download_file(
            &mut self,
            request: impl tonic::IntoRequest<super::DownloadFileRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DownloadFileResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/sited_io.commerce.v2.CommerceService/DownloadFile",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sited_io.commerce.v2.CommerceService",
                        "DownloadFile",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_file_ordering(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateFileOrderingRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateFileOrderingResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/sited_io.commerce.v2.CommerceService/UpdateFileOrdering",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sited_io.commerce.v2.CommerceService",
                        "UpdateFileOrdering",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn remove_file_from_offer(
            &mut self,
            request: impl tonic::IntoRequest<super::RemoveFileFromOfferRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RemoveFileFromOfferResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/sited_io.commerce.v2.CommerceService/RemoveFileFromOffer",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sited_io.commerce.v2.CommerceService",
                        "RemoveFileFromOffer",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn create_shop(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateShopRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateShopResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/sited_io.commerce.v2.CommerceService/CreateShop",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("sited_io.commerce.v2.CommerceService", "CreateShop"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_shop(
            &mut self,
            request: impl tonic::IntoRequest<super::GetShopRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetShopResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/sited_io.commerce.v2.CommerceService/GetShop",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("sited_io.commerce.v2.CommerceService", "GetShop"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn delete_shop(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteShopRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeleteShopResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/sited_io.commerce.v2.CommerceService/DeleteShop",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("sited_io.commerce.v2.CommerceService", "DeleteShop"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn add_offer_to_shop(
            &mut self,
            request: impl tonic::IntoRequest<super::AddOfferToShopRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AddOfferToShopResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/sited_io.commerce.v2.CommerceService/AddOfferToShop",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sited_io.commerce.v2.CommerceService",
                        "AddOfferToShop",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn remove_offer_from_shop(
            &mut self,
            request: impl tonic::IntoRequest<super::RemoveOfferFromShopRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RemoveOfferFromShopResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/sited_io.commerce.v2.CommerceService/RemoveOfferFromShop",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sited_io.commerce.v2.CommerceService",
                        "RemoveOfferFromShop",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_order(
            &mut self,
            request: impl tonic::IntoRequest<super::GetOrderRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetOrderResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/sited_io.commerce.v2.CommerceService/GetOrder",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("sited_io.commerce.v2.CommerceService", "GetOrder"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_orders(
            &mut self,
            request: impl tonic::IntoRequest<super::ListOrdersRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListOrdersResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/sited_io.commerce.v2.CommerceService/ListOrders",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("sited_io.commerce.v2.CommerceService", "ListOrders"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn create_stripe_account(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateStripeAccountRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateStripeAccountResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/sited_io.commerce.v2.CommerceService/CreateStripeAccount",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sited_io.commerce.v2.CommerceService",
                        "CreateStripeAccount",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_stripe_account(
            &mut self,
            request: impl tonic::IntoRequest<super::GetStripeAccountRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetStripeAccountResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/sited_io.commerce.v2.CommerceService/GetStripeAccount",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sited_io.commerce.v2.CommerceService",
                        "GetStripeAccount",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn buy_offer(
            &mut self,
            request: impl tonic::IntoRequest<super::BuyOfferRequest>,
        ) -> std::result::Result<
            tonic::Response<super::BuyOfferResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/sited_io.commerce.v2.CommerceService/BuyOffer",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("sited_io.commerce.v2.CommerceService", "BuyOffer"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn cancel_subscription(
            &mut self,
            request: impl tonic::IntoRequest<super::CancelSubscriptionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CancelSubscriptionResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/sited_io.commerce.v2.CommerceService/CancelSubscription",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sited_io.commerce.v2.CommerceService",
                        "CancelSubscription",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn resume_subscription(
            &mut self,
            request: impl tonic::IntoRequest<super::ResumeSubscriptionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ResumeSubscriptionResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/sited_io.commerce.v2.CommerceService/ResumeSubscription",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sited_io.commerce.v2.CommerceService",
                        "ResumeSubscription",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod commerce_service_server {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with CommerceServiceServer.
    #[async_trait]
    pub trait CommerceService: std::marker::Send + std::marker::Sync + 'static {
        async fn create_offer(
            &self,
            request: tonic::Request<super::CreateOfferRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateOfferResponse>,
            tonic::Status,
        >;
        async fn get_offer(
            &self,
            request: tonic::Request<super::GetOfferRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetOfferResponse>,
            tonic::Status,
        >;
        async fn list_offers(
            &self,
            request: tonic::Request<super::ListOffersRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListOffersResponse>,
            tonic::Status,
        >;
        async fn update_offer(
            &self,
            request: tonic::Request<super::UpdateOfferRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateOfferResponse>,
            tonic::Status,
        >;
        async fn delete_offer(
            &self,
            request: tonic::Request<super::DeleteOfferRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeleteOfferResponse>,
            tonic::Status,
        >;
        async fn put_price_to_offer(
            &self,
            request: tonic::Request<super::PutPriceToOfferRequest>,
        ) -> std::result::Result<
            tonic::Response<super::PutPriceToOfferResponse>,
            tonic::Status,
        >;
        async fn remove_price_from_offer(
            &self,
            request: tonic::Request<super::RemovePriceFromOfferRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RemovePriceFromOfferResponse>,
            tonic::Status,
        >;
        async fn put_shipping_rate_to_offer(
            &self,
            request: tonic::Request<super::PutShippingRateToOfferRequest>,
        ) -> std::result::Result<
            tonic::Response<super::PutShippingRateToOfferResponse>,
            tonic::Status,
        >;
        async fn remove_shipping_rate_from_offer(
            &self,
            request: tonic::Request<super::RemoveShippingRateFromOfferRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RemoveShippingRateFromOfferResponse>,
            tonic::Status,
        >;
        async fn add_image_to_offer(
            &self,
            request: tonic::Request<super::AddImageToOfferRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AddImageToOfferResponse>,
            tonic::Status,
        >;
        async fn update_image_ordering(
            &self,
            request: tonic::Request<super::UpdateImageOrderingRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateImageOrderingResponse>,
            tonic::Status,
        >;
        async fn remove_image_from_offer(
            &self,
            request: tonic::Request<super::RemoveImageFromOfferRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RemoveImageFromOfferResponse>,
            tonic::Status,
        >;
        async fn add_file_to_offer(
            &self,
            request: tonic::Request<super::AddFileToOfferRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AddFileToOfferResponse>,
            tonic::Status,
        >;
        async fn initiate_multipart_upload(
            &self,
            request: tonic::Request<super::InitiateMultipartUploadRequest>,
        ) -> std::result::Result<
            tonic::Response<super::InitiateMultipartUploadResponse>,
            tonic::Status,
        >;
        async fn put_multipart_chunk(
            &self,
            request: tonic::Request<super::PutMultipartChunkRequest>,
        ) -> std::result::Result<
            tonic::Response<super::PutMultipartChunkResponse>,
            tonic::Status,
        >;
        async fn complete_multipart_upload(
            &self,
            request: tonic::Request<super::CompleteMultipartUploadRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CompleteMultipartUploadResponse>,
            tonic::Status,
        >;
        async fn download_file(
            &self,
            request: tonic::Request<super::DownloadFileRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DownloadFileResponse>,
            tonic::Status,
        >;
        async fn update_file_ordering(
            &self,
            request: tonic::Request<super::UpdateFileOrderingRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateFileOrderingResponse>,
            tonic::Status,
        >;
        async fn remove_file_from_offer(
            &self,
            request: tonic::Request<super::RemoveFileFromOfferRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RemoveFileFromOfferResponse>,
            tonic::Status,
        >;
        async fn create_shop(
            &self,
            request: tonic::Request<super::CreateShopRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateShopResponse>,
            tonic::Status,
        >;
        async fn get_shop(
            &self,
            request: tonic::Request<super::GetShopRequest>,
        ) -> std::result::Result<tonic::Response<super::GetShopResponse>, tonic::Status>;
        async fn delete_shop(
            &self,
            request: tonic::Request<super::DeleteShopRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeleteShopResponse>,
            tonic::Status,
        >;
        async fn add_offer_to_shop(
            &self,
            request: tonic::Request<super::AddOfferToShopRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AddOfferToShopResponse>,
            tonic::Status,
        >;
        async fn remove_offer_from_shop(
            &self,
            request: tonic::Request<super::RemoveOfferFromShopRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RemoveOfferFromShopResponse>,
            tonic::Status,
        >;
        async fn get_order(
            &self,
            request: tonic::Request<super::GetOrderRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetOrderResponse>,
            tonic::Status,
        >;
        async fn list_orders(
            &self,
            request: tonic::Request<super::ListOrdersRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListOrdersResponse>,
            tonic::Status,
        >;
        async fn create_stripe_account(
            &self,
            request: tonic::Request<super::CreateStripeAccountRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateStripeAccountResponse>,
            tonic::Status,
        >;
        async fn get_stripe_account(
            &self,
            request: tonic::Request<super::GetStripeAccountRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetStripeAccountResponse>,
            tonic::Status,
        >;
        async fn buy_offer(
            &self,
            request: tonic::Request<super::BuyOfferRequest>,
        ) -> std::result::Result<
            tonic::Response<super::BuyOfferResponse>,
            tonic::Status,
        >;
        async fn cancel_subscription(
            &self,
            request: tonic::Request<super::CancelSubscriptionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CancelSubscriptionResponse>,
            tonic::Status,
        >;
        async fn resume_subscription(
            &self,
            request: tonic::Request<super::ResumeSubscriptionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ResumeSubscriptionResponse>,
            tonic::Status,
        >;
    }
    #[derive(Debug)]
    pub struct CommerceServiceServer<T> {
        inner: Arc<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    impl<T> CommerceServiceServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
                max_decoding_message_size: None,
                max_encoding_message_size: None,
            }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.max_decoding_message_size = Some(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.max_encoding_message_size = Some(limit);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for CommerceServiceServer<T>
    where
        T: CommerceService,
        B: Body + std::marker::Send + 'static,
        B::Error: Into<StdError> + std::marker::Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<std::result::Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            match req.uri().path() {
                "/sited_io.commerce.v2.CommerceService/CreateOffer" => {
                    #[allow(non_camel_case_types)]
                    struct CreateOfferSvc<T: CommerceService>(pub Arc<T>);
                    impl<
                        T: CommerceService,
                    > tonic::server::UnaryService<super::CreateOfferRequest>
                    for CreateOfferSvc<T> {
                        type Response = super::CreateOfferResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateOfferRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as CommerceService>::create_offer(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = CreateOfferSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/sited_io.commerce.v2.CommerceService/GetOffer" => {
                    #[allow(non_camel_case_types)]
                    struct GetOfferSvc<T: CommerceService>(pub Arc<T>);
                    impl<
                        T: CommerceService,
                    > tonic::server::UnaryService<super::GetOfferRequest>
                    for GetOfferSvc<T> {
                        type Response = super::GetOfferResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetOfferRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as CommerceService>::get_offer(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = GetOfferSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/sited_io.commerce.v2.CommerceService/ListOffers" => {
                    #[allow(non_camel_case_types)]
                    struct ListOffersSvc<T: CommerceService>(pub Arc<T>);
                    impl<
                        T: CommerceService,
                    > tonic::server::UnaryService<super::ListOffersRequest>
                    for ListOffersSvc<T> {
                        type Response = super::ListOffersResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListOffersRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as CommerceService>::list_offers(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = ListOffersSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/sited_io.commerce.v2.CommerceService/UpdateOffer" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateOfferSvc<T: CommerceService>(pub Arc<T>);
                    impl<
                        T: CommerceService,
                    > tonic::server::UnaryService<super::UpdateOfferRequest>
                    for UpdateOfferSvc<T> {
                        type Response = super::UpdateOfferResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateOfferRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as CommerceService>::update_offer(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = UpdateOfferSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/sited_io.commerce.v2.CommerceService/DeleteOffer" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteOfferSvc<T: CommerceService>(pub Arc<T>);
                    impl<
                        T: CommerceService,
                    > tonic::server::UnaryService<super::DeleteOfferRequest>
                    for DeleteOfferSvc<T> {
                        type Response = super::DeleteOfferResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteOfferRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as CommerceService>::delete_offer(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = DeleteOfferSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/sited_io.commerce.v2.CommerceService/PutPriceToOffer" => {
                    #[allow(non_camel_case_types)]
                    struct PutPriceToOfferSvc<T: CommerceService>(pub Arc<T>);
                    impl<
                        T: CommerceService,
                    > tonic::server::UnaryService<super::PutPriceToOfferRequest>
                    for PutPriceToOfferSvc<T> {
                        type Response = super::PutPriceToOfferResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::PutPriceToOfferRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as CommerceService>::put_price_to_offer(&inner, request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = PutPriceToOfferSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/sited_io.commerce.v2.CommerceService/RemovePriceFromOffer" => {
                    #[allow(non_camel_case_types)]
                    struct RemovePriceFromOfferSvc<T: CommerceService>(pub Arc<T>);
                    impl<
                        T: CommerceService,
                    > tonic::server::UnaryService<super::RemovePriceFromOfferRequest>
                    for RemovePriceFromOfferSvc<T> {
                        type Response = super::RemovePriceFromOfferResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RemovePriceFromOfferRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as CommerceService>::remove_price_from_offer(
                                        &inner,
                                        request,
                                    )
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = RemovePriceFromOfferSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/sited_io.commerce.v2.CommerceService/PutShippingRateToOffer" => {
                    #[allow(non_camel_case_types)]
                    struct PutShippingRateToOfferSvc<T: CommerceService>(pub Arc<T>);
                    impl<
                        T: CommerceService,
                    > tonic::server::UnaryService<super::PutShippingRateToOfferRequest>
                    for PutShippingRateToOfferSvc<T> {
                        type Response = super::PutShippingRateToOfferResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::PutShippingRateToOfferRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as CommerceService>::put_shipping_rate_to_offer(
                                        &inner,
                                        request,
                                    )
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = PutShippingRateToOfferSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/sited_io.commerce.v2.CommerceService/RemoveShippingRateFromOffer" => {
                    #[allow(non_camel_case_types)]
                    struct RemoveShippingRateFromOfferSvc<T: CommerceService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: CommerceService,
                    > tonic::server::UnaryService<
                        super::RemoveShippingRateFromOfferRequest,
                    > for RemoveShippingRateFromOfferSvc<T> {
                        type Response = super::RemoveShippingRateFromOfferResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::RemoveShippingRateFromOfferRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as CommerceService>::remove_shipping_rate_from_offer(
                                        &inner,
                                        request,
                                    )
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = RemoveShippingRateFromOfferSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/sited_io.commerce.v2.CommerceService/AddImageToOffer" => {
                    #[allow(non_camel_case_types)]
                    struct AddImageToOfferSvc<T: CommerceService>(pub Arc<T>);
                    impl<
                        T: CommerceService,
                    > tonic::server::UnaryService<super::AddImageToOfferRequest>
                    for AddImageToOfferSvc<T> {
                        type Response = super::AddImageToOfferResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::AddImageToOfferRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as CommerceService>::add_image_to_offer(&inner, request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = AddImageToOfferSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/sited_io.commerce.v2.CommerceService/UpdateImageOrdering" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateImageOrderingSvc<T: CommerceService>(pub Arc<T>);
                    impl<
                        T: CommerceService,
                    > tonic::server::UnaryService<super::UpdateImageOrderingRequest>
                    for UpdateImageOrderingSvc<T> {
                        type Response = super::UpdateImageOrderingResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateImageOrderingRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as CommerceService>::update_image_ordering(
                                        &inner,
                                        request,
                                    )
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = UpdateImageOrderingSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/sited_io.commerce.v2.CommerceService/RemoveImageFromOffer" => {
                    #[allow(non_camel_case_types)]
                    struct RemoveImageFromOfferSvc<T: CommerceService>(pub Arc<T>);
                    impl<
                        T: CommerceService,
                    > tonic::server::UnaryService<super::RemoveImageFromOfferRequest>
                    for RemoveImageFromOfferSvc<T> {
                        type Response = super::RemoveImageFromOfferResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RemoveImageFromOfferRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as CommerceService>::remove_image_from_offer(
                                        &inner,
                                        request,
                                    )
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = RemoveImageFromOfferSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/sited_io.commerce.v2.CommerceService/AddFileToOffer" => {
                    #[allow(non_camel_case_types)]
                    struct AddFileToOfferSvc<T: CommerceService>(pub Arc<T>);
                    impl<
                        T: CommerceService,
                    > tonic::server::UnaryService<super::AddFileToOfferRequest>
                    for AddFileToOfferSvc<T> {
                        type Response = super::AddFileToOfferResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::AddFileToOfferRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as CommerceService>::add_file_to_offer(&inner, request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = AddFileToOfferSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/sited_io.commerce.v2.CommerceService/InitiateMultipartUpload" => {
                    #[allow(non_camel_case_types)]
                    struct InitiateMultipartUploadSvc<T: CommerceService>(pub Arc<T>);
                    impl<
                        T: CommerceService,
                    > tonic::server::UnaryService<super::InitiateMultipartUploadRequest>
                    for InitiateMultipartUploadSvc<T> {
                        type Response = super::InitiateMultipartUploadResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::InitiateMultipartUploadRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as CommerceService>::initiate_multipart_upload(
                                        &inner,
                                        request,
                                    )
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = InitiateMultipartUploadSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/sited_io.commerce.v2.CommerceService/PutMultipartChunk" => {
                    #[allow(non_camel_case_types)]
                    struct PutMultipartChunkSvc<T: CommerceService>(pub Arc<T>);
                    impl<
                        T: CommerceService,
                    > tonic::server::UnaryService<super::PutMultipartChunkRequest>
                    for PutMultipartChunkSvc<T> {
                        type Response = super::PutMultipartChunkResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::PutMultipartChunkRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as CommerceService>::put_multipart_chunk(&inner, request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = PutMultipartChunkSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/sited_io.commerce.v2.CommerceService/CompleteMultipartUpload" => {
                    #[allow(non_camel_case_types)]
                    struct CompleteMultipartUploadSvc<T: CommerceService>(pub Arc<T>);
                    impl<
                        T: CommerceService,
                    > tonic::server::UnaryService<super::CompleteMultipartUploadRequest>
                    for CompleteMultipartUploadSvc<T> {
                        type Response = super::CompleteMultipartUploadResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::CompleteMultipartUploadRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as CommerceService>::complete_multipart_upload(
                                        &inner,
                                        request,
                                    )
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = CompleteMultipartUploadSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/sited_io.commerce.v2.CommerceService/DownloadFile" => {
                    #[allow(non_camel_case_types)]
                    struct DownloadFileSvc<T: CommerceService>(pub Arc<T>);
                    impl<
                        T: CommerceService,
                    > tonic::server::UnaryService<super::DownloadFileRequest>
                    for DownloadFileSvc<T> {
                        type Response = super::DownloadFileResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DownloadFileRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as CommerceService>::download_file(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = DownloadFileSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/sited_io.commerce.v2.CommerceService/UpdateFileOrdering" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateFileOrderingSvc<T: CommerceService>(pub Arc<T>);
                    impl<
                        T: CommerceService,
                    > tonic::server::UnaryService<super::UpdateFileOrderingRequest>
                    for UpdateFileOrderingSvc<T> {
                        type Response = super::UpdateFileOrderingResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateFileOrderingRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as CommerceService>::update_file_ordering(
                                        &inner,
                                        request,
                                    )
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = UpdateFileOrderingSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/sited_io.commerce.v2.CommerceService/RemoveFileFromOffer" => {
                    #[allow(non_camel_case_types)]
                    struct RemoveFileFromOfferSvc<T: CommerceService>(pub Arc<T>);
                    impl<
                        T: CommerceService,
                    > tonic::server::UnaryService<super::RemoveFileFromOfferRequest>
                    for RemoveFileFromOfferSvc<T> {
                        type Response = super::RemoveFileFromOfferResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RemoveFileFromOfferRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as CommerceService>::remove_file_from_offer(
                                        &inner,
                                        request,
                                    )
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = RemoveFileFromOfferSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/sited_io.commerce.v2.CommerceService/CreateShop" => {
                    #[allow(non_camel_case_types)]
                    struct CreateShopSvc<T: CommerceService>(pub Arc<T>);
                    impl<
                        T: CommerceService,
                    > tonic::server::UnaryService<super::CreateShopRequest>
                    for CreateShopSvc<T> {
                        type Response = super::CreateShopResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateShopRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as CommerceService>::create_shop(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = CreateShopSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/sited_io.commerce.v2.CommerceService/GetShop" => {
                    #[allow(non_camel_case_types)]
                    struct GetShopSvc<T: CommerceService>(pub Arc<T>);
                    impl<
                        T: CommerceService,
                    > tonic::server::UnaryService<super::GetShopRequest>
                    for GetShopSvc<T> {
                        type Response = super::GetShopResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetShopRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as CommerceService>::get_shop(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = GetShopSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/sited_io.commerce.v2.CommerceService/DeleteShop" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteShopSvc<T: CommerceService>(pub Arc<T>);
                    impl<
                        T: CommerceService,
                    > tonic::server::UnaryService<super::DeleteShopRequest>
                    for DeleteShopSvc<T> {
                        type Response = super::DeleteShopResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteShopRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as CommerceService>::delete_shop(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = DeleteShopSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/sited_io.commerce.v2.CommerceService/AddOfferToShop" => {
                    #[allow(non_camel_case_types)]
                    struct AddOfferToShopSvc<T: CommerceService>(pub Arc<T>);
                    impl<
                        T: CommerceService,
                    > tonic::server::UnaryService<super::AddOfferToShopRequest>
                    for AddOfferToShopSvc<T> {
                        type Response = super::AddOfferToShopResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::AddOfferToShopRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as CommerceService>::add_offer_to_shop(&inner, request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = AddOfferToShopSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/sited_io.commerce.v2.CommerceService/RemoveOfferFromShop" => {
                    #[allow(non_camel_case_types)]
                    struct RemoveOfferFromShopSvc<T: CommerceService>(pub Arc<T>);
                    impl<
                        T: CommerceService,
                    > tonic::server::UnaryService<super::RemoveOfferFromShopRequest>
                    for RemoveOfferFromShopSvc<T> {
                        type Response = super::RemoveOfferFromShopResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RemoveOfferFromShopRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as CommerceService>::remove_offer_from_shop(
                                        &inner,
                                        request,
                                    )
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = RemoveOfferFromShopSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/sited_io.commerce.v2.CommerceService/GetOrder" => {
                    #[allow(non_camel_case_types)]
                    struct GetOrderSvc<T: CommerceService>(pub Arc<T>);
                    impl<
                        T: CommerceService,
                    > tonic::server::UnaryService<super::GetOrderRequest>
                    for GetOrderSvc<T> {
                        type Response = super::GetOrderResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetOrderRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as CommerceService>::get_order(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = GetOrderSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/sited_io.commerce.v2.CommerceService/ListOrders" => {
                    #[allow(non_camel_case_types)]
                    struct ListOrdersSvc<T: CommerceService>(pub Arc<T>);
                    impl<
                        T: CommerceService,
                    > tonic::server::UnaryService<super::ListOrdersRequest>
                    for ListOrdersSvc<T> {
                        type Response = super::ListOrdersResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListOrdersRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as CommerceService>::list_orders(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = ListOrdersSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/sited_io.commerce.v2.CommerceService/CreateStripeAccount" => {
                    #[allow(non_camel_case_types)]
                    struct CreateStripeAccountSvc<T: CommerceService>(pub Arc<T>);
                    impl<
                        T: CommerceService,
                    > tonic::server::UnaryService<super::CreateStripeAccountRequest>
                    for CreateStripeAccountSvc<T> {
                        type Response = super::CreateStripeAccountResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateStripeAccountRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as CommerceService>::create_stripe_account(
                                        &inner,
                                        request,
                                    )
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = CreateStripeAccountSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/sited_io.commerce.v2.CommerceService/GetStripeAccount" => {
                    #[allow(non_camel_case_types)]
                    struct GetStripeAccountSvc<T: CommerceService>(pub Arc<T>);
                    impl<
                        T: CommerceService,
                    > tonic::server::UnaryService<super::GetStripeAccountRequest>
                    for GetStripeAccountSvc<T> {
                        type Response = super::GetStripeAccountResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetStripeAccountRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as CommerceService>::get_stripe_account(&inner, request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = GetStripeAccountSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/sited_io.commerce.v2.CommerceService/BuyOffer" => {
                    #[allow(non_camel_case_types)]
                    struct BuyOfferSvc<T: CommerceService>(pub Arc<T>);
                    impl<
                        T: CommerceService,
                    > tonic::server::UnaryService<super::BuyOfferRequest>
                    for BuyOfferSvc<T> {
                        type Response = super::BuyOfferResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::BuyOfferRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as CommerceService>::buy_offer(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = BuyOfferSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/sited_io.commerce.v2.CommerceService/CancelSubscription" => {
                    #[allow(non_camel_case_types)]
                    struct CancelSubscriptionSvc<T: CommerceService>(pub Arc<T>);
                    impl<
                        T: CommerceService,
                    > tonic::server::UnaryService<super::CancelSubscriptionRequest>
                    for CancelSubscriptionSvc<T> {
                        type Response = super::CancelSubscriptionResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CancelSubscriptionRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as CommerceService>::cancel_subscription(&inner, request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = CancelSubscriptionSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/sited_io.commerce.v2.CommerceService/ResumeSubscription" => {
                    #[allow(non_camel_case_types)]
                    struct ResumeSubscriptionSvc<T: CommerceService>(pub Arc<T>);
                    impl<
                        T: CommerceService,
                    > tonic::server::UnaryService<super::ResumeSubscriptionRequest>
                    for ResumeSubscriptionSvc<T> {
                        type Response = super::ResumeSubscriptionResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ResumeSubscriptionRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as CommerceService>::resume_subscription(&inner, request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = ResumeSubscriptionSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        let mut response = http::Response::new(empty_body());
                        let headers = response.headers_mut();
                        headers
                            .insert(
                                tonic::Status::GRPC_STATUS,
                                (tonic::Code::Unimplemented as i32).into(),
                            );
                        headers
                            .insert(
                                http::header::CONTENT_TYPE,
                                tonic::metadata::GRPC_CONTENT_TYPE,
                            );
                        Ok(response)
                    })
                }
            }
        }
    }
    impl<T> Clone for CommerceServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
                max_decoding_message_size: self.max_decoding_message_size,
                max_encoding_message_size: self.max_encoding_message_size,
            }
        }
    }
    /// Generated gRPC service name
    pub const SERVICE_NAME: &str = "sited_io.commerce.v2.CommerceService";
    impl<T> tonic::server::NamedService for CommerceServiceServer<T> {
        const NAME: &'static str = SERVICE_NAME;
    }
}

// @generated
/// Generated client implementations.
pub mod media_service_client {
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
    pub struct MediaServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl MediaServiceClient<tonic::transport::Channel> {
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
    impl<T> MediaServiceClient<T>
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
        ) -> MediaServiceClient<InterceptedService<T, F>>
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
            MediaServiceClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn create_media(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateMediaRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateMediaResponse>,
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
                "/sited_io.media.v1.MediaService/CreateMedia",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("sited_io.media.v1.MediaService", "CreateMedia"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_media(
            &mut self,
            request: impl tonic::IntoRequest<super::GetMediaRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetMediaResponse>,
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
                "/sited_io.media.v1.MediaService/GetMedia",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("sited_io.media.v1.MediaService", "GetMedia"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn download_media(
            &mut self,
            request: impl tonic::IntoRequest<super::DownloadMediaRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DownloadMediaResponse>,
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
                "/sited_io.media.v1.MediaService/DownloadMedia",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("sited_io.media.v1.MediaService", "DownloadMedia"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_media(
            &mut self,
            request: impl tonic::IntoRequest<super::ListMediaRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListMediaResponse>,
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
                "/sited_io.media.v1.MediaService/ListMedia",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("sited_io.media.v1.MediaService", "ListMedia"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_accessible_media(
            &mut self,
            request: impl tonic::IntoRequest<super::ListAccessibleMediaRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListAccessibleMediaResponse>,
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
                "/sited_io.media.v1.MediaService/ListAccessibleMedia",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sited_io.media.v1.MediaService",
                        "ListAccessibleMedia",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_media(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateMediaRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateMediaResponse>,
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
                "/sited_io.media.v1.MediaService/UpdateMedia",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("sited_io.media.v1.MediaService", "UpdateMedia"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn delete_media(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteMediaRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeleteMediaResponse>,
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
                "/sited_io.media.v1.MediaService/DeleteMedia",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("sited_io.media.v1.MediaService", "DeleteMedia"),
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
                "/sited_io.media.v1.MediaService/InitiateMultipartUpload",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sited_io.media.v1.MediaService",
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
                "/sited_io.media.v1.MediaService/PutMultipartChunk",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sited_io.media.v1.MediaService",
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
                "/sited_io.media.v1.MediaService/CompleteMultipartUpload",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sited_io.media.v1.MediaService",
                        "CompleteMultipartUpload",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn add_media_to_offer(
            &mut self,
            request: impl tonic::IntoRequest<super::AddMediaToOfferRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AddMediaToOfferResponse>,
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
                "/sited_io.media.v1.MediaService/AddMediaToOffer",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("sited_io.media.v1.MediaService", "AddMediaToOffer"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_media_offer_ordering(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateMediaOfferOrderingRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateMediaOfferOrderingResponse>,
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
                "/sited_io.media.v1.MediaService/UpdateMediaOfferOrdering",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sited_io.media.v1.MediaService",
                        "UpdateMediaOfferOrdering",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn remove_media_from_offer(
            &mut self,
            request: impl tonic::IntoRequest<super::RemoveMediaFromOfferRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RemoveMediaFromOfferResponse>,
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
                "/sited_io.media.v1.MediaService/RemoveMediaFromOffer",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sited_io.media.v1.MediaService",
                        "RemoveMediaFromOffer",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod media_service_server {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with MediaServiceServer.
    #[async_trait]
    pub trait MediaService: std::marker::Send + std::marker::Sync + 'static {
        async fn create_media(
            &self,
            request: tonic::Request<super::CreateMediaRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateMediaResponse>,
            tonic::Status,
        >;
        async fn get_media(
            &self,
            request: tonic::Request<super::GetMediaRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetMediaResponse>,
            tonic::Status,
        >;
        async fn download_media(
            &self,
            request: tonic::Request<super::DownloadMediaRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DownloadMediaResponse>,
            tonic::Status,
        >;
        async fn list_media(
            &self,
            request: tonic::Request<super::ListMediaRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListMediaResponse>,
            tonic::Status,
        >;
        async fn list_accessible_media(
            &self,
            request: tonic::Request<super::ListAccessibleMediaRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListAccessibleMediaResponse>,
            tonic::Status,
        >;
        async fn update_media(
            &self,
            request: tonic::Request<super::UpdateMediaRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateMediaResponse>,
            tonic::Status,
        >;
        async fn delete_media(
            &self,
            request: tonic::Request<super::DeleteMediaRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeleteMediaResponse>,
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
        async fn add_media_to_offer(
            &self,
            request: tonic::Request<super::AddMediaToOfferRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AddMediaToOfferResponse>,
            tonic::Status,
        >;
        async fn update_media_offer_ordering(
            &self,
            request: tonic::Request<super::UpdateMediaOfferOrderingRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateMediaOfferOrderingResponse>,
            tonic::Status,
        >;
        async fn remove_media_from_offer(
            &self,
            request: tonic::Request<super::RemoveMediaFromOfferRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RemoveMediaFromOfferResponse>,
            tonic::Status,
        >;
    }
    #[derive(Debug)]
    pub struct MediaServiceServer<T> {
        inner: Arc<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    impl<T> MediaServiceServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for MediaServiceServer<T>
    where
        T: MediaService,
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
                "/sited_io.media.v1.MediaService/CreateMedia" => {
                    #[allow(non_camel_case_types)]
                    struct CreateMediaSvc<T: MediaService>(pub Arc<T>);
                    impl<
                        T: MediaService,
                    > tonic::server::UnaryService<super::CreateMediaRequest>
                    for CreateMediaSvc<T> {
                        type Response = super::CreateMediaResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateMediaRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as MediaService>::create_media(&inner, request).await
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
                        let method = CreateMediaSvc(inner);
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
                "/sited_io.media.v1.MediaService/GetMedia" => {
                    #[allow(non_camel_case_types)]
                    struct GetMediaSvc<T: MediaService>(pub Arc<T>);
                    impl<
                        T: MediaService,
                    > tonic::server::UnaryService<super::GetMediaRequest>
                    for GetMediaSvc<T> {
                        type Response = super::GetMediaResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetMediaRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as MediaService>::get_media(&inner, request).await
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
                        let method = GetMediaSvc(inner);
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
                "/sited_io.media.v1.MediaService/DownloadMedia" => {
                    #[allow(non_camel_case_types)]
                    struct DownloadMediaSvc<T: MediaService>(pub Arc<T>);
                    impl<
                        T: MediaService,
                    > tonic::server::UnaryService<super::DownloadMediaRequest>
                    for DownloadMediaSvc<T> {
                        type Response = super::DownloadMediaResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DownloadMediaRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as MediaService>::download_media(&inner, request).await
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
                        let method = DownloadMediaSvc(inner);
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
                "/sited_io.media.v1.MediaService/ListMedia" => {
                    #[allow(non_camel_case_types)]
                    struct ListMediaSvc<T: MediaService>(pub Arc<T>);
                    impl<
                        T: MediaService,
                    > tonic::server::UnaryService<super::ListMediaRequest>
                    for ListMediaSvc<T> {
                        type Response = super::ListMediaResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListMediaRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as MediaService>::list_media(&inner, request).await
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
                        let method = ListMediaSvc(inner);
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
                "/sited_io.media.v1.MediaService/ListAccessibleMedia" => {
                    #[allow(non_camel_case_types)]
                    struct ListAccessibleMediaSvc<T: MediaService>(pub Arc<T>);
                    impl<
                        T: MediaService,
                    > tonic::server::UnaryService<super::ListAccessibleMediaRequest>
                    for ListAccessibleMediaSvc<T> {
                        type Response = super::ListAccessibleMediaResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListAccessibleMediaRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as MediaService>::list_accessible_media(&inner, request)
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
                        let method = ListAccessibleMediaSvc(inner);
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
                "/sited_io.media.v1.MediaService/UpdateMedia" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateMediaSvc<T: MediaService>(pub Arc<T>);
                    impl<
                        T: MediaService,
                    > tonic::server::UnaryService<super::UpdateMediaRequest>
                    for UpdateMediaSvc<T> {
                        type Response = super::UpdateMediaResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateMediaRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as MediaService>::update_media(&inner, request).await
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
                        let method = UpdateMediaSvc(inner);
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
                "/sited_io.media.v1.MediaService/DeleteMedia" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteMediaSvc<T: MediaService>(pub Arc<T>);
                    impl<
                        T: MediaService,
                    > tonic::server::UnaryService<super::DeleteMediaRequest>
                    for DeleteMediaSvc<T> {
                        type Response = super::DeleteMediaResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteMediaRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as MediaService>::delete_media(&inner, request).await
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
                        let method = DeleteMediaSvc(inner);
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
                "/sited_io.media.v1.MediaService/InitiateMultipartUpload" => {
                    #[allow(non_camel_case_types)]
                    struct InitiateMultipartUploadSvc<T: MediaService>(pub Arc<T>);
                    impl<
                        T: MediaService,
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
                                <T as MediaService>::initiate_multipart_upload(
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
                "/sited_io.media.v1.MediaService/PutMultipartChunk" => {
                    #[allow(non_camel_case_types)]
                    struct PutMultipartChunkSvc<T: MediaService>(pub Arc<T>);
                    impl<
                        T: MediaService,
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
                                <T as MediaService>::put_multipart_chunk(&inner, request)
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
                "/sited_io.media.v1.MediaService/CompleteMultipartUpload" => {
                    #[allow(non_camel_case_types)]
                    struct CompleteMultipartUploadSvc<T: MediaService>(pub Arc<T>);
                    impl<
                        T: MediaService,
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
                                <T as MediaService>::complete_multipart_upload(
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
                "/sited_io.media.v1.MediaService/AddMediaToOffer" => {
                    #[allow(non_camel_case_types)]
                    struct AddMediaToOfferSvc<T: MediaService>(pub Arc<T>);
                    impl<
                        T: MediaService,
                    > tonic::server::UnaryService<super::AddMediaToOfferRequest>
                    for AddMediaToOfferSvc<T> {
                        type Response = super::AddMediaToOfferResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::AddMediaToOfferRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as MediaService>::add_media_to_offer(&inner, request)
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
                        let method = AddMediaToOfferSvc(inner);
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
                "/sited_io.media.v1.MediaService/UpdateMediaOfferOrdering" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateMediaOfferOrderingSvc<T: MediaService>(pub Arc<T>);
                    impl<
                        T: MediaService,
                    > tonic::server::UnaryService<super::UpdateMediaOfferOrderingRequest>
                    for UpdateMediaOfferOrderingSvc<T> {
                        type Response = super::UpdateMediaOfferOrderingResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::UpdateMediaOfferOrderingRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as MediaService>::update_media_offer_ordering(
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
                        let method = UpdateMediaOfferOrderingSvc(inner);
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
                "/sited_io.media.v1.MediaService/RemoveMediaFromOffer" => {
                    #[allow(non_camel_case_types)]
                    struct RemoveMediaFromOfferSvc<T: MediaService>(pub Arc<T>);
                    impl<
                        T: MediaService,
                    > tonic::server::UnaryService<super::RemoveMediaFromOfferRequest>
                    for RemoveMediaFromOfferSvc<T> {
                        type Response = super::RemoveMediaFromOfferResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RemoveMediaFromOfferRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as MediaService>::remove_media_from_offer(
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
                        let method = RemoveMediaFromOfferSvc(inner);
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
    impl<T> Clone for MediaServiceServer<T> {
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
    pub const SERVICE_NAME: &str = "sited_io.media.v1.MediaService";
    impl<T> tonic::server::NamedService for MediaServiceServer<T> {
        const NAME: &'static str = SERVICE_NAME;
    }
}
/// Generated client implementations.
pub mod media_subscription_service_client {
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
    pub struct MediaSubscriptionServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl MediaSubscriptionServiceClient<tonic::transport::Channel> {
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
    impl<T> MediaSubscriptionServiceClient<T>
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
        ) -> MediaSubscriptionServiceClient<InterceptedService<T, F>>
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
            MediaSubscriptionServiceClient::new(
                InterceptedService::new(inner, interceptor),
            )
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
        pub async fn put_media_subscription(
            &mut self,
            request: impl tonic::IntoRequest<super::PutMediaSubscriptionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::PutMediaSubscriptionResponse>,
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
                "/sited_io.media.v1.MediaSubscriptionService/PutMediaSubscription",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sited_io.media.v1.MediaSubscriptionService",
                        "PutMediaSubscription",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_media_subscription(
            &mut self,
            request: impl tonic::IntoRequest<super::GetMediaSubscriptionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetMediaSubscriptionResponse>,
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
                "/sited_io.media.v1.MediaSubscriptionService/GetMediaSubscription",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sited_io.media.v1.MediaSubscriptionService",
                        "GetMediaSubscription",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_media_subscriptions(
            &mut self,
            request: impl tonic::IntoRequest<super::ListMediaSubscriptionsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListMediaSubscriptionsResponse>,
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
                "/sited_io.media.v1.MediaSubscriptionService/ListMediaSubscriptions",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sited_io.media.v1.MediaSubscriptionService",
                        "ListMediaSubscriptions",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn cancel_media_subscription(
            &mut self,
            request: impl tonic::IntoRequest<super::CancelMediaSubscriptionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CancelMediaSubscriptionResponse>,
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
                "/sited_io.media.v1.MediaSubscriptionService/CancelMediaSubscription",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sited_io.media.v1.MediaSubscriptionService",
                        "CancelMediaSubscription",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn resume_media_subscription(
            &mut self,
            request: impl tonic::IntoRequest<super::ResumeMediaSubscriptionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ResumeMediaSubscriptionResponse>,
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
                "/sited_io.media.v1.MediaSubscriptionService/ResumeMediaSubscription",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sited_io.media.v1.MediaSubscriptionService",
                        "ResumeMediaSubscription",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod media_subscription_service_server {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with MediaSubscriptionServiceServer.
    #[async_trait]
    pub trait MediaSubscriptionService: std::marker::Send + std::marker::Sync + 'static {
        async fn put_media_subscription(
            &self,
            request: tonic::Request<super::PutMediaSubscriptionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::PutMediaSubscriptionResponse>,
            tonic::Status,
        >;
        async fn get_media_subscription(
            &self,
            request: tonic::Request<super::GetMediaSubscriptionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetMediaSubscriptionResponse>,
            tonic::Status,
        >;
        async fn list_media_subscriptions(
            &self,
            request: tonic::Request<super::ListMediaSubscriptionsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListMediaSubscriptionsResponse>,
            tonic::Status,
        >;
        async fn cancel_media_subscription(
            &self,
            request: tonic::Request<super::CancelMediaSubscriptionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CancelMediaSubscriptionResponse>,
            tonic::Status,
        >;
        async fn resume_media_subscription(
            &self,
            request: tonic::Request<super::ResumeMediaSubscriptionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ResumeMediaSubscriptionResponse>,
            tonic::Status,
        >;
    }
    #[derive(Debug)]
    pub struct MediaSubscriptionServiceServer<T> {
        inner: Arc<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    impl<T> MediaSubscriptionServiceServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>>
    for MediaSubscriptionServiceServer<T>
    where
        T: MediaSubscriptionService,
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
                "/sited_io.media.v1.MediaSubscriptionService/PutMediaSubscription" => {
                    #[allow(non_camel_case_types)]
                    struct PutMediaSubscriptionSvc<T: MediaSubscriptionService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: MediaSubscriptionService,
                    > tonic::server::UnaryService<super::PutMediaSubscriptionRequest>
                    for PutMediaSubscriptionSvc<T> {
                        type Response = super::PutMediaSubscriptionResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::PutMediaSubscriptionRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as MediaSubscriptionService>::put_media_subscription(
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
                        let method = PutMediaSubscriptionSvc(inner);
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
                "/sited_io.media.v1.MediaSubscriptionService/GetMediaSubscription" => {
                    #[allow(non_camel_case_types)]
                    struct GetMediaSubscriptionSvc<T: MediaSubscriptionService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: MediaSubscriptionService,
                    > tonic::server::UnaryService<super::GetMediaSubscriptionRequest>
                    for GetMediaSubscriptionSvc<T> {
                        type Response = super::GetMediaSubscriptionResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetMediaSubscriptionRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as MediaSubscriptionService>::get_media_subscription(
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
                        let method = GetMediaSubscriptionSvc(inner);
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
                "/sited_io.media.v1.MediaSubscriptionService/ListMediaSubscriptions" => {
                    #[allow(non_camel_case_types)]
                    struct ListMediaSubscriptionsSvc<T: MediaSubscriptionService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: MediaSubscriptionService,
                    > tonic::server::UnaryService<super::ListMediaSubscriptionsRequest>
                    for ListMediaSubscriptionsSvc<T> {
                        type Response = super::ListMediaSubscriptionsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListMediaSubscriptionsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as MediaSubscriptionService>::list_media_subscriptions(
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
                        let method = ListMediaSubscriptionsSvc(inner);
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
                "/sited_io.media.v1.MediaSubscriptionService/CancelMediaSubscription" => {
                    #[allow(non_camel_case_types)]
                    struct CancelMediaSubscriptionSvc<T: MediaSubscriptionService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: MediaSubscriptionService,
                    > tonic::server::UnaryService<super::CancelMediaSubscriptionRequest>
                    for CancelMediaSubscriptionSvc<T> {
                        type Response = super::CancelMediaSubscriptionResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::CancelMediaSubscriptionRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as MediaSubscriptionService>::cancel_media_subscription(
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
                        let method = CancelMediaSubscriptionSvc(inner);
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
                "/sited_io.media.v1.MediaSubscriptionService/ResumeMediaSubscription" => {
                    #[allow(non_camel_case_types)]
                    struct ResumeMediaSubscriptionSvc<T: MediaSubscriptionService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: MediaSubscriptionService,
                    > tonic::server::UnaryService<super::ResumeMediaSubscriptionRequest>
                    for ResumeMediaSubscriptionSvc<T> {
                        type Response = super::ResumeMediaSubscriptionResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::ResumeMediaSubscriptionRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as MediaSubscriptionService>::resume_media_subscription(
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
                        let method = ResumeMediaSubscriptionSvc(inner);
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
    impl<T> Clone for MediaSubscriptionServiceServer<T> {
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
    pub const SERVICE_NAME: &str = "sited_io.media.v1.MediaSubscriptionService";
    impl<T> tonic::server::NamedService for MediaSubscriptionServiceServer<T> {
        const NAME: &'static str = SERVICE_NAME;
    }
}

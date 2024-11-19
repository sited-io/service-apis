// @generated by protoc-gen-es v2.2.2 with parameter "target=ts"
// @generated from file sited_io/media/v1/media_subscription.proto (package sited_io.media.v1, syntax proto3)
/* eslint-disable */

import type { GenFile, GenMessage, GenService } from "@bufbuild/protobuf/codegenv1";
import { fileDesc, messageDesc, serviceDesc } from "@bufbuild/protobuf/codegenv1";
import type { PaginationRequest, PaginationResponse } from "../../types/v1/pagination_pb";
import { file_sited_io_types_v1_pagination } from "../../types/v1/pagination_pb";
import type { Message } from "@bufbuild/protobuf";

/**
 * Describes the file sited_io/media/v1/media_subscription.proto.
 */
export const file_sited_io_media_v1_media_subscription: GenFile = /*@__PURE__*/
  fileDesc("CipzaXRlZF9pby9tZWRpYS92MS9tZWRpYV9zdWJzY3JpcHRpb24ucHJvdG8SEXNpdGVkX2lvLm1lZGlhLnYxIoIDChlNZWRpYVN1YnNjcmlwdGlvblJlc3BvbnNlEh0KFW1lZGlhX3N1YnNjcmlwdGlvbl9pZBgBIAEoCRIVCg1idXllcl91c2VyX2lkGAIgASgJEg8KB3Nob3BfaWQYBCABKAkSEAoIb2ZmZXJfaWQYBSABKAkSHAoUY3VycmVudF9wZXJpb2Rfc3RhcnQYBiABKAQSGgoSY3VycmVudF9wZXJpb2RfZW5kGAcgASgEEhsKE3N1YnNjcmlwdGlvbl9zdGF0dXMYCCABKAkSEAoIcGF5ZWRfYXQYCSABKAQSEwoLcGF5ZWRfdW50aWwYCiABKAQSIwoWc3RyaXBlX3N1YnNjcmlwdGlvbl9pZBgLIAEoCUgAiAEBEhgKC2NhbmNlbGVkX2F0GAwgASgESAGIAQESFgoJY2FuY2VsX2F0GA0gASgESAKIAQFCGQoXX3N0cmlwZV9zdWJzY3JpcHRpb25faWRCDgoMX2NhbmNlbGVkX2F0QgwKCl9jYW5jZWxfYXQihAMKG1B1dE1lZGlhU3Vic2NyaXB0aW9uUmVxdWVzdBIdChVtZWRpYV9zdWJzY3JpcHRpb25faWQYASABKAkSFQoNYnV5ZXJfdXNlcl9pZBgCIAEoCRIQCghvZmZlcl9pZBgDIAEoCRIcChRjdXJyZW50X3BlcmlvZF9zdGFydBgEIAEoBBIaChJjdXJyZW50X3BlcmlvZF9lbmQYBSABKAQSGwoTc3Vic2NyaXB0aW9uX3N0YXR1cxgGIAEoCRIQCghwYXllZF9hdBgHIAEoBBITCgtwYXllZF91bnRpbBgIIAEoBBIPCgdzaG9wX2lkGAkgASgJEiMKFnN0cmlwZV9zdWJzY3JpcHRpb25faWQYCiABKAlIAIgBARIYCgtjYW5jZWxlZF9hdBgLIAEoBEgBiAEBEhYKCWNhbmNlbF9hdBgMIAEoBEgCiAEBQhkKF19zdHJpcGVfc3Vic2NyaXB0aW9uX2lkQg4KDF9jYW5jZWxlZF9hdEIMCgpfY2FuY2VsX2F0Ih4KHFB1dE1lZGlhU3Vic2NyaXB0aW9uUmVzcG9uc2UifwobR2V0TWVkaWFTdWJzY3JpcHRpb25SZXF1ZXN0EiIKFW1lZGlhX3N1YnNjcmlwdGlvbl9pZBgBIAEoCUgAiAEBEhUKCG9mZmVyX2lkGAIgASgJSAGIAQFCGAoWX21lZGlhX3N1YnNjcmlwdGlvbl9pZEILCglfb2ZmZXJfaWQiaAocR2V0TWVkaWFTdWJzY3JpcHRpb25SZXNwb25zZRJIChJtZWRpYV9zdWJzY3JpcHRpb24YASABKAsyLC5zaXRlZF9pby5tZWRpYS52MS5NZWRpYVN1YnNjcmlwdGlvblJlc3BvbnNlIr0BCh1MaXN0TWVkaWFTdWJzY3JpcHRpb25zUmVxdWVzdBIUCgdzaG9wX2lkGAEgASgJSACIAQESPQoKcGFnaW5hdGlvbhgCIAEoCzIkLnNpdGVkX2lvLnR5cGVzLnYxLlBhZ2luYXRpb25SZXF1ZXN0SAGIAQESGgoNaXNfYWNjZXNzaWJsZRgDIAEoCEgCiAEBQgoKCF9zaG9wX2lkQg0KC19wYWdpbmF0aW9uQhAKDl9pc19hY2Nlc3NpYmxlIqYBCh5MaXN0TWVkaWFTdWJzY3JpcHRpb25zUmVzcG9uc2USSQoTbWVkaWFfc3Vic2NyaXB0aW9ucxgBIAMoCzIsLnNpdGVkX2lvLm1lZGlhLnYxLk1lZGlhU3Vic2NyaXB0aW9uUmVzcG9uc2USOQoKcGFnaW5hdGlvbhgCIAEoCzIlLnNpdGVkX2lvLnR5cGVzLnYxLlBhZ2luYXRpb25SZXNwb25zZSI/Ch5DYW5jZWxNZWRpYVN1YnNjcmlwdGlvblJlcXVlc3QSHQoVbWVkaWFfc3Vic2NyaXB0aW9uX2lkGAEgASgJIiEKH0NhbmNlbE1lZGlhU3Vic2NyaXB0aW9uUmVzcG9uc2UiPwoeUmVzdW1lTWVkaWFTdWJzY3JpcHRpb25SZXF1ZXN0Eh0KFW1lZGlhX3N1YnNjcmlwdGlvbl9pZBgBIAEoCSIhCh9SZXN1bWVNZWRpYVN1YnNjcmlwdGlvblJlc3BvbnNlMpEFChhNZWRpYVN1YnNjcmlwdGlvblNlcnZpY2USdwoUUHV0TWVkaWFTdWJzY3JpcHRpb24SLi5zaXRlZF9pby5tZWRpYS52MS5QdXRNZWRpYVN1YnNjcmlwdGlvblJlcXVlc3QaLy5zaXRlZF9pby5tZWRpYS52MS5QdXRNZWRpYVN1YnNjcmlwdGlvblJlc3BvbnNlEncKFEdldE1lZGlhU3Vic2NyaXB0aW9uEi4uc2l0ZWRfaW8ubWVkaWEudjEuR2V0TWVkaWFTdWJzY3JpcHRpb25SZXF1ZXN0Gi8uc2l0ZWRfaW8ubWVkaWEudjEuR2V0TWVkaWFTdWJzY3JpcHRpb25SZXNwb25zZRJ9ChZMaXN0TWVkaWFTdWJzY3JpcHRpb25zEjAuc2l0ZWRfaW8ubWVkaWEudjEuTGlzdE1lZGlhU3Vic2NyaXB0aW9uc1JlcXVlc3QaMS5zaXRlZF9pby5tZWRpYS52MS5MaXN0TWVkaWFTdWJzY3JpcHRpb25zUmVzcG9uc2USgAEKF0NhbmNlbE1lZGlhU3Vic2NyaXB0aW9uEjEuc2l0ZWRfaW8ubWVkaWEudjEuQ2FuY2VsTWVkaWFTdWJzY3JpcHRpb25SZXF1ZXN0GjIuc2l0ZWRfaW8ubWVkaWEudjEuQ2FuY2VsTWVkaWFTdWJzY3JpcHRpb25SZXNwb25zZRKAAQoXUmVzdW1lTWVkaWFTdWJzY3JpcHRpb24SMS5zaXRlZF9pby5tZWRpYS52MS5SZXN1bWVNZWRpYVN1YnNjcmlwdGlvblJlcXVlc3QaMi5zaXRlZF9pby5tZWRpYS52MS5SZXN1bWVNZWRpYVN1YnNjcmlwdGlvblJlc3BvbnNlYgZwcm90bzM", [file_sited_io_types_v1_pagination]);

/**
 * @generated from message sited_io.media.v1.MediaSubscriptionResponse
 */
export type MediaSubscriptionResponse = Message<"sited_io.media.v1.MediaSubscriptionResponse"> & {
  /**
   * @generated from field: string media_subscription_id = 1;
   */
  mediaSubscriptionId: string;

  /**
   * @generated from field: string buyer_user_id = 2;
   */
  buyerUserId: string;

  /**
   * @generated from field: string shop_id = 4;
   */
  shopId: string;

  /**
   * @generated from field: string offer_id = 5;
   */
  offerId: string;

  /**
   * @generated from field: uint64 current_period_start = 6;
   */
  currentPeriodStart: bigint;

  /**
   * @generated from field: uint64 current_period_end = 7;
   */
  currentPeriodEnd: bigint;

  /**
   * @generated from field: string subscription_status = 8;
   */
  subscriptionStatus: string;

  /**
   * @generated from field: uint64 payed_at = 9;
   */
  payedAt: bigint;

  /**
   * @generated from field: uint64 payed_until = 10;
   */
  payedUntil: bigint;

  /**
   * @generated from field: optional string stripe_subscription_id = 11;
   */
  stripeSubscriptionId?: string;

  /**
   * @generated from field: optional uint64 canceled_at = 12;
   */
  canceledAt?: bigint;

  /**
   * @generated from field: optional uint64 cancel_at = 13;
   */
  cancelAt?: bigint;
};

/**
 * Describes the message sited_io.media.v1.MediaSubscriptionResponse.
 * Use `create(MediaSubscriptionResponseSchema)` to create a new message.
 */
export const MediaSubscriptionResponseSchema: GenMessage<MediaSubscriptionResponse> = /*@__PURE__*/
  messageDesc(file_sited_io_media_v1_media_subscription, 0);

/**
 * @generated from message sited_io.media.v1.PutMediaSubscriptionRequest
 */
export type PutMediaSubscriptionRequest = Message<"sited_io.media.v1.PutMediaSubscriptionRequest"> & {
  /**
   * @generated from field: string media_subscription_id = 1;
   */
  mediaSubscriptionId: string;

  /**
   * @generated from field: string buyer_user_id = 2;
   */
  buyerUserId: string;

  /**
   * @generated from field: string offer_id = 3;
   */
  offerId: string;

  /**
   * @generated from field: uint64 current_period_start = 4;
   */
  currentPeriodStart: bigint;

  /**
   * @generated from field: uint64 current_period_end = 5;
   */
  currentPeriodEnd: bigint;

  /**
   * @generated from field: string subscription_status = 6;
   */
  subscriptionStatus: string;

  /**
   * @generated from field: uint64 payed_at = 7;
   */
  payedAt: bigint;

  /**
   * @generated from field: uint64 payed_until = 8;
   */
  payedUntil: bigint;

  /**
   * @generated from field: string shop_id = 9;
   */
  shopId: string;

  /**
   * @generated from field: optional string stripe_subscription_id = 10;
   */
  stripeSubscriptionId?: string;

  /**
   * @generated from field: optional uint64 canceled_at = 11;
   */
  canceledAt?: bigint;

  /**
   * @generated from field: optional uint64 cancel_at = 12;
   */
  cancelAt?: bigint;
};

/**
 * Describes the message sited_io.media.v1.PutMediaSubscriptionRequest.
 * Use `create(PutMediaSubscriptionRequestSchema)` to create a new message.
 */
export const PutMediaSubscriptionRequestSchema: GenMessage<PutMediaSubscriptionRequest> = /*@__PURE__*/
  messageDesc(file_sited_io_media_v1_media_subscription, 1);

/**
 * @generated from message sited_io.media.v1.PutMediaSubscriptionResponse
 */
export type PutMediaSubscriptionResponse = Message<"sited_io.media.v1.PutMediaSubscriptionResponse"> & {
};

/**
 * Describes the message sited_io.media.v1.PutMediaSubscriptionResponse.
 * Use `create(PutMediaSubscriptionResponseSchema)` to create a new message.
 */
export const PutMediaSubscriptionResponseSchema: GenMessage<PutMediaSubscriptionResponse> = /*@__PURE__*/
  messageDesc(file_sited_io_media_v1_media_subscription, 2);

/**
 * @generated from message sited_io.media.v1.GetMediaSubscriptionRequest
 */
export type GetMediaSubscriptionRequest = Message<"sited_io.media.v1.GetMediaSubscriptionRequest"> & {
  /**
   * @generated from field: optional string media_subscription_id = 1;
   */
  mediaSubscriptionId?: string;

  /**
   * @generated from field: optional string offer_id = 2;
   */
  offerId?: string;
};

/**
 * Describes the message sited_io.media.v1.GetMediaSubscriptionRequest.
 * Use `create(GetMediaSubscriptionRequestSchema)` to create a new message.
 */
export const GetMediaSubscriptionRequestSchema: GenMessage<GetMediaSubscriptionRequest> = /*@__PURE__*/
  messageDesc(file_sited_io_media_v1_media_subscription, 3);

/**
 * @generated from message sited_io.media.v1.GetMediaSubscriptionResponse
 */
export type GetMediaSubscriptionResponse = Message<"sited_io.media.v1.GetMediaSubscriptionResponse"> & {
  /**
   * @generated from field: sited_io.media.v1.MediaSubscriptionResponse media_subscription = 1;
   */
  mediaSubscription?: MediaSubscriptionResponse;
};

/**
 * Describes the message sited_io.media.v1.GetMediaSubscriptionResponse.
 * Use `create(GetMediaSubscriptionResponseSchema)` to create a new message.
 */
export const GetMediaSubscriptionResponseSchema: GenMessage<GetMediaSubscriptionResponse> = /*@__PURE__*/
  messageDesc(file_sited_io_media_v1_media_subscription, 4);

/**
 * @generated from message sited_io.media.v1.ListMediaSubscriptionsRequest
 */
export type ListMediaSubscriptionsRequest = Message<"sited_io.media.v1.ListMediaSubscriptionsRequest"> & {
  /**
   * @generated from field: optional string shop_id = 1;
   */
  shopId?: string;

  /**
   * @generated from field: optional sited_io.types.v1.PaginationRequest pagination = 2;
   */
  pagination?: PaginationRequest;

  /**
   * @generated from field: optional bool is_accessible = 3;
   */
  isAccessible?: boolean;
};

/**
 * Describes the message sited_io.media.v1.ListMediaSubscriptionsRequest.
 * Use `create(ListMediaSubscriptionsRequestSchema)` to create a new message.
 */
export const ListMediaSubscriptionsRequestSchema: GenMessage<ListMediaSubscriptionsRequest> = /*@__PURE__*/
  messageDesc(file_sited_io_media_v1_media_subscription, 5);

/**
 * @generated from message sited_io.media.v1.ListMediaSubscriptionsResponse
 */
export type ListMediaSubscriptionsResponse = Message<"sited_io.media.v1.ListMediaSubscriptionsResponse"> & {
  /**
   * @generated from field: repeated sited_io.media.v1.MediaSubscriptionResponse media_subscriptions = 1;
   */
  mediaSubscriptions: MediaSubscriptionResponse[];

  /**
   * @generated from field: sited_io.types.v1.PaginationResponse pagination = 2;
   */
  pagination?: PaginationResponse;
};

/**
 * Describes the message sited_io.media.v1.ListMediaSubscriptionsResponse.
 * Use `create(ListMediaSubscriptionsResponseSchema)` to create a new message.
 */
export const ListMediaSubscriptionsResponseSchema: GenMessage<ListMediaSubscriptionsResponse> = /*@__PURE__*/
  messageDesc(file_sited_io_media_v1_media_subscription, 6);

/**
 * @generated from message sited_io.media.v1.CancelMediaSubscriptionRequest
 */
export type CancelMediaSubscriptionRequest = Message<"sited_io.media.v1.CancelMediaSubscriptionRequest"> & {
  /**
   * @generated from field: string media_subscription_id = 1;
   */
  mediaSubscriptionId: string;
};

/**
 * Describes the message sited_io.media.v1.CancelMediaSubscriptionRequest.
 * Use `create(CancelMediaSubscriptionRequestSchema)` to create a new message.
 */
export const CancelMediaSubscriptionRequestSchema: GenMessage<CancelMediaSubscriptionRequest> = /*@__PURE__*/
  messageDesc(file_sited_io_media_v1_media_subscription, 7);

/**
 * @generated from message sited_io.media.v1.CancelMediaSubscriptionResponse
 */
export type CancelMediaSubscriptionResponse = Message<"sited_io.media.v1.CancelMediaSubscriptionResponse"> & {
};

/**
 * Describes the message sited_io.media.v1.CancelMediaSubscriptionResponse.
 * Use `create(CancelMediaSubscriptionResponseSchema)` to create a new message.
 */
export const CancelMediaSubscriptionResponseSchema: GenMessage<CancelMediaSubscriptionResponse> = /*@__PURE__*/
  messageDesc(file_sited_io_media_v1_media_subscription, 8);

/**
 * @generated from message sited_io.media.v1.ResumeMediaSubscriptionRequest
 */
export type ResumeMediaSubscriptionRequest = Message<"sited_io.media.v1.ResumeMediaSubscriptionRequest"> & {
  /**
   * @generated from field: string media_subscription_id = 1;
   */
  mediaSubscriptionId: string;
};

/**
 * Describes the message sited_io.media.v1.ResumeMediaSubscriptionRequest.
 * Use `create(ResumeMediaSubscriptionRequestSchema)` to create a new message.
 */
export const ResumeMediaSubscriptionRequestSchema: GenMessage<ResumeMediaSubscriptionRequest> = /*@__PURE__*/
  messageDesc(file_sited_io_media_v1_media_subscription, 9);

/**
 * @generated from message sited_io.media.v1.ResumeMediaSubscriptionResponse
 */
export type ResumeMediaSubscriptionResponse = Message<"sited_io.media.v1.ResumeMediaSubscriptionResponse"> & {
};

/**
 * Describes the message sited_io.media.v1.ResumeMediaSubscriptionResponse.
 * Use `create(ResumeMediaSubscriptionResponseSchema)` to create a new message.
 */
export const ResumeMediaSubscriptionResponseSchema: GenMessage<ResumeMediaSubscriptionResponse> = /*@__PURE__*/
  messageDesc(file_sited_io_media_v1_media_subscription, 10);

/**
 * @generated from service sited_io.media.v1.MediaSubscriptionService
 */
export const MediaSubscriptionService: GenService<{
  /**
   * @generated from rpc sited_io.media.v1.MediaSubscriptionService.PutMediaSubscription
   */
  putMediaSubscription: {
    methodKind: "unary";
    input: typeof PutMediaSubscriptionRequestSchema;
    output: typeof PutMediaSubscriptionResponseSchema;
  },
  /**
   * @generated from rpc sited_io.media.v1.MediaSubscriptionService.GetMediaSubscription
   */
  getMediaSubscription: {
    methodKind: "unary";
    input: typeof GetMediaSubscriptionRequestSchema;
    output: typeof GetMediaSubscriptionResponseSchema;
  },
  /**
   * @generated from rpc sited_io.media.v1.MediaSubscriptionService.ListMediaSubscriptions
   */
  listMediaSubscriptions: {
    methodKind: "unary";
    input: typeof ListMediaSubscriptionsRequestSchema;
    output: typeof ListMediaSubscriptionsResponseSchema;
  },
  /**
   * @generated from rpc sited_io.media.v1.MediaSubscriptionService.CancelMediaSubscription
   */
  cancelMediaSubscription: {
    methodKind: "unary";
    input: typeof CancelMediaSubscriptionRequestSchema;
    output: typeof CancelMediaSubscriptionResponseSchema;
  },
  /**
   * @generated from rpc sited_io.media.v1.MediaSubscriptionService.ResumeMediaSubscription
   */
  resumeMediaSubscription: {
    methodKind: "unary";
    input: typeof ResumeMediaSubscriptionRequestSchema;
    output: typeof ResumeMediaSubscriptionResponseSchema;
  },
}> = /*@__PURE__*/
  serviceDesc(file_sited_io_media_v1_media_subscription, 0);


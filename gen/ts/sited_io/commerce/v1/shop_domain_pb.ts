// @generated by protoc-gen-es v2.2.2 with parameter "target=ts"
// @generated from file sited_io/commerce/v1/shop_domain.proto (package sited_io.commerce.v1, syntax proto3)
/* eslint-disable */

import type { GenEnum, GenFile, GenMessage, GenService } from "@bufbuild/protobuf/codegenv1";
import { enumDesc, fileDesc, messageDesc, serviceDesc } from "@bufbuild/protobuf/codegenv1";
import type { Message } from "@bufbuild/protobuf";

/**
 * Describes the file sited_io/commerce/v1/shop_domain.proto.
 */
export const file_sited_io_commerce_v1_shop_domain: GenFile = /*@__PURE__*/
  fileDesc("CiZzaXRlZF9pby9jb21tZXJjZS92MS9zaG9wX2RvbWFpbi5wcm90bxIUc2l0ZWRfaW8uY29tbWVyY2UudjEikQEKFERvbWFpblN0YXR1c1Jlc3BvbnNlEg8KB3Nob3BfaWQYASABKAkSDgoGZG9tYWluGAIgASgJEjIKBnN0YXR1cxgDIAEoDjIiLnNpdGVkX2lvLmNvbW1lcmNlLnYxLkRvbWFpblN0YXR1cxIWCgljbGllbnRfaWQYBCABKAlIAIgBAUIMCgpfY2xpZW50X2lkIjkKFkFkZERvbWFpblRvU2hvcFJlcXVlc3QSDwoHc2hvcF9pZBgBIAEoCRIOCgZkb21haW4YAiABKAkiGQoXQWRkRG9tYWluVG9TaG9wUmVzcG9uc2UiKQoWR2V0RG9tYWluU3RhdHVzUmVxdWVzdBIPCgdzaG9wX2lkGAEgASgJIlwKF0dldERvbWFpblN0YXR1c1Jlc3BvbnNlEkEKDWRvbWFpbl9zdGF0dXMYASABKAsyKi5zaXRlZF9pby5jb21tZXJjZS52MS5Eb21haW5TdGF0dXNSZXNwb25zZSItChtHZXRDbGllbnRJZEZvckRvbWFpblJlcXVlc3QSDgoGZG9tYWluGAEgASgJIkQKHEdldENsaWVudElkRm9yRG9tYWluUmVzcG9uc2USFgoJY2xpZW50X2lkGAEgASgJSACIAQFCDAoKX2NsaWVudF9pZCKDAQoZVXBkYXRlRG9tYWluU3RhdHVzUmVxdWVzdBIPCgdzaG9wX2lkGAEgASgJEg4KBmRvbWFpbhgCIAEoCRIyCgZzdGF0dXMYAyABKA4yIi5zaXRlZF9pby5jb21tZXJjZS52MS5Eb21haW5TdGF0dXMSEQoJY2xpZW50X2lkGAQgASgJIhwKGlVwZGF0ZURvbWFpblN0YXR1c1Jlc3BvbnNlIj4KG1JlbW92ZURvbWFpbkZyb21TaG9wUmVxdWVzdBIPCgdzaG9wX2lkGAEgASgJEg4KBmRvbWFpbhgCIAEoCSIeChxSZW1vdmVEb21haW5Gcm9tU2hvcFJlc3BvbnNlKmIKDERvbWFpblN0YXR1cxIdChlET01BSU5fU1RBVFVTX1VOU1BFQ0lGSUVEEAASGQoVRE9NQUlOX1NUQVRVU19QRU5ESU5HEAESGAoURE9NQUlOX1NUQVRVU19BQ1RJVkUQAjLvBAoRU2hvcERvbWFpblNlcnZpY2USbgoPQWRkRG9tYWluVG9TaG9wEiwuc2l0ZWRfaW8uY29tbWVyY2UudjEuQWRkRG9tYWluVG9TaG9wUmVxdWVzdBotLnNpdGVkX2lvLmNvbW1lcmNlLnYxLkFkZERvbWFpblRvU2hvcFJlc3BvbnNlEm4KD0dldERvbWFpblN0YXR1cxIsLnNpdGVkX2lvLmNvbW1lcmNlLnYxLkdldERvbWFpblN0YXR1c1JlcXVlc3QaLS5zaXRlZF9pby5jb21tZXJjZS52MS5HZXREb21haW5TdGF0dXNSZXNwb25zZRJ9ChRHZXRDbGllbnRJZEZvckRvbWFpbhIxLnNpdGVkX2lvLmNvbW1lcmNlLnYxLkdldENsaWVudElkRm9yRG9tYWluUmVxdWVzdBoyLnNpdGVkX2lvLmNvbW1lcmNlLnYxLkdldENsaWVudElkRm9yRG9tYWluUmVzcG9uc2USdwoSVXBkYXRlRG9tYWluU3RhdHVzEi8uc2l0ZWRfaW8uY29tbWVyY2UudjEuVXBkYXRlRG9tYWluU3RhdHVzUmVxdWVzdBowLnNpdGVkX2lvLmNvbW1lcmNlLnYxLlVwZGF0ZURvbWFpblN0YXR1c1Jlc3BvbnNlEn0KFFJlbW92ZURvbWFpbkZyb21TaG9wEjEuc2l0ZWRfaW8uY29tbWVyY2UudjEuUmVtb3ZlRG9tYWluRnJvbVNob3BSZXF1ZXN0GjIuc2l0ZWRfaW8uY29tbWVyY2UudjEuUmVtb3ZlRG9tYWluRnJvbVNob3BSZXNwb25zZRoDiAIBYgZwcm90bzM");

/**
 * @generated from message sited_io.commerce.v1.DomainStatusResponse
 */
export type DomainStatusResponse = Message<"sited_io.commerce.v1.DomainStatusResponse"> & {
  /**
   * @generated from field: string shop_id = 1;
   */
  shopId: string;

  /**
   * @generated from field: string domain = 2;
   */
  domain: string;

  /**
   * @generated from field: sited_io.commerce.v1.DomainStatus status = 3;
   */
  status: DomainStatus;

  /**
   * @generated from field: optional string client_id = 4;
   */
  clientId?: string;
};

/**
 * Describes the message sited_io.commerce.v1.DomainStatusResponse.
 * Use `create(DomainStatusResponseSchema)` to create a new message.
 */
export const DomainStatusResponseSchema: GenMessage<DomainStatusResponse> = /*@__PURE__*/
  messageDesc(file_sited_io_commerce_v1_shop_domain, 0);

/**
 * @generated from message sited_io.commerce.v1.AddDomainToShopRequest
 */
export type AddDomainToShopRequest = Message<"sited_io.commerce.v1.AddDomainToShopRequest"> & {
  /**
   * @generated from field: string shop_id = 1;
   */
  shopId: string;

  /**
   * @generated from field: string domain = 2;
   */
  domain: string;
};

/**
 * Describes the message sited_io.commerce.v1.AddDomainToShopRequest.
 * Use `create(AddDomainToShopRequestSchema)` to create a new message.
 */
export const AddDomainToShopRequestSchema: GenMessage<AddDomainToShopRequest> = /*@__PURE__*/
  messageDesc(file_sited_io_commerce_v1_shop_domain, 1);

/**
 * @generated from message sited_io.commerce.v1.AddDomainToShopResponse
 */
export type AddDomainToShopResponse = Message<"sited_io.commerce.v1.AddDomainToShopResponse"> & {
};

/**
 * Describes the message sited_io.commerce.v1.AddDomainToShopResponse.
 * Use `create(AddDomainToShopResponseSchema)` to create a new message.
 */
export const AddDomainToShopResponseSchema: GenMessage<AddDomainToShopResponse> = /*@__PURE__*/
  messageDesc(file_sited_io_commerce_v1_shop_domain, 2);

/**
 * @generated from message sited_io.commerce.v1.GetDomainStatusRequest
 */
export type GetDomainStatusRequest = Message<"sited_io.commerce.v1.GetDomainStatusRequest"> & {
  /**
   * @generated from field: string shop_id = 1;
   */
  shopId: string;
};

/**
 * Describes the message sited_io.commerce.v1.GetDomainStatusRequest.
 * Use `create(GetDomainStatusRequestSchema)` to create a new message.
 */
export const GetDomainStatusRequestSchema: GenMessage<GetDomainStatusRequest> = /*@__PURE__*/
  messageDesc(file_sited_io_commerce_v1_shop_domain, 3);

/**
 * @generated from message sited_io.commerce.v1.GetDomainStatusResponse
 */
export type GetDomainStatusResponse = Message<"sited_io.commerce.v1.GetDomainStatusResponse"> & {
  /**
   * @generated from field: sited_io.commerce.v1.DomainStatusResponse domain_status = 1;
   */
  domainStatus?: DomainStatusResponse;
};

/**
 * Describes the message sited_io.commerce.v1.GetDomainStatusResponse.
 * Use `create(GetDomainStatusResponseSchema)` to create a new message.
 */
export const GetDomainStatusResponseSchema: GenMessage<GetDomainStatusResponse> = /*@__PURE__*/
  messageDesc(file_sited_io_commerce_v1_shop_domain, 4);

/**
 * @generated from message sited_io.commerce.v1.GetClientIdForDomainRequest
 */
export type GetClientIdForDomainRequest = Message<"sited_io.commerce.v1.GetClientIdForDomainRequest"> & {
  /**
   * @generated from field: string domain = 1;
   */
  domain: string;
};

/**
 * Describes the message sited_io.commerce.v1.GetClientIdForDomainRequest.
 * Use `create(GetClientIdForDomainRequestSchema)` to create a new message.
 */
export const GetClientIdForDomainRequestSchema: GenMessage<GetClientIdForDomainRequest> = /*@__PURE__*/
  messageDesc(file_sited_io_commerce_v1_shop_domain, 5);

/**
 * @generated from message sited_io.commerce.v1.GetClientIdForDomainResponse
 */
export type GetClientIdForDomainResponse = Message<"sited_io.commerce.v1.GetClientIdForDomainResponse"> & {
  /**
   * @generated from field: optional string client_id = 1;
   */
  clientId?: string;
};

/**
 * Describes the message sited_io.commerce.v1.GetClientIdForDomainResponse.
 * Use `create(GetClientIdForDomainResponseSchema)` to create a new message.
 */
export const GetClientIdForDomainResponseSchema: GenMessage<GetClientIdForDomainResponse> = /*@__PURE__*/
  messageDesc(file_sited_io_commerce_v1_shop_domain, 6);

/**
 * @generated from message sited_io.commerce.v1.UpdateDomainStatusRequest
 */
export type UpdateDomainStatusRequest = Message<"sited_io.commerce.v1.UpdateDomainStatusRequest"> & {
  /**
   * @generated from field: string shop_id = 1;
   */
  shopId: string;

  /**
   * @generated from field: string domain = 2;
   */
  domain: string;

  /**
   * @generated from field: sited_io.commerce.v1.DomainStatus status = 3;
   */
  status: DomainStatus;

  /**
   * @generated from field: string client_id = 4;
   */
  clientId: string;
};

/**
 * Describes the message sited_io.commerce.v1.UpdateDomainStatusRequest.
 * Use `create(UpdateDomainStatusRequestSchema)` to create a new message.
 */
export const UpdateDomainStatusRequestSchema: GenMessage<UpdateDomainStatusRequest> = /*@__PURE__*/
  messageDesc(file_sited_io_commerce_v1_shop_domain, 7);

/**
 * @generated from message sited_io.commerce.v1.UpdateDomainStatusResponse
 */
export type UpdateDomainStatusResponse = Message<"sited_io.commerce.v1.UpdateDomainStatusResponse"> & {
};

/**
 * Describes the message sited_io.commerce.v1.UpdateDomainStatusResponse.
 * Use `create(UpdateDomainStatusResponseSchema)` to create a new message.
 */
export const UpdateDomainStatusResponseSchema: GenMessage<UpdateDomainStatusResponse> = /*@__PURE__*/
  messageDesc(file_sited_io_commerce_v1_shop_domain, 8);

/**
 * @generated from message sited_io.commerce.v1.RemoveDomainFromShopRequest
 */
export type RemoveDomainFromShopRequest = Message<"sited_io.commerce.v1.RemoveDomainFromShopRequest"> & {
  /**
   * @generated from field: string shop_id = 1;
   */
  shopId: string;

  /**
   * @generated from field: string domain = 2;
   */
  domain: string;
};

/**
 * Describes the message sited_io.commerce.v1.RemoveDomainFromShopRequest.
 * Use `create(RemoveDomainFromShopRequestSchema)` to create a new message.
 */
export const RemoveDomainFromShopRequestSchema: GenMessage<RemoveDomainFromShopRequest> = /*@__PURE__*/
  messageDesc(file_sited_io_commerce_v1_shop_domain, 9);

/**
 * @generated from message sited_io.commerce.v1.RemoveDomainFromShopResponse
 */
export type RemoveDomainFromShopResponse = Message<"sited_io.commerce.v1.RemoveDomainFromShopResponse"> & {
};

/**
 * Describes the message sited_io.commerce.v1.RemoveDomainFromShopResponse.
 * Use `create(RemoveDomainFromShopResponseSchema)` to create a new message.
 */
export const RemoveDomainFromShopResponseSchema: GenMessage<RemoveDomainFromShopResponse> = /*@__PURE__*/
  messageDesc(file_sited_io_commerce_v1_shop_domain, 10);

/**
 * @generated from enum sited_io.commerce.v1.DomainStatus
 */
export enum DomainStatus {
  /**
   * @generated from enum value: DOMAIN_STATUS_UNSPECIFIED = 0;
   */
  UNSPECIFIED = 0,

  /**
   * @generated from enum value: DOMAIN_STATUS_PENDING = 1;
   */
  PENDING = 1,

  /**
   * @generated from enum value: DOMAIN_STATUS_ACTIVE = 2;
   */
  ACTIVE = 2,
}

/**
 * Describes the enum sited_io.commerce.v1.DomainStatus.
 */
export const DomainStatusSchema: GenEnum<DomainStatus> = /*@__PURE__*/
  enumDesc(file_sited_io_commerce_v1_shop_domain, 0);

/**
 * @generated from service sited_io.commerce.v1.ShopDomainService
 * @deprecated
 */
export const ShopDomainService: GenService<{
  /**
   * @generated from rpc sited_io.commerce.v1.ShopDomainService.AddDomainToShop
   */
  addDomainToShop: {
    methodKind: "unary";
    input: typeof AddDomainToShopRequestSchema;
    output: typeof AddDomainToShopResponseSchema;
  },
  /**
   * @generated from rpc sited_io.commerce.v1.ShopDomainService.GetDomainStatus
   */
  getDomainStatus: {
    methodKind: "unary";
    input: typeof GetDomainStatusRequestSchema;
    output: typeof GetDomainStatusResponseSchema;
  },
  /**
   * @generated from rpc sited_io.commerce.v1.ShopDomainService.GetClientIdForDomain
   */
  getClientIdForDomain: {
    methodKind: "unary";
    input: typeof GetClientIdForDomainRequestSchema;
    output: typeof GetClientIdForDomainResponseSchema;
  },
  /**
   * @generated from rpc sited_io.commerce.v1.ShopDomainService.UpdateDomainStatus
   */
  updateDomainStatus: {
    methodKind: "unary";
    input: typeof UpdateDomainStatusRequestSchema;
    output: typeof UpdateDomainStatusResponseSchema;
  },
  /**
   * @generated from rpc sited_io.commerce.v1.ShopDomainService.RemoveDomainFromShop
   */
  removeDomainFromShop: {
    methodKind: "unary";
    input: typeof RemoveDomainFromShopRequestSchema;
    output: typeof RemoveDomainFromShopResponseSchema;
  },
}> = /*@__PURE__*/
  serviceDesc(file_sited_io_commerce_v1_shop_domain, 0);


// @generated by protoc-gen-es v2.2.2 with parameter "target=ts"
// @generated from file sited_io/websites/v1/customization.proto (package sited_io.websites.v1, syntax proto3)
/* eslint-disable */

import type { GenFile, GenMessage, GenService } from "@bufbuild/protobuf/codegenv1";
import { fileDesc, messageDesc, serviceDesc } from "@bufbuild/protobuf/codegenv1";
import type { MediaUpload } from "../../media/v1/media_pb";
import { file_sited_io_media_v1_media } from "../../media/v1/media_pb";
import type { Message } from "@bufbuild/protobuf";

/**
 * Describes the file sited_io/websites/v1/customization.proto.
 */
export const file_sited_io_websites_v1_customization: GenFile = /*@__PURE__*/
  fileDesc("CihzaXRlZF9pby93ZWJzaXRlcy92MS9jdXN0b21pemF0aW9uLnByb3RvEhRzaXRlZF9pby53ZWJzaXRlcy52MSKnAQoVQ3VzdG9taXphdGlvblJlc3BvbnNlEhoKDXByaW1hcnlfY29sb3IYASABKAlIAIgBARIcCg9zZWNvbmRhcnlfY29sb3IYAiABKAlIAYgBARIbCg5sb2dvX2ltYWdlX3VybBgDIAEoCUgCiAEBQhAKDl9wcmltYXJ5X2NvbG9yQhIKEF9zZWNvbmRhcnlfY29sb3JCEQoPX2xvZ29faW1hZ2VfdXJsIpABChpVcGRhdGVDdXN0b21pemF0aW9uUmVxdWVzdBISCgp3ZWJzaXRlX2lkGAEgASgJEhoKDXByaW1hcnlfY29sb3IYAiABKAlIAIgBARIcCg9zZWNvbmRhcnlfY29sb3IYAyABKAlIAYgBAUIQCg5fcHJpbWFyeV9jb2xvckISChBfc2Vjb25kYXJ5X2NvbG9yImEKG1VwZGF0ZUN1c3RvbWl6YXRpb25SZXNwb25zZRJCCg1jdXN0b21pemF0aW9uGAEgASgLMisuc2l0ZWRfaW8ud2Vic2l0ZXMudjEuQ3VzdG9taXphdGlvblJlc3BvbnNlIlgKE1B1dExvZ29JbWFnZVJlcXVlc3QSEgoKd2Vic2l0ZV9pZBgBIAEoCRItCgVpbWFnZRgCIAEoCzIeLnNpdGVkX2lvLm1lZGlhLnYxLk1lZGlhVXBsb2FkIhYKFFB1dExvZ29JbWFnZVJlc3BvbnNlIiwKFlJlbW92ZUxvZ29JbWFnZVJlcXVlc3QSEgoKd2Vic2l0ZV9pZBgBIAEoCSIZChdSZW1vdmVMb2dvSW1hZ2VSZXNwb25zZTLpAgoUQ3VzdG9taXphdGlvblNlcnZpY2USegoTVXBkYXRlQ3VzdG9taXphdGlvbhIwLnNpdGVkX2lvLndlYnNpdGVzLnYxLlVwZGF0ZUN1c3RvbWl6YXRpb25SZXF1ZXN0GjEuc2l0ZWRfaW8ud2Vic2l0ZXMudjEuVXBkYXRlQ3VzdG9taXphdGlvblJlc3BvbnNlEmUKDFB1dExvZ29JbWFnZRIpLnNpdGVkX2lvLndlYnNpdGVzLnYxLlB1dExvZ29JbWFnZVJlcXVlc3QaKi5zaXRlZF9pby53ZWJzaXRlcy52MS5QdXRMb2dvSW1hZ2VSZXNwb25zZRJuCg9SZW1vdmVMb2dvSW1hZ2USLC5zaXRlZF9pby53ZWJzaXRlcy52MS5SZW1vdmVMb2dvSW1hZ2VSZXF1ZXN0Gi0uc2l0ZWRfaW8ud2Vic2l0ZXMudjEuUmVtb3ZlTG9nb0ltYWdlUmVzcG9uc2ViBnByb3RvMw", [file_sited_io_media_v1_media]);

/**
 * @generated from message sited_io.websites.v1.CustomizationResponse
 */
export type CustomizationResponse = Message<"sited_io.websites.v1.CustomizationResponse"> & {
  /**
   * @generated from field: optional string primary_color = 1;
   */
  primaryColor?: string;

  /**
   * @generated from field: optional string secondary_color = 2;
   */
  secondaryColor?: string;

  /**
   * @generated from field: optional string logo_image_url = 3;
   */
  logoImageUrl?: string;
};

/**
 * Describes the message sited_io.websites.v1.CustomizationResponse.
 * Use `create(CustomizationResponseSchema)` to create a new message.
 */
export const CustomizationResponseSchema: GenMessage<CustomizationResponse> = /*@__PURE__*/
  messageDesc(file_sited_io_websites_v1_customization, 0);

/**
 * @generated from message sited_io.websites.v1.UpdateCustomizationRequest
 */
export type UpdateCustomizationRequest = Message<"sited_io.websites.v1.UpdateCustomizationRequest"> & {
  /**
   * @generated from field: string website_id = 1;
   */
  websiteId: string;

  /**
   * @generated from field: optional string primary_color = 2;
   */
  primaryColor?: string;

  /**
   * @generated from field: optional string secondary_color = 3;
   */
  secondaryColor?: string;
};

/**
 * Describes the message sited_io.websites.v1.UpdateCustomizationRequest.
 * Use `create(UpdateCustomizationRequestSchema)` to create a new message.
 */
export const UpdateCustomizationRequestSchema: GenMessage<UpdateCustomizationRequest> = /*@__PURE__*/
  messageDesc(file_sited_io_websites_v1_customization, 1);

/**
 * @generated from message sited_io.websites.v1.UpdateCustomizationResponse
 */
export type UpdateCustomizationResponse = Message<"sited_io.websites.v1.UpdateCustomizationResponse"> & {
  /**
   * @generated from field: sited_io.websites.v1.CustomizationResponse customization = 1;
   */
  customization?: CustomizationResponse;
};

/**
 * Describes the message sited_io.websites.v1.UpdateCustomizationResponse.
 * Use `create(UpdateCustomizationResponseSchema)` to create a new message.
 */
export const UpdateCustomizationResponseSchema: GenMessage<UpdateCustomizationResponse> = /*@__PURE__*/
  messageDesc(file_sited_io_websites_v1_customization, 2);

/**
 * @generated from message sited_io.websites.v1.PutLogoImageRequest
 */
export type PutLogoImageRequest = Message<"sited_io.websites.v1.PutLogoImageRequest"> & {
  /**
   * @generated from field: string website_id = 1;
   */
  websiteId: string;

  /**
   * @generated from field: sited_io.media.v1.MediaUpload image = 2;
   */
  image?: MediaUpload;
};

/**
 * Describes the message sited_io.websites.v1.PutLogoImageRequest.
 * Use `create(PutLogoImageRequestSchema)` to create a new message.
 */
export const PutLogoImageRequestSchema: GenMessage<PutLogoImageRequest> = /*@__PURE__*/
  messageDesc(file_sited_io_websites_v1_customization, 3);

/**
 * @generated from message sited_io.websites.v1.PutLogoImageResponse
 */
export type PutLogoImageResponse = Message<"sited_io.websites.v1.PutLogoImageResponse"> & {
};

/**
 * Describes the message sited_io.websites.v1.PutLogoImageResponse.
 * Use `create(PutLogoImageResponseSchema)` to create a new message.
 */
export const PutLogoImageResponseSchema: GenMessage<PutLogoImageResponse> = /*@__PURE__*/
  messageDesc(file_sited_io_websites_v1_customization, 4);

/**
 * @generated from message sited_io.websites.v1.RemoveLogoImageRequest
 */
export type RemoveLogoImageRequest = Message<"sited_io.websites.v1.RemoveLogoImageRequest"> & {
  /**
   * @generated from field: string website_id = 1;
   */
  websiteId: string;
};

/**
 * Describes the message sited_io.websites.v1.RemoveLogoImageRequest.
 * Use `create(RemoveLogoImageRequestSchema)` to create a new message.
 */
export const RemoveLogoImageRequestSchema: GenMessage<RemoveLogoImageRequest> = /*@__PURE__*/
  messageDesc(file_sited_io_websites_v1_customization, 5);

/**
 * @generated from message sited_io.websites.v1.RemoveLogoImageResponse
 */
export type RemoveLogoImageResponse = Message<"sited_io.websites.v1.RemoveLogoImageResponse"> & {
};

/**
 * Describes the message sited_io.websites.v1.RemoveLogoImageResponse.
 * Use `create(RemoveLogoImageResponseSchema)` to create a new message.
 */
export const RemoveLogoImageResponseSchema: GenMessage<RemoveLogoImageResponse> = /*@__PURE__*/
  messageDesc(file_sited_io_websites_v1_customization, 6);

/**
 * @generated from service sited_io.websites.v1.CustomizationService
 */
export const CustomizationService: GenService<{
  /**
   * @generated from rpc sited_io.websites.v1.CustomizationService.UpdateCustomization
   */
  updateCustomization: {
    methodKind: "unary";
    input: typeof UpdateCustomizationRequestSchema;
    output: typeof UpdateCustomizationResponseSchema;
  },
  /**
   * @generated from rpc sited_io.websites.v1.CustomizationService.PutLogoImage
   */
  putLogoImage: {
    methodKind: "unary";
    input: typeof PutLogoImageRequestSchema;
    output: typeof PutLogoImageResponseSchema;
  },
  /**
   * @generated from rpc sited_io.websites.v1.CustomizationService.RemoveLogoImage
   */
  removeLogoImage: {
    methodKind: "unary";
    input: typeof RemoveLogoImageRequestSchema;
    output: typeof RemoveLogoImageResponseSchema;
  },
}> = /*@__PURE__*/
  serviceDesc(file_sited_io_websites_v1_customization, 0);


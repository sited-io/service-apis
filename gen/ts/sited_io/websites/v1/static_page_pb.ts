// @generated by protoc-gen-es v2.2.2 with parameter "target=ts"
// @generated from file sited_io/websites/v1/static_page.proto (package sited_io.websites.v1, syntax proto3)
/* eslint-disable */

import type { GenFile, GenMessage, GenService } from "@bufbuild/protobuf/codegenv1";
import { fileDesc, messageDesc, serviceDesc } from "@bufbuild/protobuf/codegenv1";
import type { Message } from "@bufbuild/protobuf";

/**
 * Describes the file sited_io/websites/v1/static_page.proto.
 */
export const file_sited_io_websites_v1_static_page: GenFile = /*@__PURE__*/
  fileDesc("CiZzaXRlZF9pby93ZWJzaXRlcy92MS9zdGF0aWNfcGFnZS5wcm90bxIUc2l0ZWRfaW8ud2Vic2l0ZXMudjEifwoSU3RhdGljUGFnZVJlc3BvbnNlEg8KB3BhZ2VfaWQYASABKAMSEgoKd2Vic2l0ZV9pZBgCIAEoCRIPCgd1c2VyX2lkGAMgASgJEjMKCmNvbXBvbmVudHMYBCADKAsyHy5zaXRlZF9pby53ZWJzaXRlcy52MS5Db21wb25lbnQiJwoUR2V0U3RhdGljUGFnZVJlcXVlc3QSDwoHcGFnZV9pZBgBIAEoAyJWChVHZXRTdGF0aWNQYWdlUmVzcG9uc2USPQoLc3RhdGljX3BhZ2UYASABKAsyKC5zaXRlZF9pby53ZWJzaXRlcy52MS5TdGF0aWNQYWdlUmVzcG9uc2UiXwoXVXBkYXRlU3RhdGljUGFnZVJlcXVlc3QSDwoHcGFnZV9pZBgBIAEoAxIzCgpjb21wb25lbnRzGAIgAygLMh8uc2l0ZWRfaW8ud2Vic2l0ZXMudjEuQ29tcG9uZW50IlkKGFVwZGF0ZVN0YXRpY1BhZ2VSZXNwb25zZRI9CgtzdGF0aWNfcGFnZRgBIAEoCzIoLnNpdGVkX2lvLndlYnNpdGVzLnYxLlN0YXRpY1BhZ2VSZXNwb25zZSJeCglDb21wb25lbnQSFAoMY29tcG9uZW50X2lkGAEgASgJEjsKDmNvbXBvbmVudF90eXBlGAIgASgLMiMuc2l0ZWRfaW8ud2Vic2l0ZXMudjEuQ29tcG9uZW50VHlwZSKQAQoNQ29tcG9uZW50VHlwZRI3CgZoZWFkZXIYASABKAsyJS5zaXRlZF9pby53ZWJzaXRlcy52MS5IZWFkZXJDb21wb25lbnRIABI9CglwYXJhZ3JhcGgYAiABKAsyKC5zaXRlZF9pby53ZWJzaXRlcy52MS5QYXJhZ3JhcGhDb21wb25lbnRIAEIHCgVpbm5lciIxCg9IZWFkZXJDb21wb25lbnQSDQoFbGV2ZWwYASABKAUSDwoHY29udGVudBgCIAEoCSJKChJQYXJhZ3JhcGhDb21wb25lbnQSNAoHY29udGVudBgBIAMoCzIjLnNpdGVkX2lvLndlYnNpdGVzLnYxLklubGluZUVsZW1lbnQihQEKDUlubGluZUVsZW1lbnQSMQoEdGV4dBgBIAEoCzIhLnNpdGVkX2lvLndlYnNpdGVzLnYxLlRleHRFbGVtZW50SAASMQoEbGluaxgCIAEoCzIhLnNpdGVkX2lvLndlYnNpdGVzLnYxLkxpbmtFbGVtZW50SABCDgoMZWxlbWVudF90eXBlIhsKC1RleHRFbGVtZW50EgwKBHRleHQYASABKAkiKAoLTGlua0VsZW1lbnQSDAoEdGV4dBgBIAEoCRILCgN1cmwYAiABKAky8AEKEVN0YXRpY1BhZ2VTZXJ2aWNlEmgKDUdldFN0YXRpY1BhZ2USKi5zaXRlZF9pby53ZWJzaXRlcy52MS5HZXRTdGF0aWNQYWdlUmVxdWVzdBorLnNpdGVkX2lvLndlYnNpdGVzLnYxLkdldFN0YXRpY1BhZ2VSZXNwb25zZRJxChBVcGRhdGVTdGF0aWNQYWdlEi0uc2l0ZWRfaW8ud2Vic2l0ZXMudjEuVXBkYXRlU3RhdGljUGFnZVJlcXVlc3QaLi5zaXRlZF9pby53ZWJzaXRlcy52MS5VcGRhdGVTdGF0aWNQYWdlUmVzcG9uc2ViBnByb3RvMw");

/**
 * @generated from message sited_io.websites.v1.StaticPageResponse
 */
export type StaticPageResponse = Message<"sited_io.websites.v1.StaticPageResponse"> & {
  /**
   * @generated from field: int64 page_id = 1;
   */
  pageId: bigint;

  /**
   * @generated from field: string website_id = 2;
   */
  websiteId: string;

  /**
   * @generated from field: string user_id = 3;
   */
  userId: string;

  /**
   * @generated from field: repeated sited_io.websites.v1.Component components = 4;
   */
  components: Component[];
};

/**
 * Describes the message sited_io.websites.v1.StaticPageResponse.
 * Use `create(StaticPageResponseSchema)` to create a new message.
 */
export const StaticPageResponseSchema: GenMessage<StaticPageResponse> = /*@__PURE__*/
  messageDesc(file_sited_io_websites_v1_static_page, 0);

/**
 * @generated from message sited_io.websites.v1.GetStaticPageRequest
 */
export type GetStaticPageRequest = Message<"sited_io.websites.v1.GetStaticPageRequest"> & {
  /**
   * @generated from field: int64 page_id = 1;
   */
  pageId: bigint;
};

/**
 * Describes the message sited_io.websites.v1.GetStaticPageRequest.
 * Use `create(GetStaticPageRequestSchema)` to create a new message.
 */
export const GetStaticPageRequestSchema: GenMessage<GetStaticPageRequest> = /*@__PURE__*/
  messageDesc(file_sited_io_websites_v1_static_page, 1);

/**
 * @generated from message sited_io.websites.v1.GetStaticPageResponse
 */
export type GetStaticPageResponse = Message<"sited_io.websites.v1.GetStaticPageResponse"> & {
  /**
   * @generated from field: sited_io.websites.v1.StaticPageResponse static_page = 1;
   */
  staticPage?: StaticPageResponse;
};

/**
 * Describes the message sited_io.websites.v1.GetStaticPageResponse.
 * Use `create(GetStaticPageResponseSchema)` to create a new message.
 */
export const GetStaticPageResponseSchema: GenMessage<GetStaticPageResponse> = /*@__PURE__*/
  messageDesc(file_sited_io_websites_v1_static_page, 2);

/**
 * @generated from message sited_io.websites.v1.UpdateStaticPageRequest
 */
export type UpdateStaticPageRequest = Message<"sited_io.websites.v1.UpdateStaticPageRequest"> & {
  /**
   * @generated from field: int64 page_id = 1;
   */
  pageId: bigint;

  /**
   * @generated from field: repeated sited_io.websites.v1.Component components = 2;
   */
  components: Component[];
};

/**
 * Describes the message sited_io.websites.v1.UpdateStaticPageRequest.
 * Use `create(UpdateStaticPageRequestSchema)` to create a new message.
 */
export const UpdateStaticPageRequestSchema: GenMessage<UpdateStaticPageRequest> = /*@__PURE__*/
  messageDesc(file_sited_io_websites_v1_static_page, 3);

/**
 * @generated from message sited_io.websites.v1.UpdateStaticPageResponse
 */
export type UpdateStaticPageResponse = Message<"sited_io.websites.v1.UpdateStaticPageResponse"> & {
  /**
   * @generated from field: sited_io.websites.v1.StaticPageResponse static_page = 1;
   */
  staticPage?: StaticPageResponse;
};

/**
 * Describes the message sited_io.websites.v1.UpdateStaticPageResponse.
 * Use `create(UpdateStaticPageResponseSchema)` to create a new message.
 */
export const UpdateStaticPageResponseSchema: GenMessage<UpdateStaticPageResponse> = /*@__PURE__*/
  messageDesc(file_sited_io_websites_v1_static_page, 4);

/**
 * @generated from message sited_io.websites.v1.Component
 */
export type Component = Message<"sited_io.websites.v1.Component"> & {
  /**
   * @generated from field: string component_id = 1;
   */
  componentId: string;

  /**
   * @generated from field: sited_io.websites.v1.ComponentType component_type = 2;
   */
  componentType?: ComponentType;
};

/**
 * Describes the message sited_io.websites.v1.Component.
 * Use `create(ComponentSchema)` to create a new message.
 */
export const ComponentSchema: GenMessage<Component> = /*@__PURE__*/
  messageDesc(file_sited_io_websites_v1_static_page, 5);

/**
 * @generated from message sited_io.websites.v1.ComponentType
 */
export type ComponentType = Message<"sited_io.websites.v1.ComponentType"> & {
  /**
   * @generated from oneof sited_io.websites.v1.ComponentType.inner
   */
  inner: {
    /**
     * @generated from field: sited_io.websites.v1.HeaderComponent header = 1;
     */
    value: HeaderComponent;
    case: "header";
  } | {
    /**
     * @generated from field: sited_io.websites.v1.ParagraphComponent paragraph = 2;
     */
    value: ParagraphComponent;
    case: "paragraph";
  } | { case: undefined; value?: undefined };
};

/**
 * Describes the message sited_io.websites.v1.ComponentType.
 * Use `create(ComponentTypeSchema)` to create a new message.
 */
export const ComponentTypeSchema: GenMessage<ComponentType> = /*@__PURE__*/
  messageDesc(file_sited_io_websites_v1_static_page, 6);

/**
 * @generated from message sited_io.websites.v1.HeaderComponent
 */
export type HeaderComponent = Message<"sited_io.websites.v1.HeaderComponent"> & {
  /**
   * @generated from field: int32 level = 1;
   */
  level: number;

  /**
   * @generated from field: string content = 2;
   */
  content: string;
};

/**
 * Describes the message sited_io.websites.v1.HeaderComponent.
 * Use `create(HeaderComponentSchema)` to create a new message.
 */
export const HeaderComponentSchema: GenMessage<HeaderComponent> = /*@__PURE__*/
  messageDesc(file_sited_io_websites_v1_static_page, 7);

/**
 * @generated from message sited_io.websites.v1.ParagraphComponent
 */
export type ParagraphComponent = Message<"sited_io.websites.v1.ParagraphComponent"> & {
  /**
   * @generated from field: repeated sited_io.websites.v1.InlineElement content = 1;
   */
  content: InlineElement[];
};

/**
 * Describes the message sited_io.websites.v1.ParagraphComponent.
 * Use `create(ParagraphComponentSchema)` to create a new message.
 */
export const ParagraphComponentSchema: GenMessage<ParagraphComponent> = /*@__PURE__*/
  messageDesc(file_sited_io_websites_v1_static_page, 8);

/**
 * @generated from message sited_io.websites.v1.InlineElement
 */
export type InlineElement = Message<"sited_io.websites.v1.InlineElement"> & {
  /**
   * @generated from oneof sited_io.websites.v1.InlineElement.element_type
   */
  elementType: {
    /**
     * @generated from field: sited_io.websites.v1.TextElement text = 1;
     */
    value: TextElement;
    case: "text";
  } | {
    /**
     * @generated from field: sited_io.websites.v1.LinkElement link = 2;
     */
    value: LinkElement;
    case: "link";
  } | { case: undefined; value?: undefined };
};

/**
 * Describes the message sited_io.websites.v1.InlineElement.
 * Use `create(InlineElementSchema)` to create a new message.
 */
export const InlineElementSchema: GenMessage<InlineElement> = /*@__PURE__*/
  messageDesc(file_sited_io_websites_v1_static_page, 9);

/**
 * @generated from message sited_io.websites.v1.TextElement
 */
export type TextElement = Message<"sited_io.websites.v1.TextElement"> & {
  /**
   * @generated from field: string text = 1;
   */
  text: string;
};

/**
 * Describes the message sited_io.websites.v1.TextElement.
 * Use `create(TextElementSchema)` to create a new message.
 */
export const TextElementSchema: GenMessage<TextElement> = /*@__PURE__*/
  messageDesc(file_sited_io_websites_v1_static_page, 10);

/**
 * @generated from message sited_io.websites.v1.LinkElement
 */
export type LinkElement = Message<"sited_io.websites.v1.LinkElement"> & {
  /**
   * @generated from field: string text = 1;
   */
  text: string;

  /**
   * @generated from field: string url = 2;
   */
  url: string;
};

/**
 * Describes the message sited_io.websites.v1.LinkElement.
 * Use `create(LinkElementSchema)` to create a new message.
 */
export const LinkElementSchema: GenMessage<LinkElement> = /*@__PURE__*/
  messageDesc(file_sited_io_websites_v1_static_page, 11);

/**
 * @generated from service sited_io.websites.v1.StaticPageService
 */
export const StaticPageService: GenService<{
  /**
   * @generated from rpc sited_io.websites.v1.StaticPageService.GetStaticPage
   */
  getStaticPage: {
    methodKind: "unary";
    input: typeof GetStaticPageRequestSchema;
    output: typeof GetStaticPageResponseSchema;
  },
  /**
   * @generated from rpc sited_io.websites.v1.StaticPageService.UpdateStaticPage
   */
  updateStaticPage: {
    methodKind: "unary";
    input: typeof UpdateStaticPageRequestSchema;
    output: typeof UpdateStaticPageResponseSchema;
  },
}> = /*@__PURE__*/
  serviceDesc(file_sited_io_websites_v1_static_page, 0);


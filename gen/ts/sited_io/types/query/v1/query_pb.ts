// @generated by protoc-gen-es v2.2.2 with parameter "target=ts"
// @generated from file sited_io/types/query/v1/query.proto (package sited_io.types.query.v1, syntax proto3)
/* eslint-disable */

import type { GenEnum, GenFile, GenMessage } from "@bufbuild/protobuf/codegenv1";
import { enumDesc, fileDesc, messageDesc } from "@bufbuild/protobuf/codegenv1";
import type { Message } from "@bufbuild/protobuf";

/**
 * Describes the file sited_io/types/query/v1/query.proto.
 */
export const file_sited_io_types_query_v1_query: GenFile = /*@__PURE__*/
  fileDesc("CiNzaXRlZF9pby90eXBlcy9xdWVyeS92MS9xdWVyeS5wcm90bxIXc2l0ZWRfaW8udHlwZXMucXVlcnkudjEiLwoRUGFnaW5hdGlvblJlcXVlc3QSDAoEcGFnZRgBIAEoDRIMCgRzaXplGAIgASgNIkgKElBhZ2luYXRpb25SZXNwb25zZRIMCgRwYWdlGAEgASgNEgwKBHNpemUYAiABKA0SFgoOdG90YWxfZWxlbWVudHMYAyABKA0qTQoJRGlyZWN0aW9uEhkKFURJUkVDVElPTl9VTlNQRUNJRklFRBAAEhEKDURJUkVDVElPTl9BU0MQARISCg5ESVJFQ1RJT05fREVTQxACYgZwcm90bzM");

/**
 * @generated from message sited_io.types.query.v1.PaginationRequest
 */
export type PaginationRequest = Message<"sited_io.types.query.v1.PaginationRequest"> & {
  /**
   * @generated from field: uint32 page = 1;
   */
  page: number;

  /**
   * @generated from field: uint32 size = 2;
   */
  size: number;
};

/**
 * Describes the message sited_io.types.query.v1.PaginationRequest.
 * Use `create(PaginationRequestSchema)` to create a new message.
 */
export const PaginationRequestSchema: GenMessage<PaginationRequest> = /*@__PURE__*/
  messageDesc(file_sited_io_types_query_v1_query, 0);

/**
 * @generated from message sited_io.types.query.v1.PaginationResponse
 */
export type PaginationResponse = Message<"sited_io.types.query.v1.PaginationResponse"> & {
  /**
   * @generated from field: uint32 page = 1;
   */
  page: number;

  /**
   * @generated from field: uint32 size = 2;
   */
  size: number;

  /**
   * @generated from field: uint32 total_elements = 3;
   */
  totalElements: number;
};

/**
 * Describes the message sited_io.types.query.v1.PaginationResponse.
 * Use `create(PaginationResponseSchema)` to create a new message.
 */
export const PaginationResponseSchema: GenMessage<PaginationResponse> = /*@__PURE__*/
  messageDesc(file_sited_io_types_query_v1_query, 1);

/**
 * @generated from enum sited_io.types.query.v1.Direction
 */
export enum Direction {
  /**
   * @generated from enum value: DIRECTION_UNSPECIFIED = 0;
   */
  UNSPECIFIED = 0,

  /**
   * @generated from enum value: DIRECTION_ASC = 1;
   */
  ASC = 1,

  /**
   * @generated from enum value: DIRECTION_DESC = 2;
   */
  DESC = 2,
}

/**
 * Describes the enum sited_io.types.query.v1.Direction.
 */
export const DirectionSchema: GenEnum<Direction> = /*@__PURE__*/
  enumDesc(file_sited_io_types_query_v1_query, 0);

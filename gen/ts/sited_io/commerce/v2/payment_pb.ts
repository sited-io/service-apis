// @generated by protoc-gen-es v2.2.2 with parameter "target=ts"
// @generated from file sited_io/commerce/v2/payment.proto (package sited_io.commerce.v2, syntax proto3)
/* eslint-disable */

import type { GenFile, GenMessage } from "@bufbuild/protobuf/codegenv1";
import { fileDesc, messageDesc } from "@bufbuild/protobuf/codegenv1";
import type { OrderType, PaymentMethod } from "./order_pb";
import { file_sited_io_commerce_v2_order } from "./order_pb";
import type { Message } from "@bufbuild/protobuf";

/**
 * Describes the file sited_io/commerce/v2/payment.proto.
 */
export const file_sited_io_commerce_v2_payment: GenFile = /*@__PURE__*/
  fileDesc("CiJzaXRlZF9pby9jb21tZXJjZS92Mi9wYXltZW50LnByb3RvEhRzaXRlZF9pby5jb21tZXJjZS52MiLNAQoHUGF5bWVudBIQCghvcmRlcl9pZBgBIAEoCRIQCghvZmZlcl9pZBgCIAEoCRIaCg1idXllcl91c2VyX2lkGAMgASgJSACIAQESMwoKb3JkZXJfdHlwZRgEIAEoCzIfLnNpdGVkX2lvLmNvbW1lcmNlLnYyLk9yZGVyVHlwZRI7Cg5wYXltZW50X21ldGhvZBgFIAEoCzIjLnNpdGVkX2lvLmNvbW1lcmNlLnYyLlBheW1lbnRNZXRob2RCEAoOX2J1eWVyX3VzZXJfaWRiBnByb3RvMw", [file_sited_io_commerce_v2_order]);

/**
 * @generated from message sited_io.commerce.v2.Payment
 */
export type Payment = Message<"sited_io.commerce.v2.Payment"> & {
  /**
   * @generated from field: string order_id = 1;
   */
  orderId: string;

  /**
   * @generated from field: string offer_id = 2;
   */
  offerId: string;

  /**
   * @generated from field: optional string buyer_user_id = 3;
   */
  buyerUserId?: string;

  /**
   * @generated from field: sited_io.commerce.v2.OrderType order_type = 4;
   */
  orderType?: OrderType;

  /**
   * @generated from field: sited_io.commerce.v2.PaymentMethod payment_method = 5;
   */
  paymentMethod?: PaymentMethod;
};

/**
 * Describes the message sited_io.commerce.v2.Payment.
 * Use `create(PaymentSchema)` to create a new message.
 */
export const PaymentSchema: GenMessage<Payment> = /*@__PURE__*/
  messageDesc(file_sited_io_commerce_v2_payment, 0);

// @generated by protoc-gen-connect-es v1.6.1 with parameter "target=ts"
// @generated from file sited_io/commerce/v1/shop.proto (package sited_io.commerce.v1, syntax proto3)
/* eslint-disable */
// @ts-nocheck

import { CreateShopRequest, CreateShopResponse, DeleteShopRequest, DeleteShopResponse, GetShopRequest, GetShopResponse, ListShopsRequest, ListShopsResponse, UpdateShopRequest, UpdateShopResponse } from "./shop_pb.js";
import { MethodKind } from "@bufbuild/protobuf";

/**
 * @generated from service sited_io.commerce.v1.ShopService
 */
export const ShopService = {
  typeName: "sited_io.commerce.v1.ShopService",
  methods: {
    /**
     * @generated from rpc sited_io.commerce.v1.ShopService.CreateShop
     */
    createShop: {
      name: "CreateShop",
      I: CreateShopRequest,
      O: CreateShopResponse,
      kind: MethodKind.Unary,
    },
    /**
     * @generated from rpc sited_io.commerce.v1.ShopService.GetShop
     */
    getShop: {
      name: "GetShop",
      I: GetShopRequest,
      O: GetShopResponse,
      kind: MethodKind.Unary,
    },
    /**
     * @generated from rpc sited_io.commerce.v1.ShopService.ListShops
     */
    listShops: {
      name: "ListShops",
      I: ListShopsRequest,
      O: ListShopsResponse,
      kind: MethodKind.Unary,
    },
    /**
     * @generated from rpc sited_io.commerce.v1.ShopService.UpdateShop
     */
    updateShop: {
      name: "UpdateShop",
      I: UpdateShopRequest,
      O: UpdateShopResponse,
      kind: MethodKind.Unary,
    },
    /**
     * @generated from rpc sited_io.commerce.v1.ShopService.DeleteShop
     */
    deleteShop: {
      name: "DeleteShop",
      I: DeleteShopRequest,
      O: DeleteShopResponse,
      kind: MethodKind.Unary,
    },
  }
} as const;


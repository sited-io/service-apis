// @generated by protoc-gen-es v2.2.2 with parameter "target=ts"
// @generated from file sited_io/types/country/v1/country.proto (package sited_io.types.country.v1, syntax proto3)
/* eslint-disable */

import type { GenEnum, GenFile } from "@bufbuild/protobuf/codegenv1";
import { enumDesc, fileDesc } from "@bufbuild/protobuf/codegenv1";

/**
 * Describes the file sited_io/types/country/v1/country.proto.
 */
export const file_sited_io_types_country_v1_country: GenFile = /*@__PURE__*/
  fileDesc("CidzaXRlZF9pby90eXBlcy9jb3VudHJ5L3YxL2NvdW50cnkucHJvdG8SGXNpdGVkX2lvLnR5cGVzLmNvdW50cnkudjEq9CcKC0NvdW50cnlDb2RlEhwKGENPVU5UUllfQ09ERV9VTlNQRUNJRklFRBAAEhMKD0NPVU5UUllfQ09ERV9BQxABEhMKD0NPVU5UUllfQ09ERV9BRBACEhMKD0NPVU5UUllfQ09ERV9BRRADEhMKD0NPVU5UUllfQ09ERV9BRhAEEhMKD0NPVU5UUllfQ09ERV9BRxAFEhMKD0NPVU5UUllfQ09ERV9BSRAGEhMKD0NPVU5UUllfQ09ERV9BTBAHEhMKD0NPVU5UUllfQ09ERV9BTRAIEhMKD0NPVU5UUllfQ09ERV9BTxAJEhMKD0NPVU5UUllfQ09ERV9BURAKEhMKD0NPVU5UUllfQ09ERV9BUhALEhMKD0NPVU5UUllfQ09ERV9BVBAMEhMKD0NPVU5UUllfQ09ERV9BVRANEhMKD0NPVU5UUllfQ09ERV9BVxAOEhMKD0NPVU5UUllfQ09ERV9BWBAPEhMKD0NPVU5UUllfQ09ERV9BWhAQEhMKD0NPVU5UUllfQ09ERV9CQRAREhMKD0NPVU5UUllfQ09ERV9CQhASEhMKD0NPVU5UUllfQ09ERV9CRBATEhMKD0NPVU5UUllfQ09ERV9CRRAUEhMKD0NPVU5UUllfQ09ERV9CRhAVEhMKD0NPVU5UUllfQ09ERV9CRxAWEhMKD0NPVU5UUllfQ09ERV9CSBAXEhMKD0NPVU5UUllfQ09ERV9CSRAYEhMKD0NPVU5UUllfQ09ERV9CShAZEhMKD0NPVU5UUllfQ09ERV9CTBAaEhMKD0NPVU5UUllfQ09ERV9CTRAbEhMKD0NPVU5UUllfQ09ERV9CThAcEhMKD0NPVU5UUllfQ09ERV9CTxAdEhMKD0NPVU5UUllfQ09ERV9CURAeEhMKD0NPVU5UUllfQ09ERV9CUhAfEhMKD0NPVU5UUllfQ09ERV9CUxAgEhMKD0NPVU5UUllfQ09ERV9CVBAhEhMKD0NPVU5UUllfQ09ERV9CVhAiEhMKD0NPVU5UUllfQ09ERV9CVxAjEhMKD0NPVU5UUllfQ09ERV9CWRAkEhMKD0NPVU5UUllfQ09ERV9CWhAlEhMKD0NPVU5UUllfQ09ERV9DQRAmEhMKD0NPVU5UUllfQ09ERV9DRBAnEhMKD0NPVU5UUllfQ09ERV9DRhAoEhMKD0NPVU5UUllfQ09ERV9DRxApEhMKD0NPVU5UUllfQ09ERV9DSBAqEhMKD0NPVU5UUllfQ09ERV9DSRArEhMKD0NPVU5UUllfQ09ERV9DSxAsEhMKD0NPVU5UUllfQ09ERV9DTBAtEhMKD0NPVU5UUllfQ09ERV9DTRAuEhMKD0NPVU5UUllfQ09ERV9DThAvEhMKD0NPVU5UUllfQ09ERV9DTxAwEhMKD0NPVU5UUllfQ09ERV9DUhAxEhMKD0NPVU5UUllfQ09ERV9DVhAyEhMKD0NPVU5UUllfQ09ERV9DVxAzEhMKD0NPVU5UUllfQ09ERV9DWRA0EhMKD0NPVU5UUllfQ09ERV9DWhA1EhMKD0NPVU5UUllfQ09ERV9ERRA2EhMKD0NPVU5UUllfQ09ERV9EShA3EhMKD0NPVU5UUllfQ09ERV9ESxA4EhMKD0NPVU5UUllfQ09ERV9ETRA5EhMKD0NPVU5UUllfQ09ERV9ETxA6EhMKD0NPVU5UUllfQ09ERV9EWhA7EhMKD0NPVU5UUllfQ09ERV9FQxA8EhMKD0NPVU5UUllfQ09ERV9FRRA9EhMKD0NPVU5UUllfQ09ERV9FRxA+EhMKD0NPVU5UUllfQ09ERV9FSBA/EhMKD0NPVU5UUllfQ09ERV9FUhBAEhMKD0NPVU5UUllfQ09ERV9FUxBBEhMKD0NPVU5UUllfQ09ERV9FVBBCEhMKD0NPVU5UUllfQ09ERV9GSRBDEhMKD0NPVU5UUllfQ09ERV9GShBEEhMKD0NPVU5UUllfQ09ERV9GSxBFEhMKD0NPVU5UUllfQ09ERV9GTxBGEhMKD0NPVU5UUllfQ09ERV9GUhBHEhMKD0NPVU5UUllfQ09ERV9HQRBIEhMKD0NPVU5UUllfQ09ERV9HQhBJEhMKD0NPVU5UUllfQ09ERV9HRBBKEhMKD0NPVU5UUllfQ09ERV9HRRBLEhMKD0NPVU5UUllfQ09ERV9HRhBMEhMKD0NPVU5UUllfQ09ERV9HRxBNEhMKD0NPVU5UUllfQ09ERV9HSBBOEhMKD0NPVU5UUllfQ09ERV9HSRBPEhMKD0NPVU5UUllfQ09ERV9HTBBQEhMKD0NPVU5UUllfQ09ERV9HTRBREhMKD0NPVU5UUllfQ09ERV9HThBSEhMKD0NPVU5UUllfQ09ERV9HUBBTEhMKD0NPVU5UUllfQ09ERV9HURBUEhMKD0NPVU5UUllfQ09ERV9HUhBVEhMKD0NPVU5UUllfQ09ERV9HUxBWEhMKD0NPVU5UUllfQ09ERV9HVBBXEhMKD0NPVU5UUllfQ09ERV9HVRBYEhMKD0NPVU5UUllfQ09ERV9HVxBZEhMKD0NPVU5UUllfQ09ERV9HWRBaEhMKD0NPVU5UUllfQ09ERV9ISxBbEhMKD0NPVU5UUllfQ09ERV9IThBcEhMKD0NPVU5UUllfQ09ERV9IUhBdEhMKD0NPVU5UUllfQ09ERV9IVBBeEhMKD0NPVU5UUllfQ09ERV9IVRBfEhMKD0NPVU5UUllfQ09ERV9JRBBgEhMKD0NPVU5UUllfQ09ERV9JRRBhEhMKD0NPVU5UUllfQ09ERV9JTBBiEhMKD0NPVU5UUllfQ09ERV9JTRBjEhMKD0NPVU5UUllfQ09ERV9JThBkEhMKD0NPVU5UUllfQ09ERV9JTxBlEhMKD0NPVU5UUllfQ09ERV9JURBmEhMKD0NPVU5UUllfQ09ERV9JUxBnEhMKD0NPVU5UUllfQ09ERV9JVBBoEhMKD0NPVU5UUllfQ09ERV9KRRBpEhMKD0NPVU5UUllfQ09ERV9KTRBqEhMKD0NPVU5UUllfQ09ERV9KTxBrEhMKD0NPVU5UUllfQ09ERV9KUBBsEhMKD0NPVU5UUllfQ09ERV9LRRBtEhMKD0NPVU5UUllfQ09ERV9LRxBuEhMKD0NPVU5UUllfQ09ERV9LSBBvEhMKD0NPVU5UUllfQ09ERV9LSRBwEhMKD0NPVU5UUllfQ09ERV9LTRBxEhMKD0NPVU5UUllfQ09ERV9LThByEhMKD0NPVU5UUllfQ09ERV9LUhBzEhMKD0NPVU5UUllfQ09ERV9LVxB0EhMKD0NPVU5UUllfQ09ERV9LWRB1EhMKD0NPVU5UUllfQ09ERV9MQRB2EhMKD0NPVU5UUllfQ09ERV9MQhB3EhMKD0NPVU5UUllfQ09ERV9MQxB4EhMKD0NPVU5UUllfQ09ERV9MSRB5EhMKD0NPVU5UUllfQ09ERV9MSxB6EhMKD0NPVU5UUllfQ09ERV9MUhB7EhMKD0NPVU5UUllfQ09ERV9MUxB8EhMKD0NPVU5UUllfQ09ERV9MVBB9EhMKD0NPVU5UUllfQ09ERV9MVRB+EhMKD0NPVU5UUllfQ09ERV9MVhB/EhQKD0NPVU5UUllfQ09ERV9MWRCAARIUCg9DT1VOVFJZX0NPREVfTUEQgQESFAoPQ09VTlRSWV9DT0RFX01DEIIBEhQKD0NPVU5UUllfQ09ERV9NRBCDARIUCg9DT1VOVFJZX0NPREVfTUUQhAESFAoPQ09VTlRSWV9DT0RFX01GEIUBEhQKD0NPVU5UUllfQ09ERV9NRxCGARIUCg9DT1VOVFJZX0NPREVfTUsQhwESFAoPQ09VTlRSWV9DT0RFX01MEIgBEhQKD0NPVU5UUllfQ09ERV9NTRCJARIUCg9DT1VOVFJZX0NPREVfTU4QigESFAoPQ09VTlRSWV9DT0RFX01PEIsBEhQKD0NPVU5UUllfQ09ERV9NURCMARIUCg9DT1VOVFJZX0NPREVfTVIQjQESFAoPQ09VTlRSWV9DT0RFX01TEI4BEhQKD0NPVU5UUllfQ09ERV9NVBCPARIUCg9DT1VOVFJZX0NPREVfTVUQkAESFAoPQ09VTlRSWV9DT0RFX01WEJEBEhQKD0NPVU5UUllfQ09ERV9NVxCSARIUCg9DT1VOVFJZX0NPREVfTVgQkwESFAoPQ09VTlRSWV9DT0RFX01ZEJQBEhQKD0NPVU5UUllfQ09ERV9NWhCVARIUCg9DT1VOVFJZX0NPREVfTkEQlgESFAoPQ09VTlRSWV9DT0RFX05DEJcBEhQKD0NPVU5UUllfQ09ERV9ORRCYARIUCg9DT1VOVFJZX0NPREVfTkcQmQESFAoPQ09VTlRSWV9DT0RFX05JEJoBEhQKD0NPVU5UUllfQ09ERV9OTBCbARIUCg9DT1VOVFJZX0NPREVfTk8QnAESFAoPQ09VTlRSWV9DT0RFX05QEJ0BEhQKD0NPVU5UUllfQ09ERV9OUhCeARIUCg9DT1VOVFJZX0NPREVfTlUQnwESFAoPQ09VTlRSWV9DT0RFX05aEKABEhQKD0NPVU5UUllfQ09ERV9PTRChARIUCg9DT1VOVFJZX0NPREVfUEEQogESFAoPQ09VTlRSWV9DT0RFX1BFEKMBEhQKD0NPVU5UUllfQ09ERV9QRhCkARIUCg9DT1VOVFJZX0NPREVfUEcQpQESFAoPQ09VTlRSWV9DT0RFX1BIEKYBEhQKD0NPVU5UUllfQ09ERV9QSxCnARIUCg9DT1VOVFJZX0NPREVfUEwQqAESFAoPQ09VTlRSWV9DT0RFX1BNEKkBEhQKD0NPVU5UUllfQ09ERV9QThCqARIUCg9DT1VOVFJZX0NPREVfUFIQqwESFAoPQ09VTlRSWV9DT0RFX1BTEKwBEhQKD0NPVU5UUllfQ09ERV9QVBCtARIUCg9DT1VOVFJZX0NPREVfUFkQrgESFAoPQ09VTlRSWV9DT0RFX1FBEK8BEhQKD0NPVU5UUllfQ09ERV9SRRCwARIUCg9DT1VOVFJZX0NPREVfUk8QsQESFAoPQ09VTlRSWV9DT0RFX1JTELIBEhQKD0NPVU5UUllfQ09ERV9SVRCzARIUCg9DT1VOVFJZX0NPREVfUlcQtAESFAoPQ09VTlRSWV9DT0RFX1NBELUBEhQKD0NPVU5UUllfQ09ERV9TQhC2ARIUCg9DT1VOVFJZX0NPREVfU0MQtwESFAoPQ09VTlRSWV9DT0RFX1NFELgBEhQKD0NPVU5UUllfQ09ERV9TRxC5ARIUCg9DT1VOVFJZX0NPREVfU0gQugESFAoPQ09VTlRSWV9DT0RFX1NJELsBEhQKD0NPVU5UUllfQ09ERV9TShC8ARIUCg9DT1VOVFJZX0NPREVfU0sQvQESFAoPQ09VTlRSWV9DT0RFX1NMEL4BEhQKD0NPVU5UUllfQ09ERV9TTRC/ARIUCg9DT1VOVFJZX0NPREVfU04QwAESFAoPQ09VTlRSWV9DT0RFX1NPEMEBEhQKD0NPVU5UUllfQ09ERV9TUhDCARIUCg9DT1VOVFJZX0NPREVfU1MQwwESFAoPQ09VTlRSWV9DT0RFX1NUEMQBEhQKD0NPVU5UUllfQ09ERV9TVhDFARIUCg9DT1VOVFJZX0NPREVfU1gQxgESFAoPQ09VTlRSWV9DT0RFX1NaEMcBEhQKD0NPVU5UUllfQ09ERV9UQRDIARIUCg9DT1VOVFJZX0NPREVfVEMQyQESFAoPQ09VTlRSWV9DT0RFX1REEMoBEhQKD0NPVU5UUllfQ09ERV9URhDLARIUCg9DT1VOVFJZX0NPREVfVEcQzAESFAoPQ09VTlRSWV9DT0RFX1RIEM0BEhQKD0NPVU5UUllfQ09ERV9UShDOARIUCg9DT1VOVFJZX0NPREVfVEsQzwESFAoPQ09VTlRSWV9DT0RFX1RMENABEhQKD0NPVU5UUllfQ09ERV9UTRDRARIUCg9DT1VOVFJZX0NPREVfVE4Q0gESFAoPQ09VTlRSWV9DT0RFX1RPENMBEhQKD0NPVU5UUllfQ09ERV9UUhDUARIUCg9DT1VOVFJZX0NPREVfVFQQ1QESFAoPQ09VTlRSWV9DT0RFX1RWENYBEhQKD0NPVU5UUllfQ09ERV9UVxDXARIUCg9DT1VOVFJZX0NPREVfVFoQ2AESFAoPQ09VTlRSWV9DT0RFX1VBENkBEhQKD0NPVU5UUllfQ09ERV9VRxDaARIUCg9DT1VOVFJZX0NPREVfVVMQ2wESFAoPQ09VTlRSWV9DT0RFX1VZENwBEhQKD0NPVU5UUllfQ09ERV9VWhDdARIUCg9DT1VOVFJZX0NPREVfVkEQ3gESFAoPQ09VTlRSWV9DT0RFX1ZDEN8BEhQKD0NPVU5UUllfQ09ERV9WRRDgARIUCg9DT1VOVFJZX0NPREVfVkcQ4QESFAoPQ09VTlRSWV9DT0RFX1ZOEOIBEhQKD0NPVU5UUllfQ09ERV9WVRDjARIUCg9DT1VOVFJZX0NPREVfV0YQ5AESFAoPQ09VTlRSWV9DT0RFX1dTEOUBEhQKD0NPVU5UUllfQ09ERV9YSxDmARIUCg9DT1VOVFJZX0NPREVfWUUQ5wESFAoPQ09VTlRSWV9DT0RFX1lUEOgBEhQKD0NPVU5UUllfQ09ERV9aQRDpARIUCg9DT1VOVFJZX0NPREVfWk0Q6gESFAoPQ09VTlRSWV9DT0RFX1pXEOsBEhQKD0NPVU5UUllfQ09ERV9aWhDsAWIGcHJvdG8z");

/**
 * @generated from enum sited_io.types.country.v1.CountryCode
 */
export enum CountryCode {
  /**
   * @generated from enum value: COUNTRY_CODE_UNSPECIFIED = 0;
   */
  UNSPECIFIED = 0,

  /**
   * @generated from enum value: COUNTRY_CODE_AC = 1;
   */
  AC = 1,

  /**
   * @generated from enum value: COUNTRY_CODE_AD = 2;
   */
  AD = 2,

  /**
   * @generated from enum value: COUNTRY_CODE_AE = 3;
   */
  AE = 3,

  /**
   * @generated from enum value: COUNTRY_CODE_AF = 4;
   */
  AF = 4,

  /**
   * @generated from enum value: COUNTRY_CODE_AG = 5;
   */
  AG = 5,

  /**
   * @generated from enum value: COUNTRY_CODE_AI = 6;
   */
  AI = 6,

  /**
   * @generated from enum value: COUNTRY_CODE_AL = 7;
   */
  AL = 7,

  /**
   * @generated from enum value: COUNTRY_CODE_AM = 8;
   */
  AM = 8,

  /**
   * @generated from enum value: COUNTRY_CODE_AO = 9;
   */
  AO = 9,

  /**
   * @generated from enum value: COUNTRY_CODE_AQ = 10;
   */
  AQ = 10,

  /**
   * @generated from enum value: COUNTRY_CODE_AR = 11;
   */
  AR = 11,

  /**
   * @generated from enum value: COUNTRY_CODE_AT = 12;
   */
  AT = 12,

  /**
   * @generated from enum value: COUNTRY_CODE_AU = 13;
   */
  AU = 13,

  /**
   * @generated from enum value: COUNTRY_CODE_AW = 14;
   */
  AW = 14,

  /**
   * @generated from enum value: COUNTRY_CODE_AX = 15;
   */
  AX = 15,

  /**
   * @generated from enum value: COUNTRY_CODE_AZ = 16;
   */
  AZ = 16,

  /**
   * @generated from enum value: COUNTRY_CODE_BA = 17;
   */
  BA = 17,

  /**
   * @generated from enum value: COUNTRY_CODE_BB = 18;
   */
  BB = 18,

  /**
   * @generated from enum value: COUNTRY_CODE_BD = 19;
   */
  BD = 19,

  /**
   * @generated from enum value: COUNTRY_CODE_BE = 20;
   */
  BE = 20,

  /**
   * @generated from enum value: COUNTRY_CODE_BF = 21;
   */
  BF = 21,

  /**
   * @generated from enum value: COUNTRY_CODE_BG = 22;
   */
  BG = 22,

  /**
   * @generated from enum value: COUNTRY_CODE_BH = 23;
   */
  BH = 23,

  /**
   * @generated from enum value: COUNTRY_CODE_BI = 24;
   */
  BI = 24,

  /**
   * @generated from enum value: COUNTRY_CODE_BJ = 25;
   */
  BJ = 25,

  /**
   * @generated from enum value: COUNTRY_CODE_BL = 26;
   */
  BL = 26,

  /**
   * @generated from enum value: COUNTRY_CODE_BM = 27;
   */
  BM = 27,

  /**
   * @generated from enum value: COUNTRY_CODE_BN = 28;
   */
  BN = 28,

  /**
   * @generated from enum value: COUNTRY_CODE_BO = 29;
   */
  BO = 29,

  /**
   * @generated from enum value: COUNTRY_CODE_BQ = 30;
   */
  BQ = 30,

  /**
   * @generated from enum value: COUNTRY_CODE_BR = 31;
   */
  BR = 31,

  /**
   * @generated from enum value: COUNTRY_CODE_BS = 32;
   */
  BS = 32,

  /**
   * @generated from enum value: COUNTRY_CODE_BT = 33;
   */
  BT = 33,

  /**
   * @generated from enum value: COUNTRY_CODE_BV = 34;
   */
  BV = 34,

  /**
   * @generated from enum value: COUNTRY_CODE_BW = 35;
   */
  BW = 35,

  /**
   * @generated from enum value: COUNTRY_CODE_BY = 36;
   */
  BY = 36,

  /**
   * @generated from enum value: COUNTRY_CODE_BZ = 37;
   */
  BZ = 37,

  /**
   * @generated from enum value: COUNTRY_CODE_CA = 38;
   */
  CA = 38,

  /**
   * @generated from enum value: COUNTRY_CODE_CD = 39;
   */
  CD = 39,

  /**
   * @generated from enum value: COUNTRY_CODE_CF = 40;
   */
  CF = 40,

  /**
   * @generated from enum value: COUNTRY_CODE_CG = 41;
   */
  CG = 41,

  /**
   * @generated from enum value: COUNTRY_CODE_CH = 42;
   */
  CH = 42,

  /**
   * @generated from enum value: COUNTRY_CODE_CI = 43;
   */
  CI = 43,

  /**
   * @generated from enum value: COUNTRY_CODE_CK = 44;
   */
  CK = 44,

  /**
   * @generated from enum value: COUNTRY_CODE_CL = 45;
   */
  CL = 45,

  /**
   * @generated from enum value: COUNTRY_CODE_CM = 46;
   */
  CM = 46,

  /**
   * @generated from enum value: COUNTRY_CODE_CN = 47;
   */
  CN = 47,

  /**
   * @generated from enum value: COUNTRY_CODE_CO = 48;
   */
  CO = 48,

  /**
   * @generated from enum value: COUNTRY_CODE_CR = 49;
   */
  CR = 49,

  /**
   * @generated from enum value: COUNTRY_CODE_CV = 50;
   */
  CV = 50,

  /**
   * @generated from enum value: COUNTRY_CODE_CW = 51;
   */
  CW = 51,

  /**
   * @generated from enum value: COUNTRY_CODE_CY = 52;
   */
  CY = 52,

  /**
   * @generated from enum value: COUNTRY_CODE_CZ = 53;
   */
  CZ = 53,

  /**
   * @generated from enum value: COUNTRY_CODE_DE = 54;
   */
  DE = 54,

  /**
   * @generated from enum value: COUNTRY_CODE_DJ = 55;
   */
  DJ = 55,

  /**
   * @generated from enum value: COUNTRY_CODE_DK = 56;
   */
  DK = 56,

  /**
   * @generated from enum value: COUNTRY_CODE_DM = 57;
   */
  DM = 57,

  /**
   * @generated from enum value: COUNTRY_CODE_DO = 58;
   */
  DO = 58,

  /**
   * @generated from enum value: COUNTRY_CODE_DZ = 59;
   */
  DZ = 59,

  /**
   * @generated from enum value: COUNTRY_CODE_EC = 60;
   */
  EC = 60,

  /**
   * @generated from enum value: COUNTRY_CODE_EE = 61;
   */
  EE = 61,

  /**
   * @generated from enum value: COUNTRY_CODE_EG = 62;
   */
  EG = 62,

  /**
   * @generated from enum value: COUNTRY_CODE_EH = 63;
   */
  EH = 63,

  /**
   * @generated from enum value: COUNTRY_CODE_ER = 64;
   */
  ER = 64,

  /**
   * @generated from enum value: COUNTRY_CODE_ES = 65;
   */
  ES = 65,

  /**
   * @generated from enum value: COUNTRY_CODE_ET = 66;
   */
  ET = 66,

  /**
   * @generated from enum value: COUNTRY_CODE_FI = 67;
   */
  FI = 67,

  /**
   * @generated from enum value: COUNTRY_CODE_FJ = 68;
   */
  FJ = 68,

  /**
   * @generated from enum value: COUNTRY_CODE_FK = 69;
   */
  FK = 69,

  /**
   * @generated from enum value: COUNTRY_CODE_FO = 70;
   */
  FO = 70,

  /**
   * @generated from enum value: COUNTRY_CODE_FR = 71;
   */
  FR = 71,

  /**
   * @generated from enum value: COUNTRY_CODE_GA = 72;
   */
  GA = 72,

  /**
   * @generated from enum value: COUNTRY_CODE_GB = 73;
   */
  GB = 73,

  /**
   * @generated from enum value: COUNTRY_CODE_GD = 74;
   */
  GD = 74,

  /**
   * @generated from enum value: COUNTRY_CODE_GE = 75;
   */
  GE = 75,

  /**
   * @generated from enum value: COUNTRY_CODE_GF = 76;
   */
  GF = 76,

  /**
   * @generated from enum value: COUNTRY_CODE_GG = 77;
   */
  GG = 77,

  /**
   * @generated from enum value: COUNTRY_CODE_GH = 78;
   */
  GH = 78,

  /**
   * @generated from enum value: COUNTRY_CODE_GI = 79;
   */
  GI = 79,

  /**
   * @generated from enum value: COUNTRY_CODE_GL = 80;
   */
  GL = 80,

  /**
   * @generated from enum value: COUNTRY_CODE_GM = 81;
   */
  GM = 81,

  /**
   * @generated from enum value: COUNTRY_CODE_GN = 82;
   */
  GN = 82,

  /**
   * @generated from enum value: COUNTRY_CODE_GP = 83;
   */
  GP = 83,

  /**
   * @generated from enum value: COUNTRY_CODE_GQ = 84;
   */
  GQ = 84,

  /**
   * @generated from enum value: COUNTRY_CODE_GR = 85;
   */
  GR = 85,

  /**
   * @generated from enum value: COUNTRY_CODE_GS = 86;
   */
  GS = 86,

  /**
   * @generated from enum value: COUNTRY_CODE_GT = 87;
   */
  GT = 87,

  /**
   * @generated from enum value: COUNTRY_CODE_GU = 88;
   */
  GU = 88,

  /**
   * @generated from enum value: COUNTRY_CODE_GW = 89;
   */
  GW = 89,

  /**
   * @generated from enum value: COUNTRY_CODE_GY = 90;
   */
  GY = 90,

  /**
   * @generated from enum value: COUNTRY_CODE_HK = 91;
   */
  HK = 91,

  /**
   * @generated from enum value: COUNTRY_CODE_HN = 92;
   */
  HN = 92,

  /**
   * @generated from enum value: COUNTRY_CODE_HR = 93;
   */
  HR = 93,

  /**
   * @generated from enum value: COUNTRY_CODE_HT = 94;
   */
  HT = 94,

  /**
   * @generated from enum value: COUNTRY_CODE_HU = 95;
   */
  HU = 95,

  /**
   * @generated from enum value: COUNTRY_CODE_ID = 96;
   */
  ID = 96,

  /**
   * @generated from enum value: COUNTRY_CODE_IE = 97;
   */
  IE = 97,

  /**
   * @generated from enum value: COUNTRY_CODE_IL = 98;
   */
  IL = 98,

  /**
   * @generated from enum value: COUNTRY_CODE_IM = 99;
   */
  IM = 99,

  /**
   * @generated from enum value: COUNTRY_CODE_IN = 100;
   */
  IN = 100,

  /**
   * @generated from enum value: COUNTRY_CODE_IO = 101;
   */
  IO = 101,

  /**
   * @generated from enum value: COUNTRY_CODE_IQ = 102;
   */
  IQ = 102,

  /**
   * @generated from enum value: COUNTRY_CODE_IS = 103;
   */
  IS = 103,

  /**
   * @generated from enum value: COUNTRY_CODE_IT = 104;
   */
  IT = 104,

  /**
   * @generated from enum value: COUNTRY_CODE_JE = 105;
   */
  JE = 105,

  /**
   * @generated from enum value: COUNTRY_CODE_JM = 106;
   */
  JM = 106,

  /**
   * @generated from enum value: COUNTRY_CODE_JO = 107;
   */
  JO = 107,

  /**
   * @generated from enum value: COUNTRY_CODE_JP = 108;
   */
  JP = 108,

  /**
   * @generated from enum value: COUNTRY_CODE_KE = 109;
   */
  KE = 109,

  /**
   * @generated from enum value: COUNTRY_CODE_KG = 110;
   */
  KG = 110,

  /**
   * @generated from enum value: COUNTRY_CODE_KH = 111;
   */
  KH = 111,

  /**
   * @generated from enum value: COUNTRY_CODE_KI = 112;
   */
  KI = 112,

  /**
   * @generated from enum value: COUNTRY_CODE_KM = 113;
   */
  KM = 113,

  /**
   * @generated from enum value: COUNTRY_CODE_KN = 114;
   */
  KN = 114,

  /**
   * @generated from enum value: COUNTRY_CODE_KR = 115;
   */
  KR = 115,

  /**
   * @generated from enum value: COUNTRY_CODE_KW = 116;
   */
  KW = 116,

  /**
   * @generated from enum value: COUNTRY_CODE_KY = 117;
   */
  KY = 117,

  /**
   * @generated from enum value: COUNTRY_CODE_LA = 118;
   */
  LA = 118,

  /**
   * @generated from enum value: COUNTRY_CODE_LB = 119;
   */
  LB = 119,

  /**
   * @generated from enum value: COUNTRY_CODE_LC = 120;
   */
  LC = 120,

  /**
   * @generated from enum value: COUNTRY_CODE_LI = 121;
   */
  LI = 121,

  /**
   * @generated from enum value: COUNTRY_CODE_LK = 122;
   */
  LK = 122,

  /**
   * @generated from enum value: COUNTRY_CODE_LR = 123;
   */
  LR = 123,

  /**
   * @generated from enum value: COUNTRY_CODE_LS = 124;
   */
  LS = 124,

  /**
   * @generated from enum value: COUNTRY_CODE_LT = 125;
   */
  LT = 125,

  /**
   * @generated from enum value: COUNTRY_CODE_LU = 126;
   */
  LU = 126,

  /**
   * @generated from enum value: COUNTRY_CODE_LV = 127;
   */
  LV = 127,

  /**
   * @generated from enum value: COUNTRY_CODE_LY = 128;
   */
  LY = 128,

  /**
   * @generated from enum value: COUNTRY_CODE_MA = 129;
   */
  MA = 129,

  /**
   * @generated from enum value: COUNTRY_CODE_MC = 130;
   */
  MC = 130,

  /**
   * @generated from enum value: COUNTRY_CODE_MD = 131;
   */
  MD = 131,

  /**
   * @generated from enum value: COUNTRY_CODE_ME = 132;
   */
  ME = 132,

  /**
   * @generated from enum value: COUNTRY_CODE_MF = 133;
   */
  MF = 133,

  /**
   * @generated from enum value: COUNTRY_CODE_MG = 134;
   */
  MG = 134,

  /**
   * @generated from enum value: COUNTRY_CODE_MK = 135;
   */
  MK = 135,

  /**
   * @generated from enum value: COUNTRY_CODE_ML = 136;
   */
  ML = 136,

  /**
   * @generated from enum value: COUNTRY_CODE_MM = 137;
   */
  MM = 137,

  /**
   * @generated from enum value: COUNTRY_CODE_MN = 138;
   */
  MN = 138,

  /**
   * @generated from enum value: COUNTRY_CODE_MO = 139;
   */
  MO = 139,

  /**
   * @generated from enum value: COUNTRY_CODE_MQ = 140;
   */
  MQ = 140,

  /**
   * @generated from enum value: COUNTRY_CODE_MR = 141;
   */
  MR = 141,

  /**
   * @generated from enum value: COUNTRY_CODE_MS = 142;
   */
  MS = 142,

  /**
   * @generated from enum value: COUNTRY_CODE_MT = 143;
   */
  MT = 143,

  /**
   * @generated from enum value: COUNTRY_CODE_MU = 144;
   */
  MU = 144,

  /**
   * @generated from enum value: COUNTRY_CODE_MV = 145;
   */
  MV = 145,

  /**
   * @generated from enum value: COUNTRY_CODE_MW = 146;
   */
  MW = 146,

  /**
   * @generated from enum value: COUNTRY_CODE_MX = 147;
   */
  MX = 147,

  /**
   * @generated from enum value: COUNTRY_CODE_MY = 148;
   */
  MY = 148,

  /**
   * @generated from enum value: COUNTRY_CODE_MZ = 149;
   */
  MZ = 149,

  /**
   * @generated from enum value: COUNTRY_CODE_NA = 150;
   */
  NA = 150,

  /**
   * @generated from enum value: COUNTRY_CODE_NC = 151;
   */
  NC = 151,

  /**
   * @generated from enum value: COUNTRY_CODE_NE = 152;
   */
  NE = 152,

  /**
   * @generated from enum value: COUNTRY_CODE_NG = 153;
   */
  NG = 153,

  /**
   * @generated from enum value: COUNTRY_CODE_NI = 154;
   */
  NI = 154,

  /**
   * @generated from enum value: COUNTRY_CODE_NL = 155;
   */
  NL = 155,

  /**
   * @generated from enum value: COUNTRY_CODE_NO = 156;
   */
  NO = 156,

  /**
   * @generated from enum value: COUNTRY_CODE_NP = 157;
   */
  NP = 157,

  /**
   * @generated from enum value: COUNTRY_CODE_NR = 158;
   */
  NR = 158,

  /**
   * @generated from enum value: COUNTRY_CODE_NU = 159;
   */
  NU = 159,

  /**
   * @generated from enum value: COUNTRY_CODE_NZ = 160;
   */
  NZ = 160,

  /**
   * @generated from enum value: COUNTRY_CODE_OM = 161;
   */
  OM = 161,

  /**
   * @generated from enum value: COUNTRY_CODE_PA = 162;
   */
  PA = 162,

  /**
   * @generated from enum value: COUNTRY_CODE_PE = 163;
   */
  PE = 163,

  /**
   * @generated from enum value: COUNTRY_CODE_PF = 164;
   */
  PF = 164,

  /**
   * @generated from enum value: COUNTRY_CODE_PG = 165;
   */
  PG = 165,

  /**
   * @generated from enum value: COUNTRY_CODE_PH = 166;
   */
  PH = 166,

  /**
   * @generated from enum value: COUNTRY_CODE_PK = 167;
   */
  PK = 167,

  /**
   * @generated from enum value: COUNTRY_CODE_PL = 168;
   */
  PL = 168,

  /**
   * @generated from enum value: COUNTRY_CODE_PM = 169;
   */
  PM = 169,

  /**
   * @generated from enum value: COUNTRY_CODE_PN = 170;
   */
  PN = 170,

  /**
   * @generated from enum value: COUNTRY_CODE_PR = 171;
   */
  PR = 171,

  /**
   * @generated from enum value: COUNTRY_CODE_PS = 172;
   */
  PS = 172,

  /**
   * @generated from enum value: COUNTRY_CODE_PT = 173;
   */
  PT = 173,

  /**
   * @generated from enum value: COUNTRY_CODE_PY = 174;
   */
  PY = 174,

  /**
   * @generated from enum value: COUNTRY_CODE_QA = 175;
   */
  QA = 175,

  /**
   * @generated from enum value: COUNTRY_CODE_RE = 176;
   */
  RE = 176,

  /**
   * @generated from enum value: COUNTRY_CODE_RO = 177;
   */
  RO = 177,

  /**
   * @generated from enum value: COUNTRY_CODE_RS = 178;
   */
  RS = 178,

  /**
   * @generated from enum value: COUNTRY_CODE_RU = 179;
   */
  RU = 179,

  /**
   * @generated from enum value: COUNTRY_CODE_RW = 180;
   */
  RW = 180,

  /**
   * @generated from enum value: COUNTRY_CODE_SA = 181;
   */
  SA = 181,

  /**
   * @generated from enum value: COUNTRY_CODE_SB = 182;
   */
  SB = 182,

  /**
   * @generated from enum value: COUNTRY_CODE_SC = 183;
   */
  SC = 183,

  /**
   * @generated from enum value: COUNTRY_CODE_SE = 184;
   */
  SE = 184,

  /**
   * @generated from enum value: COUNTRY_CODE_SG = 185;
   */
  SG = 185,

  /**
   * @generated from enum value: COUNTRY_CODE_SH = 186;
   */
  SH = 186,

  /**
   * @generated from enum value: COUNTRY_CODE_SI = 187;
   */
  SI = 187,

  /**
   * @generated from enum value: COUNTRY_CODE_SJ = 188;
   */
  SJ = 188,

  /**
   * @generated from enum value: COUNTRY_CODE_SK = 189;
   */
  SK = 189,

  /**
   * @generated from enum value: COUNTRY_CODE_SL = 190;
   */
  SL = 190,

  /**
   * @generated from enum value: COUNTRY_CODE_SM = 191;
   */
  SM = 191,

  /**
   * @generated from enum value: COUNTRY_CODE_SN = 192;
   */
  SN = 192,

  /**
   * @generated from enum value: COUNTRY_CODE_SO = 193;
   */
  SO = 193,

  /**
   * @generated from enum value: COUNTRY_CODE_SR = 194;
   */
  SR = 194,

  /**
   * @generated from enum value: COUNTRY_CODE_SS = 195;
   */
  SS = 195,

  /**
   * @generated from enum value: COUNTRY_CODE_ST = 196;
   */
  ST = 196,

  /**
   * @generated from enum value: COUNTRY_CODE_SV = 197;
   */
  SV = 197,

  /**
   * @generated from enum value: COUNTRY_CODE_SX = 198;
   */
  SX = 198,

  /**
   * @generated from enum value: COUNTRY_CODE_SZ = 199;
   */
  SZ = 199,

  /**
   * @generated from enum value: COUNTRY_CODE_TA = 200;
   */
  TA = 200,

  /**
   * @generated from enum value: COUNTRY_CODE_TC = 201;
   */
  TC = 201,

  /**
   * @generated from enum value: COUNTRY_CODE_TD = 202;
   */
  TD = 202,

  /**
   * @generated from enum value: COUNTRY_CODE_TF = 203;
   */
  TF = 203,

  /**
   * @generated from enum value: COUNTRY_CODE_TG = 204;
   */
  TG = 204,

  /**
   * @generated from enum value: COUNTRY_CODE_TH = 205;
   */
  TH = 205,

  /**
   * @generated from enum value: COUNTRY_CODE_TJ = 206;
   */
  TJ = 206,

  /**
   * @generated from enum value: COUNTRY_CODE_TK = 207;
   */
  TK = 207,

  /**
   * @generated from enum value: COUNTRY_CODE_TL = 208;
   */
  TL = 208,

  /**
   * @generated from enum value: COUNTRY_CODE_TM = 209;
   */
  TM = 209,

  /**
   * @generated from enum value: COUNTRY_CODE_TN = 210;
   */
  TN = 210,

  /**
   * @generated from enum value: COUNTRY_CODE_TO = 211;
   */
  TO = 211,

  /**
   * @generated from enum value: COUNTRY_CODE_TR = 212;
   */
  TR = 212,

  /**
   * @generated from enum value: COUNTRY_CODE_TT = 213;
   */
  TT = 213,

  /**
   * @generated from enum value: COUNTRY_CODE_TV = 214;
   */
  TV = 214,

  /**
   * @generated from enum value: COUNTRY_CODE_TW = 215;
   */
  TW = 215,

  /**
   * @generated from enum value: COUNTRY_CODE_TZ = 216;
   */
  TZ = 216,

  /**
   * @generated from enum value: COUNTRY_CODE_UA = 217;
   */
  UA = 217,

  /**
   * @generated from enum value: COUNTRY_CODE_UG = 218;
   */
  UG = 218,

  /**
   * @generated from enum value: COUNTRY_CODE_US = 219;
   */
  US = 219,

  /**
   * @generated from enum value: COUNTRY_CODE_UY = 220;
   */
  UY = 220,

  /**
   * @generated from enum value: COUNTRY_CODE_UZ = 221;
   */
  UZ = 221,

  /**
   * @generated from enum value: COUNTRY_CODE_VA = 222;
   */
  VA = 222,

  /**
   * @generated from enum value: COUNTRY_CODE_VC = 223;
   */
  VC = 223,

  /**
   * @generated from enum value: COUNTRY_CODE_VE = 224;
   */
  VE = 224,

  /**
   * @generated from enum value: COUNTRY_CODE_VG = 225;
   */
  VG = 225,

  /**
   * @generated from enum value: COUNTRY_CODE_VN = 226;
   */
  VN = 226,

  /**
   * @generated from enum value: COUNTRY_CODE_VU = 227;
   */
  VU = 227,

  /**
   * @generated from enum value: COUNTRY_CODE_WF = 228;
   */
  WF = 228,

  /**
   * @generated from enum value: COUNTRY_CODE_WS = 229;
   */
  WS = 229,

  /**
   * @generated from enum value: COUNTRY_CODE_XK = 230;
   */
  XK = 230,

  /**
   * @generated from enum value: COUNTRY_CODE_YE = 231;
   */
  YE = 231,

  /**
   * @generated from enum value: COUNTRY_CODE_YT = 232;
   */
  YT = 232,

  /**
   * @generated from enum value: COUNTRY_CODE_ZA = 233;
   */
  ZA = 233,

  /**
   * @generated from enum value: COUNTRY_CODE_ZM = 234;
   */
  ZM = 234,

  /**
   * @generated from enum value: COUNTRY_CODE_ZW = 235;
   */
  ZW = 235,

  /**
   * @generated from enum value: COUNTRY_CODE_ZZ = 236;
   */
  ZZ = 236,
}

/**
 * Describes the enum sited_io.types.country.v1.CountryCode.
 */
export const CountryCodeSchema: GenEnum<CountryCode> = /*@__PURE__*/
  enumDesc(file_sited_io_types_country_v1_country, 0);


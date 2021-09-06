// Copyright 2020 Datafuse Labs.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

pub const UNSUPPORTED_METHOD: u32 = 1;
pub const UNSUPPORTED_PARAMETER: u32 = 2;
pub const UNEXPECTED_END_OF_FILE: u32 = 3;
pub const EXPECTED_END_OF_FILE: u32 = 4;
pub const CANNOT_PARSE_TEXT: u32 = 6;
pub const INCORRECT_NUMBER_OF_COLUMNS: u32 = 7;
pub const THERE_IS_NO_COLUMN: u32 = 8;
pub const SIZES_OF_COLUMNS_DOESNT_MATCH: u32 = 9;
pub const NOT_FOUND_COLUMN_IN_BLOCK: u32 = 10;
pub const POSITION_OUT_OF_BOUND: u32 = 11;
pub const PARAMETER_OUT_OF_BOUND: u32 = 12;
pub const SIZES_OF_COLUMNS_IN_TUPLE_DOESNT_MATCH: u32 = 13;
pub const DUPLICATE_COLUMN: u32 = 15;
pub const NO_SUCH_COLUMN_IN_TABLE: u32 = 16;
pub const DELIMITER_IN_STRING_LITERAL_DOESNT_MATCH: u32 = 17;
pub const CANNOT_INSERT_ELEMENT_INTO_CONSTANT_COLUMN: u32 = 18;
pub const SIZE_OF_FIXED_STRING_DOESNT_MATCH: u32 = 19;
pub const NUMBER_OF_COLUMNS_DOESNT_MATCH: u32 = 20;
pub const CANNOT_READ_ALL_DATA_FROM_TAB_SEPARATED_INPUT: u32 = 21;
pub const CANNOT_PARSE_ALL_VALUE_FROM_TAB_SEPARATED_INPUT: u32 = 22;
pub const CANNOT_READ_FROM_ISTREAM: u32 = 23;
pub const CANNOT_WRITE_TO_OSTREAM: u32 = 24;
pub const CANNOT_PARSE_ESCAPE_SEQUENCE: u32 = 25;
pub const CANNOT_PARSE_QUOTED_STRING: u32 = 26;
pub const CANNOT_PARSE_INPUT_ASSERTION_FAILED: u32 = 27;
pub const CANNOT_PRINT_FLOAT_OR_DOUBLE_NUMBER: u32 = 28;
pub const CANNOT_PRINT_INTEGER: u32 = 29;
pub const CANNOT_READ_SIZE_OF_COMPRESSED_CHUNK: u32 = 30;
pub const CANNOT_READ_COMPRESSED_CHUNK: u32 = 31;
pub const ATTEMPT_TO_READ_AFTER_EOF: u32 = 32;
pub const CANNOT_READ_ALL_DATA: u32 = 33;
pub const TOO_MANY_ARGUMENTS_FOR_FUNCTION: u32 = 34;
pub const TOO_FEW_ARGUMENTS_FOR_FUNCTION: u32 = 35;
pub const BAD_ARGUMENTS: u32 = 36;
pub const UNKNOWN_ELEMENT_IN_AST: u32 = 37;
pub const CANNOT_PARSE_DATE: u32 = 38;
pub const TOO_LARGE_SIZE_COMPRESSED: u32 = 39;
pub const CHECKSUM_DOESNT_MATCH: u32 = 40;
pub const CANNOT_PARSE_DATETIME: u32 = 41;
pub const NUMBER_OF_ARGUMENTS_DOESNT_MATCH: u32 = 42;
pub const ILLEGAL_TYPE_OF_ARGUMENT: u32 = 43;
pub const ILLEGAL_COLUMN: u32 = 44;
pub const ILLEGAL_NUMBER_OF_RESULT_COLUMNS: u32 = 45;
pub const UNKNOWN_FUNCTION: u32 = 46;
pub const UNKNOWN_IDENTIFIER: u32 = 47;
pub const NOT_IMPLEMENTED: u32 = 48;
pub const LOGICAL_ERROR: u32 = 49;
pub const UNKNOWN_TYPE: u32 = 50;
pub const EMPTY_LIST_OF_COLUMNS_QUERIED: u32 = 51;
pub const COLUMN_QUERIED_MORE_THAN_ONCE: u32 = 52;
pub const TYPE_MISMATCH: u32 = 53;
pub const STORAGE_DOESNT_ALLOW_PARAMETERS: u32 = 54;
pub const STORAGE_REQUIRES_PARAMETER: u32 = 55;
pub const UNKNOWN_STORAGE: u32 = 56;
pub const TABLE_ALREADY_EXISTS: u32 = 57;
pub const TABLE_METADATA_ALREADY_EXISTS: u32 = 58;
pub const ILLEGAL_TYPE_OF_COLUMN_FOR_FILTER: u32 = 59;
pub const UNKNOWN_TABLE: u32 = 60;
pub const ONLY_FILTER_COLUMN_IN_BLOCK: u32 = 61;
pub const SYNTAX_ERROR: u32 = 62;
pub const UNKNOWN_AGGREGATE_FUNCTION: u32 = 63;
pub const CANNOT_READ_AGGREGATE_FUNCTION_FROM_TEXT: u32 = 64;
pub const CANNOT_WRITE_AGGREGATE_FUNCTION_AS_TEXT: u32 = 65;
pub const NOT_A_COLUMN: u32 = 66;
pub const ILLEGAL_KEY_OF_AGGREGATION: u32 = 67;
pub const CANNOT_GET_SIZE_OF_FIELD: u32 = 68;
pub const ARGUMENT_OUT_OF_BOUND: u32 = 69;
pub const CANNOT_CONVERT_TYPE: u32 = 70;
pub const CANNOT_WRITE_AFTER_END_OF_BUFFER: u32 = 71;
pub const CANNOT_PARSE_NUMBER: u32 = 72;
pub const UNKNOWN_FORMAT: u32 = 73;
pub const CANNOT_READ_FROM_FILE_DESCRIPTOR: u32 = 74;
pub const CANNOT_WRITE_TO_FILE_DESCRIPTOR: u32 = 75;
pub const CANNOT_OPEN_FILE: u32 = 76;
pub const CANNOT_CLOSE_FILE: u32 = 77;
pub const UNKNOWN_TYPE_OF_QUERY: u32 = 78;
pub const INCORRECT_FILE_NAME: u32 = 79;
pub const INCORRECT_QUERY: u32 = 80;
pub const UNKNOWN_DATABASE: u32 = 81;
pub const DATABASE_ALREADY_EXISTS: u32 = 82;
pub const DIRECTORY_DOESNT_EXIST: u32 = 83;
pub const DIRECTORY_ALREADY_EXISTS: u32 = 84;
pub const FORMAT_IS_NOT_SUITABLE_FOR_INPUT: u32 = 85;
pub const RECEIVED_ERROR_FROM_REMOTE_IO_SERVER: u32 = 86;
pub const CANNOT_SEEK_THROUGH_FILE: u32 = 87;
pub const CANNOT_TRUNCATE_FILE: u32 = 88;
pub const UNKNOWN_COMPRESSION_METHOD: u32 = 89;
pub const EMPTY_LIST_OF_COLUMNS_PASSED: u32 = 90;
pub const SIZES_OF_MARKS_FILES_ARE_INCONSISTENT: u32 = 91;
pub const EMPTY_DATA_PASSED: u32 = 92;
pub const UNKNOWN_AGGREGATED_DATA_VARIANT: u32 = 93;
pub const CANNOT_MERGE_DIFFERENT_AGGREGATED_DATA_VARIANTS: u32 = 94;
pub const CANNOT_READ_FROM_SOCKET: u32 = 95;
pub const CANNOT_WRITE_TO_SOCKET: u32 = 96;
pub const CANNOT_READ_ALL_DATA_FROM_CHUNKED_INPUT: u32 = 97;
pub const CANNOT_WRITE_TO_EMPTY_BLOCK_OUTPUT_STREAM: u32 = 98;
pub const UNKNOWN_PACKET_FROM_CLIENT: u32 = 99;
pub const UNKNOWN_PACKET_FROM_SERVER: u32 = 100;
pub const UNEXPECTED_PACKET_FROM_CLIENT: u32 = 101;
pub const UNEXPECTED_PACKET_FROM_SERVER: u32 = 102;
pub const RECEIVED_DATA_FOR_WRONG_QUERY_ID: u32 = 103;
pub const TOO_SMALL_BUFFER_SIZE: u32 = 104;
pub const CANNOT_READ_HISTORY: u32 = 105;
pub const CANNOT_APPEND_HISTORY: u32 = 106;
pub const FILE_DOESNT_EXIST: u32 = 107;
pub const NO_DATA_TO_INSERT: u32 = 108;
pub const CANNOT_BLOCK_SIGNAL: u32 = 109;
pub const CANNOT_UNBLOCK_SIGNAL: u32 = 110;
pub const CANNOT_MANIPULATE_SIGSET: u32 = 111;
pub const CANNOT_WAIT_FOR_SIGNAL: u32 = 112;
pub const THERE_IS_NO_SESSION: u32 = 113;
pub const CANNOT_CLOCK_GETTIME: u32 = 114;
pub const UNKNOWN_SETTING: u32 = 115;
pub const THERE_IS_NO_DEFAULT_VALUE: u32 = 116;
pub const INCORRECT_DATA: u32 = 117;
pub const ENGINE_REQUIRED: u32 = 119;
pub const CANNOT_INSERT_VALUE_OF_DIFFERENT_SIZE_INTO_TUPLE: u32 = 120;
pub const UNSUPPORTED_JOIN_KEYS: u32 = 121;
pub const INCOMPATIBLE_COLUMNS: u32 = 122;
pub const UNKNOWN_TYPE_OF_AST_NODE: u32 = 123;
pub const INCORRECT_ELEMENT_OF_SET: u32 = 124;
pub const INCORRECT_RESULT_OF_SCALAR_SUBQUERY: u32 = 125;
pub const CANNOT_GET_RETURN_TYPE: u32 = 126;
pub const ILLEGAL_INDEX: u32 = 127;
pub const TOO_LARGE_ARRAY_SIZE: u32 = 128;
pub const FUNCTION_IS_SPECIAL: u32 = 129;
pub const CANNOT_READ_ARRAY_FROM_TEXT: u32 = 130;
pub const TOO_LARGE_STRING_SIZE: u32 = 131;
pub const CANNOT_CREATE_TABLE_FROM_METADATA: u32 = 132;
pub const AGGREGATE_FUNCTION_DOESNT_ALLOW_PARAMETERS: u32 = 133;
pub const PARAMETERS_TO_AGGREGATE_FUNCTIONS_MUST_BE_LITERALS: u32 = 134;
pub const ZERO_ARRAY_OR_TUPLE_INDEX: u32 = 135;
pub const UNKNOWN_ELEMENT_IN_CONFIG: u32 = 137;
pub const EXCESSIVE_ELEMENT_IN_CONFIG: u32 = 138;
pub const NO_ELEMENTS_IN_CONFIG: u32 = 139;
pub const ALL_REQUESTED_COLUMNS_ARE_MISSING: u32 = 140;
pub const SAMPLING_NOT_SUPPORTED: u32 = 141;
pub const NOT_FOUND_NODE: u32 = 142;
pub const FOUND_MORE_THAN_ONE_NODE: u32 = 143;
pub const FIRST_DATE_IS_BIGGER_THAN_LAST_DATE: u32 = 144;
pub const UNKNOWN_OVERFLOW_MODE: u32 = 145;
pub const QUERY_SECTION_DOESNT_MAKE_SENSE: u32 = 146;
pub const NOT_FOUND_FUNCTION_ELEMENT_FOR_AGGREGATE: u32 = 147;
pub const NOT_FOUND_RELATION_ELEMENT_FOR_CONDITION: u32 = 148;
pub const NOT_FOUND_RHS_ELEMENT_FOR_CONDITION: u32 = 149;
pub const EMPTY_LIST_OF_ATTRIBUTES_PASSED: u32 = 150;
pub const INDEX_OF_COLUMN_IN_SORT_CLAUSE_IS_OUT_OF_RANGE: u32 = 151;
pub const UNKNOWN_DIRECTION_OF_SORTING: u32 = 152;
pub const ILLEGAL_DIVISION: u32 = 153;
pub const AGGREGATE_FUNCTION_NOT_APPLICABLE: u32 = 154;
pub const UNKNOWN_RELATION: u32 = 155;
pub const DICTIONARIES_WAS_NOT_LOADED: u32 = 156;
pub const ILLEGAL_OVERFLOW_MODE: u32 = 157;
pub const TOO_MANY_ROWS: u32 = 158;
pub const TIMEOUT_EXCEEDED: u32 = 159;
pub const TOO_SLOW: u32 = 160;
pub const TOO_MANY_COLUMNS: u32 = 161;
pub const TOO_DEEP_SUBQUERIES: u32 = 162;
pub const TOO_DEEP_PIPELINE: u32 = 163;
pub const READONLY: u32 = 164;
pub const TOO_MANY_TEMPORARY_COLUMNS: u32 = 165;
pub const TOO_MANY_TEMPORARY_NON_CONST_COLUMNS: u32 = 166;
pub const TOO_DEEP_AST: u32 = 167;
pub const TOO_BIG_AST: u32 = 168;
pub const BAD_TYPE_OF_FIELD: u32 = 169;
pub const BAD_GET: u32 = 170;
pub const BLOCKS_HAVE_DIFFERENT_STRUCTURE: u32 = 171;
pub const CANNOT_CREATE_DIRECTORY: u32 = 172;
pub const CANNOT_ALLOCATE_MEMORY: u32 = 173;
pub const CYCLIC_ALIASES: u32 = 174;
pub const CHUNK_NOT_FOUND: u32 = 176;
pub const DUPLICATE_CHUNK_NAME: u32 = 177;
pub const MULTIPLE_ALIASES_FOR_EXPRESSION: u32 = 178;
pub const MULTIPLE_EXPRESSIONS_FOR_ALIAS: u32 = 179;
pub const THERE_IS_NO_PROFILE: u32 = 180;
pub const ILLEGAL_FINAL: u32 = 181;
pub const ILLEGAL_PREWHERE: u32 = 182;
pub const UNEXPECTED_EXPRESSION: u32 = 183;
pub const ILLEGAL_AGGREGATION: u32 = 184;
pub const UNSUPPORTED_MYISAM_BLOCK_TYPE: u32 = 185;
pub const UNSUPPORTED_COLLATION_LOCALE: u32 = 186;
pub const COLLATION_COMPARISON_FAILED: u32 = 187;
pub const UNKNOWN_ACTION: u32 = 188;
pub const TABLE_MUST_NOT_BE_CREATED_MANUALLY: u32 = 189;
pub const SIZES_OF_ARRAYS_DOESNT_MATCH: u32 = 190;
pub const SET_SIZE_LIMIT_EXCEEDED: u32 = 191;
pub const UNKNOWN_USER: u32 = 192;
pub const WRONG_PASSWORD: u32 = 193;
pub const REQUIRED_PASSWORD: u32 = 194;
pub const IP_ADDRESS_NOT_ALLOWED: u32 = 195;
pub const UNKNOWN_ADDRESS_PATTERN_TYPE: u32 = 196;
pub const SERVER_REVISION_IS_TOO_OLD: u32 = 197;
pub const DNS_ERROR: u32 = 198;
pub const UNKNOWN_QUOTA: u32 = 199;
pub const QUOTA_DOESNT_ALLOW_KEYS: u32 = 200;
pub const QUOTA_EXPIRED: u32 = 201;
pub const TOO_MANY_SIMULTANEOUS_QUERIES: u32 = 202;
pub const NO_FREE_CONNECTION: u32 = 203;
pub const CANNOT_FSYNC: u32 = 204;
pub const NESTED_TYPE_TOO_DEEP: u32 = 205;
pub const ALIAS_REQUIRED: u32 = 206;
pub const AMBIGUOUS_IDENTIFIER: u32 = 207;
pub const EMPTY_NESTED_TABLE: u32 = 208;
pub const SOCKET_TIMEOUT: u32 = 209;
pub const NETWORK_ERROR: u32 = 210;
pub const EMPTY_QUERY: u32 = 211;
pub const UNKNOWN_LOAD_BALANCING: u32 = 212;
pub const UNKNOWN_TOTALS_MODE: u32 = 213;
pub const CANNOT_STATVFS: u32 = 214;
pub const NOT_AN_AGGREGATE: u32 = 215;
pub const QUERY_WITH_SAME_ID_IS_ALREADY_RUNNING: u32 = 216;
pub const CLIENT_HAS_CONNECTED_TO_WRONG_PORT: u32 = 217;
pub const TABLE_IS_DROPPED: u32 = 218;
pub const DATABASE_NOT_EMPTY: u32 = 219;
pub const DUPLICATE_INTERSERVER_IO_ENDPOINT: u32 = 220;
pub const NO_SUCH_INTERSERVER_IO_ENDPOINT: u32 = 221;
pub const ADDING_REPLICA_TO_NON_EMPTY_TABLE: u32 = 222;
pub const UNEXPECTED_AST_STRUCTURE: u32 = 223;
pub const REPLICA_IS_ALREADY_ACTIVE: u32 = 224;
pub const NO_ZOOKEEPER: u32 = 225;
pub const NO_FILE_IN_DATA_PART: u32 = 226;
pub const UNEXPECTED_FILE_IN_DATA_PART: u32 = 227;
pub const BAD_SIZE_OF_FILE_IN_DATA_PART: u32 = 228;
pub const QUERY_IS_TOO_LARGE: u32 = 229;
pub const NOT_FOUND_EXPECTED_DATA_PART: u32 = 230;
pub const TOO_MANY_UNEXPECTED_DATA_PARTS: u32 = 231;
pub const NO_SUCH_DATA_PART: u32 = 232;
pub const BAD_DATA_PART_NAME: u32 = 233;
pub const NO_REPLICA_HAS_PART: u32 = 234;
pub const DUPLICATE_DATA_PART: u32 = 235;
pub const ABORTED: u32 = 236;
pub const NO_REPLICA_NAME_GIVEN: u32 = 237;
pub const FORMAT_VERSION_TOO_OLD: u32 = 238;
pub const CANNOT_MUNMAP: u32 = 239;
pub const CANNOT_MREMAP: u32 = 240;
pub const MEMORY_LIMIT_EXCEEDED: u32 = 241;
pub const TABLE_IS_READ_ONLY: u32 = 242;
pub const NOT_ENOUGH_SPACE: u32 = 243;
pub const UNEXPECTED_ZOOKEEPER_ERROR: u32 = 244;
pub const CORRUPTED_DATA: u32 = 246;
pub const INCORRECT_MARK: u32 = 247;
pub const INVALID_PARTITION_VALUE: u32 = 248;
pub const NOT_ENOUGH_BLOCK_NUMBERS: u32 = 250;
pub const NO_SUCH_REPLICA: u32 = 251;
pub const TOO_MANY_PARTS: u32 = 252;
pub const REPLICA_IS_ALREADY_EXIST: u32 = 253;
pub const NO_ACTIVE_REPLICAS: u32 = 254;
pub const TOO_MANY_RETRIES_TO_FETCH_PARTS: u32 = 255;
pub const PARTITION_ALREADY_EXISTS: u32 = 256;
pub const PARTITION_DOESNT_EXIST: u32 = 257;
pub const UNION_ALL_RESULT_STRUCTURES_MISMATCH: u32 = 258;
pub const CLIENT_OUTPUT_FORMAT_SPECIFIED: u32 = 260;
pub const UNKNOWN_BLOCK_INFO_FIELD: u32 = 261;
pub const BAD_COLLATION: u32 = 262;
pub const CANNOT_COMPILE_CODE: u32 = 263;
pub const INCOMPATIBLE_TYPE_OF_JOIN: u32 = 264;
pub const NO_AVAILABLE_REPLICA: u32 = 265;
pub const MISMATCH_REPLICAS_DATA_SOURCES: u32 = 266;
pub const STORAGE_DOESNT_SUPPORT_PARALLEL_REPLICAS: u32 = 267;
pub const CPUID_ERROR: u32 = 268;
pub const INFINITE_LOOP: u32 = 269;
pub const CANNOT_COMPRESS: u32 = 270;
pub const CANNOT_DECOMPRESS: u32 = 271;
pub const CANNOT_IO_SUBMIT: u32 = 272;
pub const CANNOT_IO_GETEVENTS: u32 = 273;
pub const AIO_READ_ERROR: u32 = 274;
pub const AIO_WRITE_ERROR: u32 = 275;
pub const INDEX_NOT_USED: u32 = 277;
pub const LEADERSHIP_LOST: u32 = 278;
pub const ALL_CONNECTION_TRIES_FAILED: u32 = 279;
pub const NO_AVAILABLE_DATA: u32 = 280;
pub const DICTIONARY_IS_EMPTY: u32 = 281;
pub const INCORRECT_INDEX: u32 = 282;
pub const UNKNOWN_DISTRIBUTED_PRODUCT_MODE: u32 = 283;
pub const UNKNOWN_GLOBAL_SUBQUERIES_METHOD: u32 = 284;
pub const TOO_FEW_LIVE_REPLICAS: u32 = 285;
pub const UNSATISFIED_QUORUM_FOR_PREVIOUS_WRITE: u32 = 286;
pub const UNKNOWN_FORMAT_VERSION: u32 = 287;
pub const DISTRIBUTED_IN_JOIN_SUBQUERY_DENIED: u32 = 288;
pub const REPLICA_IS_NOT_IN_QUORUM: u32 = 289;
pub const LIMIT_EXCEEDED: u32 = 290;
pub const DATABASE_ACCESS_DENIED: u32 = 291;
pub const LEADERSHIP_CHANGED: u32 = 292;
pub const MONGODB_CANNOT_AUTHENTICATE: u32 = 293;
pub const INVALID_BLOCK_EXTRA_INFO: u32 = 294;
pub const RECEIVED_EMPTY_DATA: u32 = 295;
pub const NO_REMOTE_SHARD_FOUND: u32 = 296;
pub const SHARD_HAS_NO_CONNECTIONS: u32 = 297;
pub const CANNOT_PIPE: u32 = 298;
pub const CANNOT_FORK: u32 = 299;
pub const CANNOT_DLSYM: u32 = 300;
pub const CANNOT_CREATE_CHILD_PROCESS: u32 = 301;
pub const CHILD_WAS_NOT_EXITED_NORMALLY: u32 = 302;
pub const CANNOT_SELECT: u32 = 303;
pub const CANNOT_WAITPID: u32 = 304;
pub const TABLE_WAS_NOT_DROPPED: u32 = 305;
pub const TOO_DEEP_RECURSION: u32 = 306;
pub const TOO_MANY_BYTES: u32 = 307;
pub const UNEXPECTED_NODE_IN_ZOOKEEPER: u32 = 308;
pub const FUNCTION_CANNOT_HAVE_PARAMETERS: u32 = 309;
pub const INVALID_SHARD_WEIGHT: u32 = 317;
pub const INVALID_CONFIG_PARAMETER: u32 = 318;
pub const UNKNOWN_STATUS_OF_INSERT: u32 = 319;
pub const VALUE_IS_OUT_OF_RANGE_OF_DATA_TYPE: u32 = 321;
pub const BARRIER_TIMEOUT: u32 = 335;
pub const UNKNOWN_DATABASE_ENGINE: u32 = 336;
pub const DDL_GUARD_IS_ACTIVE: u32 = 337;
pub const UNFINISHED: u32 = 341;
pub const METADATA_MISMATCH: u32 = 342;
pub const SUPPORT_IS_DISABLED: u32 = 344;
pub const TABLE_DIFFERS_TOO_MUCH: u32 = 345;
pub const CANNOT_CONVERT_CHARSET: u32 = 346;
pub const CANNOT_LOAD_CONFIG: u32 = 347;
pub const CANNOT_INSERT_NULL_IN_ORDINARY_COLUMN: u32 = 349;
pub const INCOMPATIBLE_SOURCE_TABLES: u32 = 350;
pub const AMBIGUOUS_TABLE_NAME: u32 = 351;
pub const AMBIGUOUS_COLUMN_NAME: u32 = 352;
pub const INDEX_OF_POSITIONAL_ARGUMENT_IS_OUT_OF_RANGE: u32 = 353;
pub const ZLIB_INFLATE_FAILED: u32 = 354;
pub const ZLIB_DEFLATE_FAILED: u32 = 355;
pub const BAD_LAMBDA: u32 = 356;
pub const RESERVED_IDENTIFIER_NAME: u32 = 357;
pub const INTO_OUTFILE_NOT_ALLOWED: u32 = 358;
pub const TABLE_SIZE_EXCEEDS_MAX_DROP_SIZE_LIMIT: u32 = 359;
pub const CANNOT_CREATE_CHARSET_CONVERTER: u32 = 360;
pub const SEEK_POSITION_OUT_OF_BOUND: u32 = 361;
pub const CURRENT_WRITE_BUFFER_IS_EXHAUSTED: u32 = 362;
pub const CANNOT_CREATE_IO_BUFFER: u32 = 363;
pub const RECEIVED_ERROR_TOO_MANY_REQUESTS: u32 = 364;
pub const OUTPUT_IS_NOT_SORTED: u32 = 365;
pub const SIZES_OF_NESTED_COLUMNS_ARE_INCONSISTENT: u32 = 366;
pub const TOO_MANY_FETCHES: u32 = 367;
pub const BAD_CAST: u32 = 368;
pub const ALL_REPLICAS_ARE_STALE: u32 = 369;
pub const DATA_TYPE_CANNOT_BE_USED_IN_TABLES: u32 = 370;
pub const INCONSISTENT_CLUSTER_DEFINITION: u32 = 371;
pub const SESSION_NOT_FOUND: u32 = 372;
pub const SESSION_IS_LOCKED: u32 = 373;
pub const INVALID_SESSION_TIMEOUT: u32 = 374;
pub const CANNOT_DLOPEN: u32 = 375;
pub const CANNOT_PARSE_UUID: u32 = 376;
pub const ILLEGAL_SYNTAX_FOR_DATA_TYPE: u32 = 377;
pub const DATA_TYPE_CANNOT_HAVE_ARGUMENTS: u32 = 378;
pub const UNKNOWN_STATUS_OF_DISTRIBUTED_DDL_TASK: u32 = 379;
pub const CANNOT_KILL: u32 = 380;
pub const HTTP_LENGTH_REQUIRED: u32 = 381;
pub const CANNOT_LOAD_CATBOOST_MODEL: u32 = 382;
pub const CANNOT_APPLY_CATBOOST_MODEL: u32 = 383;
pub const PART_IS_TEMPORARILY_LOCKED: u32 = 384;
pub const MULTIPLE_STREAMS_REQUIRED: u32 = 385;
pub const NO_COMMON_TYPE: u32 = 386;
pub const DICTIONARY_ALREADY_EXISTS: u32 = 387;
pub const CANNOT_ASSIGN_OPTIMIZE: u32 = 388;
pub const INSERT_WAS_DEDUPLICATED: u32 = 389;
pub const CANNOT_GET_CREATE_TABLE_QUERY: u32 = 390;
pub const EXTERNAL_LIBRARY_ERROR: u32 = 391;
pub const QUERY_IS_PROHIBITED: u32 = 392;
pub const THERE_IS_NO_QUERY: u32 = 393;
pub const QUERY_WAS_CANCELLED: u32 = 394;
pub const FUNCTION_THROW_IF_VALUE_IS_NON_ZERO: u32 = 395;
pub const TOO_MANY_ROWS_OR_BYTES: u32 = 396;
pub const QUERY_IS_NOT_SUPPORTED_IN_MATERIALIZED_VIEW: u32 = 397;
pub const UNKNOWN_MUTATION_COMMAND: u32 = 398;
pub const FORMAT_IS_NOT_SUITABLE_FOR_OUTPUT: u32 = 399;
pub const CANNOT_STAT: u32 = 400;
pub const FEATURE_IS_NOT_ENABLED_AT_BUILD_TIME: u32 = 401;
pub const CANNOT_IOSETUP: u32 = 402;
pub const INVALID_JOIN_ON_EXPRESSION: u32 = 403;
pub const BAD_ODBC_CONNECTION_STRING: u32 = 404;
pub const PARTITION_SIZE_EXCEEDS_MAX_DROP_SIZE_LIMIT: u32 = 405;
pub const TOP_AND_LIMIT_TOGETHER: u32 = 406;
pub const DECIMAL_OVERFLOW: u32 = 407;
pub const BAD_REQUEST_PARAMETER: u32 = 408;
pub const EXTERNAL_EXECUTABLE_NOT_FOUND: u32 = 409;
pub const EXTERNAL_SERVER_IS_NOT_RESPONDING: u32 = 410;
pub const PTHREAD_ERROR: u32 = 411;
pub const NETLINK_ERROR: u32 = 412;
pub const CANNOT_SET_SIGNAL_HANDLER: u32 = 413;
pub const CANNOT_READLINE: u32 = 414;
pub const ALL_REPLICAS_LOST: u32 = 415;
pub const REPLICA_STATUS_CHANGED: u32 = 416;
pub const EXPECTED_ALL_OR_ANY: u32 = 417;
pub const UNKNOWN_JOIN_STRICTNESS: u32 = 418;
pub const MULTIPLE_ASSIGNMENTS_TO_COLUMN: u32 = 419;
pub const CANNOT_UPDATE_COLUMN: u32 = 420;
pub const CANNOT_ADD_DIFFERENT_AGGREGATE_STATES: u32 = 421;
pub const UNSUPPORTED_URI_SCHEME: u32 = 422;
pub const CANNOT_GETTIMEOFDAY: u32 = 423;
pub const CANNOT_LINK: u32 = 424;
pub const SYSTEM_ERROR: u32 = 425;
pub const NULL_POINTER_DEREFERENCE: u32 = 426;
pub const CANNOT_COMPILE_REGEXP: u32 = 427;
pub const UNKNOWN_LOG_LEVEL: u32 = 428;
pub const FAILED_TO_GETPWUID: u32 = 429;
pub const MISMATCHING_USERS_FOR_PROCESS_AND_DATA: u32 = 430;
pub const ILLEGAL_SYNTAX_FOR_CODEC_TYPE: u32 = 431;
pub const UNKNOWN_CODEC: u32 = 432;
pub const ILLEGAL_CODEC_PARAMETER: u32 = 433;
pub const CANNOT_PARSE_PROTOBUF_SCHEMA: u32 = 434;
pub const NO_DATA_FOR_REQUIRED_PROTOBUF_FIELD: u32 = 435;
pub const PROTOBUF_BAD_CAST: u32 = 436;
pub const PROTOBUF_FIELD_NOT_REPEATED: u32 = 437;
pub const DATA_TYPE_CANNOT_BE_PROMOTED: u32 = 438;
pub const CANNOT_SCHEDULE_TASK: u32 = 439;
pub const INVALID_LIMIT_EXPRESSION: u32 = 440;
pub const CANNOT_PARSE_DOMAIN_VALUE_FROM_STRING: u32 = 441;
pub const BAD_DATABASE_FOR_TEMPORARY_TABLE: u32 = 442;
pub const NO_COMMON_COLUMNS_WITH_PROTOBUF_SCHEMA: u32 = 443;
pub const UNKNOWN_PROTOBUF_FORMAT: u32 = 444;
pub const CANNOT_MPROTECT: u32 = 445;
pub const FUNCTION_NOT_ALLOWED: u32 = 446;
pub const HYPERSCAN_CANNOT_SCAN_TEXT: u32 = 447;
pub const BROTLI_READ_FAILED: u32 = 448;
pub const BROTLI_WRITE_FAILED: u32 = 449;
pub const BAD_TTL_EXPRESSION: u32 = 450;
pub const BAD_TTL_FILE: u32 = 451;
pub const SETTING_CONSTRAINT_VIOLATION: u32 = 452;
pub const MYSQL_CLIENT_INSUFFICIENT_CAPABILITIES: u32 = 453;
pub const OPENSSL_ERROR: u32 = 454;
pub const SUSPICIOUS_TYPE_FOR_LOW_CARDINALITY: u32 = 455;
pub const UNKNOWN_QUERY_PARAMETER: u32 = 456;
pub const BAD_QUERY_PARAMETER: u32 = 457;
pub const CANNOT_UNLINK: u32 = 458;
pub const CANNOT_SET_THREAD_PRIORITY: u32 = 459;
pub const CANNOT_CREATE_TIMER: u32 = 460;
pub const CANNOT_SET_TIMER_PERIOD: u32 = 461;
pub const CANNOT_DELETE_TIMER: u32 = 462;
pub const CANNOT_FCNTL: u32 = 463;
pub const CANNOT_PARSE_ELF: u32 = 464;
pub const CANNOT_PARSE_DWARF: u32 = 465;
pub const INSECURE_PATH: u32 = 466;
pub const CANNOT_PARSE_BOOL: u32 = 467;
pub const CANNOT_PTHREAD_ATTR: u32 = 468;
pub const VIOLATED_CONSTRAINT: u32 = 469;
pub const QUERY_IS_NOT_SUPPORTED_IN_LIVE_VIEW: u32 = 470;
pub const SETTINGS_ARE_NOT_SUPPORTED: u32 = 471;
pub const READONLY_SETTING: u32 = 472;
pub const DEADLOCK_AVOIDED: u32 = 473;
pub const INVALID_TEMPLATE_FORMAT: u32 = 474;
pub const INVALID_WITH_FILL_EXPRESSION: u32 = 475;
pub const WITH_TIES_WITHOUT_ORDER_BY: u32 = 476;
pub const INVALID_USAGE_OF_INPUT: u32 = 477;
pub const UNKNOWN_POLICY: u32 = 478;
pub const UNKNOWN_DISK: u32 = 479;
pub const UNKNOWN_PROTOCOL: u32 = 480;
pub const PATH_ACCESS_DENIED: u32 = 481;
pub const DICTIONARY_ACCESS_DENIED: u32 = 482;
pub const TOO_MANY_REDIRECTS: u32 = 483;
pub const INTERNAL_REDIS_ERROR: u32 = 484;
pub const SCALAR_ALREADY_EXISTS: u32 = 485;
pub const UNKNOWN_SCALAR: u32 = 486;
pub const CANNOT_GET_CREATE_DICTIONARY_QUERY: u32 = 487;
pub const UNKNOWN_DICTIONARY: u32 = 488;
pub const INCORRECT_DICTIONARY_DEFINITION: u32 = 489;
pub const CANNOT_FORMAT_DATETIME: u32 = 490;
pub const UNACCEPTABLE_URL: u32 = 491;
pub const ACCESS_ENTITY_NOT_FOUND: u32 = 492;
pub const ACCESS_ENTITY_ALREADY_EXISTS: u32 = 493;
pub const ACCESS_ENTITY_FOUND_DUPLICATES: u32 = 494;
pub const ACCESS_ENTITY_STORAGE_READONLY: u32 = 495;
pub const QUOTA_REQUIRES_CLIENT_KEY: u32 = 496;
pub const NOT_ENOUGH_PRIVILEGES: u32 = 497;
pub const LIMIT_BY_WITH_TIES_IS_NOT_SUPPORTED: u32 = 498;
pub const S3_ERROR: u32 = 499;
pub const CANNOT_CREATE_DICTIONARY_FROM_METADATA: u32 = 500;
pub const CANNOT_CREATE_DATABASE: u32 = 501;

pub const KEEPER_EXCEPTION: u32 = 999;
pub const POCO_EXCEPTION: u32 = 1000;
pub const STD_EXCEPTION: u32 = 1001;
pub const UNKNOWN_EXCEPTION: u32 = 1002;

pub const CONDITIONAL_TREE_PARENT_NOT_FOUND: u32 = 2001;
pub const ILLEGAL_PROJECTION_MANIPULATOR: u32 = 2002;

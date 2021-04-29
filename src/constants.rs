pub const ADDR_PLUS: &str = "112.137.129.129:27001";
pub const ADDR_POLY: &str = "112.137.129.129:27002";

pub const PKT_HELLO: u32 = 0;
pub const PKT_CALC: u32 = 1;
pub const PKT_RESULT: u32 = 2;
pub const PKT_BYE: u32 = 3;
pub const PKT_FLAG: u32 = 4;

pub const HEADER_SIZE: usize = 8;

pub const TYPE_OFFSET: usize = 0;
const TYPE_SIZE: usize = 4;
pub const TYPE_END: usize = TYPE_OFFSET + TYPE_SIZE;

pub const LEN_OFFSET: usize = 4;
const LEN_SIZE: usize = 4;
pub const LEN_END: usize = LEN_OFFSET + LEN_SIZE;

pub const DATA_OFFSET: usize = HEADER_SIZE;

pub const BYTES_PER_U32: usize = 4;
// RISC-V Tiny VM - Ivi Ballou / Amechania

/* 32-bit RISC-V instructions
 * 
 * Layout:
 * 00 01 02 03 04 05 06 07 08 09 0A 0B 0C 0D 0E 0F 10 11 12 13 14 15 16 17 18 19 1A 1B 1C 1D 1E 1F
 * 
 * 
 */ 

// Silly rust compiler
#![allow(dead_code)]

// Masks
#[allow(non_snake_case)]
pub mod MASK {

    // Field offsets
    mod OFF {
        pub const OP : u8 = 0;
        pub const RD : u8 = 7;
        pub const F3 : u8 = 12;
        pub const F7 : u8 = 25;
        pub const RS1: u8 = 15;
        pub const RS2: u8 = 20;
    }

    pub const OP : u32 = 0x7F << OFF::OP;
    pub const RD : u32 = 0x1F << OFF::RD;
    pub const F3 : u32 = 0x7  << OFF::F3;
    pub const F7 : u32 = 0x7F << OFF::F7;
    pub const RS1: u32 = 0x1F << OFF::RS1;
    pub const RS2: u32 = 0x1F << OFF::RS2;

    pub const LUI_IMM         : u32 = 0xFF_FF_F0_00;
    pub const JAL_IMM_20      : u32 = 0x1     << 31;
    pub const JAL_IMM_10_1    : u32 = 0x03_FF << 21;
    pub const JAL_IMM_11      : u32 = 0x1     << 20;
    pub const JAL_IMM_19_12   : u32 = 0x1     << 12;
    pub const JALR_IMM        : u32 = 0x0F_FF << 20;
    pub const LOAD_IMM        : u32 = 0x0F_FF << 20;
    pub const STORE_IMM_11_5  : u32 = 0x7F    << 25;
    pub const STORE_IMM_4_0   : u32 = 0x1F    << 7;
    pub const BRANCH_IMM_12   : u32 = 0x1     << 31;
    pub const BRANCH_IMM_11   : u32 = 0x1     << 7;
    pub const BRANCH_IMM_10_5 : u32 = 0x7F    << 25;
    pub const BRANCH_IMM_4_1  : u32 = 0xF     << 8;
    pub const ALUI_IMM        : u32 = 0x0F_FF << 20;



}

// Opcodes
#[allow(non_snake_case)]
pub(crate) mod OP {
    pub const LUI    : u8 = 0x37; // LUI
    pub const AUIPC  : u8 = 0x17; // AUIPC
    pub const JAL    : u8 = 0x6F; // JAL
    pub const JALR   : u8 = 0x67; // JALR
    pub const BRANCH : u8 = 0x63; // BEQ, BNE, BLT, BGE, BLTU, BGEU
    pub const LOAD   : u8 = 0x03; // LB, LH, LW, LBU, LHU
    pub const STORE  : u8 = 0x02; // SB, SH, SW
    pub const ALUI   : u8 = 0x13; // ADDI, SLTI, SLTIU, XORI, ORI, ANDI, SLLI, SRLI, SRAI
    pub const ALU    : u8 = 0x33; // ADD, SUB, SLL, SLT, SLTU, XOR, SRL, SRA, OR, AND
    pub const FENCE  : u8 = 0x0F; // FENCE, FENCE.I
    pub const E_C    : u8 = 0x73; // ECALL, EBREAK, CSRRW, CSRRS, CSRRC, CSRRWI, CSRRSI, CSRRCI
}

// Function 3 Codes
pub(crate) const F3_BEQ: u8 = 0x00;
pub(crate) const F3_BNE: u8 = 0x01;
pub(crate) const F3_BLT: u8 = 0x04;
pub(crate) const F3_BGE: u8 = 0x05;
pub(crate) const F3_BLTU: u8 = 0x06;
pub(crate) const F3_BGEU: u8 = 0x07;

pub(crate) const F3_LB: u8 = 0x00;
pub(crate) const F3_LH: u8 = 0x01;
pub(crate) const F3_LW: u8 = 0x02;
pub(crate) const F3_LBU: u8 = 0x04;
pub(crate) const F3_LHU: u8 = 0x05;

pub(crate) const F3_SB: u8 = 0x00;
pub(crate) const F3_SH: u8 = 0x01;
pub(crate) const F3_SW: u8 = 0x02;

pub(crate) const F3_ADDI: u8 = 0x00;
pub(crate) const F3_SLTI: u8 = 0x02;
pub(crate) const F3_SLTIU: u8 = 0x03;
pub(crate) const F3_XORI: u8 = 0x04;
pub(crate) const F3_ORI: u8 = 0x06;
pub(crate) const F3_ANDI: u8 = 0x07;

pub(crate) const F3_SLLI: u8 = 0x01;
pub(crate) const F3_SRLI_SRAI: u8 = 0x05; // check bit 30

// We check F7C to discern between ADD and SUB
pub(crate) const F3_ADD_SUB: u8 = 0x00; // check F7C

// First set of M extension instruction of F3 codes
pub(crate) const F3_MUL: u8 = 0x00;
pub(crate) const F3_MULH: u8 = 0x01;
pub(crate) const F3_MULHSU: u8 = 0x02;
pub(crate) const F3_MULHU: u8 = 0x03;
pub(crate) const F3_DIV: u8 = 0x04;
pub(crate) const F3_DIVU: u8 = 0x05;
pub(crate) const F3_REM: u8 = 0x06;
pub(crate) const F3_REMU: u8 = 0x07;

// W set of M extension instruction of F3 codes
pub(crate) const F3_MULW: u8 = 0x00;
pub(crate) const F3_DIVW: u8 = 0x04;
pub(crate) const F3_DIVUW: u8 = 0x05;
pub(crate) const F3_REMW: u8 = 0x06;
pub(crate) const F3_REMUW: u8 = 0x07;

// D set of M extension instruction of F3 codes
pub(crate) const F3_MULD: u8 = 0x00;
pub(crate) const F3_DIVD: u8 = 0x04;
pub(crate) const F3_DIVUD: u8 = 0x05;
pub(crate) const F3_REMD: u8 = 0x06;
pub(crate) const F3_REMUD: u8 = 0x07;

pub(crate) const F3_SLL: u8 = 0x01;
pub(crate) const F3_SLT: u8 = 0x02;
pub(crate) const F3_SLTU: u8 = 0x03;
pub(crate) const F3_XOR: u8 = 0x04;
pub(crate) const F3_SRL_SLA: u8 = 0x05; // check F7C
pub(crate) const F3_OR: u8 = 0x06;
pub(crate) const F3_AND: u8 = 0x07;

pub(crate) const F3_FENCE: u8 = 0x00;
pub(crate) const F3_FENCE_I: u8 = 0x01;

pub(crate) const F3_ECALL_EBREAK: u8 = 0x00; // check imm[11:0]
pub(crate) const F3_CSRRW: u8 = 0x01;
pub(crate) const F3_CSRRS: u8 = 0x02;
pub(crate) const F3_CSRRC: u8 = 0x03;
pub(crate) const F3_CSRRWI: u8 = 0x05;
pub(crate) const F3_CSRRSI: u8 = 0x06;
pub(crate) const F3_CSRRCI: u8 = 0x07;

// Function 7 codes
pub(crate) const F7_SRLI: u8 = 0x00;
pub(crate) const F7_SRAI: u8 = 0x08;

pub(crate) const F7_ADD: u8 = 0x00;
pub(crate) const F7_SUB: u8 = 0x20;

// These codes are used for every M extension instruction
pub(crate) const F7_M_EXTENSION: u8 = 0x33;
// W instructions are valid for RV64. We're only targeting RV32
// pub(crate) const F7_M_EXTENSION_W: u8 = 0x3B;

pub(crate) const F7_SRL: u8 = 0x00;
pub(crate) const F7_SRA: u8 = 0x20;

pub(crate) const F73_ADD: u16 = ((F7_ADD as u16) << 3) | (F3_ADD_SUB as u16);
pub(crate) const F73_SUB: u16 = ((F7_SUB as u16) << 3) | (F3_ADD_SUB as u16);
pub(crate) const F73_SLL: u16 = ((0x0u16) << 3) | (F3_SLL as u16);
pub(crate) const F73_SLT: u16 = ((0x0u16) << 3) | (F3_SLT as u16);
pub(crate) const F73_SLTU: u16 = ((0x0u16) << 3) | (F3_SLTU as u16);
pub(crate) const F73_XOR: u16 = ((0x0u16) << 3) | (F3_XOR as u16);
pub(crate) const F73_SRL: u16 = ((F7_SRL as u16) << 3) | (F3_SRL_SLA as u16);
pub(crate) const F73_SRA: u16 = ((F7_SRA as u16) << 3) | (F3_SRL_SLA as u16);
pub(crate) const F73_OR: u16 = ((0x0u16) << 3) | (F3_OR as u16);
pub(crate) const F73_AND: u16 = ((0x0u16) << 3) | (F3_AND as u16);
pub(crate) const F73_MUL: u16 = ((F7_M_EXTENSION as u16) << 3) | (F3_MUL as u16);
pub(crate) const F73_MULH: u16 = ((F7_M_EXTENSION as u16) << 3) | (F3_MULH as u16);
pub(crate) const F73_MULHSU: u16 = ((F7_M_EXTENSION as u16) << 3) | (F3_MULHSU as u16);
pub(crate) const F73_MULHU: u16 = ((F7_M_EXTENSION as u16) << 3) | (F3_MULHU as u16);
// W instructions are valid for RV64. We're only targeting RV32
// pub(crate) const F73_MULW: u16 = ((F7_M_EXTENSION_W as u16) << 3) | (F3_MULW as u16);

pub(crate) const F73_DIV: u16 = ((F7_M_EXTENSION as u16) << 3) | (F3_DIV as u16);
pub(crate) const F73_DIVU: u16 = ((F7_M_EXTENSION as u16) << 3) | (F3_DIVU as u16);
// W instructions are valid for RV64. We're only targeting RV32
//pub(crate) const F73_DIVW: u16 = ((F7_M_EXTENSION_W as u16) << 3) | (F3_DIVW as u16);

pub(crate) const F73_REM: u16 = ((F7_M_EXTENSION as u16) << 3) | (F3_REM as u16);
pub(crate) const F73_REMU: u16 = ((F7_M_EXTENSION as u16) << 3) | (F3_REMU as u16);
// W instructions are valid for RV64. We're only targeting RV32
//pub(crate) const F73_REMW: u16 = ((F7_M_EXTENSION_W as u16) << 3) | (F3_REMW as u16);

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
#[allow(non_snake_case)]
pub(crate) mod F3 {
    pub const BEQ      : u8 = 0x00;
    pub const BNE      : u8 = 0x01;
    pub const BLT      : u8 = 0x04;
    pub const BGE      : u8 = 0x05;
    pub const BLTU     : u8 = 0x06;
    pub const BGEU     : u8 = 0x07;
    
    pub const LB       : u8 = 0x00;
    pub const LH       : u8 = 0x01;
    pub const LW       : u8 = 0x02;
    pub const LBU      : u8 = 0x04;
    pub const LHU      : u8 = 0x05;
    
    pub const SB       : u8 = 0x00;
    pub const SH       : u8 = 0x01;
    pub const SW       : u8 = 0x02;
    
    pub const ADDI     : u8 = 0x00;
    pub const SLTI     : u8 = 0x02;
    pub const SLTIU    : u8 = 0x03;
    pub const XORI     : u8 = 0x04;
    pub const ORI      : u8 = 0x06;
    pub const ANDI     : u8 = 0x07;
    
    pub const SLLI     : u8 = 0x01;
    pub const SRLI_SRAI: u8 = 0x05; // check bit 30
    
    // We check F7C to discern between ADD and SUB
    pub const ADD_SUB  : u8 = 0x00; // check F7C
    
    // First set of M extension instruction of F3 codes
    pub const MUL      : u8 = 0x00;
    pub const MULH     : u8 = 0x01;
    pub const MULHSU   : u8 = 0x02;
    pub const MULHU    : u8 = 0x03;
    pub const DIV      : u8 = 0x04;
    pub const DIVU     : u8 = 0x05;
    pub const REM      : u8 = 0x06;
    pub const REMU     : u8 = 0x07;
    
    // W set of M extension instruction of F3 codes
    pub const MULW     : u8 = 0x00;
    pub const DIVW     : u8 = 0x04;
    pub const DIVUW    : u8 = 0x05;
    pub const REMW     : u8 = 0x06;
    pub const REMUW    : u8 = 0x07;
    
    // D set of M extension instruction of F3 codes
    pub const MULD     : u8 = 0x00;
    pub const DIVD     : u8 = 0x04;
    pub const DIVUD    : u8 = 0x05;
    pub const REMD     : u8 = 0x06;
    pub const REMUD    : u8 = 0x07;
    
    pub const SLL      : u8 = 0x01;
    pub const SLT      : u8 = 0x02;
    pub const SLTU     : u8 = 0x03;
    pub const XOR      : u8 = 0x04;
    pub const SRL_SLA  : u8 = 0x05; // check F7C
    pub const OR       : u8 = 0x06;
    pub const AND      : u8 = 0x07;
    
    pub const FENCE    : u8 = 0x00;
    pub const FENCE_I  : u8 = 0x01;
    
    pub const ECALL_EBREAK : u8 = 0x00; // check imm[11:0]

    pub const CSRRW  : u8 = 0x01;
    pub const CSRRS  : u8 = 0x02;
    pub const CSRRC  : u8 = 0x03;
    pub const CSRRWI : u8 = 0x05;
    pub const CSRRSI : u8 = 0x06;
    pub const CSRRCI : u8 = 0x07;
}

// Function 7 codes
#[allow(non_snake_case)]
pub(crate) mod F7 {
    pub const SRLI: u8 = 0x00;
    pub const F7_SRAI: u8 = 0x08;

    pub const ADD: u8 = 0x00;
    pub const SUB: u8 = 0x20;

    // These codes are used for every M extension instruction
    pub const M_EXTENSION: u8 = 0x33;
    // W instructions are valid for RV64. We're only targeting RV32 for now
    // pub const M_EXTENSION_W: u8 = 0x3B;

    pub const SRL: u8 = 0x00;
    pub const SRA: u8 = 0x20;
}

pub(crate) const F73_ADD: u16 = ((F7_ADD as u16) << 3) | (F3::ADD_SUB as u16);
pub(crate) const F73_SUB: u16 = ((F7_SUB as u16) << 3) | (F3::ADD_SUB as u16);
pub(crate) const F73_SLL: u16 = ((0x0u16) << 3) | (F3::SLL as u16);
pub(crate) const F73_SLT: u16 = ((0x0u16) << 3) | (F3::SLT as u16);
pub(crate) const F73_SLTU: u16 = ((0x0u16) << 3) | (F3::SLTU as u16);
pub(crate) const F73_XOR: u16 = ((0x0u16) << 3) | (F3::XOR as u16);
pub(crate) const F73_SRL: u16 = ((F7_SRL as u16) << 3) | (F3::SRL_SLA as u16);
pub(crate) const F73_SRA: u16 = ((F7_SRA as u16) << 3) | (F3::SRL_SLA as u16);
pub(crate) const F73_OR: u16 = ((0x0u16) << 3) | (F3::OR as u16);
pub(crate) const F73_AND: u16 = ((0x0u16) << 3) | (F3::AND as u16);
pub(crate) const F73_MUL: u16 = ((F7_M_EXTENSION as u16) << 3) | (F3::MUL as u16);
pub(crate) const F73_MULH: u16 = ((F7_M_EXTENSION as u16) << 3) | (F3::MULH as u16);
pub(crate) const F73_MULHSU: u16 = ((F7_M_EXTENSION as u16) << 3) | (F3::MULHSU as u16);
pub(crate) const F73_MULHU: u16 = ((F7_M_EXTENSION as u16) << 3) | (F3::MULHU as u16);
// W instructions are valid for RV64. We're only targeting RV32
// pub(crate) const F73_MULW: u16 = ((F7_M_EXTENSION_W as u16) << 3) | (F3::MULW as u16);

pub(crate) const F73_DIV: u16 = ((F7_M_EXTENSION as u16) << 3) | (F3::DIV as u16);
pub(crate) const F73_DIVU: u16 = ((F7_M_EXTENSION as u16) << 3) | (F3::DIVU as u16);
// W instructions are valid for RV64. We're only targeting RV32
//pub(crate) const F73_DIVW: u16 = ((F7_M_EXTENSION_W as u16) << 3) | (F3::DIVW as u16);

pub(crate) const F73_REM: u16 = ((F7_M_EXTENSION as u16) << 3) | (F3::REM as u16);
pub(crate) const F73_REMU: u16 = ((F7_M_EXTENSION as u16) << 3) | (F3::REMU as u16);
// W instructions are valid for RV64. We're only targeting RV32
//pub(crate) const F73_REMW: u16 = ((F7_M_EXTENSION_W as u16) << 3) | (F3::REMW as u16);

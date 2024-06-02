mod common;
mod adc;
mod and;
mod asl;
mod bcc;
mod bcs;
mod beq;
mod bit;
mod bmi;
mod bne;
mod bpl;
mod brk;
mod bvc;
mod bvs;
mod clc;
mod cld;
mod cli;
mod clv;
mod cmp;
mod cpx;
mod cpy;
mod dec;
mod dex;
mod dey;
mod eor;
mod inc;
mod inx;
mod iny;
mod jmp;
mod jsr;
mod lda;
mod ldx;
mod ldy;
mod lsr;
mod nop;
mod ora;
mod pha;
mod php;
mod pla;
mod plp;
mod rol;
mod ror;
mod rti;
mod rts;
mod sbc;
mod sec;
mod sed;
mod sei;
mod sta;
mod stx;
mod sty;
mod tax;
mod tay;
mod tsx;
mod txa;
mod txs;
mod tya;
// unofficial instructions
mod slo;
mod sre;
mod lax;
mod lar;
mod kil;
mod isb;
mod dcp;
mod axs;
mod sax;
mod rla;
pub use common::{Inst, InstFun, disassemble};


use std::collections::HashMap;
use lazy_static::lazy_static;

use crate::cpu::addressing_mode::AddressingMode;

struct InstructionInfo {
    name: String,
    f: InstFun,
    opcode_to_addressing_mode: &'static[(u8, AddressingMode)],
}

impl InstructionInfo {
    pub fn new(name: String, f: InstFun, opcode_to_addressing_mode: &'static[(u8, AddressingMode)]) -> Self {
        InstructionInfo {
            name,
            f,
            opcode_to_addressing_mode,
        }
    }
}
pub struct InstFactory {
    pub opcode : u8,
    pub name: String,
    pub mode: AddressingMode,
    pub f: InstFun,
}

impl InstFactory {
    // Create an instruction from the given bytes.
    // Bytes does not include the opcode.
    pub fn make(&self, bytes: &[u8]) -> Inst {
        Inst {
            opcode: self.opcode,
            name: self.name.clone(),
            param: self.mode.read_param(bytes),
            mode: self.mode,
            f: self.f,
        }
    }

    pub fn make2(&self, param: Option<u16>) -> Inst {
        Inst {
            opcode: self.opcode,
            name: self.name.clone(),
            param,
            mode: self.mode,
            f: self.f,
        }
    }
}

macro_rules! instruction_info {
    ($module:ident) => {
        InstructionInfo::new(
            stringify!($module).to_uppercase(),
            crate::instructions::$module::RUN,
            crate::instructions::$module::OPCODE_MAP
        )
    };
}

lazy_static! {
    static ref INSTRUCTIONS: Vec<InstructionInfo> =  vec![
        instruction_info!(adc),
        instruction_info!(and),
        instruction_info!(asl),
        instruction_info!(bcc),
        instruction_info!(bcs),
        instruction_info!(beq),
        instruction_info!(bit),
        instruction_info!(bmi),
        instruction_info!(bne),
        instruction_info!(bpl),
        instruction_info!(brk),
        instruction_info!(bvc),
        instruction_info!(bvs),
        instruction_info!(clc),
        instruction_info!(cld),
        instruction_info!(cli),
        instruction_info!(clv),
        instruction_info!(cmp),
        instruction_info!(cpx),
        instruction_info!(cpy),
        instruction_info!(dec),
        instruction_info!(dex),
        instruction_info!(dey),
        instruction_info!(eor),
        instruction_info!(inc),
        instruction_info!(inx),
        instruction_info!(iny),
        instruction_info!(jmp),
        instruction_info!(jsr),
        instruction_info!(lda),
        instruction_info!(ldx),
        instruction_info!(ldy),
        instruction_info!(lsr),
        instruction_info!(nop),
        instruction_info!(ora),
        instruction_info!(pha),
        instruction_info!(php),
        instruction_info!(pla),
        instruction_info!(plp),
        instruction_info!(rol),
        instruction_info!(ror),
        instruction_info!(rti),
        instruction_info!(rts),
        instruction_info!(sbc),
        instruction_info!(sec),
        instruction_info!(sed),
        instruction_info!(sei),
        instruction_info!(sta),
        instruction_info!(stx),
        instruction_info!(sty),
        instruction_info!(tax),
        instruction_info!(tay),
        instruction_info!(tsx),
        instruction_info!(txa),
        instruction_info!(txs),
        instruction_info!(tya),
        // unofficial instructions
        instruction_info!(slo),
        instruction_info!(sre),
        instruction_info!(lax),
        instruction_info!(lar),
        instruction_info!(kil),
        instruction_info!(isb),
        instruction_info!(dcp),
        instruction_info!(axs),
        instruction_info!(sax),
        instruction_info!(rla),
    ];

    pub static ref INST_FACTORIES_BY_OP_CODE: HashMap<u8, InstFactory> = {
        let mut inst_factory_by_op_code: HashMap<u8, InstFactory> = HashMap::new();
        for info in INSTRUCTIONS.iter() {
            for (op, mode) in info.opcode_to_addressing_mode {
                let res = inst_factory_by_op_code.insert(
                    *op,
                    InstFactory {
                        opcode: *op,
                        mode: *mode,
                        name: info.name.clone(),
                        f: info.f,
                    },
                );
                if res.is_some() {
                    panic!("duplicate op code: {:#x}", op);
                }
            }
        }
        inst_factory_by_op_code
    };

    pub static ref INST_FACTORIES_BY_NAME_MODE: HashMap<(String, AddressingMode), InstFactory> = {
        let mut inst_factory_by_name_mode: HashMap<(String, AddressingMode), InstFactory> = HashMap::new();
        for info in INSTRUCTIONS.iter() {
            for (op, mode) in info.opcode_to_addressing_mode {
                let res = inst_factory_by_name_mode.insert(
                    (info.name.clone(), mode.clone()),
                    InstFactory {
                        opcode: *op,
                        mode: *mode,
                        name: info.name.clone(),
                        f: info.f,
                    },
                );
                if res.is_some() {
                    panic!("duplicate op code: {:#x}", op);
                }
            }
        }
        inst_factory_by_name_mode
    };
    }
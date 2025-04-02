/* Automatically generated by parse_opcodes */
use once_cell::sync::Lazy;
use crate::args::*;

pub struct Spec {
  pub name: String,
  pub mask_bits: u32,
  pub match_bits: u32,
  pub args: Vec<fn(u32)->(Arg, String)>,
}

impl Spec {    
    pub fn new(name: &str, mask_bits: u32, match_bits: u32, args: Vec<fn(u32)->(Arg, String)>) -> Self {
        Self { name: name.to_string(), mask_bits, match_bits, args }
    }

    pub fn compare(&self, code: u32) -> bool {
        (code & self.mask_bits) == self.match_bits
    }
}

pub static RV_ISA_SPECS: Lazy<Vec<Spec>> = Lazy::new(|| vec![
    Spec::new("add", 0xfe00707f, 0x33, vec![rd, rs1, rs2]),
    Spec::new("addi", 0x707f, 0x13, vec![rd, rs1, imm12]),
    Spec::new("addiw", 0x707f, 0x1b, vec![rd, rs1, imm12]),
    Spec::new("addw", 0xfe00707f, 0x3b, vec![rd, rs1, rs2]),
    Spec::new("amoadd.d", 0xf800707f, 0x302f, vec![rd, rs1, rs2, aq, rl]),
    Spec::new("amoadd.w", 0xf800707f, 0x202f, vec![rd, rs1, rs2, aq, rl]),
    Spec::new("amoand.d", 0xf800707f, 0x6000302f, vec![rd, rs1, rs2, aq, rl]),
    Spec::new("amoand.w", 0xf800707f, 0x6000202f, vec![rd, rs1, rs2, aq, rl]),
    Spec::new("amomax.d", 0xf800707f, 0xa000302f, vec![rd, rs1, rs2, aq, rl]),
    Spec::new("amomax.w", 0xf800707f, 0xa000202f, vec![rd, rs1, rs2, aq, rl]),
    Spec::new("amomaxu.d", 0xf800707f, 0xe000302f, vec![rd, rs1, rs2, aq, rl]),
    Spec::new("amomaxu.w", 0xf800707f, 0xe000202f, vec![rd, rs1, rs2, aq, rl]),
    Spec::new("amomin.d", 0xf800707f, 0x8000302f, vec![rd, rs1, rs2, aq, rl]),
    Spec::new("amomin.w", 0xf800707f, 0x8000202f, vec![rd, rs1, rs2, aq, rl]),
    Spec::new("amominu.d", 0xf800707f, 0xc000302f, vec![rd, rs1, rs2, aq, rl]),
    Spec::new("amominu.w", 0xf800707f, 0xc000202f, vec![rd, rs1, rs2, aq, rl]),
    Spec::new("amoor.d", 0xf800707f, 0x4000302f, vec![rd, rs1, rs2, aq, rl]),
    Spec::new("amoor.w", 0xf800707f, 0x4000202f, vec![rd, rs1, rs2, aq, rl]),
    Spec::new("amoswap.d", 0xf800707f, 0x800302f, vec![rd, rs1, rs2, aq, rl]),
    Spec::new("amoswap.w", 0xf800707f, 0x800202f, vec![rd, rs1, rs2, aq, rl]),
    Spec::new("amoxor.d", 0xf800707f, 0x2000302f, vec![rd, rs1, rs2, aq, rl]),
    Spec::new("amoxor.w", 0xf800707f, 0x2000202f, vec![rd, rs1, rs2, aq, rl]),
    Spec::new("and", 0xfe00707f, 0x7033, vec![rd, rs1, rs2]),
    Spec::new("andi", 0x707f, 0x7013, vec![rd, rs1, imm12]),
    Spec::new("auipc", 0x7f, 0x17, vec![rd, imm20]),
    Spec::new("beq", 0x707f, 0x63, vec![bimm12hi, rs1, rs2, bimm12lo]),
    Spec::new("bge", 0x707f, 0x5063, vec![bimm12hi, rs1, rs2, bimm12lo]),
    Spec::new("bgeu", 0x707f, 0x7063, vec![bimm12hi, rs1, rs2, bimm12lo]),
    Spec::new("blt", 0x707f, 0x4063, vec![bimm12hi, rs1, rs2, bimm12lo]),
    Spec::new("bltu", 0x707f, 0x6063, vec![bimm12hi, rs1, rs2, bimm12lo]),
    Spec::new("bne", 0x707f, 0x1063, vec![bimm12hi, rs1, rs2, bimm12lo]),
    Spec::new("c.add", 0xf003, 0x9002, vec![rd_n0, rs1_n0, c_rs2_n0]),
    Spec::new("c.addi", 0xe003, 0x1, vec![rd_n0, rs1_n0, c_nzimm6lo, c_nzimm6hi]),
    Spec::new("c.addi16sp", 0xef83, 0x6101, vec![c_nzimm10hi, c_nzimm10lo]),
    Spec::new("c.addi4spn", 0xe003, 0x0, vec![rd_p, c_nzuimm10]),
    Spec::new("c.addiw", 0xe003, 0x2001, vec![rd_n0, rs1_n0, c_imm6lo, c_imm6hi]),
    Spec::new("c.addw", 0xfc63, 0x9c21, vec![rd_p, rs1_p, rs2_p]),
    Spec::new("c.and", 0xfc63, 0x8c61, vec![rd_p, rs1_p, rs2_p]),
    Spec::new("c.andi", 0xec03, 0x8801, vec![rd_p, rs1_p, c_imm6hi, c_imm6lo]),
    Spec::new("c.beqz", 0xe003, 0xc001, vec![rs1_p, c_bimm9lo, c_bimm9hi]),
    Spec::new("c.bnez", 0xe003, 0xe001, vec![rs1_p, c_bimm9lo, c_bimm9hi]),
    Spec::new("c.ebreak", 0xffff, 0x9002, vec![]),
    Spec::new("c.flw", 0xe003, 0x6000, vec![rd_p, rs1_p, c_uimm7lo, c_uimm7hi]),
    Spec::new("c.flwsp", 0xe003, 0x6002, vec![rd, c_uimm8sphi, c_uimm8splo]),
    Spec::new("c.fsw", 0xe003, 0xe000, vec![rs1_p, rs2_p, c_uimm7lo, c_uimm7hi]),
    Spec::new("c.fswsp", 0xe003, 0xe002, vec![c_rs2, c_uimm8sp_s]),
    Spec::new("c.j", 0xe003, 0xa001, vec![c_imm12]),
    Spec::new("c.jal", 0xe003, 0x2001, vec![c_imm12]),
    Spec::new("c.jalr", 0xf07f, 0x9002, vec![c_rs1_n0]),
    Spec::new("c.jr", 0xf07f, 0x8002, vec![rs1_n0]),
    Spec::new("c.ld", 0xe003, 0x6000, vec![rd_p, rs1_p, c_uimm8lo, c_uimm8hi]),
    Spec::new("c.ldsp", 0xe003, 0x6002, vec![rd_n0, c_uimm9sphi, c_uimm9splo]),
    Spec::new("c.li", 0xe003, 0x4001, vec![rd_n0, c_imm6lo, c_imm6hi]),
    Spec::new("c.lui", 0xe003, 0x6001, vec![rd_n2, c_nzimm18hi, c_nzimm18lo]),
    Spec::new("c.lw", 0xe003, 0x4000, vec![rd_p, rs1_p, c_uimm7lo, c_uimm7hi]),
    Spec::new("c.lwsp", 0xe003, 0x4002, vec![rd_n0, c_uimm8sphi, c_uimm8splo]),
    Spec::new("c.mv", 0xf003, 0x8002, vec![rd_n0, c_rs2_n0]),
    Spec::new("c.nop", 0xef83, 0x1, vec![c_nzimm6hi, c_nzimm6lo]),
    Spec::new("c.or", 0xfc63, 0x8c41, vec![rd_p, rs1_p, rs2_p]),
    Spec::new("c.sd", 0xe003, 0xe000, vec![rs1_p, rs2_p, c_uimm8hi, c_uimm8lo]),
    Spec::new("c.sdsp", 0xe003, 0xe002, vec![c_rs2, c_uimm9sp_s]),
    Spec::new("c.slli", 0xe003, 0x2, vec![rd_n0, rs1_n0, c_nzuimm6hi, c_nzuimm6lo]),
    Spec::new("c.srai", 0xec03, 0x8401, vec![rd_p, rs1_p, c_nzuimm6lo, c_nzuimm6hi]),
    Spec::new("c.srli", 0xec03, 0x8001, vec![rd_p, rs1_p, c_nzuimm6lo, c_nzuimm6hi]),
    Spec::new("c.sub", 0xfc63, 0x8c01, vec![rd_p, rs1_p, rs2_p]),
    Spec::new("c.subw", 0xfc63, 0x9c01, vec![rd_p, rs1_p, rs2_p]),
    Spec::new("c.sw", 0xe003, 0xc000, vec![rs1_p, rs2_p, c_uimm7lo, c_uimm7hi]),
    Spec::new("c.swsp", 0xe003, 0xc002, vec![c_rs2, c_uimm8sp_s]),
    Spec::new("c.xor", 0xfc63, 0x8c21, vec![rd_p, rs1_p, rs2_p]),
    Spec::new("csrrc", 0x707f, 0x3073, vec![rd, rs1, csr]),
    Spec::new("csrrci", 0x707f, 0x7073, vec![rd, csr, zimm5]),
    Spec::new("csrrs", 0x707f, 0x2073, vec![rd, rs1, csr]),
    Spec::new("csrrsi", 0x707f, 0x6073, vec![rd, csr, zimm5]),
    Spec::new("csrrw", 0x707f, 0x1073, vec![rd, rs1, csr]),
    Spec::new("csrrwi", 0x707f, 0x5073, vec![rd, csr, zimm5]),
    Spec::new("div", 0xfe00707f, 0x2004033, vec![rd, rs1, rs2]),
    Spec::new("divu", 0xfe00707f, 0x2005033, vec![rd, rs1, rs2]),
    Spec::new("divuw", 0xfe00707f, 0x200503b, vec![rd, rs1, rs2]),
    Spec::new("divw", 0xfe00707f, 0x200403b, vec![rd, rs1, rs2]),
    Spec::new("ebreak", 0xffffffff, 0x100073, vec![]),
    Spec::new("ecall", 0xffffffff, 0x73, vec![]),
    Spec::new("fadd.s", 0xfe00007f, 0x53, vec![rd_f, rs1_f, rs2_f, rm]),
    Spec::new("fclass.s", 0xfff0707f, 0xe0001053, vec![rd_f, rs1_f]),
    Spec::new("fcvt.l.s", 0xfff0007f, 0xc0200053, vec![rd, rs1, rm]),
    Spec::new("fcvt.lu.s", 0xfff0007f, 0xc0300053, vec![rd, rs1, rm]),
    Spec::new("fcvt.s.l", 0xfff0007f, 0xd0200053, vec![rd, rs1, rm]),
    Spec::new("fcvt.s.lu", 0xfff0007f, 0xd0300053, vec![rd, rs1, rm]),
    Spec::new("fcvt.s.w", 0xfff0007f, 0xd0000053, vec![rd_f, rs1_f, rm]),
    Spec::new("fcvt.s.wu", 0xfff0007f, 0xd0100053, vec![rd_f, rs1_f, rm]),
    Spec::new("fcvt.w.s", 0xfff0007f, 0xc0000053, vec![rd_f, rs1_f, rm]),
    Spec::new("fcvt.wu.s", 0xfff0007f, 0xc0100053, vec![rd_f, rs1_f, rm]),
    Spec::new("fdiv.s", 0xfe00007f, 0x18000053, vec![rd_f, rs1_f, rs2_f, rm]),
    Spec::new("fence", 0x707f, 0xf, vec![fm, pred, succ, rs1, rd]),
    Spec::new("feq.s", 0xfe00707f, 0xa0002053, vec![rd_f, rs1_f, rs2_f]),
    Spec::new("fle.s", 0xfe00707f, 0xa0000053, vec![rd_f, rs1_f, rs2_f]),
    Spec::new("flt.s", 0xfe00707f, 0xa0001053, vec![rd_f, rs1_f, rs2_f]),
    Spec::new("flw", 0x707f, 0x2007, vec![rd_f, rs1_f, imm12]),
    Spec::new("fmadd.s", 0x600007f, 0x43, vec![rd_f, rs1_f, rs2_f, rs3_f, rm]),
    Spec::new("fmax.s", 0xfe00707f, 0x28001053, vec![rd_f, rs1_f, rs2_f]),
    Spec::new("fmin.s", 0xfe00707f, 0x28000053, vec![rd_f, rs1_f, rs2_f]),
    Spec::new("fmsub.s", 0x600007f, 0x47, vec![rd_f, rs1_f, rs2_f, rs3_f, rm]),
    Spec::new("fmul.s", 0xfe00007f, 0x10000053, vec![rd_f, rs1_f, rs2_f, rm]),
    Spec::new("fmv.w.x", 0xfff0707f, 0xf0000053, vec![rd_f, rs1_f]),
    Spec::new("fmv.x.w", 0xfff0707f, 0xe0000053, vec![rd_f, rs1_f]),
    Spec::new("fnmadd.s", 0x600007f, 0x4f, vec![rd_f, rs1_f, rs2_f, rs3_f, rm]),
    Spec::new("fnmsub.s", 0x600007f, 0x4b, vec![rd_f, rs1_f, rs2_f, rs3_f, rm]),
    Spec::new("fsgnj.s", 0xfe00707f, 0x20000053, vec![rd_f, rs1_f, rs2_f]),
    Spec::new("fsgnjn.s", 0xfe00707f, 0x20001053, vec![rd_f, rs1_f, rs2_f]),
    Spec::new("fsgnjx.s", 0xfe00707f, 0x20002053, vec![rd_f, rs1_f, rs2_f]),
    Spec::new("fsqrt.s", 0xfff0007f, 0x58000053, vec![rd_f, rs1_f, rm]),
    Spec::new("fsub.s", 0xfe00007f, 0x8000053, vec![rd_f, rs1_f, rs2_f, rm]),
    Spec::new("fsw", 0x707f, 0x2027, vec![imm12hi, rs1_f, rs2_f, imm12lo]),
    Spec::new("jal", 0x7f, 0x6f, vec![rd, jimm20]),
    Spec::new("jalr", 0x707f, 0x67, vec![rd, rs1, imm12]),
    Spec::new("lb", 0x707f, 0x3, vec![rd, rs1, imm12]),
    Spec::new("lbu", 0x707f, 0x4003, vec![rd, rs1, imm12]),
    Spec::new("ld", 0x707f, 0x3003, vec![rd, rs1, imm12]),
    Spec::new("lh", 0x707f, 0x1003, vec![rd, rs1, imm12]),
    Spec::new("lhu", 0x707f, 0x5003, vec![rd, rs1, imm12]),
    Spec::new("lr.d", 0xf9f0707f, 0x1000302f, vec![rd, rs1, aq, rl]),
    Spec::new("lr.w", 0xf9f0707f, 0x1000202f, vec![rd, rs1, aq, rl]),
    Spec::new("lui", 0x7f, 0x37, vec![rd, imm20]),
    Spec::new("lw", 0x707f, 0x2003, vec![rd, rs1, imm12]),
    Spec::new("lwu", 0x707f, 0x6003, vec![rd, rs1, imm12]),
    Spec::new("mul", 0xfe00707f, 0x2000033, vec![rd, rs1, rs2]),
    Spec::new("mulh", 0xfe00707f, 0x2001033, vec![rd, rs1, rs2]),
    Spec::new("mulhsu", 0xfe00707f, 0x2002033, vec![rd, rs1, rs2]),
    Spec::new("mulhu", 0xfe00707f, 0x2003033, vec![rd, rs1, rs2]),
    Spec::new("mulw", 0xfe00707f, 0x200003b, vec![rd, rs1, rs2]),
    Spec::new("or", 0xfe00707f, 0x6033, vec![rd, rs1, rs2]),
    Spec::new("ori", 0x707f, 0x6013, vec![rd, rs1, imm12]),
    Spec::new("rem", 0xfe00707f, 0x2006033, vec![rd, rs1, rs2]),
    Spec::new("remu", 0xfe00707f, 0x2007033, vec![rd, rs1, rs2]),
    Spec::new("remuw", 0xfe00707f, 0x200703b, vec![rd, rs1, rs2]),
    Spec::new("remw", 0xfe00707f, 0x200603b, vec![rd, rs1, rs2]),
    Spec::new("sb", 0x707f, 0x23, vec![imm12hi, rs1, rs2, imm12lo]),
    Spec::new("sc.d", 0xf800707f, 0x1800302f, vec![rd, rs1, rs2, aq, rl]),
    Spec::new("sc.w", 0xf800707f, 0x1800202f, vec![rd, rs1, rs2, aq, rl]),
    Spec::new("sd", 0x707f, 0x3023, vec![imm12hi, rs1, rs2, imm12lo]),
    Spec::new("sh", 0x707f, 0x1023, vec![imm12hi, rs1, rs2, imm12lo]),
    Spec::new("sll", 0xfe00707f, 0x1033, vec![rd, rs1, rs2]),
    Spec::new("slli", 0xfc00707f, 0x1013, vec![rd, rs1, shamtd]),
    Spec::new("slliw", 0xfe00707f, 0x101b, vec![rd, rs1, shamtw]),
    Spec::new("sllw", 0xfe00707f, 0x103b, vec![rd, rs1, rs2]),
    Spec::new("slt", 0xfe00707f, 0x2033, vec![rd, rs1, rs2]),
    Spec::new("slti", 0x707f, 0x2013, vec![rd, rs1, imm12]),
    Spec::new("sltiu", 0x707f, 0x3013, vec![rd, rs1, imm12]),
    Spec::new("sltu", 0xfe00707f, 0x3033, vec![rd, rs1, rs2]),
    Spec::new("sra", 0xfe00707f, 0x40005033, vec![rd, rs1, rs2]),
    Spec::new("srai", 0xfc00707f, 0x40005013, vec![rd, rs1, shamtd]),
    Spec::new("sraiw", 0xfe00707f, 0x4000501b, vec![rd, rs1, shamtw]),
    Spec::new("sraw", 0xfe00707f, 0x4000503b, vec![rd, rs1, rs2]),
    Spec::new("srl", 0xfe00707f, 0x5033, vec![rd, rs1, rs2]),
    Spec::new("srli", 0xfc00707f, 0x5013, vec![rd, rs1, shamtd]),
    Spec::new("srliw", 0xfe00707f, 0x501b, vec![rd, rs1, shamtw]),
    Spec::new("srlw", 0xfe00707f, 0x503b, vec![rd, rs1, rs2]),
    Spec::new("sub", 0xfe00707f, 0x40000033, vec![rd, rs1, rs2]),
    Spec::new("subw", 0xfe00707f, 0x4000003b, vec![rd, rs1, rs2]),
    Spec::new("sw", 0x707f, 0x2023, vec![imm12hi, rs1, rs2, imm12lo]),
    Spec::new("xor", 0xfe00707f, 0x4033, vec![rd, rs1, rs2]),
    Spec::new("xori", 0x707f, 0x4013, vec![rd, rs1, imm12]),
]);

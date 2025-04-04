// tests for the disassembler
use riscv_disasm::disassembler::Disassembler;

#[test]
fn test_decode_one() {
    let disassembler = Disassembler::new();
    let code = 0x00000293;
    let insn = disassembler.disassmeble_one(code);
    println!("{:?}", insn);
}
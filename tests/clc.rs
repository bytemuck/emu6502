use emu6502::*;

#[test]
fn clc_implied() {
    let mut processor = Processor::new();

    processor.registers.set_carry(true);

    processor.memory[0xFFFC] = CLC_IMPLIED;

    let expected_cycles = 2;
    let used_cycles = processor.execute(expected_cycles);

    assert_eq!(used_cycles, expected_cycles);

    assert_eq!(processor.registers.get_carry(), false);
}

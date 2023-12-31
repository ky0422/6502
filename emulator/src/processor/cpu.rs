use crate::{
    addressing_mode::AddressingMode,
    memory::{MemoryBus, STACK_BASE},
    registers::Registers,
    CpuDebugger, DebugKind, Debugger, NoneDebugger,
};
use std::fmt;

#[doc=include_str!("../../../README.md")]
#[derive(Default)]
pub struct Cpu<T, D, R>
where
    T: MemoryBus<Data = u8, Addr = u16>,
    D: Debugger,
    R: Debugger,
{
    pub memory: T,
    pub debugger: D,
    pub registers: Registers<R>,
}

pub type NoneDebuggerCpu<T> = Cpu<T, NoneDebugger, NoneDebugger>;

impl<T, D, R> fmt::Display for Cpu<T, D, R>
where
    T: MemoryBus<Data = u8, Addr = u16>,
    D: Debugger,
    R: Debugger,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.registers)
    }
}

impl<T, D, R> Cpu<T, D, R>
where
    T: MemoryBus<Data = u8, Addr = u16> + Default,
    D: Debugger,
    R: Debugger,
{
    pub fn new(memory: T) -> Cpu<T, D, R> {
        Cpu {
            registers: Registers::default(),
            memory,
            debugger: D::default(),
        }
    }

    pub fn debug(&mut self, message: &str) {
        self.debugger.debug(message, DebugKind::Info);
    }

    pub fn reset(&mut self) {
        self.registers.reset();
        self.memory.reset();
        self.debug("Reset CPU");
    }

    pub fn load(&mut self, program: &[T::Data]) {
        self.memory.rom(program);
    }

    pub fn execute(&mut self) {
        loop {
            let opcode = self.memory.read(self.registers.pc);

            self.debug(&format!(
                "Execute 0x{:02X} at 0x{:04X}",
                opcode, self.registers.pc
            ));

            self.execute_instruction(opcode);

            if opcode == 0x00 {
                break;
            }
        }

        self.debug("Program finished");
    }

    fn execute_instruction(&mut self, opcode: u8) {
        self.registers.pc += 1;
        match opcode {
            // ADC
            0x69 => self.adc(AddressingMode::Immediate),
            0x65 => self.adc(AddressingMode::ZeroPage),
            0x75 => self.adc(AddressingMode::ZeroPageX),
            0x6D => self.adc(AddressingMode::Absolute),
            0x7D => self.adc(AddressingMode::AbsoluteX),
            0x79 => self.adc(AddressingMode::AbsoluteY),
            0x61 => self.adc(AddressingMode::IndirectX),
            0x71 => self.adc(AddressingMode::IndirectY),

            // AND
            0x29 => self.and(AddressingMode::Immediate),
            0x25 => self.and(AddressingMode::ZeroPage),
            0x35 => self.and(AddressingMode::ZeroPageX),
            0x2D => self.and(AddressingMode::Absolute),
            0x3D => self.and(AddressingMode::AbsoluteX),
            0x39 => self.and(AddressingMode::AbsoluteY),
            0x21 => self.and(AddressingMode::IndirectX),
            0x31 => self.and(AddressingMode::IndirectY),

            // ASL
            0x0A => self.asl(None), // Accumulator
            0x06 => self.asl(Some(AddressingMode::ZeroPage)),
            0x16 => self.asl(Some(AddressingMode::ZeroPageX)),
            0x0E => self.asl(Some(AddressingMode::Absolute)),
            0x1E => self.asl(Some(AddressingMode::AbsoluteX)),

            /* BCC */ 0x90 => self.bcc(),
            /* BCS */ 0xB0 => self.bcs(),
            /* BEQ */ 0xF0 => self.beq(),

            // BIT
            0x24 => self.bit(AddressingMode::ZeroPage),
            0x2C => self.bit(AddressingMode::Absolute),

            /* BMI */ 0x30 => self.bmi(),
            /* BNE */ 0xD0 => self.bne(),
            /* BPL */ 0x10 => self.bpl(),
            /* BVC */ 0x50 => self.bvc(),
            /* BVS */ 0x70 => self.bvs(),
            /* CLC */ 0x18 => self.clc(),
            /* CLD */ 0xD8 => self.cld(),
            /* CLI */ 0x58 => self.cli(),
            /* CLV */ 0xB8 => self.clv(),

            // CMP
            0xC9 => self.cmp(AddressingMode::Immediate),
            0xC5 => self.cmp(AddressingMode::ZeroPage),
            0xD5 => self.cmp(AddressingMode::ZeroPageX),
            0xCD => self.cmp(AddressingMode::Absolute),
            0xDD => self.cmp(AddressingMode::AbsoluteX),
            0xD9 => self.cmp(AddressingMode::AbsoluteY),
            0xC1 => self.cmp(AddressingMode::IndirectX),
            0xD1 => self.cmp(AddressingMode::IndirectY),

            // CPX
            0xE0 => self.cpx(AddressingMode::Immediate),
            0xE4 => self.cpx(AddressingMode::ZeroPage),
            0xEC => self.cpx(AddressingMode::Absolute),

            // CPY
            0xC0 => self.cpy(AddressingMode::Immediate),
            0xC4 => self.cpy(AddressingMode::ZeroPage),
            0xCC => self.cpy(AddressingMode::Absolute),

            // DEC
            0xC6 => self.dec(AddressingMode::ZeroPage),
            0xD6 => self.dec(AddressingMode::ZeroPageX),
            0xCE => self.dec(AddressingMode::Absolute),
            0xDE => self.dec(AddressingMode::AbsoluteX),

            /* DEX */ 0xCA => self.dex(),
            /* DEY */ 0x88 => self.dey(),

            // EOR
            0x49 => self.eor(AddressingMode::Immediate),
            0x45 => self.eor(AddressingMode::ZeroPage),
            0x55 => self.eor(AddressingMode::ZeroPageX),
            0x4D => self.eor(AddressingMode::Absolute),
            0x5D => self.eor(AddressingMode::AbsoluteX),
            0x59 => self.eor(AddressingMode::AbsoluteY),
            0x41 => self.eor(AddressingMode::IndirectX),
            0x51 => self.eor(AddressingMode::IndirectY),

            // INC
            0xE6 => self.inc(AddressingMode::ZeroPage),
            0xF6 => self.inc(AddressingMode::ZeroPageX),
            0xEE => self.inc(AddressingMode::Absolute),
            0xFE => self.inc(AddressingMode::AbsoluteX),

            /* INX */ 0xE8 => self.inx(),
            /* INY */ 0xC8 => self.iny(),

            // JMP
            0x4C => self.jmp(AddressingMode::Absolute),
            0x6C => self.jmp(AddressingMode::Indirect),

            /* JSR */ 0x20 => self.jsr(),

            // LDA
            0xA9 => self.lda(AddressingMode::Immediate),
            0xA5 => self.lda(AddressingMode::ZeroPage),
            0xB5 => self.lda(AddressingMode::ZeroPageX),
            0xAD => self.lda(AddressingMode::Absolute),
            0xBD => self.lda(AddressingMode::AbsoluteX),
            0xB9 => self.lda(AddressingMode::AbsoluteY),
            0xA1 => self.lda(AddressingMode::IndirectX),
            0xB1 => self.lda(AddressingMode::IndirectY),

            // LDX
            0xA2 => self.ldx(AddressingMode::Immediate),
            0xA6 => self.ldx(AddressingMode::ZeroPage),
            0xB6 => self.ldx(AddressingMode::ZeroPageY),
            0xAE => self.ldx(AddressingMode::Absolute),
            0xBE => self.ldx(AddressingMode::AbsoluteY),

            // LDY
            0xA0 => self.ldy(AddressingMode::Immediate),
            0xA4 => self.ldy(AddressingMode::ZeroPage),
            0xB4 => self.ldy(AddressingMode::ZeroPageX),
            0xAC => self.ldy(AddressingMode::Absolute),
            0xBC => self.ldy(AddressingMode::AbsoluteX),

            // LSR
            0x4A => self.lsr(None),
            0x46 => self.lsr(Some(AddressingMode::ZeroPage)),
            0x56 => self.lsr(Some(AddressingMode::ZeroPageX)),
            0x4E => self.lsr(Some(AddressingMode::Absolute)),
            0x5E => self.lsr(Some(AddressingMode::AbsoluteX)),

            /* NOP */ 0xEA => {}

            // ORA
            0x09 => self.ora(AddressingMode::Immediate),
            0x05 => self.ora(AddressingMode::ZeroPage),
            0x15 => self.ora(AddressingMode::ZeroPageX),
            0x0D => self.ora(AddressingMode::Absolute),
            0x1D => self.ora(AddressingMode::AbsoluteX),
            0x19 => self.ora(AddressingMode::AbsoluteY),
            0x01 => self.ora(AddressingMode::IndirectX),
            0x11 => self.ora(AddressingMode::IndirectY),

            /* PHA */ 0x48 => self.pha(),
            /* PHP */ 0x08 => self.php(),
            /* PLA */ 0x68 => self.pla(),
            /* PLP */ 0x28 => self.plp(),

            // ROL
            0x2A => self.rol(None),
            0x26 => self.rol(Some(AddressingMode::ZeroPage)),
            0x36 => self.rol(Some(AddressingMode::ZeroPageX)),
            0x2E => self.rol(Some(AddressingMode::Absolute)),
            0x3E => self.rol(Some(AddressingMode::AbsoluteX)),

            // ROR
            0x6A => self.ror(None),
            0x66 => self.ror(Some(AddressingMode::ZeroPage)),
            0x76 => self.ror(Some(AddressingMode::ZeroPageX)),
            0x6E => self.ror(Some(AddressingMode::Absolute)),
            0x7E => self.ror(Some(AddressingMode::AbsoluteX)),

            /* RTI */ 0x40 => self.rti(),
            /* RTS */ 0x60 => self.rts(),

            // SBC
            0xE9 => self.sbc(AddressingMode::Immediate),
            0xE5 => self.sbc(AddressingMode::ZeroPage),
            0xF5 => self.sbc(AddressingMode::ZeroPageX),
            0xED => self.sbc(AddressingMode::Absolute),
            0xFD => self.sbc(AddressingMode::AbsoluteX),
            0xF9 => self.sbc(AddressingMode::AbsoluteY),
            0xE1 => self.sbc(AddressingMode::IndirectX),
            0xF1 => self.sbc(AddressingMode::IndirectY),

            /* SEC */ 0x38 => self.sec(),
            /* SED */ 0xF8 => self.sed(),
            /* SEI */ 0x78 => self.sei(),

            // STA
            0x85 => self.sta(AddressingMode::ZeroPage),
            0x95 => self.sta(AddressingMode::ZeroPageX),
            0x8D => self.sta(AddressingMode::Absolute),
            0x9D => self.sta(AddressingMode::AbsoluteX),
            0x99 => self.sta(AddressingMode::AbsoluteY),
            0x81 => self.sta(AddressingMode::IndirectX),
            0x91 => self.sta(AddressingMode::IndirectY),

            // STX
            0x86 => self.stx(AddressingMode::ZeroPage),
            0x96 => self.stx(AddressingMode::ZeroPageY),
            0x8E => self.stx(AddressingMode::Absolute),

            // STY
            0x84 => self.sty(AddressingMode::ZeroPage),
            0x94 => self.sty(AddressingMode::ZeroPageX),
            0x8C => self.sty(AddressingMode::Absolute),

            /* TAX */ 0xAA => self.tax(),
            /* TAY */ 0xA8 => self.tay(),
            /* TSX */ 0xBA => self.tsx(),
            /* TXA */ 0x8A => self.txa(),
            /* TXS */ 0x9A => self.txs(),
            /* TYA */ 0x98 => self.tya(),

            /* BRK */ 0x00 => {}
            /* NOP */
            _ => self.debugger.debug(
                &format!("Unknown opcode: 0x{:02X}", opcode),
                DebugKind::Warn,
            ),
        }
    }

    fn stack_push(&mut self, data: T::Data) {
        self.memory
            .write(STACK_BASE + self.registers.sp as T::Addr, data);
        self.registers.sp = self.registers.sp.wrapping_sub(1);

        self.debug(&format!("Stack push 0x{:02X}", data));
    }

    fn stack_pop(&mut self) -> T::Data {
        self.registers.sp = self.registers.sp.wrapping_add(1);
        let data = self.memory.read(STACK_BASE + self.registers.sp as T::Addr);

        self.debug(&format!("Stack pop 0x{:02X}", data));
        data
    }

    fn stack_push_addr(&mut self, data: T::Addr) {
        let [lsb, msb] = data.to_le_bytes();

        self.stack_push(msb);
        self.stack_push(lsb);

        self.debug(&format!("Stack push 0x{:04X}", data));
    }

    fn stack_pop_addr(&mut self) -> T::Addr {
        let lsb = self.stack_pop();
        let msb = self.stack_pop();
        let data = T::Addr::from_le_bytes([lsb, msb]);

        self.debug(&format!("Stack pop 0x{:04X}", data));
        data
    }

    fn get_address_from_mode(&mut self, mode: AddressingMode) -> T::Addr {
        self.debug(&format!("Addressing mode {:?}", mode));

        match mode {
            AddressingMode::Immediate => {
                let data = self.registers.pc;
                self.registers.pc += 1;

                data
            }
            AddressingMode::Absolute => {
                let data = self.memory.read_addr(self.registers.pc);
                self.registers.pc += 2;

                data
            }
            AddressingMode::AbsoluteX => {
                let base = self.memory.read_addr(self.registers.pc);
                self.registers.pc += 2;

                base + self.registers.x as T::Addr
            }
            AddressingMode::AbsoluteY => {
                let base = self.memory.read_addr(self.registers.pc);
                self.registers.pc += 2;

                base + self.registers.y as T::Addr
            }
            AddressingMode::Indirect => {
                let ptr = self.memory.read_addr(self.registers.pc);
                self.registers.pc += 2;

                self.memory.read_addr(ptr)
            }
            AddressingMode::IndirectX => {
                let base = self.memory.read(self.registers.pc);
                self.registers.pc += 1;

                let ptr = base.wrapping_add(self.registers.x);
                let data = self.memory.read_addr(ptr as T::Addr);
                self.registers.pc += 2;

                data
            }
            AddressingMode::IndirectY => {
                let ptr = self.memory.read(self.registers.pc);
                self.registers.pc += 1;

                let data = self.memory.read_addr(ptr as T::Addr);
                self.registers.pc += 2;

                data + self.registers.y as T::Addr
            }
            AddressingMode::ZeroPage => {
                let data = self.memory.read(self.registers.pc);
                self.registers.pc += 1;

                data as T::Addr
            }
            AddressingMode::ZeroPageX => {
                let data = self.memory.read(self.registers.pc);
                self.registers.pc += 1;

                data.wrapping_add(self.registers.x) as T::Addr
            }
            AddressingMode::ZeroPageY => {
                let data = self.memory.read(self.registers.pc);
                self.registers.pc += 1;

                data.wrapping_add(self.registers.y) as T::Addr
            }
        }
    }

    fn get_data_from_addressing_mode(&mut self, mode: AddressingMode) -> T::Data {
        let address = self.get_address_from_mode(mode);
        self.memory.read(address)
    }

    fn add_to_accumulator_with_carry(&mut self, data: T::Data) {
        let sum = if self.registers.get_flag_carry() {
            self.registers.a as T::Addr + data as T::Addr + 1
        } else {
            self.registers.a as T::Addr + data as T::Addr
        };

        // Carry flag
        self.registers.set_flag_carry(sum > 0xFF);

        let sum = sum as T::Data;

        // Overflow flag
        self.registers
            .set_flag_overflow((self.registers.a ^ sum) & (data ^ sum) & 0x80 != 0);

        self.registers.set_zero_negative_flags(sum);

        self.registers.a = sum;
    }

    fn branch(&mut self) {
        let offset = self.memory.read(self.registers.pc) as i8;
        self.registers.pc += 1;

        let pc = self.registers.pc as T::Addr;
        self.registers.pc = pc.wrapping_add(offset as T::Addr);

        self.debug(&format!("Branch to 0x{:04X}", self.registers.pc));
    }

    /// ## ADC (Add with Carry)
    ///
    /// Add Memory to Accumulator with Carry
    ///
    /// `A + M + C -> A, C`, Flags affected: `N` `V` `Z` `C`
    fn adc(&mut self, mode: AddressingMode) {
        let data = self.get_data_from_addressing_mode(mode);
        self.add_to_accumulator_with_carry(data);
    }

    /// ## AND (Logical AND)
    ///
    /// AND Memory with Accumulator
    ///
    /// `A AND M -> A`, Flags affected: `N` `Z`
    fn and(&mut self, mode: AddressingMode) {
        let data = self.get_data_from_addressing_mode(mode);
        self.registers.a &= data;

        self.registers.set_zero_negative_flags(self.registers.a);
    }

    /// ## ASL (Arithmetic Shift Left)
    ///
    /// Shift Left One Bit (Memory or Accumulator)
    ///
    /// `C <- [76543210] <- 0`, Flags affected: `N` `Z` `C`
    fn asl(&mut self, mode: Option<AddressingMode>) {
        let mut data = match mode {
            Some(mode) => self.get_data_from_addressing_mode(mode),
            None => self.registers.a,
        };

        self.registers.set_flag_carry(data & 0x80 != 0);

        data <<= 1;

        self.registers.set_zero_negative_flags(data);

        if let Some(mode) = mode {
            let address = self.get_address_from_mode(mode);
            self.memory.write(address, data);
        } else {
            self.registers.a = data;
        }
    }

    /// ## BCC (Branch if Carry Clear)
    ///
    /// Branch on Carry Clear
    ///
    /// `branch on C = 0`, Flags affected: None
    fn bcc(&mut self) {
        if !self.registers.get_flag_carry() {
            self.branch();
        } else {
            self.registers.pc += 1;
        }
    }

    /// ## BCS (Branch if Carry Set)
    ///
    /// Branch on Carry Set
    ///
    /// `branch on C = 1`, Flags affected: None
    fn bcs(&mut self) {
        if self.registers.get_flag_carry() {
            self.branch();
        } else {
            self.registers.pc += 1;
        }
    }

    /// ## BEQ (Branch if Equal)
    ///
    /// Branch on Result Zero
    ///
    /// `branch on Z = 1`, Flags affected: None
    fn beq(&mut self) {
        if self.registers.get_flag_zero() {
            self.branch();
        } else {
            self.registers.pc += 1;
        }
    }

    /// ## BIT (Bit Test)
    ///
    /// Test Bits in Memory with Accumulator
    ///
    /// `A AND M, M7 -> N, M6 -> V`, Flags affected: `N` `V` `Z`
    fn bit(&mut self, mode: AddressingMode) {
        let data = self.get_data_from_addressing_mode(mode);
        let result = self.registers.a & data;

        self.registers.set_flag_negative(data & 0x80 != 0);
        self.registers.set_flag_overflow(data & 0x40 != 0);
        self.registers.set_flag_zero(result == 0);
    }

    /// ## BMI (Branch if Minus)
    ///
    /// Branch on Result Minus
    ///
    /// `branch on N = 1`, Flags affected: None
    fn bmi(&mut self) {
        if self.registers.get_flag_negative() {
            self.branch();
        } else {
            self.registers.pc += 1;
        }
    }

    /// ## BNE (Branch if Not Equal)
    ///
    /// Branch on Result not Zero
    ///
    /// `branch on Z = 0`, Flags affected: None
    fn bne(&mut self) {
        if !self.registers.get_flag_zero() {
            self.branch();
        } else {
            self.registers.pc += 1;
        }
    }

    /// ## BPL (Branch if Plus)
    ///
    /// Branch on Result Plus
    ///
    /// `branch on N = 0`, Flags affected: None
    fn bpl(&mut self) {
        if !self.registers.get_flag_negative() {
            self.branch();
        } else {
            self.registers.pc += 1;
        }
    }

    /// ## BVC (Branch if Overflow Clear)
    ///
    /// Branch on Overflow Clear
    ///
    /// `branch on V = 0`, Flags affected: None
    fn bvc(&mut self) {
        if !self.registers.get_flag_overflow() {
            self.branch();
        } else {
            self.registers.pc += 1;
        }
    }

    /// ## BVS (Branch if Overflow Set)
    ///
    /// Branch on Overflow Set
    ///
    /// `branch on V = 1`, Flags affected: None
    fn bvs(&mut self) {
        if self.registers.get_flag_overflow() {
            self.branch();
        } else {
            self.registers.pc += 1;
        }
    }

    /// ## CLC (Clear Carry Flag)
    ///
    /// Clear Carry Flag
    ///
    /// `0 -> C`, Flags affected: `C`
    fn clc(&mut self) {
        self.registers.set_flag_carry(false);
    }

    /// ## CLD (Clear Decimal Mode)
    ///
    /// Clear Decimal Mode
    ///
    /// `0 -> D`, Flags affected: `D`
    fn cld(&mut self) {
        self.registers.set_flag_decimal(false);
    }

    /// ## CLI (Clear Interrupt Disable)
    ///
    /// Clear Interrupt Disable Bit
    ///
    /// `0 -> I`, Flags affected: `I`
    fn cli(&mut self) {
        self.registers.set_flag_interrupt_disable(false);
    }

    /// ## CLV (Clear Overflow Flag)
    ///
    /// Clear Overflow Flag
    ///
    /// `0 -> V`, Flags affected: `V`
    fn clv(&mut self) {
        self.registers.set_flag_overflow(false);
    }

    /// ## CMP (Compare Memory with Accumulator)
    ///
    /// Compare Memory with Accumulator
    ///
    /// `A - M`, Flags affected: `N` `Z` `C`
    fn cmp(&mut self, mode: AddressingMode) {
        let data = self.get_data_from_addressing_mode(mode);
        let result = self.registers.a.wrapping_sub(data);
        self.registers.set_zero_negative_flags(result);
        self.registers.set_flag_carry(self.registers.a >= data);
    }

    /// ## CPX (Compare Memory and Index X)
    ///
    /// Compare Memory and Index X
    ///
    /// `X - M`, Flags affected: `N` `Z` `C`
    fn cpx(&mut self, mode: AddressingMode) {
        let data = self.get_data_from_addressing_mode(mode);
        let result = self.registers.x.wrapping_sub(data);
        self.registers.set_zero_negative_flags(result);
        self.registers.set_flag_carry(self.registers.x >= data);
    }

    /// ## CPY (Compare Memory and Index Y)
    ///
    /// Compare Memory and Index Y
    ///
    /// `Y - M`, Flags affected: `N` `Z` `C`
    fn cpy(&mut self, mode: AddressingMode) {
        let data = self.get_data_from_addressing_mode(mode);
        let result = self.registers.y.wrapping_sub(data);
        self.registers.set_zero_negative_flags(result);
        self.registers.set_flag_carry(self.registers.y >= data);
    }

    /// ## DEC (Decrement Memory by One)
    ///
    /// Decrement Memory by One
    ///
    /// `M - 1 -> M`, Flags affected: `N` `Z`
    fn dec(&mut self, mode: AddressingMode) {
        let addr = self.get_address_from_mode(mode);
        let mut data = self.memory.read(addr);
        data = data.wrapping_sub(1);
        self.memory.write(addr, data);
        self.registers.set_zero_negative_flags(data);
    }

    /// ## DEX (Decrement Index X by One)
    ///
    /// Decrement Index X by One
    ///
    /// `X - 1 -> X`, Flags affected: `N` `Z`
    fn dex(&mut self) {
        self.registers.x = self.registers.x.wrapping_sub(1);
        self.registers.set_zero_negative_flags(self.registers.x);
    }

    /// ## DEY (Decrement Index Y by One)
    ///
    /// Decrement Index Y by One
    ///
    /// `Y - 1 -> Y`, Flags affected: `N` `Z`
    fn dey(&mut self) {
        self.registers.y = self.registers.y.wrapping_sub(1);
        self.registers.set_zero_negative_flags(self.registers.y);
    }

    /// ## EOR (Exclusive OR Memory with Accumulator)
    ///
    /// Exclusive OR Memory with Accumulator
    ///
    /// `A EOR M -> A`, Flags affected: `N` `Z`
    fn eor(&mut self, mode: AddressingMode) {
        let data = self.get_data_from_addressing_mode(mode);
        self.registers.a ^= data;
        self.registers.set_zero_negative_flags(self.registers.a);
    }

    /// ## INC (Increment Memory by One)
    ///
    /// Increment Memory by One
    ///
    /// `M + 1 -> M`, Flags affected: `N` `Z`
    fn inc(&mut self, mode: AddressingMode) {
        let addr = self.get_address_from_mode(mode);
        let mut data = self.memory.read(addr);
        data = data.wrapping_add(1);
        self.memory.write(addr, data);
        self.registers.set_zero_negative_flags(data);
    }

    /// ## INX (Increment Index X by One)
    ///
    /// Increment Index X by One
    ///
    /// `X + 1 -> X`, Flags affected: `N` `Z`
    fn inx(&mut self) {
        self.registers.x = self.registers.x.wrapping_add(1);
        self.registers.set_zero_negative_flags(self.registers.x);
    }

    /// ## INY (Increment Index Y by One)
    ///
    /// Increment Index Y by One
    ///
    /// `Y + 1 -> Y`, Flags affected: `N` `Z`
    fn iny(&mut self) {
        self.registers.y = self.registers.y.wrapping_add(1);
        self.registers.set_zero_negative_flags(self.registers.y);
    }

    /// ## JMP (Jump to New Location)
    ///
    /// Jump to New Location
    ///
    /// `PC -> E`, Flags affected: None
    fn jmp(&mut self, mode: AddressingMode) {
        let address = self.get_address_from_mode(mode);
        self.registers.pc = address;
    }

    /// ## JSR (Jump to New Location Saving Return Address)
    ///
    /// Jump to New Location Saving Return Address
    ///
    /// `push (PC + 2), PC -> E`, Flags affected: None
    fn jsr(&mut self) {
        self.stack_push_addr(self.registers.pc + 1); // PC + 2
        self.registers.pc = self.get_address_from_mode(AddressingMode::Absolute);
    }

    /// ## LDA (Load Accumulator with Memory)
    ///
    /// Load Accumulator with Memory
    ///
    /// `M -> A`, Flags affected: `N` `Z`
    fn lda(&mut self, mode: AddressingMode) {
        let data = self.get_data_from_addressing_mode(mode);
        self.registers.a = data;
        self.registers.set_zero_negative_flags(self.registers.a);
    }

    /// ## LDX (Load Index X with Memory)
    ///
    /// Load Index X with Memory
    ///
    /// `M -> X`, Flags affected: `N` `Z`
    fn ldx(&mut self, mode: AddressingMode) {
        let data = self.get_data_from_addressing_mode(mode);
        self.registers.x = data;
        self.registers.set_zero_negative_flags(self.registers.x);
    }

    /// ## LDY (Load Index Y with Memory)
    ///
    /// Load Index Y with Memory
    ///
    /// `M -> Y`, Flags affected: `N` `Z`
    fn ldy(&mut self, mode: AddressingMode) {
        let data = self.get_data_from_addressing_mode(mode);
        self.registers.y = data;
        self.registers.set_zero_negative_flags(self.registers.y);
    }

    /// ## LSR (Shift One Bit Right (Memory or Accumulator))
    ///
    /// Shift One Bit Right (Memory or Accumulator)
    ///
    /// `0 -> [76543210] -> C`, Flags affected: `N` `Z` `C`
    fn lsr(&mut self, mode: Option<AddressingMode>) {
        let data = match mode {
            Some(mode) => self.get_data_from_addressing_mode(mode),
            None => self.registers.a,
        };
        self.registers.set_flag_carry(data & 0x01 == 1);

        let data = data >> 1;
        self.registers.set_zero_negative_flags(data);

        match mode {
            Some(mode) => {
                let addr = self.get_address_from_mode(mode);
                self.memory.write(addr, data);
            }
            None => self.registers.a = data,
        }
    }

    /// ## ORA (OR Memory with Accumulator)
    ///
    /// OR Memory with Accumulator
    ///
    /// `A OR M -> A`, Flags affected: `N` `Z`
    fn ora(&mut self, mode: AddressingMode) {
        let data = self.get_data_from_addressing_mode(mode);
        self.registers.a |= data;
        self.registers.set_zero_negative_flags(self.registers.a);
    }

    /// ## PHA (Push Accumulator on Stack)
    ///
    /// Push Accumulator on Stack
    ///
    /// `push A`, Flags affected: None
    fn pha(&mut self) {
        self.stack_push(self.registers.a);
    }

    /// ## PHP (Push Processor Status on Stack)
    ///
    /// Push Processor Status on Stack
    ///
    /// `push SR`, Flags affected: None
    fn php(&mut self) {
        self.stack_push(self.registers.p);
    }

    /// ## PLA (Pull Accumulator from Stack)
    ///
    /// Pull Accumulator from Stack
    ///
    /// `pull A`, Flags affected: `N` `Z`
    fn pla(&mut self) {
        self.registers.a = self.stack_pop();
        self.registers.set_zero_negative_flags(self.registers.a);
    }

    /// ## PLP (Pull Processor Status from Stack)
    ///
    /// Pull Processor Status from Stack
    ///
    /// `pull SR`, Flags affected: `N` `V` `B` `D` `I` `Z` `C`
    fn plp(&mut self) {
        self.registers.p = self.stack_pop();
    }

    /// ## ROL (Rotate One Bit Left (Memory or Accumulator))
    ///
    /// Rotate One Bit Left (Memory or Accumulator)
    ///
    /// `C <- [76543210] <- C`, Flags affected: `N` `Z` `C`
    fn rol(&mut self, mode: Option<AddressingMode>) {
        let data = match mode {
            Some(mode) => self.get_data_from_addressing_mode(mode),
            None => self.registers.a,
        };
        let carry = self.registers.get_flag_carry() as u8;
        self.registers.set_flag_carry(data & 0x80 == 0x80);

        let data = (data << 1) | carry;
        self.registers.set_zero_negative_flags(data);

        match mode {
            Some(mode) => {
                let addr = self.get_address_from_mode(mode);
                self.memory.write(addr, data);
            }
            None => self.registers.a = data,
        }
    }

    /// ## ROR (Rotate One Bit Right (Memory or Accumulator))
    ///
    /// Rotate One Bit Right (Memory or Accumulator)
    ///
    /// `C -> [76543210] -> C`, Flags affected: `N` `Z` `C`
    fn ror(&mut self, mode: Option<AddressingMode>) {
        let data = match mode {
            Some(mode) => self.get_data_from_addressing_mode(mode),
            None => self.registers.a,
        };
        let carry = self.registers.get_flag_carry() as u8;
        self.registers.set_flag_carry(data & 0x01 == 1);

        let data = (data >> 1) | carry;
        self.registers.set_zero_negative_flags(data);

        match mode {
            Some(mode) => {
                let addr = self.get_address_from_mode(mode);
                self.memory.write(addr, data);
            }
            None => self.registers.a = data,
        }
    }

    /// ## RTI (Return from Interrupt)
    ///
    /// Return from Interrupt
    ///
    /// `pull SR, pull PC`, Flags affected: `N` `V` `B` `D` `I` `Z` `C`
    fn rti(&mut self) {
        self.registers.p = self.stack_pop();
        self.registers.pc = self.stack_pop_addr();
    }

    /// ## RTS (Return from Subroutine)
    ///
    /// Return from Subroutine
    ///
    /// `pull PC, PC+1 -> PC`, Flags affected: None
    fn rts(&mut self) {
        self.registers.pc = self.stack_pop_addr() + 1;
    }

    /// ## SBC (Subtract Memory from Accumulator with Borrow)
    ///
    /// Subtract Memory from Accumulator with Borrow
    ///
    /// `A - M - C -> A`, Flags affected: `N` `V` `Z` `C`
    fn sbc(&mut self, mode: AddressingMode) {
        let data = self.get_data_from_addressing_mode(mode);

        self.add_to_accumulator_with_carry(!data - 1);
    }

    /// ## SEC (Set Carry Flag)
    ///
    /// Set Carry Flag
    ///
    /// `1 -> C`, Flags affected: `C`
    fn sec(&mut self) {
        self.registers.set_flag_carry(true);
    }

    /// ## SED (Set Decimal Flag)
    ///
    /// Set Decimal Flag
    ///
    /// `1 -> D`, Flags affected: `D`
    fn sed(&mut self) {
        self.registers.set_flag_decimal(true);
    }

    /// ## SEI (Set Interrupt Disable)
    ///
    /// Set Interrupt Disable
    ///
    /// `1 -> I`, Flags affected: `I`
    fn sei(&mut self) {
        self.registers.set_flag_interrupt_disable(true);
    }

    /// ## STA (Store Accumulator in Memory)
    ///
    /// Store Accumulator in Memory
    ///
    /// `A -> M`, Flags affected: None
    fn sta(&mut self, mode: AddressingMode) {
        let address = self.get_address_from_mode(mode);
        self.memory.write(address, self.registers.a);
    }

    /// ## STX (Store Index X in Memory)
    ///
    /// Store Index X in Memory
    ///
    /// `X -> M`, Flags affected: None
    fn stx(&mut self, mode: AddressingMode) {
        let address = self.get_address_from_mode(mode);
        self.memory.write(address, self.registers.x);
    }

    /// ## STY (Store Index Y in Memory)
    ///
    /// Store Index Y in Memory
    ///
    /// `Y -> M`, Flags affected: None
    fn sty(&mut self, mode: AddressingMode) {
        let address = self.get_address_from_mode(mode);
        self.memory.write(address, self.registers.y);
    }

    /// ## TAX (Transfer Accumulator to Index X)
    ///
    /// Transfer Accumulator to Index X
    ///
    /// `A -> X`, Flags affected: `N` `Z`
    fn tax(&mut self) {
        self.registers.x = self.registers.a;
        self.registers.set_zero_negative_flags(self.registers.x);
    }

    /// ## TAY (Transfer Accumulator to Index Y)
    ///
    /// Transfer Accumulator to Index Y
    ///
    /// `X -> A`, Flags affected: `N` `Z`
    fn tay(&mut self) {
        self.registers.y = self.registers.a;
        self.registers.set_zero_negative_flags(self.registers.y);
    }

    /// ## TSX (Transfer Stack Pointer to Index X)
    ///
    /// Transfer Stack Pointer to Index X
    ///
    /// `SP -> X`, Flags affected: `N` `Z`
    fn tsx(&mut self) {
        self.registers.x = self.registers.sp;
        self.registers.set_zero_negative_flags(self.registers.x);
    }

    /// ## TXA (Transfer Index X to Accumulator)
    ///
    /// Transfer Index X to Accumulator
    ///
    /// `X -> A`, Flags affected: `N` `Z`
    fn txa(&mut self) {
        self.registers.a = self.registers.x;
        self.registers.set_zero_negative_flags(self.registers.a);
    }

    /// ## TXS (Transfer Index X to Stack Register)
    ///
    /// Transfer Index X to Stack Register
    ///
    /// `X -> SP`, Flags affected: None
    fn txs(&mut self) {
        self.registers.sp = self.registers.x;
    }

    /// ## TYA (Transfer Index Y to Accumulator)
    ///
    /// Transfer Index Y to Accumulator
    ///
    /// `Y -> A`, Flags affected: `N` `Z`
    fn tya(&mut self) {
        self.registers.a = self.registers.y;
        self.registers.set_zero_negative_flags(self.registers.a);
    }
}

impl<T, D, R> CpuDebugger for Cpu<T, D, R>
where
    T: MemoryBus<Data = u8, Addr = u16> + Default,
    D: Debugger,
    R: Debugger,
{
    fn step(&mut self) -> u8 {
        let opcode = self.memory.read(self.registers.pc);

        self.debug(&format!(
            "Execute 0x{:02X} at 0x{:04X}",
            opcode, self.registers.pc
        ));

        self.execute_instruction(opcode);
        opcode
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{memory::Memory, NoneDebugger};

    macro_rules! assert_eq_hex {
        ($left:expr, $right:expr) => {
            assert_eq!($left, $right, "{:#X} != {:#X}", $left, $right);
        };
    }

    fn setup() -> NoneDebuggerCpu<Memory<NoneDebugger>> {
        Cpu::default()
    }

    #[cfg(test)]
    mod stack {
        use super::*;

        #[test]
        fn test_stack() {
            let mut cpu = setup();
            cpu.stack_push(0x01);
            /*
            Stack Push: [0x01]
            SP: 0xFF (0x00 - 0x01 = 0xFF)

            Stack Pop: [0x01]
            SP: 0x00
            */
            assert_eq!(cpu.registers.sp, 0xFF);
            assert_eq!(cpu.stack_pop(), 0x01);
            assert_eq!(cpu.registers.sp, 0x00);

            cpu.stack_push_addr(0x0203);
            /*
            Stack Push: [0x02, 0x03]
            SP: 0xFE (0x00 - 0x02 = 0xFE)

            Stack Pop: [0x02, 0x03]
            SP: 0x00
            */
            assert_eq!(cpu.registers.sp, 0xFE);
            assert_eq!(cpu.stack_pop_addr(), 0x0203);
            assert_eq!(cpu.registers.sp, 0x00);
        }
    }

    #[cfg(test)]
    mod memory_addressing_mode {
        use super::*;

        #[test]
        fn addressing_mode_immidiate() {
            let mut cpu = setup();
            cpu.reset();
            cpu.memory.write(0x8000, 0x01);

            assert_eq!(cpu.get_address_from_mode(AddressingMode::Immediate), 0x8000);
            assert_eq_hex!(cpu.registers.pc, 0x8001);
        }

        #[test]
        fn addressing_mode_absolute() {
            let mut cpu = setup();
            cpu.reset();
            cpu.memory.write(0x8000, 0x01);
            cpu.memory.write(0x8001, 0x02);

            assert_eq!(cpu.get_address_from_mode(AddressingMode::Absolute), 0x0201);
            assert_eq_hex!(cpu.registers.pc, 0x8002);
        }

        #[test]
        fn addressing_mode_absolute_x() {
            let mut cpu = setup();
            cpu.reset();
            cpu.memory.write(0x8000, 0x01);
            cpu.memory.write(0x8001, 0x02);
            cpu.registers.x = 0x03;

            assert_eq!(cpu.get_address_from_mode(AddressingMode::AbsoluteX), 0x0204);
            assert_eq_hex!(cpu.registers.pc, 0x8002);
        }

        #[test]
        fn addressing_mode_absolute_y() {
            let mut cpu = setup();
            cpu.reset();
            cpu.memory.write(0x8000, 0x01);
            cpu.memory.write(0x8001, 0x02);
            cpu.registers.y = 0x03;

            assert_eq!(cpu.get_address_from_mode(AddressingMode::AbsoluteY), 0x0204);
            assert_eq_hex!(cpu.registers.pc, 0x8002);
        }

        #[test]
        fn addressing_mode_indirect() {
            let mut cpu = setup();
            cpu.reset();
            cpu.memory.write(0x8000, 0x01);
            cpu.memory.write(0x8001, 0x02);
            cpu.memory.write(0x0201, 0x03);
            cpu.memory.write(0x0202, 0x04);

            assert_eq!(cpu.get_address_from_mode(AddressingMode::Indirect), 0x0403);
            assert_eq_hex!(cpu.registers.pc, 0x8002);
        }

        #[test]
        fn addressing_mode_indirect_x() {
            let mut cpu = setup();
            cpu.reset();
            cpu.memory.write(0x8000, 0x01); // `0x01` + RegX (0x03) = 0x04
            cpu.memory.write(0x8001, 0x02);
            cpu.memory.write(0x0004, 0x03);
            cpu.memory.write(0x0005, 0x04);
            cpu.registers.x = 0x03;

            assert_eq!(cpu.get_address_from_mode(AddressingMode::IndirectX), 0x0403);
            assert_eq_hex!(cpu.registers.pc, 0x8003);
        }

        #[test]
        fn addressing_mode_indirect_y() {
            let mut cpu = setup();
            cpu.reset();
            cpu.memory.write(0x8000, 0x04);
            cpu.memory.write(0x8001, 0x02);
            cpu.memory.write(0x0004, 0x03);
            cpu.memory.write(0x0005, 0x04);
            cpu.registers.y = 0x02;

            assert_eq!(cpu.get_address_from_mode(AddressingMode::IndirectY), 0x0405);
            assert_eq_hex!(cpu.registers.pc, 0x8003);
        }

        #[test]
        fn addressing_mode_zero_page() {
            let mut cpu = setup();
            cpu.reset();
            cpu.memory.write(0x8000, 0x01);

            assert_eq!(cpu.get_address_from_mode(AddressingMode::ZeroPage), 0x01);
            assert_eq_hex!(cpu.registers.pc, 0x8001);
        }

        #[test]
        fn addressing_mode_zero_page_x() {
            let mut cpu = setup();
            cpu.reset();
            cpu.memory.write(0x8000, 0x01);
            cpu.registers.x = 0x03;

            assert_eq!(cpu.get_address_from_mode(AddressingMode::ZeroPageX), 0x04);
            assert_eq_hex!(cpu.registers.pc, 0x8001);
        }

        #[test]
        fn addressing_mode_zero_page_y() {
            let mut cpu = setup();
            cpu.reset();
            cpu.memory.write(0x8000, 0x01);
            cpu.registers.y = 0x03;

            assert_eq!(cpu.get_address_from_mode(AddressingMode::ZeroPageY), 0x04);
            assert_eq_hex!(cpu.registers.pc, 0x8001);
        }
    }

    #[cfg(test)]
    mod instruction {
        use super::*;
        #[test]
        fn adc() {
            let mut cpu = setup();
            cpu.reset();
            cpu.registers.a = 0x78;
            cpu.registers.set_flag_carry(true);
            cpu.load(&[
                0x69, 0x07, // ADC #$07
                0x00,
            ]);

            cpu.execute();

            assert_eq!(cpu.registers.a, 0x80);
            assert_eq_hex!(cpu.registers.pc, 0x8003);
            assert_eq!(cpu.registers.get_flag_carry(), false);
            assert_eq!(cpu.registers.get_flag_zero(), false);
            assert_eq!(cpu.registers.get_flag_overflow(), true);
            assert_eq!(cpu.registers.get_flag_negative(), true);
        }

        #[test]
        fn and() {
            let mut cpu = setup();
            cpu.reset();
            cpu.registers.a = 0x78; // 0111 1000
            cpu.load(&[
                0x29, 0x07, // AND #$07 ; 0000 0111
                0x00,
            ]);

            cpu.execute();

            assert_eq!(cpu.registers.a, 0x00);
            assert_eq_hex!(cpu.registers.pc, 0x8003);
            assert_eq!(cpu.registers.get_flag_zero(), true);
            assert_eq!(cpu.registers.get_flag_negative(), false);
        }

        #[test]
        fn asl() {
            let mut cpu = setup();
            cpu.reset();
            cpu.registers.a = 0x78; // 0111 1000
            cpu.load(&[
                0x0A, // ASL
                0x00,
            ]);

            cpu.execute();

            assert_eq!(cpu.registers.a, 0xF0); // 1111 0000
            assert_eq_hex!(cpu.registers.pc, 0x8002);
            assert_eq!(cpu.registers.get_flag_carry(), false);
            assert_eq!(cpu.registers.get_flag_zero(), false);
            assert_eq!(cpu.registers.get_flag_negative(), true);
        }

        #[test]
        fn bcc() {
            let mut cpu = setup();
            cpu.reset();
            cpu.registers.set_flag_carry(false);
            cpu.load(&[
                0x90, 0x02, // BCC
                0x00,
            ]);

            cpu.execute();

            assert_eq_hex!(cpu.registers.pc, 0x8005);
        }

        #[test]
        fn bcs() {
            let mut cpu = setup();
            cpu.reset();
            cpu.registers.set_flag_carry(true);
            cpu.load(&[
                0xB0, 0x02, // BCS
                0x00,
            ]);

            cpu.execute();

            assert_eq_hex!(cpu.registers.pc, 0x8005);
        }

        #[test]
        fn beq() {
            let mut cpu = setup();
            cpu.reset();
            cpu.registers.set_flag_zero(true);
            cpu.load(&[
                0xF0, 0x02, // BEQ
                0x00,
            ]);

            cpu.execute();

            assert_eq_hex!(cpu.registers.pc, 0x8005);
        }

        #[test]
        fn bit() {
            let mut cpu = setup();
            cpu.reset();
            cpu.registers.a = 129;
            cpu.memory.write(0x10, 150);
            cpu.load(&[
                0x24, 0x10, // BIT
                0x00,
            ]);

            cpu.execute();

            assert_eq_hex!(cpu.registers.pc, 0x8003);
            assert_eq!(cpu.registers.get_flag_zero(), false);
            assert_eq!(cpu.registers.get_flag_overflow(), false);
            assert_eq!(cpu.registers.get_flag_negative(), true);
        }

        #[test]
        fn bmi() {
            let mut cpu = setup();
            cpu.reset();
            cpu.registers.set_flag_negative(true);
            cpu.load(&[
                0x30, 0x02, // BMI
                0x00,
            ]);

            cpu.execute();

            assert_eq_hex!(cpu.registers.pc, 0x8005);
        }

        #[test]
        fn bne() {
            let mut cpu = setup();
            cpu.reset();
            cpu.registers.set_flag_zero(false);
            cpu.load(&[
                0xD0, 0x02, // BNE
                0x00,
            ]);

            cpu.execute();

            assert_eq_hex!(cpu.registers.pc, 0x8005);
        }

        #[test]
        fn bpl() {
            let mut cpu = setup();
            cpu.reset();
            cpu.registers.set_flag_negative(false);
            cpu.load(&[
                0x10, 0x02, // BPL
                0x00,
            ]);

            cpu.execute();

            assert_eq_hex!(cpu.registers.pc, 0x8005);
        }

        #[test]
        fn brk() {}

        #[test]
        fn bvc() {
            let mut cpu = setup();
            cpu.reset();
            cpu.registers.set_flag_overflow(false);
            cpu.load(&[
                0x50, 0x02, // BVC
                0x00,
            ]);

            cpu.execute();

            assert_eq_hex!(cpu.registers.pc, 0x8005);
        }

        #[test]
        fn bvs() {
            let mut cpu = setup();
            cpu.reset();
            cpu.registers.set_flag_overflow(true);
            cpu.load(&[
                0x70, 0x02, // BVS
                0x00,
            ]);

            cpu.execute();

            assert_eq_hex!(cpu.registers.pc, 0x8005);
        }

        #[test]
        fn clc() {
            let mut cpu = setup();
            cpu.reset();
            cpu.registers.set_flag_carry(true);
            cpu.load(&[
                0x18, // CLC
                0x00,
            ]);

            cpu.execute();

            assert_eq!(cpu.registers.get_flag_carry(), false);
            assert_eq_hex!(cpu.registers.pc, 0x8002);
        }

        #[test]
        fn cld() {
            let mut cpu = setup();
            cpu.reset();
            cpu.registers.set_flag_decimal(true);
            cpu.load(&[
                0xD8, // CLD
                0x00,
            ]);

            cpu.execute();

            assert_eq!(cpu.registers.get_flag_decimal(), false);
            assert_eq_hex!(cpu.registers.pc, 0x8002);
        }

        #[test]
        fn cli() {
            let mut cpu = setup();
            cpu.reset();
            cpu.registers.set_flag_interrupt_disable(true);
            cpu.load(&[
                0x58, // CLI
                0x00,
            ]);

            cpu.execute();

            assert_eq!(cpu.registers.get_flag_interrupt_disable(), false);
            assert_eq_hex!(cpu.registers.pc, 0x8002);
        }

        #[test]
        fn clv() {
            let mut cpu = setup();
            cpu.reset();
            cpu.registers.set_flag_overflow(true);
            cpu.load(&[
                0xB8, // CLV
                0x00,
            ]);

            cpu.execute();

            assert_eq!(cpu.registers.get_flag_overflow(), false);
            assert_eq_hex!(cpu.registers.pc, 0x8002);
        }

        #[test]
        fn cmp() {
            let mut cpu = setup();
            cpu.reset();
            cpu.registers.a = 0x40;
            cpu.load(&[
                0xC9, 0x80, // CMP
                0x00,
            ]);

            cpu.execute();

            assert_eq!(cpu.registers.get_flag_carry(), false);
            assert_eq!(cpu.registers.get_flag_zero(), false);
            assert_eq!(cpu.registers.get_flag_negative(), true);
            assert_eq_hex!(cpu.registers.pc, 0x8003);
        }

        #[test]
        fn cpx() {
            let mut cpu = setup();
            cpu.reset();
            cpu.registers.x = 0x40;
            cpu.load(&[
                0xE0, 0x80, // CPX
                0x00,
            ]);

            cpu.execute();

            assert_eq!(cpu.registers.get_flag_carry(), false);
            assert_eq!(cpu.registers.get_flag_zero(), false);
            assert_eq!(cpu.registers.get_flag_negative(), true);
            assert_eq_hex!(cpu.registers.pc, 0x8003);
        }

        #[test]
        fn cpy() {
            let mut cpu = setup();
            cpu.reset();
            cpu.registers.y = 0x40;
            cpu.load(&[
                0xC0, 0x80, // CPY
                0x00,
            ]);

            cpu.execute();

            assert_eq!(cpu.registers.get_flag_carry(), false);
            assert_eq!(cpu.registers.get_flag_zero(), false);
            assert_eq!(cpu.registers.get_flag_negative(), true);
            assert_eq_hex!(cpu.registers.pc, 0x8003);
        }

        #[test]
        fn dec() {
            let mut cpu = setup();
            cpu.reset();
            cpu.memory.write(0x00, 0x01);
            cpu.load(&[
                0xC6, 0x00, // DEC
                0x00,
            ]);

            cpu.execute();

            assert_eq!(cpu.memory.read(0x00), 0x00);
            assert_eq!(cpu.registers.get_flag_zero(), true);
            assert_eq!(cpu.registers.get_flag_negative(), false);
            assert_eq_hex!(cpu.registers.pc, 0x8003);
        }

        #[test]
        fn dex() {
            let mut cpu = setup();
            cpu.reset();
            cpu.registers.x = 0x01;
            cpu.load(&[
                0xCA, // DEX
                0x00,
            ]);

            cpu.execute();

            assert_eq!(cpu.registers.x, 0x00);
            assert_eq!(cpu.registers.get_flag_zero(), true);
            assert_eq!(cpu.registers.get_flag_negative(), false);
            assert_eq_hex!(cpu.registers.pc, 0x8002);
        }

        #[test]
        fn dey() {
            let mut cpu = setup();
            cpu.reset();
            cpu.registers.y = 0x01;
            cpu.load(&[
                0x88, // DEY
                0x00,
            ]);

            cpu.execute();

            assert_eq!(cpu.registers.y, 0x00);
            assert_eq!(cpu.registers.get_flag_zero(), true);
            assert_eq!(cpu.registers.get_flag_negative(), false);
            assert_eq_hex!(cpu.registers.pc, 0x8002);
        }

        #[test]
        fn eor() {
            let mut cpu = setup();
            cpu.reset();
            cpu.registers.a = 0x78; // 0111 1000
            cpu.load(&[
                0x49, 0x07, // EOR #$07 ; 0000 0111
                0x00,
            ]);

            cpu.execute();

            assert_eq!(cpu.registers.a, 0x7F); // 0111 1111
            assert_eq_hex!(cpu.registers.pc, 0x8003);
            assert_eq!(cpu.registers.get_flag_zero(), false);
            assert_eq!(cpu.registers.get_flag_negative(), false);
        }

        #[test]
        fn inc() {
            let mut cpu = setup();
            cpu.reset();
            cpu.memory.write(0x00, 0x01);
            cpu.load(&[
                0xE6, 0x00, // INC
                0x00,
            ]);

            cpu.execute();

            assert_eq!(cpu.memory.read(0x00), 0x02);
            assert_eq!(cpu.registers.get_flag_zero(), false);
            assert_eq!(cpu.registers.get_flag_negative(), false);
            assert_eq_hex!(cpu.registers.pc, 0x8003);
        }

        #[test]
        fn inx() {
            let mut cpu = setup();
            cpu.reset();
            cpu.registers.x = 0x01;
            cpu.load(&[
                0xE8, // INX
                0x00,
            ]);

            cpu.execute();

            assert_eq!(cpu.registers.x, 0x02);
            assert_eq!(cpu.registers.get_flag_zero(), false);
            assert_eq!(cpu.registers.get_flag_negative(), false);
            assert_eq_hex!(cpu.registers.pc, 0x8002);
        }

        #[test]
        fn iny() {
            let mut cpu = setup();
            cpu.reset();
            cpu.registers.y = 0x01;
            cpu.load(&[
                0xC8, // INY
                0x00,
            ]);

            cpu.execute();

            assert_eq!(cpu.registers.y, 0x02);
            assert_eq!(cpu.registers.get_flag_zero(), false);
            assert_eq!(cpu.registers.get_flag_negative(), false);
            assert_eq_hex!(cpu.registers.pc, 0x8002);
        }

        #[test]
        fn jmp() {
            let mut cpu = setup();
            cpu.reset();
            cpu.registers.x = 0x01;
            cpu.load(&[
                /* $8000 */ 0x4C, 0x04, 0x80, // JMP $8004
                /* $8003 */ 0xE8, // INX
                /* $8004 */ 0xCA, // DEX
                /* $8005 */ 0x00,
            ]);

            cpu.execute();

            assert_eq_hex!(cpu.registers.pc, 0x8006);
            assert_eq!(cpu.registers.x, 0x00);
        }

        #[test]
        fn jsr() {
            let mut cpu = setup();
            cpu.reset();
            cpu.registers.x = 0x01;
            cpu.load(&[
                /* $8000 */ 0x20, 0x04, 0x80, // JSR $8004
                /* $8003 */ 0xE8, // INX
                /* $8004 */ 0xCA, // DEX
                /* $8005 */ 0x00,
            ]);

            cpu.execute();

            assert_eq_hex!(cpu.registers.pc, 0x8006);
            assert_eq!(cpu.registers.x, 0x00);
            assert_eq!(cpu.stack_pop(), 0x02);
            assert_eq!(cpu.stack_pop(), 0x80);
        }

        #[test]
        fn lda() {
            let mut cpu = setup();
            cpu.reset();
            cpu.memory.write(0x00, 0x01);
            cpu.load(&[
                0xA5, 0x00, // LDA
                0x00,
            ]);

            cpu.execute();

            assert_eq!(cpu.registers.a, 0x01);
            assert_eq!(cpu.registers.get_flag_zero(), false);
            assert_eq!(cpu.registers.get_flag_negative(), false);
            assert_eq_hex!(cpu.registers.pc, 0x8003);
        }

        #[test]
        fn ldx() {
            let mut cpu = setup();
            cpu.reset();
            cpu.memory.write(0x00, 0x01);
            cpu.load(&[
                0xA6, 0x00, // LDX
                0x00,
            ]);

            cpu.execute();

            assert_eq!(cpu.registers.x, 0x01);
            assert_eq!(cpu.registers.get_flag_zero(), false);
            assert_eq!(cpu.registers.get_flag_negative(), false);
            assert_eq_hex!(cpu.registers.pc, 0x8003);
        }

        #[test]
        fn ldy() {
            let mut cpu = setup();
            cpu.reset();
            cpu.memory.write(0x00, 0x01);
            cpu.load(&[
                0xA4, 0x00, // LDY
                0x00,
            ]);

            cpu.execute();

            assert_eq!(cpu.registers.y, 0x01);
            assert_eq!(cpu.registers.get_flag_zero(), false);
            assert_eq!(cpu.registers.get_flag_negative(), false);
            assert_eq_hex!(cpu.registers.pc, 0x8003);
        }

        #[test]
        fn lsr() {
            let mut cpu = setup();
            cpu.reset();
            cpu.registers.a = 10;
            cpu.load(&[
                0x4A, // LSR
                0x00,
            ]);

            cpu.execute();

            assert_eq!(cpu.registers.a, 5);
            assert_eq!(cpu.registers.get_flag_carry(), false);
            assert_eq!(cpu.registers.get_flag_zero(), false);
            assert_eq!(cpu.registers.get_flag_negative(), false);
            assert_eq_hex!(cpu.registers.pc, 0x8002);
        }

        #[test]
        fn nop() {
            let mut cpu = setup();
            cpu.reset();
            cpu.load(&[
                0xEA, // NOP
                0x00,
            ]);

            cpu.execute();

            assert_eq_hex!(cpu.registers.pc, 0x8002);
        }

        #[test]
        fn ora() {
            let mut cpu = setup();
            cpu.reset();
            cpu.registers.a = 0x01;
            cpu.memory.write(0x00, 0x01);
            cpu.load(&[
                0x05, 0x00, // ORA
                0x00,
            ]);

            cpu.execute();

            assert_eq!(cpu.registers.a, 0x01);
            assert_eq!(cpu.registers.get_flag_zero(), false);
            assert_eq!(cpu.registers.get_flag_negative(), false);
            assert_eq_hex!(cpu.registers.pc, 0x8003);
        }

        #[test]
        fn pha() {
            let mut cpu = setup();
            cpu.reset();
            cpu.registers.a = 0x01;
            cpu.load(&[
                0x48, // PHA
                0x00,
            ]);

            cpu.execute();

            assert_eq!(cpu.stack_pop(), 0x01);
            assert_eq_hex!(cpu.registers.pc, 0x8002);
        }

        #[test]
        fn php() {
            let mut cpu = setup();
            cpu.reset();
            cpu.registers.set_flag_carry(true);
            cpu.registers.set_flag_zero(true);
            cpu.registers.set_flag_interrupt_disable(true);
            cpu.registers.set_flag_decimal(true);
            cpu.registers.set_flag_break(true);
            cpu.registers.set_flag_overflow(true);
            cpu.registers.set_flag_negative(true);
            cpu.load(&[
                0x08, // PHP
                0x00,
            ]);

            cpu.execute();

            assert_eq!(cpu.stack_pop(), 0b1101_1111);
            assert_eq_hex!(cpu.registers.pc, 0x8002);
        }

        #[test]
        fn pla() {
            let mut cpu = setup();
            cpu.reset();
            cpu.stack_push(0x01);
            cpu.load(&[
                0x68, // PLA
                0x00,
            ]);

            cpu.execute();

            assert_eq!(cpu.registers.a, 0x01);
            assert_eq_hex!(cpu.registers.pc, 0x8002);
        }

        #[test]
        fn plp() {
            let mut cpu = setup();
            cpu.reset();
            cpu.stack_push(0b1101_1111);
            cpu.load(&[
                0x28, // PLP
                0x00,
            ]);

            cpu.execute();

            assert_eq!(cpu.registers.get_flag_carry(), true);
            assert_eq!(cpu.registers.get_flag_zero(), true);
            assert_eq!(cpu.registers.get_flag_interrupt_disable(), true);
            assert_eq!(cpu.registers.get_flag_decimal(), true);
            assert_eq!(cpu.registers.get_flag_break(), true);
            assert_eq!(cpu.registers.get_flag_overflow(), true);
            assert_eq!(cpu.registers.get_flag_negative(), true);
            assert_eq_hex!(cpu.registers.pc, 0x8002);
        }

        #[test]
        fn rol() {
            let mut cpu = setup();
            cpu.reset();
            cpu.registers.a = 10;
            cpu.registers.set_flag_carry(true);
            cpu.load(&[
                0x2A, // ROL
                0x00,
            ]);

            cpu.execute();

            assert_eq!(cpu.registers.a, 21);
            assert_eq!(cpu.registers.get_flag_carry(), false);
            assert_eq!(cpu.registers.get_flag_zero(), false);
            assert_eq!(cpu.registers.get_flag_negative(), false);
            assert_eq_hex!(cpu.registers.pc, 0x8002);
        }

        #[test]
        fn ror() {
            let mut cpu = setup();
            cpu.reset();
            cpu.registers.a = 10;
            cpu.load(&[
                0x6A, // ROR
                0x00,
            ]);

            cpu.execute();

            assert_eq!(cpu.registers.a, 5);
            assert_eq!(cpu.registers.get_flag_carry(), false);
            assert_eq!(cpu.registers.get_flag_zero(), false);
            assert_eq!(cpu.registers.get_flag_negative(), false);
            assert_eq_hex!(cpu.registers.pc, 0x8002);
        }

        #[test]
        fn rti() {
            let mut cpu = setup();
            cpu.reset();
            cpu.stack_push_addr(0x8001);
            cpu.stack_push(0b1101_1111);
            cpu.load(&[
                0x40, // RTI
                0x00,
            ]);

            cpu.execute();

            assert_eq_hex!(cpu.registers.pc, 0x8002);
            assert_eq!(cpu.registers.p, 0b1101_1111);
        }

        #[test]
        fn rts() {
            let mut cpu = setup();
            cpu.reset();
            cpu.stack_push_addr(0x8001);
            cpu.load(&[
                0x60, // RTS
                0xEA, 0x00,
            ]);

            cpu.execute();

            assert_eq_hex!(cpu.registers.pc, 0x8003);
        }

        #[test]
        fn sbc() {
            let mut cpu = setup();
            cpu.reset();
            cpu.registers.a = 0x08;
            cpu.registers.set_flag_carry(true);
            cpu.load(&[
                0xE9, 0x04, // SBC
                0x00,
            ]);

            cpu.execute();

            assert_eq!(cpu.registers.a, 0x03);
            assert_eq!(cpu.registers.get_flag_carry(), true);
            assert_eq!(cpu.registers.get_flag_zero(), false);
            assert_eq!(cpu.registers.get_flag_overflow(), false);
            assert_eq!(cpu.registers.get_flag_negative(), false);
            assert_eq_hex!(cpu.registers.pc, 0x8003);
        }

        #[test]
        fn sec() {
            let mut cpu = setup();
            cpu.reset();
            cpu.load(&[
                0x38, // SEC
                0x00,
            ]);

            cpu.execute();

            assert_eq!(cpu.registers.get_flag_carry(), true);
            assert_eq_hex!(cpu.registers.pc, 0x8002);
        }

        #[test]
        fn sed() {
            let mut cpu = setup();
            cpu.reset();
            cpu.load(&[
                0xF8, // SED
                0x00,
            ]);

            cpu.execute();

            assert_eq!(cpu.registers.get_flag_decimal(), true);
            assert_eq_hex!(cpu.registers.pc, 0x8002);
        }

        #[test]
        fn sei() {
            let mut cpu = setup();
            cpu.reset();
            cpu.load(&[
                0x78, // SEI
                0x00,
            ]);

            cpu.execute();

            assert_eq!(cpu.registers.get_flag_interrupt_disable(), true);
            assert_eq_hex!(cpu.registers.pc, 0x8002);
        }

        #[test]
        fn sta() {
            let mut cpu = setup();
            cpu.reset();
            cpu.registers.a = 0x01;
            cpu.load(&[
                0x85, 0x00, // STA
                0x00,
            ]);

            cpu.execute();

            assert_eq!(cpu.memory.read(0x00), 0x01);
            assert_eq_hex!(cpu.registers.pc, 0x8003);
        }

        #[test]
        fn stx() {
            let mut cpu = setup();
            cpu.reset();
            cpu.registers.x = 0x01;
            cpu.load(&[
                0x86, 0x00, // STX
                0x00,
            ]);

            cpu.execute();

            assert_eq!(cpu.memory.read(0x00), 0x01);
            assert_eq_hex!(cpu.registers.pc, 0x8003);
        }

        #[test]
        fn sty() {
            let mut cpu = setup();
            cpu.reset();
            cpu.registers.y = 0x01;
            cpu.load(&[
                0x84, 0x00, // STY
                0x00,
            ]);

            cpu.execute();

            assert_eq!(cpu.memory.read(0x00), 0x01);
            assert_eq_hex!(cpu.registers.pc, 0x8003);
        }

        #[test]
        fn tax() {
            let mut cpu = setup();
            cpu.reset();
            cpu.registers.a = 0x01;
            cpu.load(&[
                0xAA, // TAX
                0x00,
            ]);

            cpu.execute();

            assert_eq!(cpu.registers.x, 0x01);
            assert_eq!(cpu.registers.get_flag_zero(), false);
            assert_eq!(cpu.registers.get_flag_negative(), false);
            assert_eq_hex!(cpu.registers.pc, 0x8002);
        }

        #[test]
        fn tay() {
            let mut cpu = setup();
            cpu.reset();
            cpu.registers.a = 0x01;
            cpu.load(&[
                0xA8, // TAY
                0x00,
            ]);

            cpu.execute();

            assert_eq!(cpu.registers.y, 0x01);
            assert_eq!(cpu.registers.get_flag_zero(), false);
            assert_eq!(cpu.registers.get_flag_negative(), false);
            assert_eq_hex!(cpu.registers.pc, 0x8002);
        }

        #[test]
        fn tsx() {
            let mut cpu = setup();
            cpu.reset();
            cpu.registers.sp = 0x01;
            cpu.load(&[
                0xBA, // TSX
                0x00,
            ]);

            cpu.execute();

            assert_eq!(cpu.registers.x, 0x01);
            assert_eq!(cpu.registers.get_flag_zero(), false);
            assert_eq!(cpu.registers.get_flag_negative(), false);
            assert_eq_hex!(cpu.registers.pc, 0x8002);
        }

        #[test]
        fn txa() {
            let mut cpu = setup();
            cpu.reset();
            cpu.registers.x = 0x01;
            cpu.load(&[
                0x8A, // TXA
                0x00,
            ]);

            cpu.execute();

            assert_eq!(cpu.registers.a, 0x01);
            assert_eq!(cpu.registers.get_flag_zero(), false);
            assert_eq!(cpu.registers.get_flag_negative(), false);
            assert_eq_hex!(cpu.registers.pc, 0x8002);
        }

        #[test]
        fn txs() {
            let mut cpu = setup();
            cpu.reset();
            cpu.registers.x = 0x01;
            cpu.load(&[
                0x9A, // TXS
                0x00,
            ]);

            cpu.execute();

            assert_eq!(cpu.registers.sp, 0x01);
            assert_eq_hex!(cpu.registers.pc, 0x8002);
        }

        #[test]
        fn tya() {
            let mut cpu = setup();
            cpu.reset();
            cpu.registers.y = 0x01;
            cpu.load(&[
                0x98, // TYA
                0x00,
            ]);

            cpu.execute();

            assert_eq!(cpu.registers.a, 0x01);
            assert_eq!(cpu.registers.get_flag_zero(), false);
            assert_eq!(cpu.registers.get_flag_negative(), false);
            assert_eq_hex!(cpu.registers.pc, 0x8002);
        }
    }
}

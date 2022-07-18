use core::convert::TryInto;
use gdbstub::arch::Registers;

/// 64-bit ARM core registers.
///
/// Source: <https://github.com/bminor/binutils-gdb/blob/master/gdb/features/aarch64-core.xml>
#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Aarch64CoreRegs {
    /// General purpose registers (X0-X30)
    pub regs: [u64; 31],
    /// Stack Pointer
    pub sp: u64,
    /// Program Counter
    pub pc: u64,
    /// Current program status register
    pub cpsr: u32,
}

impl Registers for Aarch64CoreRegs {
    type ProgramCounter = u64;

    fn pc(&self) -> Self::ProgramCounter {
        self.pc
    }

    fn gdb_serialize(&self, mut write_byte: impl FnMut(Option<u8>)) {
        macro_rules! write_bytes {
            ($bytes:expr) => {
                for b in $bytes {
                    write_byte(Some(*b))
                }
            };
        }

        for reg in self.regs.iter() {
            write_bytes!(&reg.to_le_bytes());
        }
        write_bytes!(&self.sp.to_le_bytes());
        write_bytes!(&self.pc.to_le_bytes());
        write_bytes!(&self.cpsr.to_le_bytes());
    }

    fn gdb_deserialize(&mut self, bytes: &[u8]) -> Result<(), ()> {
        if bytes.len() < 268 {
            return Err(());
        }

        let mut regs = bytes[0..248]
            .chunks_exact(8)
            .map(|x| u64::from_le_bytes(x.try_into().unwrap()));

        for reg in self.regs.iter_mut() {
            *reg = regs.next().ok_or(())?;
        }

        self.sp = u64::from_le_bytes(bytes[248..256].try_into().unwrap());
        self.pc = u64::from_le_bytes(bytes[256..264].try_into().unwrap());
        self.cpsr = u32::from_le_bytes(bytes[264..268].try_into().unwrap());

        Ok(())
    }
}

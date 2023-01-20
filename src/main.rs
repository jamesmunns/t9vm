pub struct Instr(u8);

// bits - mnemonic - Is word, has children, last child
// 000 - SN - !word, !children, !last
//     ABORT
// 001 - TN - !word, !children,  last
//     ABORT
// 010 - UN - !word,  children, !last
//     Clear WF, push N to word stack, ptr += N, continue
// 011 - VN - !word,  children,  last
//     Clear WF, push N to word stack, ptr += N, continue
// 100 - WN -  word, !children, !last
//     Set WF, ptr++, Push N to word stack, ptr += N, yield word
// 101 - XN -  word, !children,  last
//     Set WF, ptr++, Push N to word stack, ptr += N, yield word
// 110 - YN -  word,  children, !last
//     Set WF, ptr++, Push N to word stack, ptr += N, yield word
// 111 - ZN -  word,  children,  last
//     Set WF, ptr++, Push N to word stack, ptr += N, yield word

struct Stack<T>(Vec<T>);

impl<T> Stack<T> {
    fn new() -> Self {
        Self(Vec::new())
    }

    fn push(&mut self, t: T) -> Result<(), ()> {
        Ok(self.0.push(t))
    }

    fn pop(&mut self) -> Option<T> {
        self.0.pop()
    }

    fn all(&self) -> &[T] {
        self.0.as_slice()
    }

    fn push_n<I: IntoIterator<Item = T>>(&mut self, nt: I) -> Result<(), ()> {
        self.0.extend(nt.into_iter());
        Ok(())
    }

    fn drop_n(&mut self, n: usize) -> Result<(), ()> {
        for _ in 0..n {
            self.0.pop().ok_or(())?;
        }
        Ok(())
    }
}

#[allow(non_camel_case_types)]
enum Instruction {
    S_NotWordNoChildrenNotLast,
    T_NotWordNoChildrenIsLast,
    U_NotWordHasChildrenNotLast,
    V_NotWordHasChildrenIsLast,
    W_IsWordNoChildrenNotLast,
    X_IsWordNoChildrenIsLast,
    Y_IsWordHasChildrenNotLast,
    Z_IsWordHasChildrenIsLast,
}

impl Into<u8> for Instruction {
    fn into(self) -> u8 {
        match self {
            Instruction::S_NotWordNoChildrenNotLast => 0b00000_000,
            Instruction::T_NotWordNoChildrenIsLast => 0b00000_001,
            Instruction::U_NotWordHasChildrenNotLast => 0b00000_010,
            Instruction::V_NotWordHasChildrenIsLast => 0b00000_011,
            Instruction::W_IsWordNoChildrenNotLast => 0b00000_100,
            Instruction::X_IsWordNoChildrenIsLast => 0b00000_101,
            Instruction::Y_IsWordHasChildrenNotLast => 0b00000_110,
            Instruction::Z_IsWordHasChildrenIsLast => 0b00000_111,
        }
    }
}

impl From<u8> for Instruction {
    fn from(val: u8) -> Self {
        match (val & Instr::INSTR_MASK) {
            0b00000_000 => Instruction::S_NotWordNoChildrenNotLast,
            0b00000_001 => Instruction::T_NotWordNoChildrenIsLast,
            0b00000_010 => Instruction::U_NotWordHasChildrenNotLast,
            0b00000_011 => Instruction::V_NotWordHasChildrenIsLast,
            0b00000_100 => Instruction::W_IsWordNoChildrenNotLast,
            0b00000_101 => Instruction::X_IsWordNoChildrenIsLast,
            0b00000_110 => Instruction::Y_IsWordHasChildrenNotLast,
            0b00000_111 => Instruction::Z_IsWordHasChildrenIsLast,
            _ => unreachable!(),
        }
    }
}

impl Instr {
    const INSTR_MASK: u8 = 0b00000_111;
    const LEN_MASK: u8 = 0b11111_000;
    const LEN_SHIFT: u8 = 3;

    fn from_len_instr(len: u8, instr: Instruction) -> Self {
        assert!(len <= 0b000_11111);
        let instr: u8 = instr.into();
        let instr = instr & Self::INSTR_MASK;
        let slen = (len << Self::LEN_SHIFT) & Self::LEN_MASK;

        Instr(instr | slen)
    }

    fn instr(&self) -> Instruction {
        self.0.into()
    }

    fn len(&self) -> usize {
        self.len_u8().into()
    }

    fn len_u8(&self) -> u8 {
        (self.0 & Self::LEN_MASK) >> Self::LEN_SHIFT
    }
}

struct T9Vm {
    control_stack: Stack<Instr>,
    word_stack: Stack<u8>,
    program_ctr: usize,
    program: Vec<u8>,
}

fn main() {
    println!("Hello, world!");
}

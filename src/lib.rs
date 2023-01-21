
#[derive(Copy, Clone)]
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

impl<T> Default for Stack<T> {
    fn default() -> Self {
        Stack(vec![])
    }
}

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

    fn peek(&self) -> Option<&T> {
        if self.0.is_empty() {
            None
        } else {
            self.0.get(self.0.len() - 1)
        }
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

    fn clear(&mut self) {
        self.0.clear();
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

impl Instruction {
    const fn to_u8(&self) -> u8 {
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

impl Into<u8> for Instruction {
    fn into(self) -> u8 {
        self.to_u8()
    }
}

impl From<u8> for Instruction {
    fn from(val: u8) -> Self {
        match val & Instr::INSTR_MASK {
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

impl From<u8> for Instr {
    fn from(data: u8) -> Self {
        Self(data)
    }
}

impl Instr {
    const INSTR_MASK: u8 = 0b00000_111;
    const LEN_MASK: u8 = 0b11111_000;
    const LEN_SHIFT: u8 = 3;

    const fn from_len_instr(len: u8, instr: Instruction) -> Self {
        assert!(len <= 0b000_11111);
        let instr: u8 = instr.to_u8();
        let instr = instr & Self::INSTR_MASK;
        let slen = (len << Self::LEN_SHIFT) & Self::LEN_MASK;

        Instr(instr | slen)
    }

    fn is_last(&self) -> bool {
        // TODO: Could do with bitops?
        match self.instr() {
            Instruction::S_NotWordNoChildrenNotLast => false,
            Instruction::T_NotWordNoChildrenIsLast => true,
            Instruction::U_NotWordHasChildrenNotLast => false,
            Instruction::V_NotWordHasChildrenIsLast => true,
            Instruction::W_IsWordNoChildrenNotLast => false,
            Instruction::X_IsWordNoChildrenIsLast => true,
            Instruction::Y_IsWordHasChildrenNotLast => false,
            Instruction::Z_IsWordHasChildrenIsLast => true,
        }
    }

    fn is_word(&self) -> bool {
        // TODO: Could do with bitops?
        match self.instr() {
            Instruction::S_NotWordNoChildrenNotLast => false,
            Instruction::T_NotWordNoChildrenIsLast => false,
            Instruction::U_NotWordHasChildrenNotLast => false,
            Instruction::V_NotWordHasChildrenIsLast => false,
            Instruction::W_IsWordNoChildrenNotLast => true,
            Instruction::X_IsWordNoChildrenIsLast => true,
            Instruction::Y_IsWordHasChildrenNotLast => true,
            Instruction::Z_IsWordHasChildrenIsLast => true,
        }
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

    fn has_children(&self) -> bool {
        // TODO: Could do with bitops?
        match self.instr() {
            Instruction::S_NotWordNoChildrenNotLast => false,
            Instruction::T_NotWordNoChildrenIsLast => false,
            Instruction::U_NotWordHasChildrenNotLast => true,
            Instruction::V_NotWordHasChildrenIsLast => true,
            Instruction::W_IsWordNoChildrenNotLast => false,
            Instruction::X_IsWordNoChildrenIsLast => false,
            Instruction::Y_IsWordHasChildrenNotLast => true,
            Instruction::Z_IsWordHasChildrenIsLast => true,
        }
    }
}

struct T9Vm {
    control_stack: Stack<Instr>,
    word_stack: Stack<u8>,
    prio_addr_stack: Stack<usize>,
    program_ctr: usize,
    program: Vec<u8>,
}

impl T9Vm {
    fn reset(&mut self) {
        self.control_stack.clear();
        self.word_stack.clear();
        self.prio_addr_stack.clear();
        self.program_ctr = 0;
    }

    fn pop_cstack(&mut self) -> Instr {
        let val = self.control_stack.pop().unwrap();
        self.word_stack.drop_n(val.len()).unwrap();
        if val.is_word() {
            self.prio_addr_stack.pop().unwrap();
        }
        val
    }

    fn next_word(&mut self) -> Option<&str> {
        println!("+=+= NEXT WORD =+=+");

        // if !children:
        //     pop one
        //     if popped.last:
        //         while peek.last: pop
        //         pop one
        //
        // push + execute
        //
        if let Some(i) = self.control_stack.peek() {
            // if !children:
            if !i.has_children() {

                // pop one
                let val = self.pop_cstack();

                // if popped.last:
                if val.is_last() {
                    // while peek.last: pop
                    while self.control_stack.peek()?.is_last() {
                        self.pop_cstack();
                    }

                    // pop one
                    self.pop_cstack();
                }
            }
        }

        // push + execute
        loop {
            let cur_instr: Instr = (*self.program.get(self.program_ctr)?).into();

            // Push instr onto control stack
            self.control_stack.push(cur_instr)
                .map_err(|_| { debug_assert!(false, "debug-only check failed: Control Stack Overflow"); })
                .ok()?;
            self.program_ctr += 1;

            // If it's a word, grab the priority byte
            if cur_instr.is_word() {
                self.prio_addr_stack.push(self.program_ctr)
                    .map_err(|_| { debug_assert!(false, "debug-only check failed: Prio Stack Overflow"); })
                    .ok()?;
                self.program_ctr += 1;
            }

            // Push word contents onto word stack
            for _ in 0..cur_instr.len() {
                self.word_stack.push(*self.program.get(self.program_ctr)?)
                    .map_err(|_| { debug_assert!(false, "debug-only check failed: Word Stack Overflow"); })
                    .ok()?;
                self.program_ctr += 1;
            }

            if cur_instr.is_word() {
                // TODO: at some point I will need to decode keycode to chars,
                // user should provide scratch buffer
                let word = core::str::from_utf8(self.word_stack.all())
                    .map_err(|_| { debug_assert!(false); })
                    .ok()?;
                println!(" --> {}", word);
                break Some(word);
            }
        }
    }
}

#[cfg(test)]
pub mod test {
    use crate::{Instruction, Instr, T9Vm, Stack};

    const fn u(len: u8) -> u8 {
        Instr::from_len_instr(len, Instruction::U_NotWordHasChildrenNotLast).0
    }
    const fn v(len: u8) -> u8 {
        Instr::from_len_instr(len, Instruction::V_NotWordHasChildrenIsLast).0
    }
    const fn w(len: u8) -> u8 {
        Instr::from_len_instr(len, Instruction::W_IsWordNoChildrenNotLast).0
    }
    const fn x(len: u8) -> u8 {
        Instr::from_len_instr(len, Instruction::X_IsWordNoChildrenIsLast).0
    }
    const fn y(len: u8) -> u8 {
        Instr::from_len_instr(len, Instruction::Y_IsWordHasChildrenNotLast).0
    }
    const fn z(len: u8) -> u8 {
        Instr::from_len_instr(len, Instruction::Z_IsWordHasChildrenIsLast).0
    }

    const PRIO: u8 = 0;
    const DEMO: &[u8] = &[
        y(1), PRIO, b'a',
        y(4), PRIO, b'a', b'r', b'o', b'n',
        x(1), PRIO, b's',
        y(1), PRIO, b'b',
        x(2), PRIO, b'l', b'e',
        u(2), b'p', b'p',
        y(2), PRIO, b'l', b'e',
        w(2), PRIO, b't', b's',
        x(1), PRIO, b's',
        z(4), PRIO, b'n', b'o', b't', b'e',
        v(1), b'_',
        z(1), PRIO, b'a',
        z(1), PRIO, b'b',
        y(1), PRIO, b'c',
        x(1), PRIO, b'd',
        z(1), PRIO, b'e',
        x(1), PRIO, b'f',
        x(1), PRIO, b's',
        x(4), PRIO, b'b', b'i', b't', b'e',
        ];
/*

    app
       note
           _
            a
             b
              c
               d
              e
               f
    s
*/

// if !children:
//     pop one
//     if popped.last:
//         while peek.last: pop
//         pop one
//
// push + execute

    #[test]
    fn smoke() {
        // 000 - S - !word, !children, !last
        // 001 - T - !word, !children,  last
        // 010 - U - !word,  children, !last
        // 011 - V - !word,  children,  last
        // 100 - W -  word, !children, !last
        // 101 - X -  word, !children,  last
        // 110 - Y -  word,  children, !last
        // 111 - Z -  word,  children,  last
        let program = DEMO.iter().copied().collect::<Vec<u8>>();
        let expected = [
            String::from("a"),
            String::from("aaron"),
            String::from("aarons"),
            String::from("ab"),
            String::from("able"),
            String::from("apple"),
            String::from("applets"),
            String::from("apples"),
            String::from("appnote"),
            String::from("appnote_a"),
            String::from("appnote_ab"),
            String::from("appnote_abc"),
            String::from("appnote_abcd"),
            String::from("appnote_abe"),
            String::from("appnote_abef"),
            String::from("as"),
            String::from("bite"),
        ];

        let mut vm = T9Vm {
            control_stack: Stack::<_>::default(),
            word_stack: Stack::<_>::default(),
            prio_addr_stack: Stack::<_>::default(),
            program_ctr: 0,
            program,
        };

        let mut outs = Vec::new();

        while let Some(w) = vm.next_word() {
            outs.push(w.to_string());
        }

        assert_eq!(
            expected.as_slice(),
            outs.as_slice(),
        );
    }

    #[test]
    fn sorted() {
        assert!("a" < "as");
        assert!("as" < "asd");
        assert!("as" < "at");

        // Len of proposed must be >= target
        // proposed[..target.len()] == target
    }

    // #[test]
    // fn submatch() {
    //     // 000 - S - !word, !children, !last
    //     // 001 - T - !word, !children,  last
    //     // 010 - U - !word,  children, !last
    //     // 011 - V - !word,  children,  last
    //     // 100 - W -  word, !children, !last
    //     // 101 - X -  word, !children,  last
    //     // 110 - Y -  word,  children, !last
    //     // 111 - Z -  word,  children,  last
    //     let program = DEMO.iter().copied().collect::<Vec<u8>>();
    //     let expected_a = [
    //         String::from("a"),
    //         String::from("aaron"),
    //         String::from("aarons"),
    //         String::from("ab"),
    //         String::from("able"),
    //         String::from("apple"),
    //         String::from("applets"),
    //         String::from("apples"),
    //         String::from("appnote"),
    //         String::from("as"),
    //     ];
    //     let expected_ap = [
    //         String::from("apple"),
    //         String::from("applets"),
    //         String::from("apples"),
    //         String::from("appnote"),
    //     ];
    //     let expected_app = [
    //         String::from("apple"),
    //         String::from("applets"),
    //         String::from("apples"),
    //     ];
    //     let expected_appl = [
    //         String::from("apple"),
    //         String::from("applets"),
    //         String::from("apples"),
    //     ];
    //     let expected_applz: [String; 0] = [];

    //     let mut vm = T9Vm {
    //         control_stack: Stack::<_>::default(),
    //         word_stack: Stack::<_>::default(),
    //         prio_addr_stack: Stack::<_>::default(),
    //         program_ctr: 0,
    //         program,
    //     };

    //     let mut outs = Vec::new();

    //     while let Some(w) = vm.next_word() {
    //         outs.push(w.to_string());
    //     }

    //     assert_eq!(
    //         expected.as_slice(),
    //         outs.as_slice(),
    //     );
    // }
}

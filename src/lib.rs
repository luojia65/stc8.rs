#![feature(try_from)] 

pub use self::program::Program;

mod program {
    use std::convert::TryFrom;
    use std::ops::{Deref, DerefMut};
    use ihex::reader::Reader as IHexReader;
    use ihex::reader::ReaderError;
    use ihex::record::Record;

    pub struct Program {
        inner: Vec<u8>
    }

    impl Deref for Program {
        type Target = [u8];

        fn deref(&self) -> &Self::Target {
            &self.inner
        }
    }

    impl DerefMut for Program {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.inner
        }
    }

    impl<'a> TryFrom<IHexReader<'a>> for Program {
        type Error = ReaderError;
        fn try_from(src: IHexReader<'a>) -> Result<Self, Self::Error> {
            let mut buf = vec![0u8; 64*1024];
            for line in src {
                if let Record::Data { offset, value } = line? {
                    for (i, &byte) in value.iter().enumerate() {
                        buf[offset as usize + i] = byte;
                    }
                }
            }
            Ok(Program { inner: buf })
        }
    }

    #[cfg(test)]
    mod tests {
        use std::convert::TryFrom;
        use ihex::reader::Reader;
        use super::Program;
        #[test]
        fn program_try_from() {
            let rd = Reader::new(
":0300000002011DDD
:0300C30002010037
:10010000C0E0C082C083C0BA75BA8090FE82E0545D
:10011000BFF0B290D0BAD083D082D0E03275813FA8
:1001200075BA8074C090FE80F0748090FE81F07586
:10013000BA00D2AF75BA80748190FE81F075BA00B2
:0201400080FE3F
:00000001FF");
            let Program { inner: program } = Program::try_from(rd).unwrap();
            assert_eq!(program[0x00], 0x02);
            assert_eq!(program[0xC3], 0x02);
            assert_eq!(program[0x140], 0x80);
        }
    }
}

use std::io::{Error as IoError, Read, Seek, Write};

use easy_ext::ext;

use crate::CpioIterator;

#[ext(WritePadExt)]
pub impl<T: Write> T {
    fn write_padding_with_pos(
        &mut self,
        pos: usize,
        modulo: usize,
        value: u8,
    ) -> Result<usize, IoError> {
        let rem = pos % modulo;
        if rem == 0 {
            return Ok(rem);
        }
        // a heap allocation here is fine
        let padding = vec![value; modulo - rem];
        self.write_all(&padding)?;
        Ok(rem)
    }
}

#[ext(WriteSeekPadExt)]
pub impl<T: Write + Seek> T {
    fn write_padding(&mut self, modulo: usize, value: u8) -> Result<usize, IoError> {
        let pos = self.stream_position()?;
        WritePadExt::write_padding_with_pos(self, pos as usize, modulo, value)
    }
}

#[ext(ReadExt)]
pub impl<T: Read> T {
    #[inline]
    fn read_u32_hex(&mut self) -> Result<u32, IoError> {
        let mut bytes = [0; 8];
        self.read_exact(&mut bytes)?;
        let txt = std::str::from_utf8(&bytes).unwrap();
        Ok(u32::from_str_radix(txt, 16).unwrap())
    }
    fn read_pad(&mut self, size: usize) -> Result<(), IoError> {
        match size % 4 {
            0 => (),
            1 => {
                let mut padding = [0; 3];
                self.read_exact(&mut padding)?;
            }
            2 => {
                let mut padding = [0; 2];
                self.read_exact(&mut padding)?;
            }
            3 => {
                let mut padding = [0; 1];
                self.read_exact(&mut padding)?;
            }
            _ => unreachable!(),
        }
        Ok(())
    }
}

#[ext(WriteExt)]
pub impl<T: Write> T {
    #[inline]
    fn write_u32_hex(&mut self, val: u32) -> Result<(), IoError> {
        // unnecesary heap allocation
        let txt = format!("{:08X}", val);
        let bytes = txt.as_bytes();
        assert_eq!(bytes.len(), 8);
        self.write_all(bytes)?;
        Ok(())
    }
    fn write_pad(&mut self, size: usize) -> Result<(), IoError> {
        match size % 4 {
            0 => (),
            1 => {
                let padding = [0; 3];
                self.write_all(&padding)?;
            }
            2 => {
                let padding = [0; 2];
                self.write_all(&padding)?;
            }
            3 => {
                let padding = [0; 1];
                self.write_all(&padding)?;
            }
            _ => unreachable!(),
        }
        Ok(())
    }
}

#[ext(CpioIterExt)]
pub impl<T: Read> T {
    fn read_as_cpio(&mut self) -> CpioIterator<&mut T> {
        CpioIterator::new(self, false)
    }
    fn read_as_cpio_with_trailer(&mut self) -> CpioIterator<&mut T> {
        CpioIterator::new(self, true)
    }
}

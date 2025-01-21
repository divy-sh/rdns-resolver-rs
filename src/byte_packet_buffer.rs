pub struct BytePacketBuffer {
    pub buf: [u8; 512],
    pub pos: usize,
}

impl BytePacketBuffer {
    pub fn new() -> BytePacketBuffer {
        BytePacketBuffer {
            buf: [0; 512],
            pos: 0,
        }
    }

    fn seek(&mut self, pos: usize) -> Result<(), String> {
        self.pos = pos;
        Ok(())
    }

    fn step(&mut self, steps: usize) -> Result<(), String> {
        self.pos += steps;
        Ok(())
    }

    fn get(&self, pos: usize) -> Result<u8, String> {
        if pos >= 512 {
            return Err("Buffer overflow".to_string());
        }
        Ok(self.buf[pos])
    }

    fn pos(&mut self) -> usize {
        self.pos
    }

    fn read(&mut self) -> Result<u8, String> {
        if self.pos >= 512 {
            return Err("Buffer overflow".to_string());
        }
        let val = self.buf[self.pos];
        self.pos += 1;
        Ok(val)
    }

    fn get_range(&mut self, start: usize, len: usize) -> Result<&[u8], String> {
        if start + len >= 512 {
            return Err("Buffer overflow on reading provided length".to_string());
        }
        Ok(&self.buf[start..start + len])
    }

    fn read_u16(&mut self) -> Result<u16, String> {
        let l = self.read()? as u16;
        let r = self.read()? as u16;
        Ok(l << 8 | r)
    }

    fn read_u32(&mut self) -> Result<u32, String> {
        let l = self.read_u16()? as u32;
        let r = self.read_u16()? as u32;
        Ok(l << 16 | r)
    }

    fn read_qname(&mut self, mut outstr: String) -> Result<(), String> {
        unimplemented!("Need to implement this");
        Ok(())
    }
    
}
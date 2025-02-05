use std::collections::HashMap;

#[derive(Clone)]
pub struct BytePacketBuffer {
    pub buf: [u8; 512],
    pub pos: usize,
    pub qname_pointer: HashMap<String, usize>,
}

impl Default for BytePacketBuffer {
    fn default() -> Self {
        Self::new()
    }
}

impl BytePacketBuffer {
    fn new() -> BytePacketBuffer {
        BytePacketBuffer {
            buf: [0; 512],
            pos: 0,
            qname_pointer: HashMap::new(),
        }
    }

    pub fn seek(&mut self, pos: usize) -> Result<(), String> {
        self.pos = pos;
        Ok(())
    }

    pub fn step(&mut self, steps: usize) -> Result<(), String> {
        self.pos += steps;
        Ok(())
    }

    pub fn get(&self, pos: usize) -> Result<u8, String> {
        if pos >= 512 {
            return Err("Buffer overflow".to_string());
        }
        Ok(self.buf[pos])
    }

    fn pos(&mut self) -> usize {
        self.pos
    }

    pub fn read(&mut self) -> Result<u8, String> {
        if self.pos >= 512 {
            return Err("Buffer overflow".to_string());
        }
        let val = self.buf[self.pos];
        self.pos += 1;
        Ok(val)
    }

    pub fn get_range(&mut self, start: usize, len: usize) -> Result<&[u8], String> {
        if start + len >= 512 {
            return Err("Buffer overflow on reading provided length".to_string());
        }
        Ok(&self.buf[start..start + len])
    }

    pub fn read_u16(&mut self) -> Result<u16, String> {
        let l = self.read()? as u16;
        let r = self.read()? as u16;
        Ok(l << 8 | r)
    }

    pub fn read_u32(&mut self) -> Result<u32, String> {
        let l = self.read_u16()? as u32;
        let r = self.read_u16()? as u32;
        Ok(l << 16 | r)
    }

    pub fn read_qname(&mut self, outstr: &mut String) -> Result<(), String> {
        let mut pos = self.pos();
        let mut jumped = false;
        let mut max_jumps = 10;
        let mut delimiter = "";
        loop {
            if max_jumps < 0 {
                return Err(format!("Max jump limit of {} exceeded", max_jumps).to_string());
            }
            let len = self.get(pos)?;
            if (len & 0xC0) == 0xC0 {
                if !jumped {
                    self.seek(pos + 2)?;
                }
                let b2 = self.get(pos + 1)? as u16;
                let offset = (((len as u16) ^ 0xC0) << 8) | b2;
                pos = offset as usize;
                jumped = true;
                max_jumps -= 1;
                continue;
            } else {
                pos += 1;
                if len == 0 {
                    break;
                }
                outstr.push_str(delimiter);
                let str_buffer = self.get_range(pos, len as usize)?;
                outstr.push_str(&String::from_utf8_lossy(str_buffer).to_lowercase());
                delimiter = ".";
                pos += len as usize;
            }
        }
        if !jumped {
            self.seek(pos)?;
        }
        Ok(())
    }

    pub fn write(&mut self, val: u8) -> Result<(), String> {
        if self.pos >= 512 {
            return Err("Buffer overflow".to_string());
        }
        self.buf[self.pos] = val;
        self.pos += 1;
        Ok(())
    }

    pub fn write_u8(&mut self, val: u8) -> Result<(), String> {
        self.write(val)?;
        Ok(())
    }

    pub fn write_u16(&mut self, val: u16) -> Result<(), String> {
        self.write((val >> 8) as u8)?;
        self.write(val as u8)?;
        Ok(())
    }

    pub fn write_u32(&mut self, val: u32) -> Result<(), String> {
        self.write_u16((val >> 16) as u16)?;
        self.write_u16(val as u16)?;
        Ok(())
    }

    pub fn write_qname(&mut self, qname: &str) -> Result<(), String> {
        let original_pos = self.pos;
        let mut current_pos = 0;
        let qname_bytes = qname.as_bytes();
        let qname_len = qname_bytes.len();
        while current_pos < qname_len {
            let remaining = &qname[current_pos..];
            if let Some(&pointer) = self.qname_pointer.get(remaining) {
                return self.write_u16(0xc000 | pointer as u16);
            }
            let label_end = qname_bytes[current_pos..]
                .iter()
                .position(|&b| b == b'.')
                .map_or(qname_len - current_pos, |p| p);
            if label_end > 63 {
                return Err("DNS label too long".to_string());
            }
            self.write_u8(label_end as u8)?;
            let label_bytes = &qname_bytes[current_pos..current_pos + label_end];
            for &byte in label_bytes {
                self.write(byte)?;
            }
            current_pos += label_end + 1;
            if current_pos >= qname_len {
                break;
            }
        }
        self.qname_pointer.insert(qname.to_string(), original_pos);
        self.write_u8(0)
    }

    pub fn set(&mut self, pos: usize, val: u8) -> Result<(), String> {
        self.buf[pos] = val;
        Ok(())
    }

    pub fn set_u16(&mut self, pos: usize, val: u16) -> Result<(), String> {
        self.set(pos, (val >> 8) as u8)?;
        self.set(pos + 1, val as u8)?;
        Ok(())
    }
}


pub struct Packet {
    data: Vec<u8>,
    offset: usize,
}

impl Packet {
    pub fn new(data: Vec<u8>) -> Self {
        Packet {
            data,
            offset: 0,
        }
    }

    pub fn read_i32(&mut self) -> Option<i32> {
        if self.data.len() < self.offset + 4 {
            println!("invalid read");
            return None;
        }
        let data = &self.data[self.offset .. self.offset + 4];
        let result = i32::from_ne_bytes(data.try_into().unwrap());
        
        self.offset = self.offset + 4;
        return Some(result);
    }

    pub fn read_string(&mut self) -> Option<String> {
        let size_result = self.read_i32();
        if let None = size_result {
            println!("invalid string size");
            return None;
        }

        let size = size_result.unwrap() as usize;
        if self.data.len() < self.offset + size {
            println!("invalid string data");
            return None;
        }

        let result = String::from_utf8_lossy(&self.data[self.offset..self.offset + size]).into_owned();

        self.offset = self.offset + size;
        
        return Some(result);
    }
}
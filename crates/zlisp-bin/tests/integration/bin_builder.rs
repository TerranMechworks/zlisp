pub const INT: i32 = 1;
pub const FLOAT: i32 = 2;
pub const STRING: i32 = 3;
pub const LIST: i32 = 4;
pub const OUTER_LIST_LEN: i32 = 2;
pub const INVALID_TYPE: i32 = 5;

pub struct BinBuilder(Vec<u8>);

impl BinBuilder {
    pub const fn empty() -> Self {
        Self(Vec::new())
    }

    pub fn root() -> Self {
        let mut v = Self(Vec::new());
        v.push_i32(LIST);
        v.push_i32(OUTER_LIST_LEN);
        v
    }

    pub fn build(self) -> Vec<u8> {
        self.0
    }

    fn push_i32(&mut self, v: i32) {
        self.0.extend_from_slice(&v.to_le_bytes());
    }

    fn push_slice(&mut self, v: &[u8]) {
        self.0.extend_from_slice(v);
    }

    pub fn i32(mut self, v: i32) -> Self {
        self.push_i32(v);
        self
    }

    pub fn slice(mut self, v: &[u8]) -> Self {
        self.push_slice(v);
        self
    }

    pub fn int(mut self, v: i32) -> Self {
        self.push_i32(INT);
        self.push_i32(v);
        self
    }

    pub fn float(mut self, v: f32) -> Self {
        self.push_i32(FLOAT);
        self.push_slice(&v.to_le_bytes());
        self
    }

    pub fn str(mut self, s: &str) -> Self {
        let v = s.as_bytes();
        let len = v.len().try_into().unwrap();
        self.push_i32(STRING);
        self.push_i32(len);
        self.push_slice(v);
        self
    }

    pub fn list(mut self, len: usize) -> Self {
        let count: i32 = (len + 1).try_into().unwrap();
        self.push_i32(LIST);
        self.push_i32(count);
        self
    }
}

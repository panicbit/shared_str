
macro_rules! impl_common { () => {
    pub fn new(s: impl Into<String>) -> Self {
        Self::from(s.into())
    }

    pub fn from_slice(owner: &Self, s: &str) -> Option<Self> {
        owner.sliced(s)
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn as_bytes(&self) -> &[u8] {
        unsafe {
            slice::from_raw_parts(self.ptr, self.len)
        }
    }

    pub fn as_str(&self) -> &str {
        unsafe {
            str::from_utf8_unchecked(self.as_bytes())
        }
    }

    pub fn trim(&self) -> Self {
        self.sliced(self.as_str().trim()).unwrap()
    }

    pub fn trim_start(&self) -> Self {
        self.sliced(self.as_str().trim_start()).unwrap()
    }

    pub fn trim_end(&self) -> Self {
        self.sliced(self.as_str().trim_end()).unwrap()
    }

    pub fn sliced(&self, s: &str) -> Option<Self> {
        let start_ptr = self.inner.as_ptr();
        let end_ptr = self.inner[self.inner.len()..].as_ptr();
        let ptr = s.as_ptr();

        if ptr < start_ptr || end_ptr < ptr {
            return None;
        }

        Some(Self {
            ptr,
            len: s.len(),
            inner: self.inner.clone(),
        })
    }

    pub fn slice_with<F>(&self, f: F) -> Option<Self>
    where
        F: FnOnce(&str) -> &str
    {
        self.sliced(f(self.as_str()))
    }
}}

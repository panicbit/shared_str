
macro_rules! impl_common { () => {
    pub fn new(s: impl Into<String>) -> Self {
        Self::from(s.into())
    }

    pub fn from_slice(owner: &Self, s: &str) -> Option<Self> {
        owner.sliced(s)
    }

    pub fn as_bytes(&self) -> &[u8] {
        self.as_str().as_bytes()
    }

    pub fn as_str(&self) -> &str {
        unsafe {
            self.ptr.as_ref()
        }
    }

    pub fn trim(&self) -> Self {
        unsafe { self.slice_with_unchecked(str::trim) }
    }

    pub fn trim_start(&self) -> Self {
        unsafe { self.slice_with_unchecked(str::trim_start) }
    }

    pub fn trim_end(&self) -> Self {
        unsafe { self.slice_with_unchecked(str::trim_end) }
    }

    pub fn owns(&self, slice: &str) -> bool {
        let start_ptr = self.inner.as_ptr();
        let end_ptr = self.inner[self.inner.len()..].as_ptr();
        let ptr = slice.as_ptr();

        (start_ptr ..= end_ptr).contains(&ptr)
    }

    pub fn sliced(&self, slice: &str) -> Option<Self> {
        unsafe {
            if slice.is_empty() {
                return Some(self.sliced_unchecked(&self[..0]));
            }

            if !self.owns(slice) {
                return None;
            }

            Some(self.sliced_unchecked(slice))
        }
    }

    pub unsafe fn sliced_unchecked(&self, slice: &str) -> Self {
        Self {
            ptr: NonNull::from(slice),
            inner: self.inner.clone(),
        }
    }

    pub fn slice_with<F>(&self, f: F) -> Option<Self>
    where
        F: FnOnce(&str) -> &str
    {
        self.sliced(f(self.as_str()))
    }

    pub unsafe fn slice_with_unchecked<F>(&self, f: F) -> Self
    where
        F: FnOnce(&str) -> &str
    {
        self.sliced_unchecked(f(self.as_str()))
    }
}}

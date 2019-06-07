use std::str;
use std::sync::Arc;
use std::slice;
use std::ops::Deref;

pub struct ArcStr {
    ptr: *const u8,
    len: usize,
    inner: Arc<str>,
}

impl ArcStr {
    pub fn new(s: impl Into<String>) -> Self {
        ArcStr::from(s.into())
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
}

impl From<String> for ArcStr {
    fn from(s: String) -> Self {
        Self::from(Arc::<str>::from(s))
    }
}

impl From<Arc<str>> for ArcStr {
    fn from(inner: Arc<str>) -> Self {
        Self {
            ptr: inner.as_ptr(),
            len: inner.len(),
            inner,
        }
    }
}

impl Deref for ArcStr {
    type Target = str;

    fn deref(&self) -> &str {
        self.as_str()
    }
}

#[cfg(test)]
mod tests {
    use super::ArcStr;

    const STR: &str = "hello world";

    #[test]
    fn sliced() {
        let rcs = ArcStr::new(STR);
        let str = rcs.as_str();

        assert_eq!(rcs.sliced(&str[0..]).unwrap().as_str(), &STR[0..]);
        assert_eq!(rcs.sliced(&str[..str.len()]).unwrap().as_str(), &STR[..STR.len()]);
        assert_eq!(rcs.sliced(&str[3..6]).unwrap().as_str(), &STR[3..6]);
        assert!(rcs.sliced("foo").is_none());
    }

    #[test]
    fn len() {
        let rcs = ArcStr::new(STR);
        assert_eq!(rcs.len(), STR.len());
    }

    #[test]
    fn as_str() {
        let rcs = ArcStr::new(STR);
        assert_eq!(rcs.as_str(), STR);
    }
}

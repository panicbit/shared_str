

macro_rules! impl_shared_str { ($OUTER:ident, $INNER:ident) => {

use std::ptr::NonNull;

#[derive(Clone,Debug,PartialEq,Eq)]
pub struct $OUTER {
    ptr: NonNull<str>,
    inner: $INNER<str>,
}

impl $OUTER {
    pub fn new(s: impl Into<String>) -> Self {
        Self::from(s.into())
    }

    pub fn from_slice(owner: &Self, s: &str) -> Option<Self> {
        owner.rejoin(s)
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

    pub fn rejoin(&self, slice: &str) -> Option<Self> {
        unsafe {
            if slice.is_empty() {
                return Some(self.rejoin_unchecked(&self[..0]));
            }

            if !self.owns(slice) {
                return None;
            }

            Some(self.rejoin_unchecked(slice))
        }
    }

    pub unsafe fn rejoin_unchecked(&self, slice: &str) -> Self {
        Self {
            ptr: NonNull::from(slice),
            inner: self.inner.clone(),
        }
    }

    pub fn slice_with<F>(&self, f: F) -> Option<Self>
    where
        F: FnOnce(&str) -> &str
    {
        self.rejoin(f(self.as_str()))
    }

    pub unsafe fn slice_with_unchecked<F>(&self, f: F) -> Self
    where
        F: FnOnce(&str) -> &str
    {
        self.rejoin_unchecked(f(self.as_str()))
    }
}

impl From<$INNER<str>> for $OUTER {
    fn from(inner: $INNER<str>) -> Self {
        Self {
            ptr: NonNull::from(&*inner),
            inner,
        }
    }
}

impl From<String> for $OUTER {
    fn from(s: String) -> Self {
        Self::from($INNER::<str>::from(s))
    }
}

impl From<&'_ str> for $OUTER {
    fn from(s: &'_ str) -> Self {
        Self::from($INNER::<str>::from(s))
    }
}

impl std::ops::Deref for $OUTER {
    type Target = str;

    fn deref(&self) -> &str {
        Self::as_str(self)
    }
}

impl std::fmt::Display for $OUTER {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::$OUTER;

    const STR: &str = "hello world";

    #[test]
    fn rejoin() {
        let rcs = $OUTER::new(STR);
        let str = rcs.as_str();

        assert_eq!(rcs.rejoin(&str[0..]).unwrap().as_str(), &STR[0..]);
        assert_eq!(rcs.rejoin(&str[..str.len()]).unwrap().as_str(), &STR[..STR.len()]);
        assert_eq!(rcs.rejoin(&str[3..6]).unwrap().as_str(), &STR[3..6]);
        assert!(rcs.rejoin("foo").is_none());
    }

    #[test]
    fn len() {
        let rcs = $OUTER::new(STR);
        assert_eq!(rcs.len(), STR.len());
    }

    #[test]
    fn as_str() {
        let rcs = $OUTER::new(STR);
        assert_eq!(rcs.as_str(), STR);
    }
}

}}

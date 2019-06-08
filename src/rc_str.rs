use std::str;
use std::rc::Rc;
use std::slice;
use std::ops::Deref;

#[derive(Clone)]
pub struct RcStr {
    ptr: *const u8,
    len: usize,
    inner: Rc<str>,
}

impl RcStr {
    impl_common!();
}

impl From<&'_ str> for RcStr {
    fn from(s: &'_ str) -> Self {
        Self::from(Rc::<str>::from(s))
    }
}

impl From<String> for RcStr {
    fn from(s: String) -> Self {
        Self::from(Rc::<str>::from(s))
    }
}

impl From<Rc<str>> for RcStr {
    fn from(inner: Rc<str>) -> Self {
        Self {
            ptr: inner.as_ptr(),
            len: inner.len(),
            inner,
        }
    }
}

impl Deref for RcStr {
    type Target = str;

    fn deref(&self) -> &str {
        self.as_str()
    }
}

#[cfg(test)]
mod tests {
    use super::RcStr;

    const STR: &str = "hello world";

    #[test]
    fn sliced() {
        let rcs = RcStr::new(STR);
        let str = rcs.as_str();

        assert_eq!(rcs.sliced(&str[0..]).unwrap().as_str(), &STR[0..]);
        assert_eq!(rcs.sliced(&str[..str.len()]).unwrap().as_str(), &STR[..STR.len()]);
        assert_eq!(rcs.sliced(&str[3..6]).unwrap().as_str(), &STR[3..6]);
        assert!(rcs.sliced("foo").is_none());
    }

    #[test]
    fn len() {
        let rcs = RcStr::new(STR);
        assert_eq!(rcs.len(), STR.len());
    }

    #[test]
    fn as_str() {
        let rcs = RcStr::new(STR);
        assert_eq!(rcs.as_str(), STR);
    }
}

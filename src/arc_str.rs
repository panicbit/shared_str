use std::str;
use std::sync::Arc;
use std::slice;
use std::ops::Deref;
use std::ptr::NonNull;

#[derive(Clone)]
pub struct ArcStr {
    ptr: NonNull<str>,
    inner: Arc<str>,
}

unsafe impl Send for ArcStr {}
unsafe impl Sync for ArcStr {}

impl ArcStr {
    impl_common!();
}

impl From<&'_ str> for ArcStr {
    fn from(s: &'_ str) -> Self {
        Self::from(Arc::<str>::from(s))
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
            ptr: NonNull::from(&*inner),
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

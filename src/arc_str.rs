use std::sync::Arc;

impl_shared_str!(ArcStr, Arc);

unsafe impl Send for ArcStr {}
unsafe impl Sync for ArcStr {}

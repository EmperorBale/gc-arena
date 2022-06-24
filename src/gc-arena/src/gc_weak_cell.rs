use crate::collect::Collect;
use crate::GcCell;

use core::fmt::{self, Debug};

pub struct GcWeakCell<'gc, T: 'gc + Collect> {
    pub(crate) inner: GcCell<'gc, T>,
}

impl<'gc, T: Collect + 'gc> Copy for GcWeakCell<'gc, T> {}

impl<'gc, T: Collect + 'gc> Clone for GcWeakCell<'gc, T> {
    fn clone(&self) -> GcWeakCell<'gc, T> {
        Self { inner: self.inner }
    }
}

impl<'gc, T: 'gc + Collect> Debug for GcWeakCell<'gc, T> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "(GcWeakCell)")
    }
}

unsafe impl<'gc, T: 'gc + Collect> Collect for GcWeakCell<'gc, T> {
    fn trace(&self, _cc: crate::CollectionContext) {
        unsafe {
            self.inner
                .get_inner()
                .ptr
                .as_ref()
                .flags
                .set_has_weak_ref(true);
        }
    }
}

impl<'gc, T: Collect + 'gc> GcWeakCell<'gc, T> {
    pub fn upgrade(&self) -> Option<GcCell<'gc, T>> {
        unsafe {
            self.inner
                .get_inner()
                .ptr
                .as_ref()
                .flags
                .alive()
                .then(|| self.inner)
        }
    }
}

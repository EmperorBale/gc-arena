use crate::collect::Collect;
use crate::gc::Gc;

use alloc::rc::Rc;
use core::cell::Cell;
use core::fmt::{self, Debug};

pub struct GcWeak<'gc, T: 'gc + Collect> {
    pub(crate) alive: Rc<Cell<bool>>,
    pub(crate) inner: Gc<'gc, T>,
}

impl<'gc, T: Collect + 'gc> Clone for GcWeak<'gc, T> {
    fn clone(&self) -> GcWeak<'gc, T> {
        Self {
            alive: self.alive.clone(),
            inner: self.inner,
        }
    }
}

impl<'gc, T: 'gc + Collect> Debug for GcWeak<'gc, T> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "(GcWeak)")
    }
}

unsafe impl<'gc, T: 'gc + Collect> Collect for GcWeak<'gc, T> {
    fn needs_trace() -> bool {
        false
    }
}

impl<'gc, T: Collect + 'gc> GcWeak<'gc, T> {
    pub fn upgrade(&self) -> Option<Gc<'gc, T>> {
        if !self.alive.get() {
            return None;
        }

        Some(self.inner.clone())
    }
}

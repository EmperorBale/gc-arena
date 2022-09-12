use crate::collect::Collect;
use crate::gc::Gc;
use crate::types::GcColor;
use crate::{CollectionContext, MutationContext};

use core::fmt::{self, Debug};

pub struct GcWeak<'gc, T: 'gc + Collect> {
    pub(crate) inner: Gc<'gc, T>,
}

impl<'gc, T: Collect + 'gc> Copy for GcWeak<'gc, T> {}

impl<'gc, T: Collect + 'gc> Clone for GcWeak<'gc, T> {
    fn clone(&self) -> GcWeak<'gc, T> {
        *self
    }
}

impl<'gc, T: 'gc + Collect> Debug for GcWeak<'gc, T> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "(GcWeak)")
    }
}

unsafe impl<'gc, T: 'gc + Collect> Collect for GcWeak<'gc, T> {
    fn trace(&self, _cc: CollectionContext) {
        unsafe {
            let gc = self.inner.ptr.as_ref();
            gc.flags.set_has_weak_ref(true);
            if gc.flags.color() == GcColor::FreshWhite {
                gc.flags.set_color(GcColor::White);
            }
        }
    }
}

impl<'gc, T: Collect + 'gc> GcWeak<'gc, T> {
    pub fn upgrade(&self, mc: MutationContext<'gc, '_>) -> Option<Gc<'gc, T>> {
        unsafe { mc.upgrade(self.inner.ptr).then(|| self.inner) }
    }
}

use core::cell::{Cell, UnsafeCell};
use core::marker::PhantomData;
use core::ptr::NonNull;

use crate::collect::Collect;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub(crate) enum GcColor {
    White,
    Gray,
    Black,
}

pub(crate) struct GcBox<T: Collect + ?Sized> {
    pub(crate) flags: GcFlags,
    pub(crate) sweep_id: usize,
    pub(crate) next: Cell<Option<NonNull<GcBox<dyn Collect>>>>,
    pub(crate) value: UnsafeCell<T>,
}

pub(crate) struct GcFlags(Cell<u8>);

impl GcFlags {
    #[inline]
    pub(crate) fn new() -> GcFlags {
        GcFlags(Cell::new(0))
    }

    #[inline]
    pub(crate) fn color(&self) -> GcColor {
        match self.0.get() & 0x3 {
            0x0 => GcColor::White,
            0x1 => GcColor::Gray,
            0x2 => GcColor::Black,
            // this is needed for the compiler to codegen a simple AND.
            // SAFETY: only possible extra value is 0x3,
            // and the only place where we set these bits is in set_color
            _ => unsafe { std::hint::unreachable_unchecked() },
        }
    }

    #[inline]
    pub(crate) fn set_color(&self, color: GcColor) {
        self.0.set(
            (self.0.get() & !0x3)
                | match color {
                    GcColor::White => 0x0,
                    GcColor::Gray => 0x1,
                    GcColor::Black => 0x2,
                },
        )
    }

    #[inline]
    pub(crate) fn needs_trace(&self) -> bool {
        self.0.get() & 0x4 != 0x0
    }

    #[inline]
    pub(crate) fn has_weak_ref(&self) -> bool {
        self.0.get() & 0x8 != 0x0
    }

    #[inline]
    pub(crate) fn alive(&self) -> bool {
        self.0.get() & 0x10 != 0x0
    }

    #[inline]
    pub(crate) fn set_needs_trace(&self, needs_trace: bool) {
        self.0
            .set((self.0.get() & !0x4) | if needs_trace { 0x4 } else { 0x0 });
    }

    #[inline]
    pub(crate) fn set_has_weak_ref(&self, has_weak_ref: bool) {
        self.0
            .set((self.0.get() & !0x8) | if has_weak_ref { 0x8 } else { 0x0 });
    }

    #[inline]
    pub(crate) fn set_alive(&self, alive: bool) {
        self.0
            .set((self.0.get() & !0x10) | if alive { 0x10 } else { 0x0 });
    }
}

// Phantom type that holds a lifetime and ensures that it is invariant.
pub(crate) type Invariant<'a> = PhantomData<Cell<&'a ()>>;

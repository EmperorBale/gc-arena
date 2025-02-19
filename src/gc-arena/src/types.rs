use core::cell::{Cell, UnsafeCell};
use core::marker::PhantomData;
use core::ptr::NonNull;

use crate::collect::Collect;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub(crate) enum GcColor {
    FreshWhite,
    White,
    Gray,
    Black,
}

pub(crate) struct GcBox<T: Collect + ?Sized> {
    pub(crate) flags: GcFlags,
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
            0x0 => GcColor::FreshWhite,
            0x1 => GcColor::White,
            0x2 => GcColor::Gray,
            0x3 => GcColor::Black,
            _ => unreachable!(),
        }
    }

    #[inline]
    pub(crate) fn set_color(&self, color: GcColor) {
        self.0.set(
            (self.0.get() & !0x3)
                | match color {
                    GcColor::FreshWhite => 0x0,
                    GcColor::White => 0x1,
                    GcColor::Gray => 0x2,
                    GcColor::Black => 0x3,
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

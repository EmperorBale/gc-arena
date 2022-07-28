#![no_std]

#[cfg(feature = "std")]
extern crate std;

extern crate alloc;

#[doc(hidden)]
pub use gc_arena_derive::*;

mod arena;
mod collect;
mod collect_impl;
mod context;
mod gc;
mod gc_cell;
mod gc_static;
mod gc_static_cell;
mod no_drop;
mod static_collect;
mod types;

pub use self::{
    arena::{rootless_arena, ArenaParameters},
    collect::Collect,
    context::{CollectionContext, Context, MutationContext, SharedGcData},
    gc::Gc,
    gc_cell::GcCell,
    no_drop::MustNotImplDrop,
    static_collect::StaticCollect,
};

#[macro_export]
macro_rules! static_gc {
    ($arena: ident, $typ: ty) => {
        pub mod $arena {
            use $crate::{Gc, GcCell};
            #[derive(Debug)]
            pub struct StaticArena<'gc> {
                root: Gc<'gc, $typ>,
                shared: std::rc::Rc<core::cell::RefCell<$crate::SharedGcData>>,
            }

            pub struct RootWrapper<'gc> {
                root: Gc<'gc, $typ>,
            }

            impl<'gc> std::ops::Deref for RootWrapper<'gc> {
                type Target = Gc<'gc, $typ>;

                fn deref(&self) -> &Self::Target {
                    &self.root
                }
            }

            impl<'gc> StaticArena<'gc> {
                pub fn wrap(
                    mc: $crate::MutationContext<'gc, '_>,
                    root: $crate::Gc<'gc, $typ>,
                ) -> StaticArena<'static> {
                    unsafe {
                        $crate::Gc::make_static(mc, root);
                        let arena = Self {
                            root,
                            shared: mc.shared_data(),
                        };
                        std::mem::transmute(arena)
                    }
                }
            }

            impl StaticArena<'static> {
                pub fn read(&self, f: impl for<'read> FnOnce(RootWrapper<'read>)) {
                    assert!(self.shared.borrow().alive_flag);
                    if !self.shared.borrow().read_lock {
                        self.shared.borrow_mut().read_lock = true;
                        f(RootWrapper { root: self.root });
                        self.shared.borrow_mut().read_lock = false;
                    } else {
                        f(RootWrapper { root: self.root });
                    }
                }
            }
        }
    };
}

// SPDX-License-Identifier: Apache-2.0 OR MIT

macro_rules! atomic64 {
    ($atomic_type:ident, $int_type:ident, $atomic_max:ident, $atomic_min:ident) => {
        #[repr(C, align(8))]
        pub(crate) struct $atomic_type {
            v: core::cell::UnsafeCell<$int_type>,
        }

        // Send is implicitly implemented.
        // SAFETY: any data races are prevented by atomic intrinsics, the kernel user helper, or the lock.
        unsafe impl Sync for $atomic_type {}

        impl_default_no_fetch_ops!($atomic_type, $int_type);
        impl_default_bit_opts!($atomic_type, $int_type);
        impl $atomic_type {
            #[inline]
            pub(crate) const fn new(v: $int_type) -> Self {
                Self { v: core::cell::UnsafeCell::new(v) }
            }

            #[inline]
            pub(crate) fn is_lock_free() -> bool {
                is_lock_free()
            }
            pub(crate) const IS_ALWAYS_LOCK_FREE: bool = IS_ALWAYS_LOCK_FREE;

            #[inline]
            pub(crate) fn get_mut(&mut self) -> &mut $int_type {
                // SAFETY: the mutable reference guarantees unique ownership.
                // (UnsafeCell::get_mut requires Rust 1.50)
                unsafe { &mut *self.v.get() }
            }

            #[inline]
            #[cfg_attr(all(debug_assertions, not(portable_atomic_no_track_caller)), track_caller)]
            pub(crate) fn load(&self, order: Ordering) -> $int_type {
                crate::utils::assert_load_ordering(order);
                #[allow(clippy::cast_possible_wrap, clippy::cast_sign_loss)]
                // SAFETY: any data races are prevented by atomic intrinsics, the kernel user helper, or the lock
                // and the raw pointer passed in is valid because we got it from a reference.
                unsafe {
                    atomic_load(self.v.get().cast::<u64>(), order) as $int_type
                }
            }

            #[inline]
            #[cfg_attr(all(debug_assertions, not(portable_atomic_no_track_caller)), track_caller)]
            pub(crate) fn store(&self, val: $int_type, order: Ordering) {
                crate::utils::assert_store_ordering(order);
                #[allow(clippy::cast_possible_wrap, clippy::cast_sign_loss)]
                // SAFETY: any data races are prevented by atomic intrinsics, the kernel user helper, or the lock
                // and the raw pointer passed in is valid because we got it from a reference.
                unsafe {
                    atomic_store(self.v.get().cast::<u64>(), val as u64, order)
                }
            }

            #[inline]
            pub(crate) fn swap(&self, val: $int_type, order: Ordering) -> $int_type {
                #[allow(clippy::cast_possible_wrap, clippy::cast_sign_loss)]
                // SAFETY: any data races are prevented by atomic intrinsics, the kernel user helper, or the lock
                // and the raw pointer passed in is valid because we got it from a reference.
                unsafe {
                    atomic_swap(self.v.get().cast::<u64>(), val as u64, order) as $int_type
                }
            }

            #[inline]
            #[cfg_attr(all(debug_assertions, not(portable_atomic_no_track_caller)), track_caller)]
            pub(crate) fn compare_exchange(
                &self,
                current: $int_type,
                new: $int_type,
                success: Ordering,
                failure: Ordering,
            ) -> Result<$int_type, $int_type> {
                crate::utils::assert_compare_exchange_ordering(success, failure);
                #[allow(clippy::cast_possible_wrap, clippy::cast_sign_loss)]
                // SAFETY: any data races are prevented by atomic intrinsics, the kernel user helper, or the lock
                // and the raw pointer passed in is valid because we got it from a reference.
                unsafe {
                    match atomic_compare_exchange(
                        self.v.get().cast::<u64>(),
                        current as u64,
                        new as u64,
                        success,
                        failure,
                    ) {
                        Ok(v) => Ok(v as $int_type),
                        Err(v) => Err(v as $int_type),
                    }
                }
            }

            #[inline]
            #[cfg_attr(all(debug_assertions, not(portable_atomic_no_track_caller)), track_caller)]
            pub(crate) fn compare_exchange_weak(
                &self,
                current: $int_type,
                new: $int_type,
                success: Ordering,
                failure: Ordering,
            ) -> Result<$int_type, $int_type> {
                crate::utils::assert_compare_exchange_ordering(success, failure);
                #[allow(clippy::cast_possible_wrap, clippy::cast_sign_loss)]
                // SAFETY: any data races are prevented by atomic intrinsics, the kernel user helper, or the lock
                // and the raw pointer passed in is valid because we got it from a reference.
                unsafe {
                    match atomic_compare_exchange_weak(
                        self.v.get().cast::<u64>(),
                        current as u64,
                        new as u64,
                        success,
                        failure,
                    ) {
                        Ok(v) => Ok(v as $int_type),
                        Err(v) => Err(v as $int_type),
                    }
                }
            }

            #[inline]
            pub(crate) fn fetch_add(&self, val: $int_type, order: Ordering) -> $int_type {
                #[allow(clippy::cast_possible_wrap, clippy::cast_sign_loss)]
                // SAFETY: any data races are prevented by atomic intrinsics, the kernel user helper, or the lock
                // and the raw pointer passed in is valid because we got it from a reference.
                unsafe {
                    atomic_add(self.v.get().cast::<u64>(), val as u64, order) as $int_type
                }
            }

            #[inline]
            pub(crate) fn fetch_sub(&self, val: $int_type, order: Ordering) -> $int_type {
                #[allow(clippy::cast_possible_wrap, clippy::cast_sign_loss)]
                // SAFETY: any data races are prevented by atomic intrinsics, the kernel user helper, or the lock
                // and the raw pointer passed in is valid because we got it from a reference.
                unsafe {
                    atomic_sub(self.v.get().cast::<u64>(), val as u64, order) as $int_type
                }
            }

            #[inline]
            pub(crate) fn fetch_and(&self, val: $int_type, order: Ordering) -> $int_type {
                #[allow(clippy::cast_possible_wrap, clippy::cast_sign_loss)]
                // SAFETY: any data races are prevented by atomic intrinsics, the kernel user helper, or the lock
                // and the raw pointer passed in is valid because we got it from a reference.
                unsafe {
                    atomic_and(self.v.get().cast::<u64>(), val as u64, order) as $int_type
                }
            }

            #[inline]
            pub(crate) fn fetch_nand(&self, val: $int_type, order: Ordering) -> $int_type {
                #[allow(clippy::cast_possible_wrap, clippy::cast_sign_loss)]
                // SAFETY: any data races are prevented by atomic intrinsics, the kernel user helper, or the lock
                // and the raw pointer passed in is valid because we got it from a reference.
                unsafe {
                    atomic_nand(self.v.get().cast::<u64>(), val as u64, order) as $int_type
                }
            }

            #[inline]
            pub(crate) fn fetch_or(&self, val: $int_type, order: Ordering) -> $int_type {
                #[allow(clippy::cast_possible_wrap, clippy::cast_sign_loss)]
                // SAFETY: any data races are prevented by atomic intrinsics, the kernel user helper, or the lock
                // and the raw pointer passed in is valid because we got it from a reference.
                unsafe {
                    atomic_or(self.v.get().cast::<u64>(), val as u64, order) as $int_type
                }
            }

            #[inline]
            pub(crate) fn fetch_xor(&self, val: $int_type, order: Ordering) -> $int_type {
                #[allow(clippy::cast_possible_wrap, clippy::cast_sign_loss)]
                // SAFETY: any data races are prevented by atomic intrinsics, the kernel user helper, or the lock
                // and the raw pointer passed in is valid because we got it from a reference.
                unsafe {
                    atomic_xor(self.v.get().cast::<u64>(), val as u64, order) as $int_type
                }
            }

            #[inline]
            pub(crate) fn fetch_max(&self, val: $int_type, order: Ordering) -> $int_type {
                #[allow(clippy::cast_possible_wrap, clippy::cast_sign_loss)]
                // SAFETY: any data races are prevented by atomic intrinsics, the kernel user helper, or the lock
                // and the raw pointer passed in is valid because we got it from a reference.
                unsafe {
                    $atomic_max(self.v.get().cast::<u64>(), val as u64, order) as $int_type
                }
            }

            #[inline]
            pub(crate) fn fetch_min(&self, val: $int_type, order: Ordering) -> $int_type {
                #[allow(clippy::cast_possible_wrap, clippy::cast_sign_loss)]
                // SAFETY: any data races are prevented by atomic intrinsics, the kernel user helper, or the lock
                // and the raw pointer passed in is valid because we got it from a reference.
                unsafe {
                    $atomic_min(self.v.get().cast::<u64>(), val as u64, order) as $int_type
                }
            }

            #[inline]
            pub(crate) fn fetch_not(&self, order: Ordering) -> $int_type {
                #[allow(clippy::cast_possible_wrap, clippy::cast_sign_loss)]
                // SAFETY: any data races are prevented by atomic intrinsics, the kernel user helper, or the lock
                // and the raw pointer passed in is valid because we got it from a reference.
                unsafe {
                    atomic_not(self.v.get().cast::<u64>(), order) as $int_type
                }
            }
            #[inline]
            pub(crate) fn not(&self, order: Ordering) {
                self.fetch_not(order);
            }

            #[inline]
            pub(crate) fn fetch_neg(&self, order: Ordering) -> $int_type {
                #[allow(clippy::cast_possible_wrap, clippy::cast_sign_loss)]
                // SAFETY: any data races are prevented by atomic intrinsics, the kernel user helper, or the lock
                // and the raw pointer passed in is valid because we got it from a reference.
                unsafe {
                    atomic_neg(self.v.get().cast::<u64>(), order) as $int_type
                }
            }
            #[inline]
            pub(crate) fn neg(&self, order: Ordering) {
                self.fetch_neg(order);
            }

            #[inline]
            pub(crate) const fn as_ptr(&self) -> *mut $int_type {
                self.v.get()
            }
        }
    };
}

#[cfg(target_arch = "riscv32")]
macro_rules! atomic_rmw_by_atomic_update {
    () => {
        #[inline]
        #[cfg_attr(miri, track_caller)] // even without panics, this helps for Miri backtraces
        unsafe fn atomic_swap(dst: *mut u64, val: u64, order: Ordering) -> u64 {
            // SAFETY: the caller must uphold the safety contract.
            unsafe { atomic_update(dst, order, |_| val) }
        }
        #[inline]
        #[cfg_attr(miri, track_caller)] // even without panics, this helps for Miri backtraces
        unsafe fn atomic_add(dst: *mut u64, val: u64, order: Ordering) -> u64 {
            // SAFETY: the caller must uphold the safety contract.
            unsafe { atomic_update(dst, order, |x| x.wrapping_add(val)) }
        }
        #[inline]
        #[cfg_attr(miri, track_caller)] // even without panics, this helps for Miri backtraces
        unsafe fn atomic_sub(dst: *mut u64, val: u64, order: Ordering) -> u64 {
            // SAFETY: the caller must uphold the safety contract.
            unsafe { atomic_update(dst, order, |x| x.wrapping_sub(val)) }
        }
        #[inline]
        #[cfg_attr(miri, track_caller)] // even without panics, this helps for Miri backtraces
        unsafe fn atomic_and(dst: *mut u64, val: u64, order: Ordering) -> u64 {
            // SAFETY: the caller must uphold the safety contract.
            unsafe { atomic_update(dst, order, |x| x & val) }
        }
        #[inline]
        #[cfg_attr(miri, track_caller)] // even without panics, this helps for Miri backtraces
        unsafe fn atomic_nand(dst: *mut u64, val: u64, order: Ordering) -> u64 {
            // SAFETY: the caller must uphold the safety contract.
            unsafe { atomic_update(dst, order, |x| !(x & val)) }
        }
        #[inline]
        #[cfg_attr(miri, track_caller)] // even without panics, this helps for Miri backtraces
        unsafe fn atomic_or(dst: *mut u64, val: u64, order: Ordering) -> u64 {
            // SAFETY: the caller must uphold the safety contract.
            unsafe { atomic_update(dst, order, |x| x | val) }
        }
        #[inline]
        #[cfg_attr(miri, track_caller)] // even without panics, this helps for Miri backtraces
        unsafe fn atomic_xor(dst: *mut u64, val: u64, order: Ordering) -> u64 {
            // SAFETY: the caller must uphold the safety contract.
            unsafe { atomic_update(dst, order, |x| x ^ val) }
        }
        #[inline]
        #[cfg_attr(miri, track_caller)] // even without panics, this helps for Miri backtraces
        unsafe fn atomic_not(dst: *mut u64, order: Ordering) -> u64 {
            // SAFETY: the caller must uphold the safety contract.
            unsafe { atomic_update(dst, order, |x| !x) }
        }
        #[inline]
        #[cfg_attr(miri, track_caller)] // even without panics, this helps for Miri backtraces
        unsafe fn atomic_neg(dst: *mut u64, order: Ordering) -> u64 {
            // SAFETY: the caller must uphold the safety contract.
            unsafe { atomic_update(dst, order, u64::wrapping_neg) }
        }
        atomic_rmw_by_atomic_update!(cmp);
    };
    (cmp) => {
        #[inline]
        #[cfg_attr(miri, track_caller)] // even without panics, this helps for Miri backtraces
        unsafe fn atomic_max(dst: *mut u64, val: u64, order: Ordering) -> u64 {
            #[allow(clippy::cast_possible_wrap, clippy::cast_sign_loss)]
            // SAFETY: the caller must uphold the safety contract.
            unsafe {
                atomic_update(dst, order, |x| core::cmp::max(x as i64, val as i64) as u64)
            }
        }
        #[inline]
        #[cfg_attr(miri, track_caller)] // even without panics, this helps for Miri backtraces
        unsafe fn atomic_umax(dst: *mut u64, val: u64, order: Ordering) -> u64 {
            // SAFETY: the caller must uphold the safety contract.
            unsafe { atomic_update(dst, order, |x| core::cmp::max(x, val)) }
        }
        #[inline]
        #[cfg_attr(miri, track_caller)] // even without panics, this helps for Miri backtraces
        unsafe fn atomic_min(dst: *mut u64, val: u64, order: Ordering) -> u64 {
            #[allow(clippy::cast_possible_wrap, clippy::cast_sign_loss)]
            // SAFETY: the caller must uphold the safety contract.
            unsafe {
                atomic_update(dst, order, |x| core::cmp::min(x as i64, val as i64) as u64)
            }
        }
        #[inline]
        #[cfg_attr(miri, track_caller)] // even without panics, this helps for Miri backtraces
        unsafe fn atomic_umin(dst: *mut u64, val: u64, order: Ordering) -> u64 {
            // SAFETY: the caller must uphold the safety contract.
            unsafe { atomic_update(dst, order, |x| core::cmp::min(x, val)) }
        }
    };
}

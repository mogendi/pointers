use crate::cell::Cell;
use std::cell::UnsafeCell;
use std::ops::{Deref, DerefMut};

#[derive(Clone, Copy)]
enum RefState {
    UnReferenced,
    Referenced(isize),
    Exclusive,
}

/// !Sync implied by UnsafeCell
pub struct RefCell<T> {
    value: UnsafeCell<T>,
    ref_state: Cell<RefState>,
}

impl<T> RefCell<T> {
    pub fn new(value: T) -> Self {
        RefCell {
            value: UnsafeCell::new(value),
            ref_state: Cell::new(RefState::UnReferenced),
        }
    }

    pub fn borrow(&self) -> Option<Ref<'_, T>> {
        match self.ref_state.get() {
            RefState::UnReferenced => {
                self.ref_state.set(RefState::Referenced(1));
                Some(Ref { refcell: self })
            }
            RefState::Referenced(n) => {
                self.ref_state.set(RefState::Referenced(n + 1));
                Some(Ref { refcell: self })
            }
            RefState::Exclusive => None,
        }
    }

    pub fn borrow_mut(&self) -> Option<RefMut<'_, T>> {
        match self.ref_state.get() {
            RefState::UnReferenced => {
                self.ref_state.set(RefState::Exclusive);
                Some(RefMut { refcell: self })
            }
            _ => None,
        }
    }
}

pub struct Ref<'refcell, T> {
    refcell: &'refcell RefCell<T>,
}

impl<T> Deref for Ref<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        // Only given out if state of the parent
        // refcell is Unreferenced/ Referenced (shared)
        unsafe { &*self.refcell.value.get() }
    }
}

impl<T> Drop for Ref<'_, T> {
    fn drop(&mut self) {
        match self.refcell.ref_state.get() {
            RefState::Referenced(1) => self.refcell.ref_state.set(RefState::UnReferenced),
            RefState::Referenced(n) => self.refcell.ref_state.set(RefState::Referenced(n - 1)),
            _ => unreachable!(),
        }
    }
}

pub struct RefMut<'refcell, T> {
    refcell: &'refcell RefCell<T>,
}

impl<T> Deref for RefMut<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.refcell.value.get() }
    }
}

impl<T> DerefMut for RefMut<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.refcell.value.get() }
    }
}

impl<T> Drop for RefMut<'_, T> {
    fn drop(&mut self) {
        match self.refcell.ref_state.get() {
            RefState::Referenced(_) | RefState::UnReferenced => unreachable!(),
            RefState::Exclusive => {
                self.refcell.ref_state.set(RefState::UnReferenced);
            }
        }
    }
}

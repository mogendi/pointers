use crate::cell::Cell;
use std::cell::UnsafeCell;

#[derive(Copy)]
enum RefState {
    Exclusive,
    Shared,
    Unreferenced,
}

pub struct RwLock<T: ?Sized> {
    value: UnsafeCell<T>,
    ref_state: Cell<RefState>,
}

impl<T> RwLock<T> {
    pub fn read(&self) -> &T {}

    pub fn write(&self) -> &mut T {}
}

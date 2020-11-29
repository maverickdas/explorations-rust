use crate::cell::Cell;
use std::cell::UnsafeCell;

#[derive(Copy, Clone)]
enum RefState {
    Unshared,
    Shared(usize),
    Exclusive,
}

pub struct RefCell<T> {
    value: UnsafeCell<T>,
    state: Cell<RefState>,
}

impl<T> RefCell<T> {
    pub fn new(value: T) -> Self {
        Self {
            value: UnsafeCell::new(value),
            state: Cell::new(RefState::Unshared),
        }
    }

    pub fn borrow(&self) -> Option<Ref<'_, T>> {
        //                      We need Ref type to track when the reference is released
        match self.state.get() {
            RefState::Unshared => {
                self.state.set(RefState::Shared(1));
                // SAFETY: No exclusive references will be given out since state would be Exclusive
                // Some(unsafe { &*self.value.get() })
                Some(Ref { refcell: self })
            }
            RefState::Shared(n) => {
                self.state.set(RefState::Shared(n + 1));
                // SAFETY: No exclusive references will be given out since state would be Exclusive
                // Some(unsafe { &*self.value.get() })
                Some(Ref { refcell: self })
            }
            RefState::Exclusive => None,
        }
    }

    pub fn borrow_mut(&self) -> Option<RefMut<'_, T>> {
        //                      We need RefMut type to track when the mutable reference is released
        // If already borrowed, cannot allow mutable references, then we pass None
        if let RefState::Unshared = self.state.get() {
            self.state.set(RefState::Exclusive);
            // SAFETY: No other references given out since then state would be Shared or Exclusive
            // Some(unsafe { &mut *self.value.get() })
            Some(RefMut { refcell: self })
        } else {
            None
        }
    }
}

pub struct Ref<'refcell, T> {
    refcell: &'refcell RefCell<T>,
}

impl<T> std::ops::Deref for Ref<'_, T> {
    type Target = T;
    // ^ gets invoked when invoking '.' operator
    fn deref(&self) -> &Self::Target {
        // SAFETY
        // a Ref is created if no exclusive references have been given out.
        // once it is given out, state is set to Shared, so no exlcusve references cam be given out
        // Hence, dereferencing a shared reference is fine
        unsafe { &*self.refcell.value.get() }
    }
}

impl<T> Drop for Ref<'_, T> {
    fn drop(&mut self) {
        match self.refcell.state.get() {
            RefState::Exclusive | RefState::Unshared => unreachable!(),
            RefState::Shared(1) => {
                self.refcell.state.set(RefState::Unshared);
            }
            RefState::Shared(n) => {
                self.refcell.state.set(RefState::Shared(n - 1));
            }
        }
    }
}

pub struct RefMut<'refcell, T> {
    refcell: &'refcell RefCell<T>,
}

impl<T> std::ops::Deref for RefMut<'_, T> {
    type Target = T;
    // ^ gets invoked when invoking '.' operator
    fn deref(&self) -> &Self::Target {
        // SAFETY: see safety for DerefMut
        unsafe { &*self.refcell.value.get() }
    }
}
impl<T> std::ops::DerefMut for RefMut<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        // SAFETY
        // a Ref is created if no exclusive references have been given out.
        // once it is given out, state is set to Exclusive, so no future references cam be given out
        // Hence, we have an exclusive lease on the inner value, so MUTABLY dereferencing is fine
        unsafe { &mut *self.refcell.value.get() }
    }
}

impl<T> Drop for RefMut<'_, T> {
    fn drop(&mut self) {
        match self.refcell.state.get() {
            RefState::Shared(_) | RefState::Unshared => unreachable!(),
            RefState::Exclusive => {
                self.refcell.state.set(RefState::Unshared);
            }
        }
    }
}

use std::cell::UnsafeCell;
// The only way in Rust to go from shared reference to exclusive reference is via UnsafeCell
// Casting is NOT allowed

pub struct Cell<T> {
    value: UnsafeCell<T>,
}

// Impled by UnsafeCell
// impl !Sync for Cell<T> {}

impl<T> Cell<T> {
    pub fn new(value: T) -> Self {
        Cell {
            value: UnsafeCell::new(value),
        }
    }

    pub fn set(&self, value: T) {
        // SAFETY: We know no-one else is concurrently mutating self.value (because !Sync)
        // SAFETY: We know we're not invalidating any references, because we never give any out
        unsafe { *self.value.get() = value };
    }

    pub fn get(&self) -> T
    where
        T: Copy,
    {
        // SAFETY: We know non-one else is modifying this value, since only this thread can mutate
        // (because !Sync), and it is executing this function instead
        unsafe { *self.value.get() }
    }
}

#[cfg(test)]
mod test {
    use super::Cell;

    // fn bad() {
    //     use std::sync::Arc;
    //     let x = Arc::new(Cell::new(42));
    //     let x1 = Arc::clone(&x);
    //     std::thread::spawn(|| {
    //         x1.set(43);
    //     });
    //     let x2 = Arc::clone(&x);
    //     std::thread::spawn(|| {
    //         x2.set(44);
    //     });
    // }
}

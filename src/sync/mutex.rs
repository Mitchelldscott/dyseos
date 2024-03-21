/********************************************************************************
 *
 *      ____                     ____          __           __       _
 *     / __ \__  __________     /  _/___  ____/ /_  _______/ /______(_)__  _____
 *    / / / / / / / ___/ _ \    / // __ \/ __  / / / / ___/ __/ ___/ / _ \/ ___/
 *   / /_/ / /_/ (__  )  __/  _/ // / / / /_/ / /_/ (__  ) /_/ /  / /  __(__  )
 *  /_____/\__, /____/\___/  /___/_/ /_/\__,_/\__,_/____/\__/_/  /_/\___/____/
 *        /____/
 *
 *
 ********************************************************************************/
//! # DyseOS Mutual Exclusion primative
//!
//! Typestate an RAII gaurd or something.
//!
//! Author: Mitchell Scott <scott.mitchell913@gmail.com>
//!
//! ## Resources
//!
//!   - <https://doc.rust-lang.org/std/sync/struct.Mutex.html>
//!   - <https://stdrs.dev/nightly/x86_64-unknown-linux-gnu/src/std/sys/unix/locks/futex_mutex.rs.html#21-23>
//!   - <https://man7.org/linux/man-pages/man7/futex.7.html>
//!   - <https://doc.rust-lang.org/rust-by-example/error/multiple_error_types/define_error_type.html>
//!   - <https://docs.rust-embedded.org/book/static-guarantees/typestate-programming.html>
//!

#[derive(Debug)]
/// An error type for the mutex
///
/// Allows printing a custom message with [format_args!()]
pub struct LockError;

/// Allows printing the error
impl core::fmt::Display for LockError {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.write_str("Failed to aquire lock")
    }
}

/// Mutual Exclusion Primative
///
/// TypeState mutex implementation, data is not accessible through [Mutex]. Instead a
/// [MutexGuard] must be aquired and then can be dereferenced. The [MutexGuard] can only be
/// aquired by a single thread at a time so it is safe to have a mutable borrow. 
///
/// The futex is an atomic value that allows thread safe reading and writing of the lock value.
/// 
/// 
/// ### TODO:
/// - add syscall for futex_wait to improve lock aquisitions. 
///
pub struct Mutex<T: ?Sized> {
    futex: core::sync::atomic::AtomicBool,
    data: core::cell::UnsafeCell<T>,
}

// these are the only places where `T: Send` matters; all other
// functionality works fine on a single thread.
unsafe impl<T: ?Sized + Send> Send for Mutex<T> {}
unsafe impl<T: ?Sized + Send> Sync for Mutex<T> {}

/// TypeState Mutex Guard
///
/// This guard should only be constructed from a [Mutex]. The guard will unlock the mutex when it
/// is dropped (by call or scope). A mutex will only give out a single guard at a time, this allows
/// mutable borrowing of the data.
///
/// ### Dev note
///
/// The lifetime of [Mutex] must be longer than the lifetime of any constructed [MutexGuard]s.
///
/// std::sync::mutex packages a reference to the mutex in the guard. 
/// This requires the mutex be able to mutate it's data and is not considered typestate programming.
///
/// This guard contains references to the feilds of the mutex. 
///
/// ### Examples
/// see [crate::drivers::console]
///
pub struct MutexGuard<'a, T: ?Sized + 'a> {
    futex: &'a core::sync::atomic::AtomicBool,
    data: &'a core::cell::UnsafeCell<T>,
}

// impl<T: ?Sized> !Send for MutexGuard<'_, T> {}
unsafe impl<T: ?Sized + Sync> Sync for MutexGuard<'_, T> {}

impl<T> Mutex<T> {
    /// Creates a new mutex in an unlocked state ready for use.
    ///
    ///
    /// ## TODO:
    /// Write an actual futex call
    ///
    /// ## Examples
    ///
    /// ```
    /// use dyseos::sync::Mutex;
    ///
    /// let mutex = Mutex::new(0);
    /// ```
    #[inline]
    pub const fn new(t: T) -> Mutex<T> {
        Mutex {
            futex: core::sync::atomic::AtomicBool::new(false),
            data: core::cell::UnsafeCell::new(t),
        }
    }
}

impl<T: ?Sized> Mutex<T> {

    /// Builds a [MutexGuard] from a [Mutex]
    ///
    /// This can only be used when the lock is already aquired, otherwise accessing the 
    /// guard's feilds is UB.
    fn as_guard(&self) -> MutexGuard<'_, T> {
        MutexGuard {
            futex: &self.futex,
            data: &self.data,
        }
    }

    /// Attempts to Acquire a mutex.
    ///
    /// This function will attempt to aquire a mutex, returning None if the mutex could
    /// not be aquired. Currently this does not support poison.
    ///
    /// ## Examples
    ///
    /// ```
    /// use dyseos::sync::Mutex;
    ///
    /// let mutex = Mutex::new(0);
    /// match mutex.try_lock() {
    ///     Some(raii_guard) => {   
    ///         
    ///         // immutable borrow
    ///         {
    ///             let data = &*raii_guard;
    ///         }
    ///     
    ///         // mutable borrow
    ///         {
    ///             let data = &mut *raii_guard;
    ///         }
    ///     }
    ///     None => {},
    /// }
    ///
    /// ```
    pub fn try_lock(&self) -> Option<MutexGuard<'_, T>> {
        match self.futex.compare_exchange(
            false,
            true,
            core::sync::atomic::Ordering::Acquire,
            core::sync::atomic::Ordering::Relaxed,
        ) {
            Ok(_) => Some(self.as_guard()), // Locked!
            Err(_) => None,
        }
    }
    
    /// Aquire the [MutexGuard]
    ///
    /// Loops until the timeout is reached or the lock is aquired. Underneath this uses [Mutex::try_lock()].
    ///
    /// ## Examples
    ///
    /// ```
    /// use dyseos::sync::Mutex;
    ///
    /// let mutex = Mutex::new(0);
    /// match mutex.lock() {
    ///     Ok(raii_guard) => {   
    ///         
    ///         // immutable borrow
    ///         {
    ///             let data = &*raii_guard;
    ///         }
    ///     
    ///         // mutable borrow
    ///         {
    ///             let data = &mut *raii_guard;
    ///         }
    ///     }
    ///     Err(e) => println!("{e:?}"),
    /// }
    ///
    /// ```
    pub fn lock(&self) -> Result<MutexGuard<'_, T>, LockError> {
        for _ in 0..100 {
            // should timeout be a user input? if so no need for futex_wait()
            match self.try_lock() {
                Some(raii_guard) => return Ok(raii_guard),
                None => {
                    // delay until futex changes... need a system call ig
                }
            }
        }

        Err(LockError)
    }
}

impl<T: ?Sized> core::ops::Deref for MutexGuard<'_, T> {
    type Target = T;

    fn deref(&self) -> &T {
        unsafe { &*self.data.get() }
    }
}

impl<T: ?Sized> core::ops::DerefMut for MutexGuard<'_, T> {

    fn deref_mut(&mut self) -> &mut T {
        unsafe { &mut *self.data.get() }
    }
}

impl<T: ?Sized> Drop for MutexGuard<'_, T> {
    #[inline]
    fn drop(&mut self) {
        self.futex.store(false, core::sync::atomic::Ordering::Release);
    }
}

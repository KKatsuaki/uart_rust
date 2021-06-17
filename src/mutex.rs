use core::cell::UnsafeCell;

pub trait Mutex{
    type Data;
    fn lock<F,R>(&self, f : F) -> R  where F : FnOnce(&mut Self::Data) -> R;
}

pub struct NullLock<T>
    where T : ?Sized
{
    data : UnsafeCell<T>
}

unsafe impl<T> Send for NullLock<T> where T: ?Sized + Send {}
unsafe impl<T> Sync for NullLock<T> where T: ?Sized + Send {}

impl <T>  NullLock <T>{
    pub const fn new(data : T) -> Self {
        Self{
            data : UnsafeCell::new(data)
        }
    }    
}

impl<T> Mutex for NullLock<T> {
    type Data = T;
    fn lock<F,R>(&self, f : F) -> R  where F : FnOnce(&mut Self::Data) -> R {
        let data = unsafe {&mut *self.data.get()};
        f(data)
    }
}

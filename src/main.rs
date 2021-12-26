#[cxx::bridge(namespace = mitama)]
pub mod ffi {
    unsafe extern "C++" {
        include!("cxx-demo/include/semaphore.h");

        type Semaphore;

        fn acquire(&self);
        fn release(&self);
        fn try_acquire(&self) -> bool;

        fn new_counting_semaphore(count: u64) -> UniquePtr<Semaphore>;
    }
}

pub mod sync {
    use crate::ffi::Semaphore;
    use cxx::UniquePtr;

    pub struct CountingSemaphore {
        token: UniquePtr<Semaphore>,
    }

    impl CountingSemaphore {
        pub fn new(init: u64) -> CountingSemaphore {
            CountingSemaphore{
                token: crate::ffi::new_counting_semaphore(init)
            }
        }

        pub fn acquire(&mut self) {
            (*self.token).acquire();
        }

        pub fn release(&mut self) {
            (*self.token).release();
        }

        pub fn try_acquire(&mut self) -> bool {
            (*self.token).try_acquire()
        }
    }
}

fn main() {
    let mut binary_semephore = sync::CountingSemaphore::new(1);
    binary_semephore.try_acquire();
    binary_semephore.release();
    binary_semephore.acquire();
}

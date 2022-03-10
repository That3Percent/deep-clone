use std::ops::Deref;

use {
    deep_clone::DeepClone,
    std::{fmt::Debug, sync::Mutex},
};

// Test ensures that a deep_clone has equality,
// and that deep_clone_from has equality but re-uses memory
// via the ptrs fn to check specific memory addresses for equality
// before and after the clone.
#[track_caller]
fn test<T, F>(mut left: T, right: T, ptrs: F)
where
    T: DeepClone + PartialEq + Debug,
    F: Fn(&T) -> Vec<*const u8>,
{
    assert_eq!(right, right.deep_clone());
    let ptrs_left = ptrs(&left);
    left.deep_clone_from(&right);
    assert_eq!(left, right);
    assert_eq!(ptrs_left, ptrs(&left));
}

fn vec_ptr<T>(v: &Vec<T>) -> Vec<*const u8> {
    let r = &v[..];
    let r = r as *const [T];
    vec![r as *const u8]
}

fn no_ptr<T>(_v: &T) -> Vec<*const u8> {
    Vec::new()
}

#[test]
fn vec() {
    test(vec![1u32, 2, 3], vec![0], vec_ptr);
    test(vec![], vec![1u32, 2, 3], no_ptr);
    test(vec![1u32, 2, 3], vec![1, 2, 3, 4, 5], no_ptr);
    test(Vec::<i32>::new(), vec![], vec_ptr);
}

#[test]
fn nums() {
    test(1, 2, no_ptr);
    test(0.1f64, 0.2f64, no_ptr);
}

#[test]
fn mutex() {
    #[derive(Debug)]
    struct MutexEq<T>(Mutex<T>);
    impl<T: PartialEq> PartialEq for MutexEq<T> {
        fn eq(&self, other: &Self) -> bool {
            let l = self.0.lock().unwrap();
            l.deref() == other.0.lock().unwrap().deref()
        }
    }
    impl<T: DeepClone> DeepClone for MutexEq<T> {
        fn deep_clone(&self) -> Self {
            MutexEq(self.0.deep_clone())
        }
        fn deep_clone_from(&mut self, other: &Self) {
            self.0.deep_clone_from(&other.0)
        }
    }

    fn ptr(v: &MutexEq<Vec<u32>>) -> Vec<*const u8> {
        let l = v.0.lock().unwrap();
        let mut r = vec_ptr(&l);
        r.push(l.deref() as *const Vec<u32> as *const u8);
        r
    }
    test(
        MutexEq(Mutex::new(vec![0u32, 1])),
        MutexEq(Mutex::new(vec![1, 1])),
        ptr,
    );
}

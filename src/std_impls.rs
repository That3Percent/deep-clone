use {
    crate::{from_clone, DeepClone},
    std::{
        collections::{HashMap, HashSet},
        hash::Hash,
        ops::Deref,
        rc::Rc,
        sync::Arc,
        sync::Mutex,
    },
};

from_clone!(u8, i8, u16, i16, u32, i32, u64, i64, u128, i128);
from_clone!(f32, f64);

impl<T: DeepClone> DeepClone for Rc<T> {
    fn deep_clone(&self) -> Self {
        Rc::new(self.deref().deep_clone())
    }
}

impl<T: DeepClone> DeepClone for Arc<T> {
    fn deep_clone(&self) -> Self {
        Arc::new(self.deref().deep_clone())
    }
}

impl<T: DeepClone> DeepClone for Mutex<T> {
    fn deep_clone(&self) -> Self {
        Mutex::new(self.lock().unwrap().deep_clone())
    }

    fn deep_clone_from(&mut self, source: &Self) {
        self.get_mut()
            .unwrap()
            .deep_clone_from(&mut source.lock().unwrap())
    }
}

impl<T: DeepClone> DeepClone for Vec<T> {
    fn deep_clone(&self) -> Self {
        self.iter().map(DeepClone::deep_clone).collect()
    }

    fn deep_clone_from(&mut self, source: &Self) {
        let mut source = source.iter();

        let mut i = 0;
        while i < self.len() {
            if let Some(n) = source.next() {
                self[i].deep_clone_from(n);
            } else {
                self.drain(i..);
                break;
            }
            i += 1;
        }
        self.extend(source.map(DeepClone::deep_clone));
    }
}

impl<T: DeepClone + Hash + Eq> DeepClone for HashSet<T> {
    fn deep_clone(&self) -> Self {
        self.iter().map(DeepClone::deep_clone).collect()
    }
    fn deep_clone_from(&mut self, other: &Self) {
        // TODO (performance): This can use deep_clone_from on some items
        self.clear();
        self.extend(other.iter().map(DeepClone::deep_clone));
    }
}

impl<K: DeepClone + Hash + Eq, V: DeepClone> DeepClone for HashMap<K, V> {
    fn deep_clone(&self) -> Self {
        self.iter()
            .map(|(k, v)| (k.deep_clone(), v.deep_clone()))
            .collect()
    }
    fn deep_clone_from(&mut self, other: &Self) {
        // TODO (performance): This can use deep_clone_from on some items
        self.clear();
        self.extend(other.iter().map(|(k, v)| (k.deep_clone(), v.deep_clone())));
    }
}

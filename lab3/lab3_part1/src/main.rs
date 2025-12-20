use std::borrow::Cow;
use std::collections::HashMap;

trait Storage<K, V> {
    fn set(&mut self, key: K, val: V);
    fn get(&self, key: &K) -> Option<&V>;
    fn remove(&mut self, key: &K) -> Option<V>;
}

struct User {
    id: u64,
    email: Cow<'static, str>,
    activated: bool,
}


struct MemStorage {
    data: HashMap<u64, User>,
}

impl Storage<u64, User> for MemStorage {
    fn set(&mut self, key: u64, val: User) { self.data.insert(key, val); }
    fn get(&self, key: &u64) -> Option<&User> { self.data.get(key) }
    fn remove(&mut self, key: &u64) -> Option<User> { self.data.remove(key) }
}

// Generics
struct UserRepositoryStatic<S: Storage<u64, User>> {
    storage: S,
}

impl<S: Storage<u64, User>> UserRepositoryStatic<S> {
    fn new(storage: S) -> Self { Self { storage } }
    fn add(&mut self, user: User) { self.storage.set(user.id, user); }
    fn get(&self, id: u64) -> Option<&User> { self.storage.get(&id) }
    fn delete(&mut self, id: u64) -> Option<User> { self.storage.remove(&id) }
}

struct UserRepositoryDynamic {
    storage: Box<dyn Storage<u64, User>>,
}

impl UserRepositoryDynamic {
    fn new(storage: Box<dyn Storage<u64, User>>) -> Self { Self { storage } }
    fn add(&mut self, user: User) { self.storage.set(user.id, user); }
    fn get(&self, id: u64) -> Option<&User> { self.storage.get(&id) }
    fn delete(&mut self, id: u64) -> Option<User> { self.storage.remove(&id) }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_user() -> User {
        User { id: 42, email: Cow::Borrowed("vsiroedov@gmail.com"), activated: true }
    }

    #[test]
    fn test_static_dispatch() {
        let mut repo = UserRepositoryStatic::new(MemStorage { data: HashMap::new() });
        repo.add(create_user());
        assert_eq!(repo.get(42).unwrap().email, "vsiroedov@gmail.com");
    }

    #[test]
    fn test_dynamic_dispatch() {
        let mut repo = UserRepositoryDynamic::new(Box::new(MemStorage { data: HashMap::new() }));
        repo.add(create_user());
        assert_eq!(repo.get(42).unwrap().email, "vsiroedov@gmail.com");
    }
}

fn main() {
    println!("Lab 3 Part 1 compiled successfully!");
}
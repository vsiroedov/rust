pub struct New;
pub struct Unmoderated;
pub struct Published;
pub struct Deleted;

pub struct Post<S> {
    pub content: String,
    _state: std::marker::PhantomData<S>,
}

impl Post<New> {
    pub fn new(content: &str) -> Post<New> {
        Post {
            content: content.to_string(),
            _state: std::marker::PhantomData,
        }
    }
    pub fn publish(self) -> Post<Unmoderated> {
        Post { content: self.content, _state: std::marker::PhantomData }
    }
}

impl Post<Unmoderated> {
    pub fn allow(self) -> Post<Published> {
        Post { content: self.content, _state: std::marker::PhantomData }
    }
    pub fn deny(self) -> Post<Deleted> {
        Post { content: self.content, _state: std::marker::PhantomData }
    }
}

impl Post<Published> {
    pub fn delete(self) -> Post<Deleted> {
        Post { content: self.content, _state: std::marker::PhantomData }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_valid_flow() {
        let post = Post::new("Rust is cool");
        let unmoderated = post.publish();
        let published = unmoderated.allow();
        let _deleted = published.delete();
    }
}
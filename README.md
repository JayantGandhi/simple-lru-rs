## Usage
initializes a new lru cache of size 2
```
let mut lru = LruCache::new(2);
lru.put(1, 1);
lru.put(2, 2);
lru.get(1); // Some(1)
```

Note: keys can be of any type that implements Eq, Clone, and Hash
values can be of any type that implements Clone

### delete
deletes the item from the cache based on key
```
lru.delete(1);
```

### reset
clears the lru caches completely
```
lru.reset();
```

## Demo
Run the demo using `cargo run`.
You can edit the `main.rs` file to play around with the cache itself.
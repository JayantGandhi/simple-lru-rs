//for demo purposes only

use lru::LruCache;


fn main() {
    let mut lru = LruCache::new(2);
    lru.put(1, 1);
    lru.put(2, 2);
    println!("{:?}", lru.get(1));
    lru.put(3, 3);
    println!("{:?}", lru.get(2));
    lru.put(4, 4);
    println!("{:?}", lru.get(1));
    println!("{:?}", lru.get(3));
    println!("{:?}", lru.get(4));
    lru.delete(3);
    println!("{:?}", lru.get(3));
    lru.reset();
    println!("{:?}", lru.get(4));
}
use std::{collections::HashMap, fmt::Debug};

pub struct Cache<V> {
    max_size: usize,
    hashmap: HashMap<String, V>,
    usage_history: Vec<String>,
}

impl<V> Cache<V> {
    pub fn prime(
        max_size: usize,
        hashmap: HashMap<String, V>,
        usage_history: Vec<String>
    ) -> Cache<V> {
        return Cache {
            max_size,
            hashmap,
            usage_history,
        };
    }

    pub fn new(max_size: usize) -> Cache<V> {
        return Cache {
            max_size,
            hashmap: HashMap::new(),
            usage_history: Vec::new(),
        }
    }

    fn remove_least_recently_used(&mut self) {
        if let Some(least_recent) = self.usage_history.first() {
            self.hashmap.remove(least_recent);
            self.usage_history.remove(0);
        }
    }

    fn mark_as_most_recently_used(&mut self, username: String) {
        if let Some(pos) = self.usage_history.iter().position(|x| *x == username) {
            self.usage_history.remove(pos);
        }
        self.usage_history.push(username);
    }

    pub fn get_chat(&mut self, username: &str) -> Option<&mut V> {
        if self.hashmap.contains_key(username) {
            self.mark_as_most_recently_used(username.to_string());
            return self.hashmap.get_mut(username);
        }
        return None;
    }

    pub fn insert_chat(&mut self, username: String, chat: V) {
        println!("Insert {username} into cache:");
        println!("Cache before inserting: -----");
        println!("{:?}", self);
        println!("-----------------------------");

        self.hashmap.insert(username.clone(), chat);
        self.mark_as_most_recently_used(username);
        if self.hashmap.len() > self.max_size {
            self.remove_least_recently_used();
        }

        println!("Cache after inserting: ------");
        println!("{:?}", self);
        println!("-----------------------------");
    }
}

impl<V> Debug for Cache<V> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let map: HashMap<_, _> = self.hashmap.iter()
            .map(|(k, _v)| (k, "<chat>"))
            .collect();
        write!(f, "Cache {{\n  hashmap = {:?},\n  usage_history = {:?}\n}}", map, self.usage_history)
    }
}

pub fn main() {
    println!("Creating cache with capacity 3");
    let mut cache = Cache::new(3);
    cache.insert_chat(String::from("user1"), String::from("v1"));
    cache.insert_chat(String::from("user2"), String::from("v2"));
    cache.insert_chat(String::from("user3"), String::from("v3"));
    cache.insert_chat(String::from("user4"), String::from("v4"));

    let v4 = cache.get_chat("user4").unwrap();
    assert_eq!(v4, "v4");

    let v1 = cache.get_chat("user1");
    match v1 {
        None => {},
        Some(_) => {
            println!("Error: the implementation of slow cache is either incomplete or not correct.");
            println!("We set up the cache to have a max size = 3.");
            println!("We then inserted 4 different values into it: user1, user2, user3, and user4 in order!");
            println!("When user 4 gets inserted, a correct implementation should realize the size is now larger than 3, so the cache should remove the entry that was least recently used.");
            println!("In this case, that entry is user1.");
            println!("But user1 does not get removed.");
            return;
        }
    }

    assert_eq!(cache.get_chat("user2"), Some(&mut String::from("v2")));

    cache.insert_chat(String::from("user5"), String::from("v5"));

    let v2 = cache.get_chat("user2");
    match v2 {
        None => {
            println!("Error: the implementation of slow cache is either not correct.");
            println!("We set up the cache to have a max size = 3.");
            println!("We performed these operations in order:");
            println!(" Insert user1");
            println!(" Insert user2");
            println!(" Insert user3");
            println!(" Insert user4");
            println!(" Read user4");
            println!(" Read user1");
            println!(" Read user2");
            println!(" Insert user5");
            println!(" Read user2 <--- this is removed by your cache, but it should not be");
            println!("Your implementation removes user2 to make space for inserting user 5.");
            println!("This is incorrect.");
            println!("Which element should it remove instead? Use a pen and paper and try to keep track of which user is the least recently used with the operations above.");
            return;
        },
        Some(v2) => {
            assert_eq!(v2, "v2");
        }
    }
}
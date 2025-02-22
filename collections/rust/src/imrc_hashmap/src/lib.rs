use im_rc::HashMap;
use std::cell::RefCell;

struct Random {
    state: u64,
    size: Option<u32>,
    ind: u32,
}
impl Random {
    pub fn new(size: Option<u32>, seed: u64) -> Self {
        Random {
            state: seed,
            size,
            ind: 0,
        }
    }
}
impl Iterator for Random {
    type Item = u64;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(size) = self.size {
            self.ind += 1;
            if self.ind > size {
                return None;
            }
        }
        self.state = self.state * 48271 % 0x7fffffff;
        Some(self.state)
    }
}

thread_local! {
    static MAP: RefCell<HashMap<u64, u64>> = RefCell::default();
    static RAND: RefCell<Random> = RefCell::new(Random::new(None, 42));
}

#[ic_cdk::update]
fn generate(size: u32) {
    let rand = Random::new(Some(size), 1);
    let iter = rand.map(|x| (x, x));
    MAP.with(|map| {
        let mut map = map.borrow_mut();
        for (k, v) in iter {
            map.insert(k, v);
        }
    });
}

#[ic_cdk::query]
fn get_mem() -> (u128, u128, u128) {
    let size = core::arch::wasm32::memory_size(0) as u128 * 32768;
    (size, size, size)
}

#[ic_cdk::update]
fn batch_get(n: u32) {
    MAP.with(|map| {
        let map = map.borrow_mut(); // mut to ensure get doesn't get inlined by the compiler
        RAND.with(|rand| {
            let mut rand = rand.borrow_mut();
            for _ in 0..n {
                let k = rand.next().unwrap();
                drop(map.get(&k));
            }
        })
    })
}

#[ic_cdk::update]
fn batch_put(n: u32) {
    MAP.with(|map| {
        let mut map = map.borrow_mut();
        RAND.with(|rand| {
            let mut rand = rand.borrow_mut();
            for _ in 0..n {
                let k = rand.next().unwrap();
                map.insert(k, k);
            }
        })
    })
}

#[ic_cdk::update]
fn batch_remove(n: u32) {
    let mut rand = Random::new(None, 1);
    MAP.with(|map| {
        let mut map = map.borrow_mut();
        for _ in 0..n {
            let k = rand.next().unwrap();
            map.remove(&k);
        }
    })
}

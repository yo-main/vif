use crate::global::Global;

pub struct GlobalStore {
    storage: [Option<Global>; 1000],
    top: usize,
}

impl GlobalStore {
    pub fn new() -> Self {
        GlobalStore {
            storage: std::array::from_fn(|_| None),
            top: 0,
        }
    }

    pub fn as_vec(&self) -> Vec<&Global> {
        self.storage
            .iter()
            .filter(|v| match v {
                None => false,
                _ => true,
            })
            .map(|v| v.as_ref().unwrap())
            .collect::<Vec<&Global>>()
    }

    pub fn push(&mut self, variable: Global) {
        let _ = self.storage[self.top].insert(variable);
        self.top += 1;
    }

    pub fn len(&self) -> usize {
        self.top
    }

    pub fn get(&self, index: usize) -> &Global {
        unsafe { self.storage.get_unchecked(index).as_ref().unwrap() }
        // &self.storage[index]
    }

    pub fn find(&self, variable: &Global) -> Option<usize> {
        self.storage
            .iter()
            .position(|v| v.as_ref() == Some(variable))
    }
}

impl std::fmt::Debug for GlobalStore {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:?}",
            self.storage
                .iter()
                .filter(|v| match v {
                    None => false,
                    _ => true,
                })
                .map(|v| v.as_ref().unwrap())
                .collect::<Vec<&Global>>()
        )
    }
}

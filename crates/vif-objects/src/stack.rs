use crate::stack_value::StackValue;

pub struct Stack<'value> {
    stack: [Option<StackValue<'value>>; 1000],
    pub top: usize,
}

// don't laugh at me, I was desperate
macro_rules! create_array {
    () => {
        [
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None, None, None, None, None,
        ]
    };
}

impl<'value> Stack<'value> {
    pub fn new() -> Self {
        Self {
            stack: create_array!(),
            top: 0,
        }
    }

    #[inline]
    pub fn pop_raw(&mut self) -> StackValue<'value> {
        self.top -= 1;
        self.stack[self.top].take().unwrap()
    }

    #[inline]
    pub fn pop_and_get_value(&mut self) -> StackValue<'value> {
        match self.pop_raw() {
            StackValue::StackReference(i) => self.peek(i).clone(),
            v => v,
        }
    }

    pub fn peek_till_scope(&mut self, index: usize, scope: usize) -> StackValue<'value> {
        match self.peek_raw(index) {
            StackValue::StackReference(i) if scope >= *i => self.peek_raw(*i).clone(),
            StackValue::StackReference(i) if scope < *i => self.peek_till_scope(*i, scope),
            v => v.clone(),
        }
    }

    #[inline]
    pub fn pop_till_scope(&mut self, scope: usize) -> StackValue<'value> {
        match self.pop_raw() {
            StackValue::StackReference(i) if scope >= i => self.peek_raw(i).clone(),
            StackValue::StackReference(i) if scope < i => self.peek_till_scope(i, scope),
            v => v,
        }
    }

    #[inline]
    pub fn drop_last(&mut self) {
        self.top -= 1;
    }

    pub fn set_last(&mut self, value: StackValue<'value>) {
        self.stack[self.top - 1] = Some(value);
    }

    pub fn push(&mut self, value: StackValue<'value>) {
        // TODO: would probably be a good idea to add a control here even if we loose perf :shrug:
        unsafe {
            *(self.stack.get_unchecked_mut(self.top)) = Some(value);
        }
        self.top += 1;
    }

    pub fn set(&mut self, n: usize, value: StackValue<'value>) {
        let value = match value {
            StackValue::StackReference(i) => self.peek_to_set(i, &value).clone(),
            _ => value,
        };
        // println!("{value}");
        // if n >= self.top {
        //     panic!("Badadoom {value} {n} {}", self.top);
        // }
        // println!("SET {n} TO {value} {}", self.top);
        let value_ref = match value {
            StackValue::StackReference(i) => Some(i),
            _ => None,
        };

        match self.stack.get(n).unwrap() {
            Some(StackValue::StackReference(i)) => self.set(*i, value),
            _ => {
                if value_ref != Some(n) {
                    self.stack[n].insert(value);
                }
            }
        };
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.top
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.top == 0
    }

    #[inline]
    fn peek_raw(&self, n: usize) -> &StackValue<'value> {
        unsafe { self.stack.get_unchecked(n).as_ref().unwrap() }
    }

    #[inline]
    fn peek_mut_raw(&mut self, n: usize) -> &mut StackValue<'value> {
        unsafe { self.stack.get_unchecked_mut(n).as_mut().unwrap() }
    }

    #[inline]
    pub fn peek_last_raw(&self) -> &StackValue<'value> {
        unsafe { self.stack.get_unchecked(self.top - 1).as_ref().unwrap() }
    }

    #[inline]
    fn peek_last_mut_raw(&mut self) -> &mut StackValue<'value> {
        unsafe { self.stack.get_unchecked_mut(self.top - 1).as_mut().unwrap() }
    }

    #[inline]
    pub fn peek(&self, n: usize) -> &StackValue<'value> {
        match self.peek_raw(n) {
            StackValue::StackReference(i) => self.peek(*i),
            value => value,
        }
    }

    #[inline]
    pub fn peek_last(&self) -> &StackValue<'value> {
        match self.peek_last_raw() {
            StackValue::StackReference(i) => self.peek(*i),
            value => value,
        }
    }

    #[inline]
    fn peek_to_set<'a, 'b>(
        &'b self,
        index: usize,
        prev: &'a StackValue<'value>,
    ) -> &'a StackValue<'value>
    where
        'b: 'a,
    {
        let value = self.peek_raw(index);
        match value {
            StackValue::StackReference(i) => self.peek_to_set(*i, value),
            _ => prev,
        }
    }

    #[inline]
    pub fn peek_first_ref_from_last(&self) -> &StackValue<'value> {
        let value = self.peek_last_raw();
        match value {
            StackValue::StackReference(i) => self.peek_to_set(*i, value),
            value => value,
        }
    }

    #[inline]
    pub fn peek_first_ref_as_ref(&self, index: usize) -> StackValue<'value> {
        let value = self.peek_raw(index);
        match value {
            StackValue::StackReference(i) => self.peek_to_set(*i, value).clone(),
            _ => StackValue::StackReference(index),
        }
    }

    #[inline]
    pub fn peek_first_ref(&self, index: usize) -> &StackValue<'value> {
        let value = self.peek_raw(index);
        match value {
            StackValue::StackReference(i) => self.peek_to_set(*i, value),
            value => value,
        }
    }

    #[inline]
    pub fn peek_mut(&mut self, n: usize) -> &mut StackValue<'value> {
        match self.peek_raw(n) {
            StackValue::StackReference(i) => self.peek_mut(*i),
            _ => self.peek_mut_raw(n),
        }
    }

    #[inline]
    pub fn peek_last_mut(&mut self) -> &mut StackValue<'value> {
        match self.peek_last_raw() {
            StackValue::StackReference(i) => self.peek_mut(*i),
            _ => self.peek_last_mut_raw(),
        }
    }

    pub fn get_slice(&self, start: usize) -> Vec<&StackValue<'value>> {
        self.stack[start..self.top]
            .iter()
            .filter_map(Option::as_ref)
            .map(|v| match v {
                StackValue::StackReference(i) => self.peek(*i),
                v => v,
            })
            .collect()
    }

    pub fn truncate(&mut self, n: usize) {
        self.top = n;
    }

    pub fn get_items(&self) -> Vec<&StackValue<'_>> {
        self.stack
            .iter()
            .filter(|v| match v {
                None => false,
                _ => true,
            })
            .map(|v| v.as_ref().unwrap())
            .collect::<Vec<&StackValue<'_>>>()
    }
}

impl std::fmt::Display for Stack<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[{}]",
            self.stack
                .iter()
                .enumerate()
                .map_while(|(_, v)| {
                    if v.is_none() {
                        return None;
                    }
                    Some(format!("{}", v.as_ref().unwrap()))
                })
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}

impl std::fmt::Debug for Stack<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:?}",
            self.stack
                .iter()
                .filter(|v| match v {
                    None => false,
                    _ => true,
                })
                .map(|v| v.as_ref().unwrap())
                .collect::<Vec<&StackValue<'_>>>()
        )
    }
}

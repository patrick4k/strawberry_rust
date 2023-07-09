use serde::Deserialize;

#[derive(Clone)]
pub enum OneOrMore<T: Clone> {
    One(T),
    More(Vec<T>)
}

impl<T: Clone> OneOrMore<T> {
    pub(crate) fn to_vec(self) -> Vec<T> {
        match self {
            OneOrMore::One(one) => vec![one],
            OneOrMore::More(more) => more
        }
    }
    
    pub(crate) fn from(val: Vec<T>) -> OneOrMore<T> {
        let size = val.len().clone();
        match size {
            1 => OneOrMore::One(val.into_iter().next().unwrap()),
            _ => OneOrMore::More(val)
        }
    }
}

#[derive(Clone)]
pub enum ZeroOrMore<T: Clone> {
    Zero,
    More(Vec<T>)
}

impl<T: Clone> ZeroOrMore<T> {
    pub(crate) fn to_vec(self) -> Vec<T> {
        match self {
            ZeroOrMore::Zero => vec![],
            ZeroOrMore::More(more) => more
        }
    }

    pub(crate) fn from(val: Vec<T>) -> ZeroOrMore<T> {
        match val.len() {
            0 => ZeroOrMore::Zero,
            _ => ZeroOrMore::More(val)
        }
    }
}

impl<T:Clone> Default for ZeroOrMore<T> {
    fn default() -> Self {
        ZeroOrMore::Zero
    }
}

impl<T:Clone> ZeroOrMore<T> {
    pub fn push(&mut self, val: T) {
        match self {
            ZeroOrMore::Zero => {
                *self = ZeroOrMore::More(vec![val]);
            },
            ZeroOrMore::More(more) => {
                more.push(val);
            }
        }
    }
}
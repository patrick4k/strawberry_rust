use serde::Deserialize;

#[derive(Clone, Deserialize)]
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
        match val.len() { 
            1 => OneOrMore::One(val[0].clone()),
            _ => OneOrMore::More(val)
        }
    }
}

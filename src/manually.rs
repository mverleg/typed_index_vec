use std::marker::PhantomData;
use std::ops::Index;
use std::prelude::v1::Vec;

pub trait ProtoTIVec<T> {

    fn new() -> Self;

    fn with_capacity(capacity: usize) -> Self;

    // Should only be visible for impl, not for other files / todo
    fn expose(&self) -> &Vec<T>;

    fn expose_mut(&mut self) -> &mut Vec<T>;
}

pub trait TIVec<T>: ProtoTIVec<T> {
    fn get(&self, index: Id<TIVec<T>>) -> Option<&T>;
    fn pop(&mut self) -> Option<T>;
    fn push(&mut self, value: T);
    // most of the other Vec methods...
}

// TODO @mverleg: is this ?Sized going to be a problem?
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Id<Y: ?Sized> {
    typ: PhantomData<Y>,
    index: usize,
}

impl<T> Id<TIVec<T>> {
    pub fn new(value: usize) -> Self {
        Id { index: value, typ: PhantomData }
    }

    pub fn from<IU: Into<usize>>(value: IU) -> Self {
        Id { index: value.into(), typ: PhantomData }
    }

    /// This should not be exposed
    fn as_usize(&self) -> usize {
        self.index as usize
    }
}

impl<X, T> TIVec<T> for X where X: ProtoTIVec<T> {
    fn get(&self, index: Id<TIVec<T>>) -> Option<&T> {
        self.expose().get(index.as_usize())
    }

    fn pop(&mut self) -> Option<T> {
        self.expose_mut().pop()
    }

    fn push(&mut self, value: T) {
        self.expose_mut().push(value)
    }
}

impl<X, T> Index<Id<TIVec<T>>> for X where X: TIVec<T> {
    type Output = T;

    fn index(&self, index: Id<TIVec<T>>) -> &<Self as Index<Id<TIVec<T>>>>::Output {
        self.expose()[index]
    }
}

// // // // // // // // // // // // // // //

#[derive(Debug)]
struct MyVec<T> {
    data: Vec<T>
}

impl<T> ProtoTIVec<T> for MyVec<T> {
    fn new() -> Self {
        MyVec { data: Vec::new() }
    }

    fn with_capacity(capacity: usize) -> Self {
        MyVec { data: Vec::with_capacity(capacity) }
    }

    fn expose(&self) -> &Vec<T> {
        &self.data
    }

    fn expose_mut(&mut self) -> &mut Vec<T> {
        &mut self.data
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_indexing() {
        let mut v: MyVec<f64> = MyVec::new();
        v.push(0.0);
        v.push(1.0);
        v.push(2.0);
        let r = v.get(Id::new(1));
        assert_eq!(&1.0, r.unwrap());
    }
}



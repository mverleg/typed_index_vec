#![feature(try_reserve)]
#![feature(shrink_to)]
#![feature(vec_resize_with)]

use std::collections::CollectionAllocErr;
use std::boxed::Box;
use std::marker::PhantomData;
use std::prelude::v1::Vec;
use std::fmt::Debug;
use std::fmt::Formatter;
use std::fmt::Error;
use std::hash::Hash;
use std::hash::Hasher;
use std::ops::{Deref, DerefMut, Index, IndexMut};
use std::iter::FromIterator;
use std::vec::IntoIter;
use std::slice::SliceIndex;
use std::slice;

// TODO @mverleg: does this make $name a local type that users can implement traits for?

// TODO @mverleg: is this ?Sized going to be a problem?
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Id<Y: ?Sized> {
    typ: PhantomData<Y>,
    index: usize,
}

macro_rules! typed_vec {
    ( $name: ident ) => {

        struct $name<T> ( Vec<T> ) ;

        #[allow(dead_code)]
        impl<T> $name<T> {
            fn from_vec(data: Vec<T>) -> Self {
                $name( data )
            }

            #[inline]
            pub fn new() -> Self {
                $name( Vec::new() )
            }

            #[inline]
            pub fn with_capacity(capacity: usize) -> Self {
                $name( Vec::with_capacity(capacity) )
            }

            #[inline]
            pub fn capacity(&self) -> usize {
                self.0.capacity()
            }

            pub fn reserve(&mut self, additional: usize) {
                self.0.reserve(additional)
            }

            pub fn reserve_exact(&mut self, additional: usize) {
                self.0.reserve_exact(additional)
            }

            pub fn try_reserve(&mut self, additional: usize) -> Result<(), CollectionAllocErr> {
                self.0.try_reserve(additional)
            }

            pub fn try_reserve_exact(&mut self, additional: usize) -> Result<(), CollectionAllocErr>  {
                self.0.try_reserve_exact(additional)
            }

            pub fn shrink_to_fit(&mut self) {
                self.0.shrink_to_fit()
            }

            pub fn shrink_to(&mut self, min_capacity: usize) {
                self.0.shrink_to(min_capacity)
            }

            pub fn into_boxed_slice(self) -> Box<[T]> {
                self.0.into_boxed_slice()
            }

            pub fn truncate(&mut self, len: usize) {
                self.0.truncate(len)
            }

            #[inline]
            pub fn as_slice(&self) -> &[T] {
                self.0.as_slice()
            }

            #[inline]
            pub fn as_mut_slice(&mut self) -> &mut [T] {
                self.0.as_mut_slice()
            }

            #[inline]
            pub fn swap_remove(&mut self, index: usize) -> T {
                self.0.swap_remove(index)
            }

            pub fn insert(&mut self, index: usize, element: T) {
                self.0.insert(index, element)
            }

            pub fn remove(&mut self, index: usize) -> T {
                self.0.remove(index)
            }

            pub fn retain<F>(&mut self, f: F) where F: FnMut(&T) -> bool {
                self.0.retain(f)
            }

            #[inline]
            pub fn dedup_by_key<F, K>(&mut self, key: F) where F: FnMut(&mut T) -> K, K: PartialEq {
                self.0.dedup_by_key(key)
            }

            pub fn dedup_by<F>(&mut self, same_bucket: F) where F: FnMut(&mut T, &mut T) -> bool {
                self.0.dedup_by(same_bucket)
            }

            #[inline]
            pub fn push(&mut self, value: T) {
                self.0.push(value)
            }

            #[inline]
            pub fn pop(&mut self) -> Option<T> {
                self.0.pop()
            }

            #[inline]
            pub fn append(&mut self, other: &mut Self) {
                self.0.append(&mut other.0)
            }

            // TODO:
//            pub fn drain<R>(&mut self, range: R) -> Drain<T> where R: RangeBounds<usize> {
//                self.0.drain(range)
//            }

            #[inline]
            pub fn clear(&mut self) {
                self.0.clear()
            }

            #[inline]
            pub fn len(&self) -> usize {
                self.0.len()
            }

            pub fn is_empty(&self) -> bool {
                self.0.is_empty()
            }

            #[inline]
            pub fn split_off(&mut self, at: usize) -> Self {
                $name::from_vec(self.0.split_off(at))
            }

            pub fn resize_with<F>(&mut self, new_len: usize, f: F) where F: FnMut() -> T {
                self.0.resize_with(new_len, f)
            }
        }

        impl<T: Debug> Debug for $name<T> {
            fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
                write!(f, stringify!($name));
                write!(f, "{{");
                write!(f, "{:?}", self.0)?;
                write!(f, "}}")
            }
        }

        impl<T: Clone> Clone for $name<T> {
            fn clone(&self) -> Self {
                $name::from_vec(self.0.clone())
            }

            fn clone_from(&mut self, other: &Self) {
                self.0.clone_from(&other.0)
            }
        }

        impl<T: PartialEq> PartialEq for $name<T> {
            fn eq(&self, other: &Self) -> bool {
                self.0 == other.0
            }
        }

        impl<T: Eq> Eq for $name<T> {}

        impl<T: Hash> Hash for $name<T> {
            #[inline]
            fn hash<H: Hasher>(&self, state: &mut H) {
                stringify!($name).hash(state);
                self.0.hash(state)
            }
        }

        impl<T> Deref for $name<T> {
            type Target = [T];

            fn deref(&self) -> &[T] {
                self.0.deref()
            }
        }

        impl<T> DerefMut for $name<T> {
            fn deref_mut(&mut self) -> &mut [T] {
                self.0.deref_mut()
            }
        }

        impl<T> FromIterator<T> for $name<T> {
            #[inline]
            fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> $name<T> {
                $name ( Vec::from_iter(iter) )
            }
        }

        impl<T> IntoIterator for $name<T> {
            type Item = T;
            type IntoIter = IntoIter<T>;

            #[inline]
            fn into_iter(self) -> IntoIter<T> {
               self.0.into_iter()
            }
        }

        impl<'a, T> IntoIterator for &'a $name<T> {
            type Item = &'a T;
            type IntoIter = slice::Iter<'a, T>;

            fn into_iter(self) -> slice::Iter<'a, T> {
                self.0.iter()
            }
        }

        impl<'a, T> IntoIterator for &'a mut $name<T> {
            type Item = &'a mut T;
            type IntoIter = slice::IterMut<'a, T>;

            fn into_iter(self) -> slice::IterMut<'a, T> {
                self.0.iter_mut()
            }
        }

        // Indexing is the main thing that differentiates this from standard Vec<T>
        impl<T, I> Index<I> for $name<T> where I: SliceIndex<[T]> {
            type Output = I::Output;

            #[inline]
            fn index(&self, index: I) -> &Self::Output {
                Index::index(&**self, index)
            }
        }

        impl<T, I> IndexMut<I> for $name<T> where I: SliceIndex<[T]> {
            #[inline]
            fn index_mut(&mut self, index: I) -> &mut Self::Output {
                IndexMut::index_mut(&mut **self, index)
            }
        }
    };
}

//struct Q();
//impl Hash for Q {
//    #[inline]
//    fn hash<H: Hasher>(&self, state: &mut H) {
//        "$name".hash(state);
//        self.0.hash(state)
//    }
//}

#[allow(dead_code)]
typed_vec!(MyVec);

#[allow(unused_variables, dead_code)]
fn tmp() {
    // TODO @mverleg: remove
    let m = MyVec::<i32>::new();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_debug() {
        typed_vec!(TestVec);
        let mv = TestVec::<i32>::new();
        assert_eq!("TestVec{[]}", format!("{:?}", mv));
        // todo: with elements
    }

    #[test]
    fn test_capacity() {
        typed_vec!(TestVec);
        let mut mv = TestVec::<i32>::with_capacity(7);
        assert_eq!(7, mv.capacity());
        mv.push(1);
        mv.push(1);
        // Note that 'reserve' is relative to current length, not capacity
        mv.reserve(7);
        assert!(mv.capacity() >= 9);
    }

    // TODO @mverleg: test more methods
    // TODO @mverleg: test more traits
}

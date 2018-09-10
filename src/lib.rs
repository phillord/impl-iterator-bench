extern crate either;

use std::collections::HashSet;
use either::Either;

pub fn get_iter_flat_map<'a>(hs:&'a Option<&'a HashSet<i64>>)
            -> impl Iterator<Item=&'a i64> {
    hs.iter().flat_map(|hs| hs.iter())
}


pub fn get_iter_either<'a>(hs:&'a Option<&'a HashSet<i64>>)
    -> impl Iterator<Item=&'a i64>
{
    hs.map(|hs| Either::Right(hs.iter()))
        .unwrap_or_else(|| Either::Left(::std::iter::empty()))
}

enum MaybeIter<A> {
    Be(A),
    May,
}

impl <A>Iterator for MaybeIter<A>
    where A:Iterator
{
    type Item = A::Item;
    fn next(&mut self) -> Option<A::Item> {
        match self {
            MaybeIter::Be(i) => i.next(),
            MaybeIter::May => None
        }
    }
}

pub fn get_iter_enum<'a>(hs:&'a Option<&'a HashSet<i64>>)
                     -> impl Iterator<Item=&'a i64>
{
    match hs {
        Some(hs) =>
            MaybeIter::Be(hs.iter()),
        None => {
            MaybeIter::May
        }
    }
}

pub fn get_iter_dyn<'a>(hs:&'a Option<&'a HashSet<i64>>)
                    -> Box<Iterator<Item=&'a i64> + 'a>
{
    match hs {
        Some(hs) =>
            Box::new(hs.iter()),
        None =>
            Box::new(::std::iter::empty::<&'a i64>())
    }
}

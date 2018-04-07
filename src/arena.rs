use std::ops::{Index, IndexMut};
use std::marker::PhantomData;

use super::types::*;

/// Index for masses; used as an opaque handle
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd)]
pub struct MassIndex(u32);

impl From<MassIndex> for usize {
    fn from (mi: MassIndex) -> usize {
        mi.0 as usize
    }
}

impl From<usize> for MassIndex {
    fn from(v: usize) -> MassIndex {
        MassIndex(v as u32)
    }
}

/// Index for springs; used as an opaque handle
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd)]
pub struct SpringIndex(u32);

impl From<SpringIndex> for usize {
    fn from (si: SpringIndex) -> usize {
        si.0 as usize
    }
}


impl From<usize> for SpringIndex {
    fn from(v: usize) -> SpringIndex {
        SpringIndex(v as u32)
    }
}

/// An arena to store all the mass and spring data for the system
/// It wraps operations around masses and springs so that the indices from
/// springs to masses remain consistent
pub struct Arena {
    masses_vec: Vec<Mass>,
    springs_vec: Vec<Spring<MassIndex>>,
}

/// Used internally in conjunction with the View{,Mut} traits to ensure
/// the arena remains consistent
pub trait AddRemoveHooks<I> {
    fn add_hook(&mut self, I);
    fn remove_hook(&mut self, I);
}

/// A view into the `Arena` structure; it allows read-only access to some structure
pub struct View<'a, I, T: 'a> {
    idx_: PhantomData<I>,
    data: &'a Vec<T>
}

/// A view into the `Arena` structure; it allows full access to some structure
pub struct ViewMut<'a, I, T: 'a, H: 'a> {
    idx_: PhantomData<I>,
    data: &'a mut Vec<T>,
    hooks: H,
}

impl <'a, I: Into<usize>, T> Index<I> for View<'a, I, T> where I: Into<usize> {
    type Output = T;
    fn index(&self, idx: I) -> &T {
        &self.data[idx.into()]
    }
}

impl <'a, I, T, H> ViewMut<'a, I, T, H> where I: Into<usize> + From<usize> + Copy, H: AddRemoveHooks<I> {
    fn add(&mut self, t: T) -> I {
        self.data.push(t);
        let idx = self.data.len() - 1;
        self.hooks.add_hook(I::from(idx));
        I::from(idx)
    }

    fn remove(&mut self, idx: I) {
        self.hooks.remove_hook(idx);
        let _ = self.data.remove(idx.into());
    }
}

impl <'a, I, T, H> Index<I> for ViewMut<'a, I, T, H> where I: Into<usize> {
    type Output = T;
    fn index(&self, idx: I) -> &T { &self.data[idx.into()] }
}
impl <'a, I, T, H> IndexMut<I> for ViewMut<'a, I, T, H> where I: Into<usize> {
    fn index_mut(&mut self, idx: I) -> &mut T { &mut self.data[idx.into()] }
}

impl Arena {
    pub fn new() -> Arena {
        Arena {
            masses_vec: Vec::new(),
            springs_vec: Vec::new(),
        }
    }

    pub fn masses<'a>(&'a self) -> View<'a, MassIndex, Mass> {
        View { 
            idx_: PhantomData,
            data: &self.masses_vec
        }
    }
    pub fn masses_mut<'a>(&'a mut self) -> ViewMut<'a, MassIndex, Mass, MassHooks<'a>> {
        ViewMut { 
            idx_: PhantomData,
            data: &mut self.masses_vec,
            hooks: MassHooks {
                springs: &mut self.springs_vec,
            }
        }
    }

    pub fn springs<'a>(&'a self) -> View<'a, SpringIndex, Spring<MassIndex>> {
        View {
            idx_: PhantomData,
            data: &self.springs_vec
        }
    }

    pub fn springs_mut<'a>(&'a mut self) -> ViewMut<'a, SpringIndex, Spring<MassIndex>, SpringHooks> {
        ViewMut {
            idx_: PhantomData,
            data: &mut self.springs_vec,
            hooks: SpringHooks
        }
    }

    pub fn with_mut<F, R>(&mut self, f: F) -> R where F: FnOnce(&mut [Mass], &mut [Spring<MassIndex>]) -> R {
        f(&mut self.masses_vec, &mut self.springs_vec)
    }
}

pub struct MassHooks<'a> {
    springs: &'a mut Vec<Spring<MassIndex>>,
}

impl <'a> AddRemoveHooks<MassIndex> for MassHooks<'a> {
    fn add_hook(&mut self, _: MassIndex) { }
    fn remove_hook(&mut self, mi: MassIndex) {
        self.springs.retain(|s| {
            (s.endpoints[0] != mi) && (s.endpoints[1] != mi)
        });
        for ref mut s in self.springs.iter_mut() {
            // subtract one from endpoints with a higher index, because all the
            // masses past `mi` will be shifted down one when it's removed
            if s.endpoints[0] >= mi {
                let new_idx: usize = s.endpoints[0].into();
                s.endpoints[0] = MassIndex::from(new_idx - 1);
            }
            if s.endpoints[1] >= mi {
                let new_idx: usize = s.endpoints[1].into();
                s.endpoints[1] = MassIndex::from(new_idx - 1);
            }
        }
    }
}

pub struct SpringHooks;

impl AddRemoveHooks<SpringIndex> for SpringHooks {
    fn add_hook(&mut self, _: SpringIndex) { }
    fn remove_hook(&mut self, _: SpringIndex) { }
}


#[cfg(test)]
mod test {
    use super::*;

    // type check
    fn check_view() {
        let mut a = Arena::new();
        {
            let _ = a.masses()[MassIndex(5)];
            let _ = a.springs()[SpringIndex(5)];
        }
        {
            let mut masses = a.masses_mut();
            masses.remove(MassIndex(6));
        }
    }
}

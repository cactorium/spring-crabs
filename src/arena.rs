use std::ops::{Index, IndexMut};
use std::marker::PhantomData;

use super::types::*;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct MassIdx(u32);

impl Into<usize> for MassIdx {
    fn into(self) -> usize {
        let MassIdx(v) = self;
        v as usize
    }
}

impl From<usize> for MassIdx {
    fn from(v: usize) -> MassIdx {
        MassIdx(v as u32)
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct SpringIdx(u32);

impl Into<usize> for SpringIdx {
    fn into(self) -> usize {
        let SpringIdx(v) = self;
        v as usize
    }
}

impl From<usize> for SpringIdx {
    fn from(v: usize) -> SpringIdx {
        SpringIdx(v as u32)
    }
}

pub struct Arena {
    masses_vec: Vec<Mass>,
    springs_vec: Vec<Spring<MassIdx>>,
}

pub trait AddRemoveHooks<I> {
    fn add_hook(&mut self, I);
    fn remove_hook(&mut self, I);
}

// Note: indices should be considered invalidated when add or remove is called
pub struct View<'a, I, T: 'a> {
    idx_: PhantomData<I>,
    data: &'a Vec<T>
}
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

    pub fn masses<'a>(&'a self) -> View<'a, MassIdx, Mass> {
        View { 
            idx_: PhantomData,
            data: &self.masses_vec
        }
    }
    pub fn masses_mut<'a>(&'a mut self) -> ViewMut<'a, MassIdx, Mass, MassHooks<'a>> {
        ViewMut { 
            idx_: PhantomData,
            data: &mut self.masses_vec,
            hooks: MassHooks {
                springs: &mut self.springs_vec,
            }
        }
    }

    pub fn springs<'a>(&'a self) -> View<'a, SpringIdx, Spring<MassIdx>> {
        View {
            idx_: PhantomData,
            data: &self.springs_vec
        }
    }

    pub fn springs_mut<'a>(&'a mut self) -> ViewMut<'a, SpringIdx, Spring<MassIdx>, SpringHooks> {
        ViewMut {
            idx_: PhantomData,
            data: &mut self.springs_vec,
            hooks: SpringHooks
        }
    }

    pub fn with_mut<F, R>(&mut self, f: F) -> R where F: FnOnce(&mut [Mass], &mut [Spring<MassIdx>]) -> R {
        unimplemented!()
    }
}

pub struct MassHooks<'a> {
    springs: &'a mut Vec<Spring<MassIdx>>,
}

impl <'a> AddRemoveHooks<MassIdx> for MassHooks<'a> {
    fn add_hook(&mut self, _: MassIdx) { }
    fn remove_hook(&mut self, mi: MassIdx) {
        self.springs.retain(|s| {
            (s.endpoints[0] != mi) && (s.endpoints[1] != mi)
        })
    }
}

pub struct SpringHooks;

impl AddRemoveHooks<SpringIdx> for SpringHooks {
    fn add_hook(&mut self, _: SpringIdx) { }
    fn remove_hook(&mut self, _: SpringIdx) { }
}


#[cfg(test)]
mod test {
    use super::*;

    // type check
    fn check_view() {
        let mut a = Arena::new();
        {
            let _ = a.masses()[MassIdx(5)];
            let _ = a.springs()[SpringIdx(5)];
        }
        {
            let mut masses = a.masses_mut();
            masses.remove(MassIdx(6));
        }
    }
}

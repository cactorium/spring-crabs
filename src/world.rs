use std::cell::RefCell;
use std::rc::Weak;

use std::ops::Index;
use std::ops::IndexMut;

use std::marker::PhantomData;

use super::types::*;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct MassRef(usize);

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct SpringRef(usize);

trait Id: From<usize> {
    fn id(&self) -> usize;
}

impl From<usize> for MassRef {
    fn from(v: usize) -> MassRef { MassRef(v) }
}
impl From<usize> for SpringRef {
    fn from(v: usize) -> SpringRef { SpringRef(v) }
}

impl Id for MassRef {
    fn id(&self) -> usize {
        self.0
    }
}
impl Id for SpringRef {
    fn id(&self) -> usize {
        self.0
    }
}


pub struct OptionalVec<T, R> {
    _id_ty: PhantomData<R>,
    vec: Vec<Option<T>>
}

impl <T, R: Id> OptionalVec<T, R> {
    fn new() -> OptionalVec<T, R> {
        OptionalVec {
            _id_ty: PhantomData,
            vec: Vec::new(),
        }
    }
    fn add(&mut self, nt: T) -> R {
        for (i, t) in self.vec.iter_mut().enumerate() {
            if t.is_none() {
                *t = Some(nt);
                return R::from(i);
            }
        }
        self.vec.push(Some(nt));
        return R::from(self.vec.len() - 1);
    }
    fn remove(&mut self, r: R) {
        self.vec[r.id()] = None;
    }
    pub fn iter(&self) -> impl Iterator<Item=(R, &T)> {
        self.vec.iter()
            .enumerate()
            .filter(|(_, v)| v.is_some())
            .map(|(id, v)| (R::from(id), v.as_ref().unwrap()))
    }
    pub fn iter_mut(&mut self) -> impl Iterator<Item=(R, &mut T)> {
        self.vec.iter_mut()
            .enumerate()
            .filter(|(_, v)| v.is_some())
            .map(|(id, v)| (R::from(id), v.as_mut().unwrap()))
    }
}

impl <T, R: Id> Index<R> for OptionalVec<T, R> {
    type Output = T;
    fn index(&self, id: R) -> &T {
        self.vec[id.id()].as_ref().unwrap()
    }
}

impl <T, R: Id> IndexMut<R> for OptionalVec<T, R> {
    fn index_mut(&mut self, id: R) -> &mut T {
        self.vec[id.id()].as_mut().unwrap()
    }
}

pub struct World {
    pub masses: OptionalVec<Mass, MassRef>,
    pub springs: OptionalVec<Spring<MassRef>, SpringRef>,
    pub assemblies: Assemblies,
}

impl World {
    pub fn new() -> World {
        World {
            masses: OptionalVec::new(),
            springs: OptionalVec::new(),
            assemblies: Assemblies::new(),
        }
    }
    pub fn mass_add(&mut self, mass: Mass) -> MassRef {
        self.masses.add(mass)
    }
    pub fn mass_delete(&mut self, mr: MassRef) {
        self.assemblies.mass_delete(mr);
        let to_remove = self.assemblies.find_connected_springs(mr, &self);
        for s_ref in to_remove {
            self.springs.remove(s_ref);
            self.assemblies.spring_delete(s_ref);
        }
        self.masses.remove(mr);
    }
    pub fn spring_add(&mut self, spring: Spring<MassRef>) -> SpringRef {
        self.springs.add(spring)
    }
    pub fn spring_delete(&mut self, sr: SpringRef) {
        self.assemblies.spring_delete(sr);
        self.springs.remove(sr)
    }
}

impl Index<MassRef> for World {
    type Output = Mass;
    fn index(&self, mr: MassRef) -> &Mass {
        &self.masses[mr]
    }
}
impl IndexMut<MassRef> for World {
    fn index_mut(&mut self, mr: MassRef) -> &mut Mass {
        &mut self.masses[mr]
    }
}


impl Index<SpringRef> for World {
    type Output = Spring<MassRef>;
    fn index(&self, sr: SpringRef) -> &Spring<MassRef> {
        &self.springs[sr]
    }
}
impl IndexMut<SpringRef> for World {
    fn index_mut(&mut self, sr: SpringRef) -> &mut Spring<MassRef> {
        &mut self.springs[sr]
    }
}


pub struct Assemblies {
    pub assemblies: Vec<Assembly>,
    pub free: Assembly
}

impl Assemblies {
    fn new() -> Assemblies {
        Assemblies {
            assemblies: Vec::new(),
            free: Assembly::new(),
        }
    }
    fn mass_add(&mut self, mr: MassRef) {
        for ref mut assembly in &mut self.assemblies {
            assembly.mass_add(mr);
        }
        self.free.mass_add(mr);
    }
    fn mass_delete(&mut self, mr: MassRef) {
        for ref mut assembly in &mut self.assemblies {
            assembly.mass_delete(mr);
        }
        self.free.mass_delete(mr);
    }
    fn spring_add(&mut self, sr: SpringRef) {
        for ref mut assembly in &mut self.assemblies {
            assembly.spring_add(sr);
        }
        self.free.spring_add(sr);
    }
    fn spring_delete(&mut self, sr: SpringRef) {
        for ref mut assembly in &mut self.assemblies {
            assembly.spring_delete(sr);
        }
        self.free.spring_delete(sr);
    }
    fn find_connected_springs(&self, mr: MassRef, w: &World) -> impl Iterator<Item=SpringRef> {
        self.assemblies
            .iter()
            .flat_map(|assembly| assembly.find_connected_springs(mr, w))
            .chain(self.free.find_connected_springs(mr, w))
            .collect::<Vec<_>>()
            .into_iter()
    }
}

pub struct Assembly {
    pub masses: Vec<MassRef>,
    pub springs: Vec<SpringRef>,
    pub subassemblies: Vec<Assembly>
}

impl Assembly {
    fn new() -> Assembly {
        Assembly {
            masses: Vec::new(),
            springs: Vec::new(),
            subassemblies: Vec::new()
        }
    }
    fn mass_add(&mut self, mr: MassRef) {
        self.masses.push(mr);
    }
    fn mass_delete(&mut self, mr: MassRef) {
        let to_remove = self.masses
            .iter()
            .enumerate()
            .rev()
            .filter(|(_, &r)| r == mr)
            .map(|(i, _)| i)
            .collect::<Vec<_>>();
        // this works because the index is order is reversed;
        // swap removing causes problems only if one of the elements to be
        // removed is the last one and we're not currently removing the last one
        for i in to_remove {
            self.masses.swap_remove(i);
        }
    }
    fn spring_add(&mut self, sr: SpringRef) {
        self.springs.push(sr);
    }
    fn spring_delete(&mut self, sr: SpringRef) {
        let to_remove = self.springs
            .iter()
            .enumerate()
            .rev()
            .filter(|(_, &r)| r == sr)
            .map(|(i, _)| i)
            .collect::<Vec<_>>();
        for i in to_remove {
            self.springs.swap_remove(i);
        }
    }

    fn find_connected_springs(&self, mr: MassRef, w: &World) -> impl Iterator<Item=SpringRef> {
        self.springs
            .iter()
            .filter(|&s| w[*s].endpoints[0] == mr || w[*s].endpoints[1] == mr)
            .map(|s| *s)
            .chain(self.subassemblies.iter()
                   .flat_map(|assembly| assembly.find_connected_springs(mr, w)))
            .collect::<Vec<_>>()
            .into_iter()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    // make sure the interface is strong enough to do stuff
    #[test]
    fn test_spring_borrow() {
        let mut w = World::new();
        for (_, s) in w.springs.iter() {
            w.masses[s.endpoints[0]].acceleration = w.masses[s.endpoints[1]].acceleration;
        }
    }
    // TODO
}

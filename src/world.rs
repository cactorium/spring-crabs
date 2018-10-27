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
    // root assembly
    pub root: Assembly,
}

impl World {
    pub fn new() -> World {
        World {
            masses: OptionalVec::new(),
            springs: OptionalVec::new(),
            root: Assembly::new(String::from("$root_assembly")),
        }
    }
    pub fn add_mass(&mut self, mass: Mass) -> MassRef {
        let ret = self.masses.add(mass);
        self.root.add_mass(ret);
        ret
    }
    pub fn add_mass_to(&mut self, mass: Mass, path: &[usize]) -> Option<MassRef> {
        if path.len() == 0 {
            Some(self.add_mass(mass))
        } else {
            if self.root.check_path(path) {
                let ret = self.masses.add(mass);
                self.root.add_mass_to(ret, path);
                Some(ret)
            } else {
                None
            }
        }
    }
    pub fn move_mass_to(&mut self, mr: MassRef, path: &[usize]) -> bool {
        if !self.root.check_path(path) {
            false
        } else {
            self.root.delete_mass(mr);
            self.root.add_mass_to(mr, path)
        }
    }
    pub fn delete_mass(&mut self, mr: MassRef) {
        self.root.delete_mass(mr);
        let to_remove = self.root.find_connected_springs(mr, &self);
        for s_ref in to_remove {
            self.springs.remove(s_ref);
            self.root.delete_spring(s_ref);
        }
        self.masses.remove(mr);
    }
    pub fn add_spring(&mut self, spring: Spring<MassRef>) -> SpringRef {
        let ret = self.springs.add(spring);
        self.root.springs.push(ret);
        ret
    }
    pub fn add_spring_to(&mut self, spring: Spring<MassRef>, path: &[usize]) -> Option<SpringRef> {
        if path.len() == 0 {
            Some(self.add_spring(spring))
        } else {
            if self.root.check_path(path) {
                let ret = self.springs.add(spring);
                self.root.add_spring_to(ret, path);
                Some(ret)
            } else {
                None
            }
        }
    }
    pub fn move_spring_to(&mut self, sr: SpringRef, path: &[usize]) -> bool {
        if !self.root.check_path(path) {
            false
        } else {
            self.root.delete_spring(sr);
            self.root.add_spring_to(sr, path)
        }
    }

    pub fn delete_spring(&mut self, sr: SpringRef) {
        self.root.delete_spring(sr);
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


pub struct Assembly {
    pub name: String,
    pub masses: Vec<MassRef>,
    pub springs: Vec<SpringRef>,
    pub subassemblies: Vec<Assembly>
}

impl Assembly {
    fn new(assembly_name: String) -> Assembly {
        Assembly {
            name: assembly_name,
            masses: Vec::new(),
            springs: Vec::new(),
            subassemblies: Vec::new()
        }
    }
    fn check_path(&self, path: &[usize]) -> bool {
        if path.len() == 0 {
            true
        } else {
            if self.subassemblies.len() <= path[0] {
                false
            } else {
                self.subassemblies[path[0]].check_path(&path[1..])
            }
        }
    }
    fn add_mass(&mut self, mr: MassRef) {
        self.masses.push(mr);
    }
    fn add_mass_to(&mut self, mr: MassRef, path: &[usize]) -> bool {
        if path.len() == 0 {
            self.add_mass(mr);
            true
        } else {
            if self.subassemblies.len() > path[0] {
                self.subassemblies[path[0]].add_mass_to(mr, &path[1..])
            } else {
                false
            }
        }
    }
    fn delete_mass(&mut self, mr: MassRef) {
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
    fn add_spring(&mut self, sr: SpringRef) {
        self.springs.push(sr);
    }
    fn add_spring_to(&mut self, sr: SpringRef, path: &[usize]) -> bool {
        if path.len() == 0 {
            self.add_spring(sr);
            true
        } else {
            if self.subassemblies.len() > path[0] {
                self.subassemblies[path[0]].add_spring_to(sr, &path[1..])
            } else {
                false
            }
        }
    }
    fn delete_spring(&mut self, sr: SpringRef) {
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
            w.masses[s.endpoints[0]].acc = w.masses[s.endpoints[1]].acc;
        }
    }
    // TODO
}

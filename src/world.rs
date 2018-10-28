use std::ops::Index;
use std::ops::IndexMut;

use std::marker::PhantomData;

use std::rc::Rc;

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

pub trait Extension {
    /// NOTE: all of these are listed as immutable so any implementer
    /// will need to use RefCells in their implementations
    fn add_mass(&self, mr: MassRef);
    fn delete_mass(&self, mr: MassRef);
    fn add_spring(&self, sr: SpringRef);
    fn delete_spring(&self, sr: SpringRef);
    fn pre_tick(&self, timestep: Unit, world: &mut World) {}
    fn post_tick(&self, timestep: Unit, world: &mut World) {}
}

pub struct World {
    pub masses: OptionalVec<Mass, MassRef>,
    pub springs: OptionalVec<Spring<MassRef>, SpringRef>,
    // root assembly; masses and springs will be referred to from exactly one
    // assembly within the root assembly
    // this allows assemblies and subassemblies of masses/springs
    // to be made, making it easy to duplicate features
    pub root: Assembly,
    // extensions; different features that need to be kept in lockstep with
    // the mass/spring lists
    // like selection lists, barsprings, muscles, etc.
    pub extensions: Vec<Rc<dyn Extension>>
}

impl World {
    pub fn new() -> World {
        World {
            masses: OptionalVec::new(),
            springs: OptionalVec::new(),
            root: Assembly::new(String::from("$root_assembly")),
            extensions: Vec::new(),
        }
    }
    pub fn add_mass(&mut self, mass: Mass) -> MassRef {
        let ret = self.masses.add(mass);
        self.root.add_mass(ret);
        for extension in &self.extensions {
            extension.add_mass(ret);
        }
        ret
    }
    pub fn add_mass_to(&mut self, mass: Mass, path: &[usize]) -> Option<MassRef> {
        if path.len() == 0 {
            Some(self.add_mass(mass))
        } else {
            if self.root.check_path(path) {
                let ret = self.masses.add(mass);
                self.root.add_mass_to(ret, path);
                for extension in &self.extensions {
                    extension.add_mass(ret);
                }
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
        let to_remove = self.root.find_and_delete_connected_springs(mr, &self.springs);
        for s_ref in to_remove {
            self.springs.remove(s_ref);
            // self.root.delete_spring(s_ref);
            for extension in &self.extensions {
                extension.delete_spring(s_ref);
            }
        }
        for extension in &self.extensions {
            extension.delete_mass(mr);
        }
        self.masses.remove(mr);
    }
    pub fn add_spring(&mut self, spring: Spring<MassRef>) -> SpringRef {
        let ret = self.springs.add(spring);
        self.root.springs.push(ret);
        for extension in &self.extensions {
            extension.add_spring(ret);
        }
        ret
    }
    pub fn add_spring_to(&mut self, spring: Spring<MassRef>, path: &[usize]) -> Option<SpringRef> {
        if path.len() == 0 {
            Some(self.add_spring(spring))
        } else {
            if self.root.check_path(path) {
                let ret = self.springs.add(spring);
                self.root.add_spring_to(ret, path);
                for extension in &self.extensions {
                    extension.add_spring(ret);
                }
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
        for extension in &self.extensions {
            extension.delete_spring(sr);
        }
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
    fn find_and_delete_connected_springs(&mut self, mr: MassRef, springs: &OptionalVec<Spring<MassRef>, SpringRef>) -> impl Iterator<Item=SpringRef> {
        let to_remove = self.springs
            .iter()
            .enumerate()
            .filter(|(_, &s)| springs[s].endpoints[0] == mr || springs[s].endpoints[1] == mr)
            .map(|(i, &s)| (i, s))
            .collect::<Vec<_>>();
        for (idx, _) in to_remove.iter().rev() {
            self.springs.remove(*idx);
        }
        to_remove
            .into_iter()
            .map(|(_, s)| s)
            .chain(self.subassemblies.iter_mut()
                   .flat_map(|assembly| assembly.find_and_delete_connected_springs(mr, springs)))
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
    //
    struct TestExtension;
    impl Extension for TestExtension {
        fn add_mass(&self, _: MassRef) {}
        fn delete_mass(&self, _: MassRef) {}
        fn add_spring(&self, _: SpringRef) {}
        fn delete_spring(&self, _: SpringRef) {}
    }

    #[test]
    fn test_extension_cast() {
        let test = Rc::new(TestExtension);
        let mut w = World::new();
        w.extensions.push(test.clone());
    }
}

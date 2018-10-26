use std::cell::RefCell;
use std::rc::Weak;

use std::ops::Index;

use super::types::*;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct MassRef {
    id: usize,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct SpringRef {
    id: usize,
}

pub struct World {
    masses: Vec<Option<Mass>>,
    springs: Vec<Option<Spring<MassRef>>>,
    pub assemblies: Assemblies,
}

impl World {
    pub fn mass_add(&mut self, mass: Mass) -> MassRef {
        for (i, m) in self.masses.iter_mut().enumerate() {
            if m.is_none() {
                *m = Some(mass);
                return MassRef{ id: i };
            }
        }
        self.masses.push(Some(mass));
        return MassRef {
            id: self.masses.len() - 1,
        }
    }
    pub fn mass_delete(&mut self, mr: MassRef) {
        self.assemblies.mass_delete(mr);
        let to_remove = self.assemblies.find_connected_springs(mr, &self);
        for s_ref in to_remove {
            self.springs[s_ref.id] = None;
            self.assemblies.spring_delete(s_ref);
        }
        self.masses[mr.id] = None;
    }
    pub fn mass_iter(&self) -> impl Iterator<Item=(MassRef, &Mass)> {
        self.masses
            .iter()
            .enumerate()
            .filter(|(_, m)| m.is_some())
            .map(|(id, m)| (MassRef{ id:id }, m.as_ref().unwrap()))
    }
    pub fn mass_iter_mut(&mut self) -> impl Iterator<Item=(MassRef, &mut Mass)> {
        self.masses
            .iter_mut()
            .enumerate()
            .filter(|(_, m)| m.is_some())
            .map(|(id, m)| (MassRef{ id:id }, m.as_mut().unwrap()))
    }
    pub fn spring_add(&mut self, spring: Spring<MassRef>) -> SpringRef {
        for (i, s) in self.springs.iter_mut().enumerate() {
            if s.is_none() {
                *s = Some(spring);
                return SpringRef { id: i };
            }
        }
        self.springs.push(Some(spring));
        return SpringRef {
            id: self.springs.len() - 1,
        }
    }
    pub fn spring_delete(&mut self, sr: SpringRef) {
        self.assemblies.spring_delete(sr);
        self.springs[sr.id] = None;
    }
    pub fn spring_iter(&self) -> impl Iterator<Item=(SpringRef, &Spring<MassRef>)> {
        self.springs
            .iter()
            .enumerate()
            .filter(|(_, s)| s.is_some())
            .map(|(id, s)| (SpringRef{ id:id }, s.as_ref().unwrap()))
    }
    pub fn spring_iter_mut(&mut self) -> impl Iterator<Item=(SpringRef, &mut Spring<MassRef>)> {
        self.springs
            .iter_mut()
            .enumerate()
            .filter(|(_, s)| s.is_some())
            .map(|(id, s)| (SpringRef{ id:id }, s.as_mut().unwrap()))
    }
}

impl Index<SpringRef> for World {
    type Output = Spring<MassRef>;
    fn index(&self, sr: SpringRef) -> &Spring<MassRef> {
        self.springs[sr.id].as_ref().unwrap()
    }
}

pub struct Assemblies {
    pub assemblies: Vec<Assembly>,
    pub free: Assembly
}

impl Assemblies {
    fn mass_delete(&mut self, mr: MassRef) {
        for ref mut assembly in &mut self.assemblies {
            assembly.mass_delete(mr);
        }
        self.free.mass_delete(mr);
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
    fn mass_delete(&mut self, mr: MassRef) {
        unimplemented!()
    }
    fn spring_delete(&mut self, sr: SpringRef) {
        unimplemented!()
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



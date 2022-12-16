use std::{cell::RefCell, rc::Rc};

#[derive(Clone, Debug)]
pub struct Valve {
    name: String,
    flow_rate: usize,
    pub tunnels: Vec<String>,
}

impl Valve {
    pub fn new(name: String, flow_rate: usize, tunnels: Vec<String>) -> Self {
        Self {
            name,
            flow_rate,
            tunnels,
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_tunnels(&self) -> &Vec<String> {
        &self.tunnels
    }

    pub fn get_flow_rate(&self) -> usize {
        self.flow_rate
    }
}

// Dijkstra of Grim
pub struct Dog {
    visited: RefCell<bool>,
    distance: RefCell<usize>,
    friends: RefCell<Vec<Rc<Dog>>>,
}

impl Dog {
    pub fn new() -> Rc<Dog> {
        Rc::new(Self {
            visited: RefCell::new(false),
            distance: RefCell::new(usize::MAX),
            friends: RefCell::new(Vec::new()),
        })
    }

    pub fn push(&self, friend: Rc<Dog>) {
        self.friends.borrow_mut().push(friend)
    }

    pub fn set_distance(&self, distance: usize) {
        *self.distance.borrow_mut() = distance
    }

    pub fn get_distance(&self) -> usize {
        *self.distance.borrow()
    }

    pub fn get_friends(&self) -> Vec<Rc<Dog>> {
        self.friends.borrow().clone()
    }

    pub fn is_visited(&self) -> bool {
        *self.visited.borrow()
    }

    pub fn set_visited(&self) {
        *self.visited.borrow_mut() = true
    }

    pub fn reset(&self) {
        *self.visited.borrow_mut() = false;
        *self.distance.borrow_mut() = usize::MAX
    }
}

#[derive(Clone)]
pub struct Path {
    cost: usize,
    tunnel: String,
}

impl std::fmt::Debug for Path {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} to {}", self.cost, self.tunnel)
    }
}

impl Path {
    pub fn new(tunnel: String, cost: usize) -> Self {
        Self { tunnel, cost }
    }

    pub fn get_cost(&self) -> usize {
        self.cost
    }

    pub fn get_tunnel(&self) -> &str {
        &self.tunnel
    }
}

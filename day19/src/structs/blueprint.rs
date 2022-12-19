pub struct Blueprint {
    pub id: usize,
    pub ore_robot_cost: usize,
    pub clay_robot_cost: usize,
    pub obsidian_robot_cost: (usize, usize),
    pub geode_robot_cost: (usize, usize),
}

#[derive(Default)]
pub struct BlueprintBuilder {
    id: usize,
    ore_robot_cost: Option<usize>,
    clay_robot_cost: Option<usize>,
    obsidian_robot_cost: Option<(usize, usize)>,
    geode_robot_cost: Option<(usize, usize)>,
}

impl BlueprintBuilder {
    pub fn new(id: usize) -> Self {
        Self {
            id,
            ..Default::default()
        }
    }

    pub fn set_ore_robot_cost(mut self, ore_cost: usize) -> Self {
        self.ore_robot_cost = Some(ore_cost);
        self
    }

    pub fn set_clay_robot_cost(mut self, ore_cost: usize) -> Self {
        self.clay_robot_cost = Some(ore_cost);
        self
    }

    pub fn set_obsidian_robot_cost(mut self, ore_cost: usize, clay_cost: usize) -> Self {
        self.obsidian_robot_cost = Some((ore_cost, clay_cost));
        self
    }

    pub fn set_geode_robot_cost(mut self, ore_cost: usize, obsidian_cost: usize) -> Self {
        self.geode_robot_cost = Some((ore_cost, obsidian_cost));
        self
    }

    pub fn build(self) -> Blueprint {
        Blueprint {
            id: self.id,
            ore_robot_cost: self.ore_robot_cost.unwrap(),
            clay_robot_cost: self.clay_robot_cost.unwrap(),
            obsidian_robot_cost: self.obsidian_robot_cost.unwrap(),
            geode_robot_cost: self.geode_robot_cost.unwrap(),
        }
    }
}

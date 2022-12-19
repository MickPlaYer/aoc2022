mod blueprint;

pub use blueprint::{Blueprint, BlueprintBuilder};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum RobotType {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

impl RobotType {
    fn all_types() -> Vec<RobotType> {
        vec![
            RobotType::Geode,
            RobotType::Obsidian,
            RobotType::Clay,
            RobotType::Ore,
        ]
    }
}

#[derive(Default, Clone, Debug, Eq, PartialEq, Hash)]
pub struct State {
    pub ore_robots: usize,
    pub clay_robots: usize,
    pub obsidian_robots: usize,
    pub geode_robots: usize,
    pub ore: usize,
    pub clay: usize,
    pub obsidian: usize,
    pub geode: usize,
    pub factory: Option<RobotType>,
}

impl State {
    pub fn new() -> Self {
        Self {
            ore_robots: 1,
            ..Default::default()
        }
    }

    pub fn collect_resource(&mut self) {
        self.ore += self.ore_robots;
        self.clay += self.clay_robots;
        self.obsidian += self.obsidian_robots;
        self.geode += self.geode_robots;
    }

    pub fn try_build_robot(mut self, blueprint: &Blueprint) -> Vec<Self> {
        let mut futures = Vec::new();

        let robot_type = RobotType::Geode;
        if self.can_build(blueprint, &robot_type) {
            self.start_build_robot(blueprint, robot_type);
            futures.push(self);
            return futures;
        }

        for robot_type in RobotType::all_types() {
            if self.can_build(blueprint, &robot_type) {
                let mut future = self.clone();
                future.start_build_robot(blueprint, robot_type);
                futures.push(future);
            }
        }
        futures.push(self);
        futures
    }

    fn can_build(&self, blueprint: &Blueprint, robot_type: &RobotType) -> bool {
        match robot_type {
            RobotType::Ore => self.ore >= blueprint.ore_robot_cost,
            RobotType::Clay => self.ore >= blueprint.clay_robot_cost,
            RobotType::Obsidian => {
                let (ore_cost, clay_cost) = blueprint.obsidian_robot_cost;
                self.ore >= ore_cost && self.clay >= clay_cost
            }
            RobotType::Geode => {
                let (ore_cost, obsidian_cost) = blueprint.geode_robot_cost;
                self.ore >= ore_cost && self.obsidian >= obsidian_cost
            }
        }
    }

    fn start_build_robot(&mut self, blueprint: &Blueprint, robot_type: RobotType) {
        match robot_type {
            RobotType::Ore => self.ore -= blueprint.ore_robot_cost,
            RobotType::Clay => self.ore -= blueprint.clay_robot_cost,
            RobotType::Obsidian => {
                let (ore_cost, clay_cost) = blueprint.obsidian_robot_cost;
                self.ore -= ore_cost;
                self.clay -= clay_cost;
            }
            RobotType::Geode => {
                let (ore_cost, obsidian_cost) = blueprint.geode_robot_cost;
                self.ore -= ore_cost;
                self.obsidian -= obsidian_cost;
            }
        }
        self.factory = Some(robot_type);
    }

    pub fn finished_robot(&mut self) {
        if let Some(robot_type) = &self.factory {
            match robot_type {
                RobotType::Ore => self.ore_robots += 1,
                RobotType::Clay => self.clay_robots += 1,
                RobotType::Obsidian => self.obsidian_robots += 1,
                RobotType::Geode => self.geode_robots += 1,
            }
            self.factory = None
        }
    }
}

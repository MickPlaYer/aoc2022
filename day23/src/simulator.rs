use crate::{
    crater_map::find_neighbors,
    structs::{Coordinate, Creature, Direction},
};
use std::collections::HashMap;

pub fn simluate_round(
    map: &HashMap<Coordinate, Creature>,
    directions: &Vec<Direction>,
) -> HashMap<Coordinate, Creature> {
    let mut proposals = HashMap::new();
    for creature in map.values() {
        let neighbors = find_neighbors(&map, creature.coordinate());
        if neighbors.is_empty() {
            let proposal = creature.just_stay();
            proposals.insert(proposal.to, proposal);
            continue;
        }
        let mut proposes_moving = false;
        for direction in directions {
            if neighbors.have_space(direction) {
                let proposal = creature.proposes_moving(direction);
                if let Some(previous_proposal) = proposals.get_mut(&proposal.to) {
                    previous_proposal.reject();
                } else {
                    proposals.insert(proposal.to, proposal);
                    proposes_moving = true;
                }
                break;
            }
        }
        if !proposes_moving {
            let proposal = creature.just_stay();
            proposals.insert(proposal.to, proposal);
        }
    }
    let mut map = HashMap::new();
    for proposal in proposals.into_values() {
        let creature = proposal.perform();
        map.insert(creature.coordinate(), creature);
    }
    map
}

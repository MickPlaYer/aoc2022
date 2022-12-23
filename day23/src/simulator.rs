use crate::{
    crater_map::find_neighbors,
    structs::{Coordinate, Creature, Direction, Neighbors, Proposal},
};
use std::collections::HashMap;

pub fn simluate_round(
    map: &HashMap<Coordinate, Creature>,
    directions: &Vec<Direction>,
) -> Result<HashMap<Coordinate, Creature>, String> {
    let mut proposals = HashMap::new();
    for creature in map.values() {
        let neighbors = find_neighbors(&map, creature.coordinate());
        if neighbors.is_empty() {
            let proposal = creature.just_stay();
            proposals.insert(proposal.coordinate(), proposal);
            continue;
        }
        let proposes_moving = try_each_direction(directions, neighbors, creature, &mut proposals);
        if !proposes_moving {
            let proposal = creature.just_stay();
            proposals.insert(proposal.coordinate(), proposal);
        }
    }
    if proposals
        .values()
        .all(|proposal| proposal.is_stay() || proposal.is_rejeted())
    {
        return Err("Can't not move anymore!".to_string());
    }
    let mut map = HashMap::new();
    for proposal in proposals.into_values() {
        let creature = proposal.perform();
        map.insert(creature.coordinate(), creature);
    }
    Ok(map)
}

fn try_each_direction(
    directions: &Vec<Direction>,
    neighbors: Neighbors,
    creature: &Creature,
    proposals: &mut HashMap<Coordinate, Proposal>,
) -> bool {
    for direction in directions {
        if neighbors.have_space(direction) {
            let proposal = creature.proposes_moving(direction);
            if let Some(previous_proposal) = proposals.get_mut(&proposal.coordinate()) {
                previous_proposal.reject();
                return false;
            } else {
                proposals.insert(proposal.coordinate(), proposal);
                return true;
            }
        }
    }
    false
}

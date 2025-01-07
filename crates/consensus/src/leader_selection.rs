use std::{cmp::Ordering, collections::HashSet};

use alloy::primitives::BlockNumber;
use angstrom_types::primitive::PeerId;

// https://github.com/tendermint/tendermint/pull/2785#discussion_r235038971
// 1.125
const PENALTY_FACTOR: u64 = 1125;
/// do the math with fixed here to avoid floats
const ONE_E3: u64 = 1000;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct AngstromValidator {
    pub peer_id:  PeerId,
    voting_power: u64,
    priority:     i64
}

impl AngstromValidator {
    pub fn new(name: PeerId, voting_power: u64) -> Self {
        AngstromValidator {
            peer_id:      name,
            voting_power: voting_power * ONE_E3,
            priority:     0
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct WeightedRoundRobin {
    validators:                HashSet<AngstromValidator>,
    new_joiner_penalty_factor: u64,
    block_number:              BlockNumber,
    last_proposer:             Option<PeerId>
}

impl WeightedRoundRobin {
    pub fn new(validators: Vec<AngstromValidator>, block_number: BlockNumber) -> Self {
        WeightedRoundRobin {
            validators: HashSet::from_iter(validators),
            new_joiner_penalty_factor: PENALTY_FACTOR,
            block_number,
            last_proposer: None
        }
    }

    fn proposer_selection(&mut self) -> PeerId {
        let total_voting_power: u64 = self.validators.iter().map(|v| v.voting_power).sum();

        //  apply all priorities.
        self.validators = self
            .validators
            .drain()
            .map(|mut validator| {
                validator.priority += validator.voting_power as i64;
                validator
            })
            .collect();

        // find the max
        let mut proposer = self
            .validators
            .iter()
            .max_by(Self::priority)
            .unwrap()
            .clone();
        proposer.priority -= total_voting_power as i64;
        let proposer_name = proposer.peer_id;

        self.validators.replace(proposer);

        proposer_name
    }

    fn priority(a: &&AngstromValidator, b: &&AngstromValidator) -> Ordering {
        let out = a.priority.partial_cmp(&b.priority);
        if out == Some(Ordering::Equal) {
            // TODO: not the best because it encourages mining lower peer ids
            // however we need a way for this to be uniform across nodes and
            // this is the easiest
            return a.peer_id.cmp(&b.peer_id)
        }
        out.unwrap()
    }

    fn center_priorities(&mut self) {
        let avg_priority = (self.validators.iter().map(|v| v.priority).sum::<i64>()
            * ONE_E3 as i64)
            / self.validators.len() as i64;

        self.validators = self
            .validators
            .drain()
            .map(|mut validator| {
                validator.priority -= avg_priority;
                validator
            })
            .collect();
    }

    fn scale_priorities(&mut self) {
        let max_priority = self
            .validators
            .iter()
            .map(|v| v.priority)
            .fold(i64::MIN, i64::max);
        let min_priority = self
            .validators
            .iter()
            .map(|v| v.priority)
            .fold(i64::MAX, i64::min);

        let total_voting_power: u64 = self.validators.iter().map(|v| v.voting_power).sum();
        let diff = max_priority - min_priority;
        let threshold = 2 * total_voting_power as i64;

        if diff > threshold {
            let scale = (diff * ONE_E3 as i64) / threshold;

            self.validators = self
                .validators
                .drain()
                .map(|mut validator| {
                    let new_pri = validator.priority * ONE_E3 as i64;
                    validator.priority = new_pri / scale;
                    validator
                })
                .collect();
        }
    }

    pub fn choose_proposer(&mut self, block_number: BlockNumber) -> Option<PeerId> {
        // 1. this is not ideal, since on multi-block reorgs the same proposer will be
        //    chosen for the length of the reorg
        // 2. reverting the block number (self.block_number = block_number) is also not
        //    ideal, since nodes who were offline will not have seen the reorg, thus
        //    would not have executed the extra rounds after this if statement
        if block_number <= self.block_number {
            if self.last_proposer.is_none() {
                self.last_proposer = Some(self.proposer_selection());
            }

            return self.last_proposer
        }

        let rounds_to_catchup = (block_number - self.block_number) as usize;
        let mut leader = None;
        for _ in 0..rounds_to_catchup {
            self.center_priorities();
            self.scale_priorities();
            leader = Some(self.proposer_selection());
            self.last_proposer = leader
        }
        self.block_number = block_number;
        leader
    }

    #[allow(dead_code)]
    fn remove_validator(&mut self, peer_id: &PeerId) {
        let validator = AngstromValidator::new(*peer_id, 0);
        self.validators.remove(&validator);
    }

    #[allow(dead_code)]
    fn add_validator(&mut self, peer_id: PeerId, voting_power: u64) {
        let mut new_validator = AngstromValidator::new(peer_id, voting_power);
        let total_voting_power: u64 = self.validators.iter().map(|v| v.voting_power).sum();
        new_validator.priority -=
            ((self.new_joiner_penalty_factor * total_voting_power) / ONE_E3) as i64;
        self.validators.insert(new_validator);
    }
}

impl PartialEq for AngstromValidator {
    fn eq(&self, other: &Self) -> bool {
        self.peer_id == other.peer_id
    }
}

impl Eq for AngstromValidator {}

impl std::hash::Hash for AngstromValidator {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.peer_id.hash(state);
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    #[test]
    fn test_round_robin_simulation() {
        let peers = HashMap::from([
            ("Alice".to_string(), PeerId::random()),
            ("Bob".to_string(), PeerId::random()),
            ("Charlie".to_string(), PeerId::random())
        ]);
        let validators = vec![
            AngstromValidator::new(peers["Alice"], 100),
            AngstromValidator::new(peers["Bob"], 200),
            AngstromValidator::new(peers["Charlie"], 300),
        ];
        let mut algo = WeightedRoundRobin::new(validators, BlockNumber::default());

        fn simulate_rounds(algo: &mut WeightedRoundRobin, rounds: usize) -> HashMap<PeerId, usize> {
            let mut stats = HashMap::new();
            for i in 1..=rounds {
                let proposer = algo.choose_proposer(BlockNumber::from(i as u64)).unwrap();
                *stats.entry(proposer).or_insert(0) += 1;
            }
            stats
        }

        let rounds = 1000;
        let stats = simulate_rounds(&mut algo, rounds);

        assert_eq!(stats.len(), 3);

        let total_selections: usize = stats.values().sum();
        assert_eq!(total_selections, rounds);

        let alice_ratio = *stats.get(&peers["Alice"]).unwrap() as f64 / rounds as f64;
        let bob_ratio = *stats.get(&peers["Bob"]).unwrap() as f64 / rounds as f64;
        let charlie_ratio = *stats.get(&peers["Charlie"]).unwrap() as f64 / rounds as f64;

        assert!((alice_ratio - 0.167).abs() < 0.05);
        assert!((bob_ratio - 0.333).abs() < 0.05);
        assert!((charlie_ratio - 0.5).abs() < 0.05);
    }

    #[test]
    fn test_add_remove_validator() {
        let peers = HashMap::from([
            ("Alice".to_string(), PeerId::random()),
            ("Bob".to_string(), PeerId::random()),
            ("Charlie".to_string(), PeerId::random())
        ]);
        let validators = vec![
            AngstromValidator::new(peers["Alice"], 100),
            AngstromValidator::new(peers["Bob"], 200),
        ];
        let mut algo = WeightedRoundRobin::new(validators, BlockNumber::default());

        fn simulate_rounds(
            algo: &mut WeightedRoundRobin,
            rounds: usize,
            offset: usize
        ) -> HashMap<PeerId, usize> {
            let mut stats = HashMap::new();
            for i in offset..(offset + rounds) {
                let proposer = algo.choose_proposer(BlockNumber::from(i as u64)).unwrap();
                *stats.entry(proposer).or_insert(0) += 1;
            }
            stats
        }

        let rounds = 1000;
        let initial_stats = simulate_rounds(&mut algo, rounds, 1);
        assert_eq!(initial_stats.len(), 2);

        algo.add_validator(peers["Charlie"], 300);

        let after_add_stats = simulate_rounds(&mut algo, rounds, 1001);
        assert_eq!(after_add_stats.len(), 3);
        assert!(after_add_stats.contains_key(&peers["Charlie"]));

        algo.remove_validator(&peers["Bob"]);

        let after_remove_stats = simulate_rounds(&mut algo, rounds, 2001);
        assert_eq!(after_remove_stats.len(), 2);
        assert!(!after_remove_stats.contains_key(&peers["Bob"]));
    }
}

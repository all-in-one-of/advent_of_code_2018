day!(
    day24,
    "https://adventofcode.com/2018/day/24/input",
    part1,
    part2
);

use regex::Regex;
use std::collections::HashSet;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum DamageType {
    Fire,
    Cold,
    Bludgeoning,
    Slashing,
    Radiation,
}

impl FromStr for DamageType {
    type Err = Error;
    fn from_str(s: &str) -> Result<DamageType> {
        Ok(match s {
            "fire" => DamageType::Fire,
            "cold" => DamageType::Cold,
            "bludgeoning" => DamageType::Bludgeoning,
            "slashing" => DamageType::Slashing,
            "radiation" => DamageType::Radiation,
            _ => return Err(Error::Input("invalid damage type")),
        })
    }
}

#[derive(Debug, Clone)]
struct Group {
    units: u32,
    hp: u32,
    attack: (u32, DamageType),
    initiative: u32,
    weaknesses: HashSet<DamageType>,
    immunities: HashSet<DamageType>,
}

impl FromStr for Group {
    type Err = Error;
    fn from_str(s: &str) -> Result<Group> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^(\d+) units each with (\d+) hit points (\([^)]+\) )?with an attack that does (\d+) ([a-z]+) damage at initiative (\d+)$").unwrap();
        }

        let c = RE.captures(s).ok_or(Error::Input("invalid group"))?;
        let units = c[1].parse()?;
        let hp = c[2].parse()?;
        let special = c.get(3).map(|x| x.as_str()).map(|x| &x[1..x.len() - 2]);
        let attack_damage = c[4].parse()?;
        let attack_type = c[5].parse()?;
        let initiative = c[6].parse()?;

        let mut weaknesses = HashSet::new();
        let mut immunities = HashSet::new();
        macro_rules! err {
            () => {
                Error::Input("invalid special")
            };
        }
        if let Some(special) = special {
            for special in special.split("; ") {
                let mut words = special.split(' ');
                let target = match words.next().ok_or(err!())? {
                    "weak" => &mut weaknesses,
                    "immune" => &mut immunities,
                    _ => return Err(err!()),
                };
                if words.next().ok_or(err!())? != "to" {
                    return Err(err!());
                }
                let mut item = words.next();
                if item.is_none() {
                    return Err(err!());
                }
                while let Some(mut damage_type) = item {
                    if damage_type.ends_with(',') {
                        damage_type = &damage_type[0..damage_type.len() - 1];
                    }
                    target.insert(damage_type.parse()?);
                    item = words.next();
                }
            }
        }

        if weaknesses.intersection(&immunities).next().is_some() {
            return Err(Error::Input(
                "a unit cannot be immune and weak to the same damage type",
            ));
        }

        Ok(Group {
            units,
            hp,
            attack: (attack_damage, attack_type),
            initiative,
            weaknesses,
            immunities,
        })
    }
}

#[derive(Debug, Clone)]
struct Armies {
    immune_system: Vec<Group>,
    infection: Vec<Group>,
}

impl FromStr for Armies {
    type Err = Error;
    fn from_str(s: &str) -> Result<Armies> {
        lazy_static! {
            static ref RE: Regex = Regex::new(
                r"^Immune System:
((?s).+)

Infection:
((?s).+)$"
            )
            .unwrap();
        }
        let c = RE.captures(s).ok_or(Error::Input("invalid input format"))?;
        Ok(Armies {
            immune_system: c[1].lines().map(Group::from_str).collect::<Result<_>>()?,
            infection: c[2].lines().map(Group::from_str).collect::<Result<_>>()?,
        })
    }
}

impl Group {
    #[inline(always)]
    fn effective_power(&self) -> u32 {
        self.units * self.attack.0
    }

    fn predict_damage_against(&self, target: &Group) -> u32 {
        if target.immunities.contains(&self.attack.1) {
            return 0;
        }
        if target.weaknesses.contains(&self.attack.1) {
            return self.effective_power() * 2;
        }
        self.effective_power()
    }
}

enum Victor {
    ImmuneSystem,
    Infection,
    Stalemate,
}

impl Armies {
    fn prune_empty_groups(&mut self) {
        self.immune_system.drain_filter(|group| group.units == 0);
        self.infection.drain_filter(|group| group.units == 0);
    }

    fn fight(&mut self) -> u32 {
        self.prune_empty_groups();

        // Sort by decreasing effective power
        fn targeting_order(a: &Group, b: &Group) -> ::std::cmp::Ordering {
            b.effective_power()
                .cmp(&a.effective_power())
                .then_with(|| b.initiative.cmp(&a.initiative))
        }
        self.immune_system.sort_by(targeting_order);
        self.infection.sort_by(targeting_order);

        // Choose targets
        let mut already_chosen_targets = HashSet::new();
        fn choose_target(
            group: &Group,
            targets: &Vec<Group>,
            already_chosen_targets: &mut HashSet<usize>,
        ) -> Option<usize> {
            let target = targets
                .iter()
                .enumerate()
                .filter(|(i, _)| !already_chosen_targets.contains(&i))
                .map(|(i, target)| (i, target, group.predict_damage_against(target)))
                .filter(|(_, _, damage)| *damage != 0)
                .max_by(|(_, at, ad), (_, bt, bd)| {
                    ad.cmp(bd)
                        .then_with(|| at.effective_power().cmp(&bt.effective_power()))
                        .then_with(|| at.initiative.cmp(&bt.initiative))
                })
                .map(|(i, _, _)| i);
            if let Some(target) = target {
                already_chosen_targets.insert(target);
            }
            target
        }
        let immune_system_targets = self
            .immune_system
            .iter()
            .map(|group| choose_target(group, &self.infection, &mut already_chosen_targets))
            .collect::<Vec<_>>();
        already_chosen_targets.clear();
        let infection_targets = self
            .infection
            .iter()
            .map(|group| choose_target(group, &self.immune_system, &mut already_chosen_targets))
            .collect::<Vec<_>>();

        // Create attack order
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        enum Index {
            ImmuneSystem(usize),
            Infection(usize),
        }
        impl Index {
            fn get<'s, 'a>(&'s self, armies: &'a Armies) -> &'a Group {
                match *self {
                    Index::ImmuneSystem(i) => &armies.immune_system[i],
                    Index::Infection(i) => &armies.infection[i],
                }
            }
            fn get_mut<'s, 'a>(&'s self, armies: &'a mut Armies) -> &'a mut Group {
                match *self {
                    Index::ImmuneSystem(i) => &mut armies.immune_system[i],
                    Index::Infection(i) => &mut armies.infection[i],
                }
            }
        }

        let mut attack_order = (0..self.immune_system.len())
            .map(|i| Index::ImmuneSystem(i))
            .chain((0..self.infection.len()).map(|i| Index::Infection(i)))
            .collect::<Vec<_>>();
        attack_order.sort_by(|a, b| b.get(self).initiative.cmp(&a.get(self).initiative));

        // Attack
        let mut total_units_killed = 0;
        for unit in attack_order {
            let target = match unit {
                Index::ImmuneSystem(i) => immune_system_targets[i].map(|j| Index::Infection(j)),
                Index::Infection(i) => infection_targets[i].map(|j| Index::ImmuneSystem(j)),
            };
            // Skip if it has no target
            let target = if let Some(target) = target {
                target
            } else {
                continue;
            };

            // Damage target
            let intended_damage = unit.get(self).predict_damage_against(target.get(self));
            let target = target.get_mut(self);
            let units_killed = target.units.min(intended_damage / target.hp);
            target.units -= units_killed;
            total_units_killed += units_killed;
        }

        total_units_killed
    }

    fn fight_to_victory(&mut self) -> Victor {
        while self.fight() > 0 {}
        self.prune_empty_groups();
        if self.immune_system.is_empty() {
            Victor::Infection
        } else if self.infection.is_empty() {
            Victor::ImmuneSystem
        } else {
            Victor::Stalemate
        }
    }

    fn boost_immune_system(&mut self, strength: u32) {
        for unit in &mut self.immune_system {
            unit.attack.0 += strength;
        }
    }
}

fn part1(input: &str) -> Result<u32> {
    let mut armies: Armies = input.parse()?;

    while armies.fight() > 0 {}
    Ok(armies
        .immune_system
        .iter()
        .chain(armies.infection.iter())
        .map(|group| group.units)
        .sum())
}

fn part2(input: &str) -> Result<String> {
    let initial_armies: Armies = input.parse()?;

    // Find an amount that'd let the immune system win
    let mut min_bonus = 0; // exclusive
    let mut max_bonus = 1; // inclusive
    let bonus = loop {
        let mut armies = initial_armies.clone();
        armies.boost_immune_system(max_bonus);
        match armies.fight_to_victory() {
            Victor::Stalemate | Victor::Infection => {
                min_bonus = max_bonus;
                max_bonus *= 2;
                continue;
            }
            Victor::ImmuneSystem => {}
        }
        std::mem::drop(armies);

        break loop {
            // min => loss
            // max => win
            let delta = max_bonus - min_bonus;
            if delta <= 1 {
                break max_bonus;
            }
            let midpoint = min_bonus + delta / 2;
            let mut armies = initial_armies.clone();
            armies.boost_immune_system(midpoint);
            match armies.fight_to_victory() {
                Victor::Stalemate | Victor::Infection => {
                    min_bonus = midpoint;
                }
                Victor::ImmuneSystem => {
                    max_bonus = midpoint;
                }
            }
        };
    };

    let mut armies = initial_armies;
    armies.boost_immune_system(bonus);
    armies.fight_to_victory();
    
    Ok(format!(
        "{} => {}",
        bonus,
        armies
            .immune_system
            .iter()
            .map(|group| group.units)
            .sum::<u32>()
    ))
}

#[test]
fn day24_test() {
    const EXAMPLE: &str = "\
Immune System:
17 units each with 5390 hit points (weak to radiation, bludgeoning) with an attack that does 4507 fire damage at initiative 2
989 units each with 1274 hit points (immune to fire; weak to bludgeoning, slashing) with an attack that does 25 slashing damage at initiative 3

Infection:
801 units each with 4706 hit points (weak to radiation) with an attack that does 116 bludgeoning damage at initiative 1
4485 units each with 2961 hit points (immune to radiation; weak to fire, cold) with an attack that does 12 slashing damage at initiative 4";

    assert_results!(part1, EXAMPLE => 5216);
    assert_results!(part2, EXAMPLE => "1570 => 51");
}

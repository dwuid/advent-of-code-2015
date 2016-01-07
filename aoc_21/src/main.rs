
extern crate itertools;

use std::ops::Add;
use itertools::Itertools;

struct Character {
    hitpoints: u16,
    damage:    u16,
    armor:     u16
}

impl Character {
    fn rounds(&self, opponent: &Character) -> u16 {
        let round_damage: f64 =
            if opponent.armor >= self.damage { 1 }
            else { self.damage - opponent.armor } as f64;

        ((opponent.hitpoints as f64) / round_damage).ceil() as u16
    }

    fn beats(&self, opponent: &Character) -> bool {
        let a = self.rounds(opponent);
        let b = opponent.rounds(&self);

        a <= b
    }
}

#[derive(Clone)]
struct Inventory {
    cost:   u16,
    damage: u16,
    armor:  u16
}

impl<'a, 'b> Add<&'b Inventory> for &'a Inventory {
    type Output = Inventory;

    fn add(self, other: &'b Inventory) -> Inventory {
        Inventory {
            cost:   self.cost   + other.cost,
            damage: self.damage + other.damage,
            armor:  self.armor  + other.armor
        }
    }
}

fn main() {
    let weapons = [
        Inventory { cost: 8,  damage: 4, armor: 0 },
        Inventory { cost: 10, damage: 5, armor: 0 },
        Inventory { cost: 25, damage: 6, armor: 0 },
        Inventory { cost: 40, damage: 7, armor: 0 },
        Inventory { cost: 74, damage: 8, armor: 0 },
    ];

    let armor = [
        Inventory { cost: 13,  damage: 0, armor: 1 },
        Inventory { cost: 31,  damage: 0, armor: 2 },
        Inventory { cost: 53,  damage: 0, armor: 3 },
        Inventory { cost: 75,  damage: 0, armor: 4 },
        Inventory { cost: 102, damage: 0, armor: 5 },
    ];

    let rings = [
        Inventory { cost: 25,  damage: 1, armor: 0 },
        Inventory { cost: 50,  damage: 2, armor: 0 },
        Inventory { cost: 100, damage: 3, armor: 0 },
        Inventory { cost: 20,  damage: 0, armor: 1 },
        Inventory { cost: 40,  damage: 0, armor: 2 },
        Inventory { cost: 80,  damage: 0, armor: 3 },
    ];

    let boss = Character { hitpoints: 104, damage: 8, armor: 1 };
    let mut minimal_inventory = Inventory {
        cost: u16::max_value(), damage: 0, armor: 0
    };

    let mut maximal_inventory = Inventory {
        cost: 0, damage: 0, armor: 0
    };

    for weapon in &weapons {
        let mut inventories = Vec::new();
        inventories.push(weapon.clone());

        for armor in &armor {
            inventories.push(weapon + armor);
        }

        let mut additional = Vec::with_capacity(rings.len() * 3);
        for rings in rings.iter().combinations() {
            for inventory in inventories.iter() {
                additional.push(inventory + &rings.0);
                additional.push(inventory + &rings.1);
                additional.push(inventory + &(rings.0 + rings.1));
            }
        }

        inventories.extend(additional);
        for inventory in &inventories {
            let player = Character {
                hitpoints: 100,
                damage:    inventory.damage,
                armor:     inventory.armor
            };

            let winner = player.beats(&boss);
            if winner && inventory.cost < minimal_inventory.cost {
                minimal_inventory = inventory.clone();
            } else if !winner && inventory.cost > maximal_inventory.cost {
                maximal_inventory = inventory.clone();
            }
        }
    }

    println!("Uncle Scrooge wins by spending merely {} bucks.",
             minimal_inventory.cost);
    println!("Betrayed by the shopkeeper, he pays {} bucks and loses.",
             maximal_inventory.cost);
}


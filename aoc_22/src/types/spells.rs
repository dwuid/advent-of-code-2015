
use super::*;
use super::SpellEffect;

pub struct MagicMissile;
pub struct Drain;
pub struct Shield;
pub struct Poison;
pub struct Recharge;

pub fn available_spells() -> Vec<Box<Spell>> {
    let mut spells: Vec<Box<Spell>> = Vec::new();

    spells.push(Box::new(MagicMissile));
    spells.push(Box::new(Drain));
    spells.push(Box::new(Shield));
    spells.push(Box::new(Poison));
    spells.push(Box::new(Recharge));

    spells
}

impl Spell for MagicMissile {
    fn cost(&self) -> u16 { 53 }
    fn cast(&self, contestants: &mut Contestants) -> Option<Effect> {
        if !self.available(&contestants) {
            return None;
        }

        decrease!(contestants.player.mana, self.cost());
        decrease!(contestants.opponent.hitpoints, 4);
        None
    }
}

impl Spell for Drain {
    fn cost(&self) -> u16 { 73 }
    fn cast(&self, contestants: &mut Contestants) -> Option<Effect> {
        if !self.available(&contestants) {
            return None;
        }

        decrease!(contestants.player.mana, self.cost());
        decrease!(contestants.opponent.hitpoints, 2);
        contestants.player.hitpoints += 2;
        None
    }
}

impl Spell for Shield {
    fn cost(&self) -> u16 { 113 }
    fn cast(&self, contestants: &mut Contestants) -> Option<Effect> {
        if !self.available(&contestants) {
            return None;
        }

        decrease!(contestants.player.mana, self.cost());
        let concrete = AlteratingEffect {
            apply: Shield::apply, undo: Shield::undo, active: false
        };

        let effect = Effect::new(self as &Spell, 6,
                                 ConcreteEffect::Alterating(concrete));
        Some(effect)
    }
}

impl SpellEffect for Shield {
    fn apply(c: &mut Contestants) {
        c.player.armor += 7;
    }

    fn undo(c: &mut Contestants) {
        c.player.armor -= 7;
    }
}

impl Spell for Poison {
    fn cost(&self) -> u16 { 173 }
    fn cast(&self, contestants: &mut Contestants) -> Option<Effect> {
        if !self.available(&contestants) {
            return None;
        }

        decrease!(contestants.player.mana, self.cost());
        let concrete = RecurringEffect { apply: Poison::apply };

        let effect = Effect::new(self as &Spell, 6,
                                 ConcreteEffect::Recurring(concrete));
        Some(effect)
    }
}

impl SpellEffect for Poison {
    fn apply(c: &mut Contestants) {
        decrease!(c.opponent.hitpoints, 3);
    }
}

impl Spell for Recharge {
    fn cost(&self) -> u16 { 229 }
    fn cast(&self, contestants: &mut Contestants) -> Option<Effect> {
        if !self.available(&contestants) {
            return None;
        }

        decrease!(contestants.player.mana, self.cost());
        let concrete = RecurringEffect { apply: Recharge::apply };
        let effect = Effect::new(self as &Spell, 5,
                                 ConcreteEffect::Recurring(concrete));
        Some(effect)
    }
}

impl SpellEffect for Recharge {
    fn apply(c: &mut Contestants) {
        c.player.mana += 101;
    }
}


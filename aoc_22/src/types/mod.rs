
#[macro_export]
macro_rules! decrease {
    ($i: expr, $e: expr) => {
        if $i < $e { $i = 0; } else { $i -= $e; }
    }
}

mod spells;
pub use self::spells::available_spells;

#[derive(Clone, Debug)]
pub struct Character {
    pub hitpoints:  u16,
    pub damage:     u16,
    pub armor:      u16,
    pub mana:       u16,
    pub mana_spent: u32
}

#[derive(Clone, Debug)]
pub struct Contestants {
    pub player:   Character,
    pub opponent: Character,
}

type Application = fn(&mut Contestants);

pub struct RecurringEffect {
    apply: Application
}

impl Clone for RecurringEffect {
    fn clone(&self) -> RecurringEffect {
        RecurringEffect { apply: self.apply }
    }
}

pub struct AlteratingEffect {
    active: bool,
    apply:  Application,
    undo:   Application
}

impl Clone for AlteratingEffect {
    fn clone(&self) -> AlteratingEffect {
        AlteratingEffect {
            active: self.active,
            apply:  self.apply,
            undo:   self.undo
        }
    }
}

#[derive(Clone)]
pub enum ConcreteEffect {
    Recurring(RecurringEffect),
    Alterating(AlteratingEffect)
}

#[derive(Clone)]
pub struct Effect<'a> {
    pub spell:  &'a (Spell + 'a),
    pub rounds: u16,
    pub effect: ConcreteEffect
}

impl<'a> Effect<'a> {
    fn new(spell: &'a Spell, rounds: u16, effect: ConcreteEffect)
        -> Effect<'a> {
        Effect { spell: spell, rounds: rounds, effect: effect }
    }

    pub fn render(&mut self, contestants: &mut Contestants) -> bool {
        use self::ConcreteEffect::*;
        if self.rounds == 0 {
            return false;
        }

        self.rounds -= 1;
        match self.effect {
            Recurring(ref effect) => {
                (effect.apply)(contestants);
            },

            Alterating(ref mut effect) => {
                if !effect.active {
                    effect.active = true;
                    (effect.apply)(contestants);
                }

                if self.rounds == 0 {
                    effect.active = false;
                    (effect.undo)(contestants);
                }
            },
        }

        self.rounds != 0
    }
}

pub trait Spell {
    fn cost(&self) -> u16;
    fn id(&self) -> u16 { self.cost() }

    fn available(&self, contestants: &Contestants) -> bool {
        contestants.player.mana >= self.cost()
    }

    fn cast(&self, &mut Contestants) -> Option<Effect>;
}

trait SpellEffect : Spell {
    fn apply(&mut Contestants) {  }
    fn undo(&mut Contestants)  {  }
}


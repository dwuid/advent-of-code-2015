
#[macro_use]
mod types;

use std::cmp::min;
use std::mem::replace;

use types::*;

#[derive(Clone)]
struct State<'a> {
    effects:     Vec<Effect<'a>>,
    player_turn: bool,
    contestants: Contestants
}

fn apply_effects(state: &mut State) {
    let effects = state.effects.clone()
                               .into_iter()
                               .filter_map(|mut effect| {
        if effect.render(&mut state.contestants) {
            Some(effect)
        } else {
            None
        }
    }).collect::<Vec<_>>();

    replace(&mut state.effects, effects);
}

fn handle_opponent_attack(contestant: &mut Contestants) {
    let p = &mut contestant.player;
    let o = &mut contestant.opponent;

    if o.damage <= p.armor {
        p.hitpoints -= 1;
    } else {
        decrease!(p.hitpoints, o.damage - p.armor);
    }
}

fn player_wins(contestants: &Contestants) -> Option<bool> {
    match (contestants.player.hitpoints, contestants.opponent.hitpoints) {
        (a, b) if b > 0 && a > 0  => None,
        (a, b) if a > 0 && b == 0 => Some(true),
        (a, b) if b > 0 && a == 0 => Some(false),

        _ => panic!("Both contestants are dead.")
    }
}

macro_rules! handle_win {
    ($state: expr, $minimal: ident) => {
        if let Some(wins) = player_wins(&$state.contestants) {
            if wins {
                let mana = $state.contestants.player.mana_spent;
                $minimal = min($minimal, mana);
            }

            continue;
        }
    }
}

fn solve(contestants: &Contestants, spells: &Vec<Box<Spell>>,
         hard_mode: bool) -> u32 {

    let state = State {
        effects:     Vec::new(),
        player_turn: true,
        contestants: contestants.clone()
    };

    let mut minimal_mana = u32::max_value();
    let mut work = Vec::new();

    work.push(state);
    while !work.is_empty() {
        let mut state = work.pop().unwrap();

        if hard_mode && state.player_turn {
            state.contestants.player.hitpoints -= 1;
            if state.contestants.player.hitpoints == 0 {
                continue;
            }
        }

        apply_effects(&mut state);
        handle_win!(state, minimal_mana);

        if !state.player_turn {
            handle_opponent_attack(&mut state.contestants);
            handle_win!(state, minimal_mana);

            state.player_turn = !state.player_turn;
            work.push(state);

        } else {
            for spell in spells {
                if state.effects.iter().any(|e| e.spell.id() == spell.id()) {
                    continue;
                }

                if !spell.available(&state.contestants) {
                    continue;
                }

                let mut new_state = state.clone();
                new_state.player_turn = !state.player_turn;

                new_state.contestants.player.mana_spent += spell.cost() as u32;
                if new_state.contestants.player.mana_spent > minimal_mana {
                    continue;
                }

                if let Some(effect) = spell.cast(&mut new_state.contestants) {
                    new_state.effects.push(effect);
                }

                handle_win!(new_state, minimal_mana);
                work.push(new_state);
            }
        }
    }

    minimal_mana
}

fn main() {
    let boss = Character {
        hitpoints:   55,
        damage:      8,
        armor:       0,
        mana:        0,
        mana_spent:  0
    };

    let player = Character {
        hitpoints:   50,
        damage:      0,
        armor:       0,
        mana:        500,
        mana_spent:  0
    };

    let contestants = Contestants { player: player, opponent: boss };
    let spells = available_spells();

    let solution_1 = solve(&contestants, &spells, false);
    println!("Uncle Scrooge wins using {} of mana.", solution_1);

    let solution_2 = solve(&contestants, &spells, true);
    println!("Uncle Scrooge wins hard mode using {} mana.", solution_2);
}


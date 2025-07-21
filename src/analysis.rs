use mmolb_parsing::enums::Slot;

use crate::{
    attributes::{DefenceAttribute, DividedAttribute},
    item::UnderstoodItem,
};

pub fn analyse(item: &UnderstoodItem, slot: Slot) -> f64 {
    item.effects
        .iter()
        .map(|(a, v)| slot_multiplier(slot, DividedAttribute::from(*a)) * v)
        .sum()
}

pub fn slot_multiplier(slot: Slot, attribute: DividedAttribute) -> f64 {
    match (slot, attribute) {
        //// Universal don't use
        (
            // Batters don't use pitching stats
            Slot::Catcher
            | Slot::FirstBaseman
            | Slot::SecondBaseman
            | Slot::ThirdBaseman
            | Slot::ShortStop
            | Slot::LeftField
            | Slot::CenterField
            | Slot::RightField
            | Slot::DesignatedHitter,
            DividedAttribute::Pitcher(_),
        ) => 0.0,
        (
            // Pitchers don't use batting or running stats
            Slot::StartingPitcher(_) | Slot::ReliefPitcher(_) | Slot::Closer,
            DividedAttribute::Batter(_) | DividedAttribute::Baserunning(_),
        ) => 0.0,
        (
            // Designated hitters don't defend
            Slot::DesignatedHitter,
            DividedAttribute::Defence(_),
        ) => 0.0,

        //// Types of balls to different positions
        (
            // Pitchers don't field fly balls
            Slot::StartingPitcher(_) | Slot::ReliefPitcher(_) | Slot::Closer,
            DividedAttribute::Defence(DefenceAttribute::Agility),
        ) => 0.0,
        (
            // Pitchers rarely field at all
            Slot::StartingPitcher(_) | Slot::ReliefPitcher(_) | Slot::Closer,
            DividedAttribute::Defence(
                DefenceAttribute::Reaction
                | DefenceAttribute::Acrobatics
                | DefenceAttribute::Patience,
            ),
        ) => 0.1,
        (
            // Fly balls and line drives rarely go to infielders
            Slot::Catcher
            | Slot::FirstBaseman
            | Slot::SecondBaseman
            | Slot::ThirdBaseman
            | Slot::ShortStop,
            DividedAttribute::Defence(DefenceAttribute::Agility | DefenceAttribute::Acrobatics),
        ) => 0.1,
        (
            // Pop ups don't go to outfielders
            Slot::LeftField | Slot::CenterField | Slot::RightField,
            DividedAttribute::Defence(DefenceAttribute::Patience),
        ) => 0.0,
        (
            // Ground balls rarely go to outfielders
            Slot::LeftField | Slot::CenterField | Slot::RightField,
            DividedAttribute::Defence(DefenceAttribute::Reaction),
        ) => 0.1,
        _ => 1.0,
    }
}

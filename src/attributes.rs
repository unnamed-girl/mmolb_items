use mmolb_parsing::enums::Attribute;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DividedAttribute {
    Generic(GenericAttribute),
    Batter(BatterAttribute),
    Pitcher(PitcherAttribute),
    Defence(DefenceAttribute),
    Baserunning(BaserunningAttribute),
}

impl From<Attribute> for DividedAttribute {
    fn from(value: Attribute) -> Self {
        match value {
            Attribute::Priority => DividedAttribute::Generic(GenericAttribute::Priority),
            Attribute::Luck => DividedAttribute::Generic(GenericAttribute::Luck),
            Attribute::Aiming => DividedAttribute::Batter(BatterAttribute::Aiming),
            Attribute::Contact => DividedAttribute::Batter(BatterAttribute::Contact),
            Attribute::Cunning => DividedAttribute::Batter(BatterAttribute::Cunning),
            Attribute::Discipline => DividedAttribute::Batter(BatterAttribute::Discipline),
            Attribute::Insight => DividedAttribute::Batter(BatterAttribute::Insight),
            Attribute::Intimidation => DividedAttribute::Batter(BatterAttribute::Intimidation),
            Attribute::Lift => DividedAttribute::Batter(BatterAttribute::Lift),
            Attribute::Vision => DividedAttribute::Batter(BatterAttribute::Vision),
            Attribute::Determination => DividedAttribute::Batter(BatterAttribute::Determination),
            Attribute::Wisdom => DividedAttribute::Batter(BatterAttribute::Wisdom),
            Attribute::Muscle => DividedAttribute::Batter(BatterAttribute::Muscle),
            Attribute::Selflessness => DividedAttribute::Batter(BatterAttribute::Selflessness),
            Attribute::Accuracy => DividedAttribute::Pitcher(PitcherAttribute::Accuracy),
            Attribute::Rotation => DividedAttribute::Pitcher(PitcherAttribute::Rotation),
            Attribute::Presence => DividedAttribute::Pitcher(PitcherAttribute::Presence),
            Attribute::Persuasion => DividedAttribute::Pitcher(PitcherAttribute::Persuasion),
            Attribute::Stamina => DividedAttribute::Pitcher(PitcherAttribute::Stamina),
            Attribute::Velocity => DividedAttribute::Pitcher(PitcherAttribute::Velocity),
            Attribute::Control => DividedAttribute::Pitcher(PitcherAttribute::Control),
            Attribute::Stuff => DividedAttribute::Pitcher(PitcherAttribute::Stuff),
            Attribute::Defiance => DividedAttribute::Pitcher(PitcherAttribute::Defiance),
            Attribute::Acrobatics => DividedAttribute::Defence(DefenceAttribute::Acrobatics),
            Attribute::Agility => DividedAttribute::Defence(DefenceAttribute::Agility),
            Attribute::Arm => DividedAttribute::Defence(DefenceAttribute::Arm),
            Attribute::Awareness => DividedAttribute::Defence(DefenceAttribute::Awareness),
            Attribute::Composure => DividedAttribute::Defence(DefenceAttribute::Composure),
            Attribute::Dexterity => DividedAttribute::Defence(DefenceAttribute::Dexterity),
            Attribute::Patience => DividedAttribute::Defence(DefenceAttribute::Patience),
            Attribute::Reaction => DividedAttribute::Defence(DefenceAttribute::Reaction),
            Attribute::Greed => DividedAttribute::Baserunning(BaserunningAttribute::Greed),
            Attribute::Performance => {
                DividedAttribute::Baserunning(BaserunningAttribute::Performance)
            }
            Attribute::Speed => DividedAttribute::Baserunning(BaserunningAttribute::Speed),
            Attribute::Stealth => DividedAttribute::Baserunning(BaserunningAttribute::Stealth),
            Attribute::Guts => DividedAttribute::Pitcher(PitcherAttribute::Guts),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GenericAttribute {
    Priority,
    Luck,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BatterAttribute {
    Aiming,
    Contact,
    Cunning,
    Discipline,
    Insight,
    Intimidation,
    Lift,
    Vision,
    Determination,
    Wisdom,
    Muscle,
    Selflessness,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PitcherAttribute {
    Accuracy,
    Rotation,
    Presence,
    Persuasion,
    Stamina,
    Velocity,
    Control,
    Stuff,
    Defiance,
    Guts,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DefenceAttribute {
    Acrobatics,
    Agility,
    Arm,
    Awareness,
    Composure,
    Dexterity,
    Patience,
    Reaction,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BaserunningAttribute {
    Greed,
    Performance,
    Speed,
    Stealth,
}

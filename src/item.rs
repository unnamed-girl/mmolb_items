use std::iter::once;

use mmolb_parsing::{
    NotRecognized,
    enums::{Attribute, EquipmentEffectType, ItemPrefix, ItemSuffix, ItemType},
    player::PlayerEquipment,
};

#[derive(Debug, Clone)]
pub struct UnderstoodItem {
    pub effects: Vec<(Attribute, f64)>,
    pub item: Option<ItemType>,
    pub name: String,
}

impl Default for UnderstoodItem {
    fn default() -> Self {
        Self {
            effects: Vec::new(),
            item: None,
            name: "[Empty Slot]".to_string(),
        }
    }
}

impl TryFrom<PlayerEquipment> for UnderstoodItem {
    type Error = &'static str;

    fn try_from(value: PlayerEquipment) -> Result<Self, Self::Error> {
        if let Ok(item) = value.name {
            let effects = value
                .effects
                .unwrap_or_default()
                .into_iter()
                .flat_map(|e| e)
                .filter(|e| matches!(e.effect_type, Ok(EquipmentEffectType::FlatBonus)))
                .flat_map(|e| Ok::<(_, _), NotRecognized>((e.attribute?, e.value)))
                .collect::<Vec<_>>();

            let name = match value.rare_name {
                Some(rare_name) => format!("{rare_name} {item}"),
                None => value
                    .prefixes
                    .iter()
                    .map(|p| p.as_ref().map(ItemPrefix::to_string).unwrap())
                    .chain(once(item.to_string()))
                    .chain(
                        value
                            .suffixes
                            .iter()
                            .map(|s| s.as_ref().map(ItemSuffix::to_string).unwrap()),
                    )
                    .collect::<Vec<_>>()
                    .join(" "),
            };

            return Ok(UnderstoodItem {
                effects,
                item: Some(item),
                name,
            });
        }

        Err("Didn't recognize name")
    }
}

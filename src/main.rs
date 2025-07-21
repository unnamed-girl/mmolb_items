use std::{collections::{HashMap, HashSet}, error::Error};

use clap::Parser;
use mmolb_parsing::{
    enums::{ItemType, Slot},
    player::{Player, PlayerEquipment},
    team::Team,
};
use reqwest::blocking::Client;

use crate::{analysis::analyse, item::UnderstoodItem};

#[derive(Parser, Debug)]
struct Args {
    /// The id of the team to analyse
    team_id: String,
}

mod analysis;
mod attributes;
mod fetching;
mod item;

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let client = Client::default();

    let team = fetching::mmolb_fetch::<Team>(&client, &args.team_id)?;

    let inventory = team
        .inventory
        .into_iter()
        .flat_map(|a| UnderstoodItem::try_from(a))
        .collect::<Vec<_>>();

    let mut current: HashMap<(ItemType, Slot), (String, UnderstoodItem, f64)> = HashMap::new();

    let mut slots = HashSet::new();

    for player in team.players {
        let slot = player.slot?;
        let player = fetching::mmolb_fetch::<Player>(&client, &player.player_id)?;

        let items: Vec<PlayerEquipment> = player.equipment?.into();
        for item in items.into_iter().flat_map(UnderstoodItem::try_from) {
            let rating = analyse(&item, slot);
            assert!(current.insert((item.item, slot), (player.first_name.clone(), item, rating)).is_none());
        }

        slots.insert(slot);
    }

    let mut transitions = Vec::new();

    for inventory_item in inventory {
        for slot in &slots {
            let (player_name, player_item, player_item_rating) =
                current.get(&(inventory_item.item, *slot)).unwrap();

            let inventory_rating = analyse(&inventory_item, *slot);
            if *player_item_rating < inventory_rating {
                transitions.push((
                    format!(
                        "Inventory {} <-> {} {}",
                        inventory_item.item, player_name, player_item.name
                    ),
                    inventory_rating - player_item_rating,
                ));
            }
        }
    }

    for ((_, player_slot), (player, player_item, player_item_rating)) in &current {
        for other_slot in &slots {
            let (other_player, other_player_item, other_player_rating) =
                current.get(&(player_item.item, *other_slot)).unwrap();

            let diff = (analyse(player_item, *other_slot)
                + analyse(other_player_item, *player_slot))
                - (player_item_rating + other_player_rating);
            if diff > 0.0 {
                transitions.push((
                    format!(
                        "{} {} <-> {} {}",
                        player, player_item.name, other_player, other_player_item.name
                    ),
                    diff,
                ));
            }
        }
    }

    transitions.sort_by(|(_, t), (_, t2)| t.total_cmp(t2));

    for (name, diff) in transitions {
        if diff > 0.0 {
            println!("{name}: +{diff}")
        }
    }

    Ok(())
}

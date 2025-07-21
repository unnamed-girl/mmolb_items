use std::{
    collections::{HashMap, HashSet},
    error::Error,
};

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

    let default_understood_item = UnderstoodItem::default();

    let client = Client::default();

    let team = fetching::mmolb_fetch::<Team>(&client, &args.team_id)?;

    let inventory = team
        .inventory
        .into_iter()
        .flat_map(|a| UnderstoodItem::try_from(a))
        .collect::<Vec<_>>();

    let mut current: HashMap<Slot, (String, HashMap<ItemType, (UnderstoodItem, f64)>)> =
        HashMap::new();

    let mut slots = HashSet::new();

    for player in team.players {
        let slot = player.slot?;
        let player = fetching::mmolb_fetch::<Player>(&client, &player.player_id)?;

        let items: Vec<PlayerEquipment> = player.equipment?.into();
        let mut rated_items = HashMap::new();
        for item in items {
            let name = item.name.clone()?;
            if let Ok(understood) = UnderstoodItem::try_from(item) {
                let rating = analyse(&understood, slot);
                rated_items.insert(name, (understood, rating));
            }
        }

        current.insert(slot, (player.first_name, rated_items));

        slots.insert(slot);
    }

    let mut transitions = Vec::new();

    for inventory_item in inventory {
        for slot in &slots {
            let (player_name, items) = current.get(slot).expect("all slots should be filled");

            let (player_item, player_item_rating) = items
                .get(&inventory_item.item.expect("Inventory items to have a type"))
                .map(|(a, b)| (a, b))
                .unwrap_or((&default_understood_item, &0.0));

            let inventory_rating = analyse(&inventory_item, *slot);
            if *player_item_rating < inventory_rating {
                transitions.push((
                    format!(
                        "Inventory {} <-> {} {}",
                        inventory_item.name,
                        player_name,
                        player_item.name
                    ),
                    inventory_rating - player_item_rating,
                ));
            }
        }
    }

    for (player_slot, (player, player_items)) in &current {
        for (player_item_type, (player_item, player_item_rating)) in player_items {
            for other_slot in &slots {
                let (other_player, other_player_items) = current
                    .get(other_slot)
                    .expect("Iterating over slots that exist");

                let (other_player_item, other_player_rating) = other_player_items
                    .get(player_item_type)
                    .map(|(a, b)| (a, b))
                    .unwrap_or((&default_understood_item, &0.0));

                let diff = (analyse(player_item, *other_slot)
                    + analyse(other_player_item, *player_slot))
                    - (player_item_rating + other_player_rating);
                if diff > 0.0 {
                    transitions.push((
                        format!(
                            "{} {} <-> {} {}",
                            player,
                            player_item.name,
                            other_player,
                            other_player_item.name
                        ),
                        diff,
                    ));
                }
            }
        }
    }

    transitions.sort_by(|(_, t), (_, t2)| t.total_cmp(t2));

    for (name, diff) in transitions {
        if diff > 0.0 {
            println!("{name}: +{:.2}", diff * 100.0)
        }
    }

    Ok(())
}

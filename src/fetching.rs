use std::error::Error;

use reqwest::blocking::Client;
use serde::de::DeserializeOwned;

use mmolb_parsing::player::Player;
use mmolb_parsing::team::Team;

pub trait Fetchable {
    const URL: &str;
}

impl Fetchable for Player {
    const URL: &str = "https://mmolb.com/api/player";
}

impl Fetchable for Team {
    const URL: &str = "https://mmolb.com/api/team";
}

pub fn mmolb_fetch<'a, T: Fetchable + DeserializeOwned>(
    client: &'a Client,
    id: &str,
) -> Result<T, Box<dyn Error>> {
    let url = format!("{}/{id}", T::URL);

    Ok(client.get(url).send()?.json::<T>()?)
}

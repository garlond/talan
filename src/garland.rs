use failure::{Error};
use reqwest;
use serde_json;
use url::form_urlencoded;
use std::fmt;

impl fmt::Display for JsonItem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "item {{\n");
        write!(f, "\tname: {}\n", self.item.name);
        write!(f, "\tid:   {}\n", self.item.id);
        write!(f, "\tingredients; {{\n");
        for (i, elem) in self.ingredients.iter().enumerate() {
            write!(f, "\t\t {}x {} (id: {})\n", self.item.craft[0].ingredients[i].amount, elem.name, elem.id);
        }
        write!(f, "\t}}\n");
        write!(f, "}}\n")
    }
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug)]
struct JsonItemSearchResult {
    id: String,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug)]
struct JsonItem {
    item: JsonItemData,
    ingredients: Vec<JsonItemIngredient>,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug)]
struct JsonItemData {
    name: String,
    id: u64,
    craft: Vec<JsonCraft>,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug)]
struct JsonItemIngredient {
    id: u64,
    name: String,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug)]
struct JsonCraft {
    job: u64,
    quality: u64,
    progress: u64,
    ingredients: Vec<JsonCraftIngredient>,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug)]
struct JsonCraftIngredient {
    id: u64,
    amount: u64,
    quality: Option<u64>,
}


// URL for individual item info is http://www.garlandtools.org/db/doc/item/en/3/23821.json
fn query_item_id(item_name: &str) -> Result<Option<u64>, Error> {
    let garland_search_url = String::from("https://www.garlandtools.org/api/search.php?");
    let encoded_url: String = form_urlencoded::Serializer::new(garland_search_url)
        .append_pair("craftable", "1")
        .append_pair("type", "item")
        .append_pair("text", item_name)
        .append_pair("lang", "en")
        .finish();
    let body = reqwest::get(&encoded_url)?.text()?;
    let items: Vec<JsonItemSearchResult> = serde_json::from_str(&body)?;
    // We should not get duplicates, but use just the first if we do
    let id: u64 = items[0].id.parse()?;
    Ok(Some(id))
}

#[test]
fn query_rakshasa_dogi_of_casting() {
    const RAKSHASA_DOGI_OF_CASTING_ID: u64 = 23821;
    let id = query_item_id(&"Rakshasa Dogi of Casting").unwrap().unwrap();
    assert_eq!(id, RAKSHASA_DOGI_OF_CASTING_ID);
}

#[test] 
fn query_crimson_cider_recipe() {
    const CRIMSON_CIDER_ID: u64 = 22436;
    let data = fetch_item_info(CRIMSON_CIDER_ID).unwrap();
    println!("{}", data);
}

fn fetch_item_info(id: u64) -> Result<JsonItem, Error> {
    let garland_item_url = String::from("http://www.garlandtools.org/db/doc/item/en/3/");
    let body = reqwest::get(&format!("{}{}.json", garland_item_url, id))?.text()?;
    let item: JsonItem = serde_json::from_str(&body)?;

    Ok(item)
}

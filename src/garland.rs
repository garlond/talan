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

#[derive(Debug)]
pub struct Item {
    name: String,
    materials: Vec<Material>,
}

#[derive(Debug)]
pub struct Material {
    name: String,
    count: u64,
}

impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}\n", self.name);
        write!(f, "[\n");
        for m in &self.materials {
            write!(f, "  {}x {}\n", m.count, m.name);
        }
        write!(f, "]\n")
    }
}

// Convert Garland's json layout to a structure easier to use
// for Talan's purposes.
impl From<JsonItem> for Item {
    fn from(json_item: JsonItem) -> Self {
        let mut v = Vec::new();
        for i in 0..json_item.ingredients.len() {
            v.push(Material {
                name: json_item.ingredients[i].name.clone(),
                count: json_item.item.craft[0].ingredients[i].amount,
            })
        }
        Item { name: json_item.item.name,  materials: v }
    }
}

// Return the item id for the provided item name
pub fn query_item_id(item_name: &str) -> Result<Option<u64>, Error> {
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

// Get the materials and other information for a given item
pub fn fetch_item_info(name: &str) -> Result<Item, Error> {
    let id = query_item_id(&name)?.unwrap();
    let garland_item_url = String::from("http://www.garlandtools.org/db/doc/item/en/3/");
    let body = reqwest::get(&format!("{}{}.json", garland_item_url, id))?.text()?;
    let item: JsonItem = serde_json::from_str(&body)?;

    Ok(Item::from(item))
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
    let item = fetch_item_info("Crimson Cider").unwrap();
    assert_eq!(item.name, "Crimson Cider");
    assert_eq!(item.materials[0].name, "Crimson Pepper");
    assert_eq!(item.materials[0].count, 1);
    assert_eq!(item.materials[1].name, "Jhammel Ginger");
    assert_eq!(item.materials[1].count, 1);
    assert_eq!(item.materials[2].name, "Cumin Seeds");
    assert_eq!(item.materials[2].count, 1);
    assert_eq!(item.materials[3].name, "Kudzu Root");
    assert_eq!(item.materials[3].count, 1);
    assert_eq!(item.materials[4].name, "Loquat");
    assert_eq!(item.materials[4].count, 3);
}

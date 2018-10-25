use failure::Error;
use pretty_env_logger;

use reqwest;
use serde_json::Value;
use url::form_urlencoded;
#[macro_use(serde_derive)]
#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug)]
struct ItemSearchResult {
    id: String,
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
    let items: Vec<ItemSearchResult> = serde_json::from_str(&body)?;
    if items.len() > 1 {
        println!("? more than 1?");
    }

    let id: u64 = items[0].id.parse()?;
    Ok(Some(id))
}

#[test]
fn query_rakshasa_dogi_of_casting() {
    const RAKSHASA_DOGI_OF_CASTING_ID: u64 = 23821;
    let id = query_item_id(&"Rakshasa Dogi of Casting").unwrap().unwrap();
    assert_eq!(id, RAKSHASA_DOGI_OF_CASTING_ID);
}

fn fetch_item_info(id: u64) -> Result<(), Error> {
    let garland_item_url = String::from("http://www.garlandtools.org/db/doc/item/en/3/");
    println!("id: {}", id);
    let body2 = reqwest::get(&format!("{}{}.json", garland_item_url, id))?.text()?;
    println!("body 2: {}", body2);

    Ok(())
}

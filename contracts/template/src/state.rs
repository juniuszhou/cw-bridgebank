use cw_storage_plus::{Item, Map};

pub const ITEM_NAME: Item<String> = Item::new("item_name");
pub const MAP_NAME: Map<&str, bool> = Map::new("map_name");

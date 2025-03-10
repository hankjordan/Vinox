use std::net::{IpAddr, Ipv4Addr};

use bevy::prelude::*;
use bevy_quinnet::server::*;
use vinox_common::{
    storage::{
        blocks::load::load_all_blocks, crafting::load::load_all_recipes,
        items::load::item_from_block,
    },
    world::chunks::storage::{BlockTable, ItemTable, RecipeTable},
};

pub fn setup_loadables(
    mut block_table: ResMut<BlockTable>,
    mut item_table: ResMut<ItemTable>,
    mut recipe_table: ResMut<RecipeTable>,
) {
    for block in load_all_blocks() {
        let mut name = block.clone().namespace;
        name.push(':');
        name.push_str(&block.name);
        if let Some(has_item) = block.has_item {
            if has_item {
                item_table.insert(name.clone(), item_from_block(block.clone()));
            }
        }
        block_table.insert(name, block);
    }
    for recipe in load_all_recipes() {
        let mut name = recipe.clone().namespace;
        name.push(':');
        name.push_str(&recipe.name);
        recipe_table.insert(name, recipe);
    }
}

pub fn new_server(mut server: ResMut<Server>) {
    server
        .start_endpoint(
            ServerConfiguration::from_ip(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 25565),
            certificate::CertificateRetrievalMode::GenerateSelfSigned {
                server_hostname: "vinox".to_string(), //TODO: Change to computer hostname
            },
        )
        .unwrap();
    server
        .endpoint_mut()
        .set_default_channel(bevy_quinnet::shared::channel::ChannelId::UnorderedReliable);
}

#![deny(clippy::clone_on_ref_ptr)]

use std::{
    error::Error,
    io::Write,
    ops::{Div, Mul},
};
use structopt::StructOpt;

use veloren_common::{
    assets::{self, AssetExt},
    comp::{
        self,
        item::{
            armor::{ArmorKind, Protection},
            tool::{Hands, MaterialStatManifest, Tool, ToolKind},
            Item, ItemKind,
        },
    },
    generation::EntityConfig,
    lottery::{LootSpec, Lottery},
};

#[derive(StructOpt)]
struct Cli {
    /// Available arguments: "armor-stats", "weapon-stats", "all-items",
    /// "loot-table", "entity-drops"
    function: String,
}

fn armor_stats() -> Result<(), Box<dyn Error>> {
    let mut wtr = csv::Writer::from_path("armorstats.csv")?;
    wtr.write_record(&[
        "Path",
        "Kind",
        "Name",
        "Quality",
        "Protection",
        "Poise Resilience",
        "Max Energy",
        "Energy Reward",
        "Crit Power",
        "Stealth",
        "Description",
    ])?;

    for item in comp::item::Item::new_from_asset_glob("common.items.armor.*")
        .expect("Failed to iterate over item folders!")
    {
        match item.kind() {
            comp::item::ItemKind::Armor(armor) => {
                let kind = get_armor_kind(&armor.kind);
                if kind == "Bag" {
                    continue;
                }

                let protection = match armor.protection() {
                    Some(Protection::Invincible) => "Invincible".to_string(),
                    Some(Protection::Normal(value)) => value.to_string(),
                    None => "0.0".to_string(),
                };
                let poise_resilience = match armor.poise_resilience() {
                    Some(Protection::Invincible) => "Invincible".to_string(),
                    Some(Protection::Normal(value)) => value.to_string(),
                    None => "0.0".to_string(),
                };
                let max_energy = armor.energy_max().unwrap_or(0.0).to_string();
                let energy_reward = armor.energy_reward().unwrap_or(0.0).to_string();
                let crit_power = armor.crit_power().unwrap_or(0.0).to_string();
                let stealth = armor.stealth().unwrap_or(0.0).to_string();

                wtr.write_record(&[
                    item.item_definition_id(),
                    &kind,
                    item.name(),
                    &format!("{:?}", item.quality()),
                    &protection,
                    &poise_resilience,
                    &max_energy,
                    &energy_reward,
                    &crit_power,
                    &stealth,
                    item.description(),
                ])?;
            },
            _ => println!("Skipping non-armor item: {:?}\n", item),
        }
    }

    wtr.flush()?;
    Ok(())
}

fn weapon_stats() -> Result<(), Box<dyn Error>> {
    let mut wtr = csv::Writer::from_path("weaponstats.csv")?;
    wtr.write_record(&[
        "Path",
        "Kind",
        "Name",
        "Hands",
        "Quality",
        "Power",
        "Effect Power",
        "Speed",
        "Crit Chance",
        "Range",
        "Energy Efficiency",
        "Buff Strength",
        "Equip Time (s)",
        "Description",
    ])?;

    let msm = MaterialStatManifest::default();

    // Does all items even outside weapon folder since we check if itemkind was a
    // tool anyways
    let items: Vec<comp::Item> = comp::Item::new_from_asset_glob("common.items.*")
        .expect("Failed to iterate over item folders!");

    for item in items.iter() {
        if let comp::item::ItemKind::Tool(tool) = item.kind() {
            let power = tool.base_power(&msm, &[]).to_string();
            let effect_power = tool.base_effect_power(&msm, &[]).to_string();
            let speed = tool.base_speed(&msm, &[]).to_string();
            let crit_chance = tool.base_crit_chance(&msm, &[]).to_string();
            let range = tool.base_range(&msm, &[]).to_string();
            let energy_efficiency = tool.base_energy_efficiency(&msm, &[]).to_string();
            let buff_strength = tool.base_buff_strength(&msm, &[]).to_string();
            let equip_time = tool.equip_time(&msm, &[]).as_secs_f32().to_string();
            let kind = get_tool_kind(&tool.kind);
            let hands = get_tool_hands(tool);

            wtr.write_record(&[
                item.item_definition_id(),
                &kind,
                item.name(),
                &hands,
                &format!("{:?}", item.quality()),
                &power,
                &effect_power,
                &speed,
                &crit_chance,
                &range,
                &energy_efficiency,
                &buff_strength,
                &equip_time,
                item.description(),
            ])?;
        }
    }

    wtr.flush()?;
    Ok(())
}

fn get_tool_kind(kind: &ToolKind) -> String {
    match kind {
        ToolKind::Sword => "Sword".to_string(),
        ToolKind::Axe => "Axe".to_string(),
        ToolKind::Hammer => "Hammer".to_string(),
        ToolKind::Bow => "Bow".to_string(),
        ToolKind::Dagger => "Dagger".to_string(),
        ToolKind::Staff => "Staff".to_string(),
        ToolKind::Sceptre => "Sceptre".to_string(),
        ToolKind::Shield => "Shield".to_string(),
        ToolKind::Spear => "Spear".to_string(),
        ToolKind::Blowgun => "Blowgun".to_string(),
        ToolKind::Debug => "Debug".to_string(),
        ToolKind::Farming => "Farming".to_string(),
        ToolKind::Pick => "Pick".to_string(),
        ToolKind::Natural => "Natural".to_string(),
        ToolKind::Empty => "Empty".to_string(),
    }
}

fn get_tool_hands(tool: &Tool) -> String {
    match tool.hands {
        Hands::One => "One".to_string(),
        Hands::Two => "Two".to_string(),
    }
}

fn get_armor_kind(kind: &ArmorKind) -> String {
    match kind {
        ArmorKind::Shoulder(_) => "Shoulder".to_string(),
        ArmorKind::Chest(_) => "Chest".to_string(),
        ArmorKind::Belt(_) => "Belt".to_string(),
        ArmorKind::Hand(_) => "Hand".to_string(),
        ArmorKind::Pants(_) => "Pants".to_string(),
        ArmorKind::Foot(_) => "Foot".to_string(),
        ArmorKind::Back(_) => "Back".to_string(),
        ArmorKind::Ring(_) => "Ring".to_string(),
        ArmorKind::Neck(_) => "Neck".to_string(),
        ArmorKind::Head(_) => "Head".to_string(),
        ArmorKind::Tabard(_) => "Tabard".to_string(),
        ArmorKind::Bag(_) => "Bag".to_string(),
    }
}

fn get_armor_kind_kind(kind: &ArmorKind) -> String {
    match kind {
        ArmorKind::Shoulder(x) => x.clone(),
        ArmorKind::Chest(x) => x.clone(),
        ArmorKind::Belt(x) => x.clone(),
        ArmorKind::Hand(x) => x.clone(),
        ArmorKind::Pants(x) => x.clone(),
        ArmorKind::Foot(x) => x.clone(),
        ArmorKind::Back(x) => x.clone(),
        ArmorKind::Ring(x) => x.clone(),
        ArmorKind::Neck(x) => x.clone(),
        ArmorKind::Head(x) => x.clone(),
        ArmorKind::Tabard(x) => x.clone(),
        ArmorKind::Bag(x) => x.clone(),
    }
}

fn all_items() -> Result<(), Box<dyn Error>> {
    let mut wtr = csv::Writer::from_path("items.csv")?;
    wtr.write_record(&["Path", "Name", "Kind"])?;

    for item in comp::item::Item::new_from_asset_glob("common.items.*")
        .expect("Failed to iterate over item folders!")
    {
        let kind = match item.kind() {
            ItemKind::Armor(armor) => get_armor_kind_kind(&armor.kind),
            ItemKind::Lantern(lantern) => lantern.kind.clone(),
            _ => "".to_owned(),
        };

        wtr.write_record(&[item.item_definition_id(), item.name(), &kind])?;
    }

    wtr.flush()?;
    Ok(())
}

fn loot_table(loot_table: &str) -> Result<(), Box<dyn Error>> {
    let mut wtr = csv::Writer::from_path("loot_table.csv")?;
    wtr.write_record(&[
        "Relative Chance",
        "Kind",
        "Item",
        "Lower Amount",
        "Upper Amount",
    ])?;

    let loot_table = "common.loot_tables.".to_owned() + loot_table;

    let loot_table = Lottery::<LootSpec<String>>::load_expect(&loot_table).read();

    for (i, (chance, item)) in loot_table.iter().enumerate() {
        let chance = if let Some((next_chance, _)) = loot_table.iter().nth(i + 1) {
            next_chance - chance
        } else {
            loot_table.total() - chance
        }
        .mul(10_f32.powi(5))
        .round()
        .div(10_f32.powi(5))
        .to_string();

        match item {
            LootSpec::Item(item) => wtr.write_record(&[&chance, "Item", item, "", ""])?,
            LootSpec::ItemQuantity(item, lower, upper) => wtr.write_record(&[
                &chance,
                "Item",
                item,
                &lower.to_string(),
                &upper.to_string(),
            ])?,
            LootSpec::LootTable(table) => {
                wtr.write_record(&[&chance, "LootTable", table, "", ""])?
            },
            LootSpec::Nothing => wtr.write_record(&[&chance, "Nothing", "", ""])?,
        }
    }

    wtr.flush()?;
    Ok(())
}

fn entity_drops(entity_config: &str) -> Result<(), Box<dyn Error>> {
    let mut wtr = csv::Writer::from_path("drop_table.csv")?;
    wtr.write_record(&["Entity Path", "Percent Chance", "Item Path", "Quantity"])?;

    fn write_entity_loot<W: std::io::Write>(
        wtr: &mut csv::Writer<W>,
        asset_path: &str,
    ) -> Result<(), Box<dyn Error>> {
        let entity_config = EntityConfig::load_expect(asset_path).read();

        // Create initial entry in drop table
        let entry: (f32, LootSpec<String>) = (1.0, entity_config.loot.clone());

        let mut table = vec![entry];

        // Keep converting loot table lootspecs into non-loot table lootspecs until no
        // more loot tables
        while table
            .iter()
            .any(|(_, loot_spec)| matches!(loot_spec, LootSpec::LootTable(_)))
        {
            // Partition table of loot specs into a table of items and nothings, and another
            // table of loot tables
            let (sub_tables, main_table): (Vec<_>, Vec<_>) = table
                .into_iter()
                .partition(|(_, loot_spec)| matches!(loot_spec, LootSpec::LootTable(_)));
            table = main_table;

            // Change table of loot tables to only contain the string that loads the loot
            // table
            let sub_tables = sub_tables.iter().filter_map(|(chance, loot_spec)| {
                if let LootSpec::LootTable(loot_table) = loot_spec {
                    Some((chance, loot_table))
                } else {
                    None
                }
            });
            for (chance, loot_table) in sub_tables {
                let loot_table = Lottery::<LootSpec<String>>::load_expect(loot_table).read();
                // Converts from lottery's weight addition for each consecutive entry to keep
                // the weights as they are in the ron file
                let loot_table: Vec<_> = loot_table
                    .iter()
                    .enumerate()
                    .map(|(i, (chance, item))| {
                        let chance = if let Some((next_chance, _)) = loot_table.iter().nth(i + 1) {
                            next_chance - chance
                        } else {
                            loot_table.total() - chance
                        };
                        (chance, item)
                    })
                    .collect();
                // Gets sum of all weights to use in normalization of entries
                let weights_sum: f32 = loot_table.iter().map(|(chance, _)| chance).sum();
                // Normalizes each entry in sub-loot table
                let loot_table = loot_table
                    .iter()
                    .map(|(chance, item)| (chance / weights_sum, item));
                for (sub_chance, &item) in loot_table {
                    // Multiplies normalized entry within each loot table by the chance for the loot
                    // table to drop in the above table
                    let entry = (chance * sub_chance, item.clone());
                    table.push(entry);
                }
            }
        }

        // Normalizes each item drop entry so that everything adds to 1
        let table_weight_sum: f32 = table.iter().map(|(chance, _)| chance).sum();
        let table = table
            .iter()
            .map(|(chance, item)| (chance / table_weight_sum, item));

        for (chance, item) in table {
            // Changes normalized weight to add to 100, and rounds at 2nd decimal
            let percent_chance = chance
                .mul(10_f32.powi(4))
                .round()
                .div(10_f32.powi(2))
                .to_string();

            let (item_asset, quantity) = match item {
                LootSpec::Item(item) => (Some(item), "1".to_string()),
                LootSpec::ItemQuantity(item, lower, upper) => {
                    // Tab needed so excel doesn't think it is a date...
                    (Some(item), format!("{}-{}\t", lower, upper))
                },
                LootSpec::LootTable(_) => panic!("Shouldn't exist"),
                LootSpec::Nothing => (None, "-".to_string()),
            };

            let item = item_asset.map(|asset| Item::new_from_asset_expect(asset));

            let item_name = if let Some(item) = &item {
                item.name()
            } else {
                "Nothing"
            };

            wtr.write_record(&[asset_path, &percent_chance, item_name, &quantity])?
        }

        Ok(())
    }

    if entity_config.eq_ignore_ascii_case("all") {
        let configs = assets::load_dir::<EntityConfig>("common.entity", true)
            .expect("Entity files moved somewhere else maybe?")
            .ids();
        for config in configs {
            write_entity_loot(&mut wtr, config)?;
        }
    } else {
        let entity_config = "common.entity.".to_owned() + entity_config;
        write_entity_loot(&mut wtr, &entity_config)?;
    }

    wtr.flush()?;
    Ok(())
}

fn main() {
    let args = Cli::from_args();
    if args.function.eq_ignore_ascii_case("armor-stats") {
        if let Err(e) = armor_stats() {
            println!("Error: {}\n", e)
        }
    } else if args.function.eq_ignore_ascii_case("weapon-stats") {
        if let Err(e) = weapon_stats() {
            println!("Error: {}\n", e)
        }
    } else if args.function.eq_ignore_ascii_case("all-items") {
        if let Err(e) = all_items() {
            println!("Error: {}\n", e)
        }
    } else if args.function.eq_ignore_ascii_case("loot-table") {
        let loot_table_name = get_input(
            "Specify the name of the loot table to export to csv. Assumes loot table is in \
             directory: assets.common.loot_tables.\n",
        );
        if let Err(e) = loot_table(&loot_table_name) {
            println!("Error: {}\n", e)
        }
    } else if args.function.eq_ignore_ascii_case("entity-drops") {
        let entity_config = get_input(
            "Specify the name of the entity to export loot drops to csv. Assumes entity config is \
             in directory: assets.common.entity.\nCan also use \"all\" to export loot from all \
             entity configs.\n",
        );
        if let Err(e) = entity_drops(&entity_config) {
            println!("Error: {}\n", e)
        }
    } else {
        println!(
            "Invalid argument, available \
             arguments:\n\"armor-stats\"\n\"weapon-stats\"\n\"all-items\"\n\"loot-table [table]\""
        )
    }
}

pub fn get_input(prompt: &str) -> String {
    // Function to get input from the user with a prompt
    let mut input = String::new();
    print!("{}", prompt);
    std::io::stdout().flush().unwrap();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Error reading input");

    String::from(input.trim())
}

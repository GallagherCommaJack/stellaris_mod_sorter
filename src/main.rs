use human_panic::*;
use serde::*;
use std::{cmp::Ordering, collections::HashMap, fs, path::PathBuf};
use structopt::StructOpt;

pub type ModsRegistry<'a> = HashMap<&'a str, Mod<'a>>;

#[derive(Debug, Serialize, Deserialize)]
pub struct Mod<'a> {
    #[serde(rename = "steamId")]
    #[serde(borrow)]
    steam_id: &'a str,
    #[serde(rename = "displayName")]
    #[serde(borrow)]
    display_name: &'a str,
    #[serde(borrow)]
    tags: Option<Vec<&'a str>>,
    #[serde(rename = "timeUpdated")]
    time_updated: i64,
    source: Source,
    #[serde(rename = "thumbnailUrl")]
    #[serde(borrow)]
    thumbnail_url: Option<&'a str>,
    #[serde(rename = "dirPath")]
    #[serde(borrow)]
    dir_path: &'a str,
    status: Status,
    #[serde(borrow)]
    id: &'a str,
    #[serde(rename = "gameRegistryId")]
    #[serde(borrow)]
    game_registry_id: Option<&'a str>,
    #[serde(rename = "thumbnailPath")]
    #[serde(borrow)]
    thumbnail_path: &'a str,
    #[serde(rename = "requiredVersion")]
    #[serde(borrow)]
    required_version: Option<&'a str>,
    #[serde(rename = "archivePath")]
    #[serde(borrow)]
    archive_path: Option<&'a str>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Source {
    #[serde(rename = "local")]
    Local,
    #[serde(rename = "steam")]
    Steam,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "invalid_mod")]
    InvalidMod,
    #[serde(rename = "ready_to_play")]
    ReadyToPlay,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GameData<'a> {
    #[serde(rename = "modsOrder")]
    #[serde(borrow)]
    mods_order: Vec<&'a str>,
    #[serde(rename = "isEulaAccepted")]
    is_eula_accepted: bool,
}

fn modname_cmp(name1: &str, name2: &str) -> Ordering {
    let n1_s: String = name1.chars().take_while(|c| !c.is_alphanumeric()).collect();
    let n2_s: String = name2.chars().take_while(|c| !c.is_alphanumeric()).collect();

    n1_s.cmp(&n2_s).then(name1.cmp(name2))
}

fn mods_to_game_data_sorted<'a>(mut mods: Vec<Mod<'a>>) -> GameData<'a> {
    mods.sort_unstable_by(|m1, m2| modname_cmp(m1.display_name, m2.display_name));
    GameData {
        mods_order: mods.into_iter().map(|m| m.id).collect(),
        is_eula_accepted: true,
    }
}

#[derive(StructOpt, Debug)]
struct Opt {
    /// Stellaris install directory, default '.'
    #[structopt(short = "d", long, default_value = ".", parse(from_os_str))]
    install_dir: PathBuf,
}

fn main() {
    // setup_panic!();

    let opt = Opt::from_args();

    let mut mr_path = opt.install_dir.clone();
    mr_path.push("mods_registry.json");

    let data = fs::read_to_string(mr_path).expect("failed to read mod registry");
    let mods =
        serde_json::from_str::<ModsRegistry>(&data).expect("failed to parse mod registry file");

    let mut gd_path = opt.install_dir;
    gd_path.push("game_data.json");

    let gd = mods_to_game_data_sorted(mods.into_iter().map(|(_, m)| m).collect());
    fs::write(
        &gd_path,
        serde_json::to_vec(&gd).expect("failed to serialize game data"),
    )
    .expect("failed to write game data to file");

    println!("done!");
}

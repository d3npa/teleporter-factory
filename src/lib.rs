use mustache::MapBuilder;
use serde::{Deserialize, Serialize};
use std::ops::{Deref, DerefMut};

const PF_RULES_TPL: &str = include_str!("../templates/pf_rules.txt");
const HUB_ENTRY_TPL: &str = include_str!("../templates/hub_entry.txt");
const HOSTNAME_IF_TPL: &str = include_str!("../templates/hostname_if.txt");
const HUB_CONFIG_TPL: &str = include_str!("../templates/hub_config.txt");
const COMBINED_PF_RULES: &str = include_str!("../templates/combined_pf_rules.txt");

#[derive(Serialize, Deserialize)]
pub struct ExitInfo {
    pub display_name: String,
    pub pf_id: String,
    pub iface: String,
    /// wireguard local ip
    pub iface_ip: String,
    pub rdomain: i32,
    /// wireguard private key
    pub wg_key: String,
    /// wireguard peer public key
    pub wg_peer_pub: String,
    pub wg_peer_psk: Option<String>,
    /// wireguard peer endpoint ip
    pub wg_peer_endpoint_ip: String,
    /// wireguard peer endpoint port
    pub wg_peer_endpoint_port: u16,
    #[serde(default = "allow_all_ips")]
    pub wg_peer_allowed_ips: String,
    /// remote gateway ip over wireguard
    pub gateway_ip: String,
    pub table_persist_path: String,
}

impl ExitInfo {
    pub fn gen_hostname_if(&self) -> String {
        let tpl = mustache::compile_str(HOSTNAME_IF_TPL).unwrap();
        tpl.render_to_string(&self).unwrap()
    }

    pub fn gen_pf_rules(&self) -> String {
        let tpl = mustache::compile_str(PF_RULES_TPL).unwrap();
        tpl.render_to_string(&self).unwrap()
    }

    pub fn gen_hub_entry(&self) -> String {
        let tpl = mustache::compile_str(HUB_ENTRY_TPL).unwrap();
        tpl.render_to_string(&self).unwrap().to_owned()
    }
}

fn allow_all_ips() -> String {
    "0.0.0.0/0".into()
}

#[derive(Serialize)]
pub struct Exits(Vec<ExitInfo>);

impl Exits {
    pub fn new() -> Self {
        Self(vec![])
    }

    pub fn gen_pf_rules(&self) -> String {
        let tpl = mustache::compile_str(COMBINED_PF_RULES).unwrap();
        let data = MapBuilder::new()
            .insert_str(
                "rules",
                self.0
                    .iter()
                    .map(|e| format!("{}\n", e.gen_pf_rules()))
                    .collect::<String>()
                    .trim()
                    .to_owned(),
            )
            .build();

        tpl.render_data_to_string(&data).unwrap()
    }

    pub fn gen_hub_config(&self) -> String {
        let tpl = mustache::compile_str(HUB_CONFIG_TPL).unwrap();
        let data = MapBuilder::new()
            .insert_str(
                "exits",
                self.0
                    .iter()
                    .map(|e| format!("\t{}", e.gen_hub_entry()))
                    .collect::<String>()
                    .trim()
                    .to_owned(),
            )
            .build();

        tpl.render_data_to_string(&data).unwrap()
    }
}

impl Deref for Exits {
    type Target = Vec<ExitInfo>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Exits {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

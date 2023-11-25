use std::fs;
use std::result::Result;
use teleporter_factory::{ExitInfo, Exits};

const EXITS_PATH: &str = "./exits";

fn main() {
    let entries = fs::read_dir(EXITS_PATH).unwrap();

    let mut exits = Exits::new();

    for entry in entries
        .filter_map(Result::ok)
        .filter(|ent| ent.path().extension().is_some_and(|ext| ext == "toml"))
    {
        let contents = fs::read_to_string(entry.path())
            .expect(&format!("could not read '{:?}'", entry.path()));
        let exit: ExitInfo = toml::from_str(&contents)
            .expect(&format!("could not parse toml in '{:?}'", entry.path()));

        exits.push(exit);
    }

    exits.sort_by(|a, b| a.iface.cmp(&b.iface));

    for exit in &*exits {
        println!("\x1b[95m[*] '/etc/hostname.{}'\x1b[0m\n", exit.iface);
        println!("{}", exit.gen_hostname_if());
    }

    println!("\x1b[95m[*] /etc/pf/teleport_hub.conf\x1b[0m\n");
    println!("{}", exits.gen_pf_rules());

    println!("\x1b[95m[*] /etc/teleport-rs/exits.toml\x1b[0m\n");
    println!("{}", exits.gen_hub_config());
}

use std::io::Write;
use std::path::Path;
use std::result::Result;
use std::{env, fs};
use teleporter_factory::{ExitInfo, Exits};

const EXITS_PATH: &str = "./exits";

fn generate_stdout(exits: Exits) {
    for exit in &*exits {
        println!("\x1b[95m[*] '/etc/hostname.{}'\x1b[0m\n", exit.iface);
        println!("{}", exit.gen_hostname_if());
    }

    println!("\x1b[95m[*] /etc/pf/teleport_hub.conf\x1b[0m\n");
    println!("{}", exits.gen_pf_rules());

    println!("\x1b[95m[*] /etc/teleport-rs/exits.toml\x1b[0m\n");
    println!("{}", exits.gen_hub_config());
}

fn generate_tar(exits: Exits) {
    let wrk_dir = "./tmp/teleporter_factory_wrkdir";

    if fs::metadata(wrk_dir).is_ok() {
        // if it exists
        println!("[*] cleaning '{wrk_dir}'");
        fs::remove_dir_all(wrk_dir).unwrap();
    }

    // creating files
    {
        /// short-hand to write-to-file lol
        fn wtf(path: impl AsRef<Path>, contents: impl AsRef<[u8]>) {
            let path = path.as_ref();
            println!("[*] creating '{}'", path.display());
            if let Ok(mut file) =
                fs::OpenOptions::new().create(true).write(true).open(&path)
            {
                file.write_all(contents.as_ref())
                    .expect(&format!("writing to {}", path.display()));
            }
        }

        for sub_dir in vec!["", "etc", "etc/pf", "etc/teleport-rs"] {
            let dir = format!("{wrk_dir}/{sub_dir}");
            fs::create_dir_all(&dir)
                .expect(&format!("creating directory {dir}"));
        }

        for exit in &*exits {
            let path = format!("{wrk_dir}/etc/hostname.{}", exit.iface);
            wtf(path, exit.gen_hostname_if());
        }

        let pf_rules = format!("{wrk_dir}/etc/pf/teleport_hub.conf");
        wtf(pf_rules, exits.gen_pf_rules());

        let exits_toml = format!("{wrk_dir}/etc/teleport-rs/exits.toml");
        wtf(exits_toml, exits.gen_hub_config());
    }

    // creating archive
    {
        let archive_name = "teleporters.tar";
        let archive_path = format!("{wrk_dir}/{archive_name}");
        let archive =
            fs::File::create(&archive_path).expect("creating {archive_path}");

        let mut tar_builder = tar::Builder::new(archive);
        tar_builder
            .append_dir_all("etc", format!("{wrk_dir}/etc"))
            .unwrap();
        tar_builder
            .finish()
            .expect(&format!("creating tar archive at {archive_path}"));

        fs::copy(archive_path, format!("./{archive_name}"))
            .expect("copying {archive_path} to cwd");

        println!("[*] saved to './{archive_name}'");
    }

    fs::remove_dir_all("./tmp").expect("removing './tmp'");
}

fn main() {
    /* default behavior is to output configs to stdout to be copy-pasted.
     * the --tar flag instead requests an extractable tar archive be prepared
     */
    let mut tar_flag = false;
    let args: Vec<String> = env::args().collect();
    if let Some(arg1) = args.get(1) {
        if arg1 == "--tar" {
            tar_flag = true;
        }
    }

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

    if tar_flag {
        generate_tar(exits)
    } else {
        generate_stdout(exits)
    }
}

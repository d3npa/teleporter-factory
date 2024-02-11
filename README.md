# teleporter factory

adding a vpn exit on openbsd involves creating an interface via hostname.if(5), some pf rules, and ideally a way to control the exit that clients will use.

teleporter-factory takes a single configuration file for each exit and uses templating to generate the corresponding hostname.if(5), pf.conf(5), and teleport-hub configuration files.

the default behavior is to print the generated configuration to stdout, but a conveniently extractable tar archive can be requested by passing `--tar` at the command-line.

```
$ cargo run -- --tar
    Finished dev [unoptimized + debuginfo] target(s) in 0.28s
     Running `target/debug/teleporter-factory --tar`
[*] creating './tmp/teleporter_factory_wrkdir/etc/hostname.wg51'
[*] creating './tmp/teleporter_factory_wrkdir/etc/hostname.wg52'
[*] creating './tmp/teleporter_factory_wrkdir/etc/hostname.wg53'
[*] creating './tmp/teleporter_factory_wrkdir/etc/pf/teleport_hub.conf'
[*] creating './tmp/teleporter_factory_wrkdir/etc/teleport-hub/exits.toml'
[*] saved to './teleporters.tar'
```

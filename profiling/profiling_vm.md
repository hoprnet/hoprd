# Setting Up the jemalloc Profiling VM

jemalloc heap profiling is Linux-only. To profile `hoprd` on a macOS host, you need a Linux VM (or any remote Linux machine) reachable via SSH. The VM handles building and running the profiling binary; your macOS machine drives everything via the `jeprof-vm.sh` helper script.

This is a **one-time setup**. Once complete, follow [jeprof-vm-usage.md](./jeprof-vm-usage.md) for day-to-day profiling.

## Requirements

Any Linux environment with SSH access works. You need:

- A Linux host (VM or remote machine) reachable by SSH
- [Nix](https://nixos.org/download/) installed with flakes enabled (for `nix build`)
- `jemalloc`, `graphviz`, `perl`, `git` available (installed below)

The examples in this guide use **OrbStack + NixOS** — a convenient local VM option for macOS. If you use a different hypervisor (Lima, UTM, Multipass, QEMU) or a remote Linux machine, substitute your own SSH target and host-reachability address where noted.

---

## 1. Create the VM

### OrbStack (example)

Install [OrbStack](https://orbstack.dev/), then create a NixOS machine:

```bash
orb create nixos nixos-test
```

OrbStack automatically manages SSH keys and wires them into `~/.ssh/config`. The VM is accessible at `nixos-test@orb`, and SSH keys are stored at `~/.orbstack/ssh/id_ed25519`.

### Other hypervisors

Create a NixOS (or any Linux) VM using your preferred tool, note the SSH target, and proceed with the same steps below. For non-NixOS distros, install Nix via the [Determinate Systems installer](https://github.com/DeterminateSystems/nix-installer) and replace `nix-env -iA nixos.*` commands with the appropriate package manager.

---

## 2. Verify SSH Access

```bash
ssh nixos-test@orb          # OrbStack example
ssh user@<your-vm-ip>       # any other setup
```

---

## 3. Configure NixOS

Log into the VM and perform these one-time setup steps.

### Enable Nix Flakes

```bash
mkdir -p ~/.config/nix
echo "experimental-features = nix-command flakes" >> ~/.config/nix/nix.conf
```

### Add Trusted Users

Nix builds via flakes require the current user to be trusted. Edit `/etc/nixos/configuration.nix` with `sudo`:

```nix
nix.settings.trusted-users = [ "root" "@wheel" ];
```

Apply the change:

```bash
sudo nixos-rebuild switch
```

### Install Required Tools

```bash
nix-env -iA \
  nixos.git \
  nixos.rsync \
  nixos.jemalloc \
  nixos.graphviz \
  nixos.perl \
  nixos.binutils \
  nixos.openssl \
  nixos.python3
```

This provides `jeprof`, `dot`, `objdump`, `addr2line`, `nm`, `libssl.so.3`, and `python3` — everything needed to build and analyze heap dumps. `rsync` is required by the `sync` subcommand.

---

## 4. Networking

### Host reachability from the VM

The `localcluster` subcommand needs to reach `anvil_blokli` running on the macOS host. How you do this depends on your hypervisor:

| Setup            | `CHAIN_URL` to use                        |
| ---------------- | ----------------------------------------- |
| OrbStack         | `http://host.orb.internal:8080` (default) |
| Lima             | `http://192.168.5.2:8080`                 |
| UTM / QEMU (NAT) | `http://192.168.64.1:8080`                |
| Remote Linux box | `http://<your-mac-ip>:8080`               |

Override via the `CHAIN_URL` env variable:

```bash
CHAIN_URL=http://192.168.64.1:8080 ./scripts/jeprof-vm.sh localcluster 3
```

### Port forwarding (OrbStack)

OrbStack automatically forwards ports from the VM to the macOS loopback interface. A `hoprd` node on port `3000` inside the VM is accessible at `http://localhost:3000` on macOS. Other hypervisors may require manual port-forward rules.

---

## 5. Next Steps

Set `VM_HOST` to your SSH target if it differs from the default (`nixos-test@orb`), then use the helper script from your macOS host:

```bash
VM_HOST=user@<your-vm-ip> ./scripts/jeprof-vm.sh all
```

See [jeprof-vm-usage.md](./jeprof-vm-usage.md) for the complete reference.

---

## Troubleshooting

### Identity password mismatch

```
An identity file is present at /tmp/hoprd/identity but the provided password is not sufficient to decrypt it
```

An existing identity on the VM was created with a different password. Wipe the transient profiling data and start fresh:

```bash
./scripts/jeprof-vm.sh clean
```

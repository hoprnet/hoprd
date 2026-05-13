# Setting Up the jemalloc Profiling VM (OrbStack + NixOS)

jemalloc heap profiling is Linux-only. To profile `hoprd` on a macOS host, this workflow uses a NixOS virtual machine running under OrbStack. The VM handles building and running the profiling binary; your macOS machine drives everything via SSH and the `jeprof-vm.sh` helper script.

This is a **one-time setup**. Once complete, follow [JEPROF-VM-USAGE.md](./JEPROF-VM-USAGE.md) for day-to-day profiling.

---

## 1. Create the VM

Install [OrbStack](https://orbstack.dev/) if you haven't already, then create a NixOS machine:

```bash
orb create nixos nixos-test
```

## 2. Verify SSH Access

OrbStack automatically manages SSH keys and wires them into `~/.ssh/config`. The VM is accessible at `nixos-test@orb`. Verify the connection:

```bash
ssh nixos-test@orb
```

SSH keys are stored at `~/.orbstack/ssh/id_ed25519`.

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
  nixos.jemalloc \
  nixos.graphviz \
  nixos.perl \
  nixos.binutils \
  nixos.openssl \
  nixos.python3
```

This provides `jeprof`, `dot`, `objdump`, `addr2line`, `nm`, `libssl.so.3`, and `python3` — everything needed to build and analyze heap dumps.

---

## 4. Networking

### DNS

OrbStack provides automatic local DNS. From the macOS host, the VM is reachable at `nixos-test.orb` or `nixos-test.local`.

### Port Forwarding

OrbStack automatically forwards ports from the VM to the macOS loopback interface. A `hoprd` node running on port `3000` inside the VM is accessible at `http://localhost:3000` on macOS. For `localcluster` runs, OrbStack handles the incrementing ports (3000, 3001, …) automatically.

---

## 5. Next Steps

Once the VM is set up, use the helper script from your macOS host to drive the full profiling workflow:

```bash
# Sync repo to VM, build profiling binary, and run a single node
./scripts/jeprof-vm.sh all
```

See [JEPROF-VM-USAGE.md](./JEPROF-VM-USAGE.md) for the complete reference.

---

## Troubleshooting

### Identity password mismatch

```
An identity file is present at /tmp/hoprd/identity but the provided password is not sufficient to decrypt it
```

This happens when an existing identity on the VM was created with a different password. Wipe the transient profiling data and start fresh:

```bash
./scripts/jeprof-vm.sh clean
```

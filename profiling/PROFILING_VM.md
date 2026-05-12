# Setting up the jemalloc Profiling VM (OrbStack + NixOS)

Since `jemalloc` profiling is Linux-only, we use a NixOS virtual machine running on OrbStack to build and run `hoprd` with heap profiling enabled on macOS hosts.

## 1. Create the VM

If you haven't already, install [OrbStack](https://orbstack.dev/). Then, create a new NixOS machine:

```bash
orb create nixos nixos-test
```

## 2. Configure SSH

OrbStack automatically manages SSH keys. The VM is accessible at `nixos-test@orb`. Your SSH keys are typically located at `~/.orbstack/ssh/id_ed25519`.

Verify the connection:

```bash
ssh nixos-test@orb
```

## 3. Initial NixOS Setup

Once logged into the VM, perform these one-time setup steps:

### Enable Nix Flakes

```bash
mkdir -p ~/.config/nix
echo "experimental-features = nix-command flakes" >> ~/.config/nix/nix.conf
```

### Configure Trusted Users

To allow building from the repo using Nix flakes, you need to be a trusted user. Edit `/etc/nixos/configuration.nix` (using `sudo`):

```nix
# Add this to your configuration.nix
nix.settings.trusted-users = [ "root" "@wheel" ];
```

Then apply the changes:

```bash
sudo nixos-rebuild switch
```

### Install Required Tools

Install the base dependencies needed for building and analyzing heap dumps:

```bash
nix-env -iA nixos.git \
        nixos.jemalloc \
        nixos.graphviz \
        nixos.perl \
        nixos.binutils \
        nixos.openssl \
        nixos.python3
```

## 4. Networking and Port Forwarding

### Domain Name

OrbStack provides automatic local DNS. The VM is reachable from your macOS host at:

- `nixos-test.orb`
- `nixos-test.local`

### Port Mapping

OrbStack automatically forwards ports from the VM to the host's loopback interface. If you run a `hoprd` node on port `3000` inside the VM, it will be accessible at `http://localhost:3000` on your macOS host.

> **Note:** For `localcluster` runs where multiple nodes use incrementing ports (3000, 3001, etc.), OrbStack handles this mapping automatically.

## 5. Usage

Once the VM is set up, use the helper script from your macOS host to drive the profiling workflow:

```bash
# Sync repo to VM, build profiling binary, and run
./scripts/jeprof-vm.sh all
```

For detailed usage of the profiling script, see [JEPROF-VM-USAGE.md](./JEPROF-VM-USAGE.md).

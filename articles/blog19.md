# Nix

Years ago I had been introduced to the concept of [infrastructure as code](https://en.wikipedia.org/wiki/Infrastructure_as_code)(IaC) to provision (cloud) resources in a declarative way (as code). Before IaC, developers were forced to master the art of [ClickOps](https://dev.to/terraformmonkey/what-is-clickops-27f9) and provision resources manually through a web interface. The latter is intuitive to those who have not been introduced to IaC, at least, this is how I experienced it. There is something that is very satisfying about maintaining your infrastructure with code that lives next to the application code that is running on it. At that point you can't really go back to the (AWS|GCP|Azure) developer console or shell scripts with complex CLI commands.

With this in mind, I had a look at my [dotfiles](https://github.com/danielsteman/.dotfiles) repository, where I maintain a bunch of config files that I use on my development machine, and noticed all the complex shell scripts and manual dependency management. Surely there must be a way to treat my machine the same way I treat my cloud infrastructure stack. I remember a conversation I had with a colleague who mentioned a fully declarative Linux distro called [NixOS](https://nixos.org/). Instead of `apt update && apt install ...` (or whatever package manager you'd use) with every fresh install, you'd describe your system as code, using the [Nix programming language](https://nix.dev/tutorials/nix-language.html). The code files can be kept in version control and outlive the OS installation. You can wipe your system, boot NixOS, clone your Nix configuration and rebuild your system. This enables full reproducability of the your OS. Pretty awesome right? 

I had a good run with NixOS on my machine at home but professionally I always work on macOS, mainly because the rest of the company does. Luckily, there is [nix-darwin](https://github.com/nix-darwin/nix-darwin) which is similar to NixOS but for macOS. It's fairly easy to install because of the installer built by [determinate](https://github.com/DeterminateSystems/nix-installer?tab=readme-ov-file#determinate-nix-installer), which is an interesting project on its own 🦀. But let's start at the basics: NixOS.

## configuration.nix

So how does it work? This is a minimal example of `configuration.nix` which declares how your NixOS should be build. 

```nix
{ config, pkgs, ... }:

{
  # Bootloader setup
  boot.loader.systemd-boot.enable = true;
  boot.loader.efi.canTouchEfiVariables = true;

  # Networking
  networking.hostName = "my-nixos";   # Set hostname
  networking.networkmanager.enable = true;

  # Set your time zone
  time.timeZone = "Europe/Amsterdam";

  # Select internationalisation properties.
  i18n.defaultLocale = "en_US.UTF-8";

  # Enable the X11 windowing system + GNOME desktop
  services.xserver.enable = true;
  services.xserver.displayManager.gdm.enable = true;
  services.xserver.desktopManager.gnome.enable = true;

  # Users
  users.users.daniel = {
    isNormalUser = true;
    extraGroups = [ "wheel" "networkmanager" ];
    shell = pkgs.zsh;
  };

  # Allow unfree packages (like Chrome, Spotify)
  nixpkgs.config.allowUnfree = true;

  # Packages installed system-wide
  environment.systemPackages = with pkgs; [
    vim
    git
    firefox
    htop
  ];

  # Enable the firewall
  networking.firewall.enable = true;

  # Enable OpenSSH daemon
  services.openssh.enable = true;

  # System state version (don’t change unless upgrading)
  system.stateVersion = "24.05"; 
}
```

This is an example configuration file for NixOS, the Linux distro based on Nix package manager. You can see that the file takes some inputs, `pkgs` and `config`. We'll go over them one by one. `pkgs` is set by the [module system](https://nix.dev/tutorials/module-system/index.html) and usually points to [nixpkgs](https://search.nixos.org/packages), the biggest package collection. As you can see, `pkgs` includes software like `vim` and `git`. The other input, `config`, contains the current system configuration which can be read inside the function to potential apply changes. Even though `config` is not directly used inside the function, it's idiomatic to declare it as input because other modules might rely on it. NixOS also passes some other arguments that are not explicitly named, these are captured by the `...` spread operator. This example module declares options and their values, which is often called an _attribute set_. When you run `sudo nixos-rebuild switch`, the module system gathers all modules and merges them in a clever way, where arrays are concatenated and attribute sets are merged recursively. You can leverage this mechanism by splitting up your configuration in more modules, which can be useful when it grows over time. 


## Flakes

An experimental, but widely used feature of Nix is [flakes](https://nixos.wiki/wiki/Flakes). What are flakes and how does it differ from the `configuration.nix` we discussed earlier? I like this quote from the [zero-to-nix](https://zero-to-nix.com/concepts/flakes/) tutorial:

"It may be helpful to think of flakes as processors of Nix code. They take Nix expressions as input and output things that Nix can use, like package definitions, development environments, or NixOS configurations." Whereas our `configuration.nix` only outputs OS configuration, flakes can output other things, such as shells, applications, full nix-darwin macOS configuration (more on that later) and more. 

## Development environments

One of such _other things_ is a [development environment](https://zero-to-nix.com/start/nix-develop/). This is kind of like a Python `virtualenv`, which you might know, but on steroids. Whereas Python's `virtualenv` _just_ takes care of Python packages, Nix development environments take care of OS level dependencies such as compilers and libraries. Such development environments are often declared in a flake, using builtin [devShells] output. 

```nix
{
  description = "Rust development environment";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  outputs = { self, nixpkgs }:
    let
      system = "aarch64-darwin"; # or "x86_64-linux" on Linux
      pkgs = import nixpkgs { inherit system; };
    in {
      devShells.${system}.default = pkgs.mkShell {
        buildInputs = [
          pkgs.rustc
          pkgs.cargo
        ];

        shellHook = ''
          echo "Rust dev environment ready!"
          rustc --version
          cargo --version
        '';
      };
    };
}
```

This is stored in a `flake.nix` in the current working directory and can easily be entered: 

```bash
❯ nix develop
Rust dev environment ready!
rustc 1.89.0 (29483883e 2025-08-04) (built from a source tarball)
cargo 1.89.0 (c24e10642 2025-06-23)
(nix:nix-shell-env) bash-5.3$ cargo
Rust's package manager

Usage: cargo [OPTIONS] [COMMAND]
       cargo [OPTIONS] -Zscript <MANIFEST_RS> [ARGS]...
```

This development environment can be shared across more machines, which can be powerful for engineering teams working on the same project. At this point you might ask yourself: "isn't this why we have Docker?". This is a valid question, but dev shells with Nix flakes have some advantages. First, all packages that are installed come from [nixpkgs](https://search.nixos.org/packages) and are pinned to the commit, instead of external sources such as `apt` and `npm`. which are less deterministic. Also, with Docker you don't _have_ to pin the base image, making it less deterministic by design. Another advantage is that there is no overhead of a VM or container that is running on your machine. Instead, dependencies are kept locally in `/nix/store`, which you can actually see when you open the dev shell: 

```bash
(nix:nix-shell-env) bash-5.3$ which cargo
/nix/store/gzv6psv17kv7x9s01w8jhi0h2cg6z15p-cargo-1.89.0/bin/cargo
```

To be fair, where Docker does have an advantage is security. Since a container runs in its own environment, fully isolated from the host system, it can safely run untrusted code, whereas a Nix dev shell would run on the host machine, in which case it would not be safe. 

## nix-darwin

Now that we have a better understanding of NixOS and flakes, let's continue to most relevant bit (at least for my fellow MacOS devs): [nix-darwin](https://github.com/nix-darwin/nix-darwin). This combines Nix and flakes to make MacOS declarative. Since we are using flake, we haveto use `nixpkgs-unstable` as input, alongside `nix-darwin`: 

```nix
  description = "My configuration for macOS";
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    nix-darwin = {
      url = "github:nix-darwin/nix-darwin/master";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };
```

Instead of manually installing packages with something like `brew`, it's now possible to declare them in `flake.nix`: 

```nix
environment.systemPackages = with pkgs; [
    aerospace
    bun
    firefox
    kitty
    vim
    # Just to give you an example
]
```

And instead of configuring your MacOS instance through the settings interface, you can now declare settings, which is super awesome if you ever switch to another MacBook: 

```nix
# Unlock sudo commands with our fingerprint.
security.pam.services.sudo_local.touchIdAuth = true;

system.defaults = {
    trackpad.Clicking = true;
    dock.autohide = true;
    screencapture.target = "clipboard";

    finder = {
      AppleShowAllExtensions = true;
      ShowPathbar = true;
      FXEnableExtensionChangeWarning = false;
    };
};

system.keyboard.enableKeyMapping = true;

system.defaults.NSGlobalDomain.AppleInterfaceStyle = "Dark";
```

Another tool that allows you to declare dotfiles is [home-manager](https://github.com/nix-community/home-manager). Without home-manager, it is up to the developer to manage [symbolic links](https://en.wikipedia.org/wiki/Symbolic_link) to get configuration files in places where they're expected by tools (e.g. `~/.config/nvim`) and making sure that dependencies are installed (e.g. installing `neovim`). Both solutions are valid, but the former ensure reproducability of the entire environment. 

These are just some examples of what you can declare with flakes, but it only scratches the surface of what's possible. My own [nix-darwin-config](https://github.com/danielsteman/.dotfiles/tree/master/nix-darwin-config) is still in its early phase but if you're looking for inspiration, there are many muture configs you can find publicly. 

## Going fully Nix ✨

Up until now we mostly discussed how Nix can help a developer to setup their machine, but it's capable of more than just that. We already discussed development environments, but let's zoom in on that. If we were to build and deploy a Python application, what would the setup look like using Nix? 

Let's go over some of the requirements. My workflow to build a Python application, such as a FastAPI app, includes at least:

- Running the app locally
- Running tests
- Running linters
- Building a Docker image
- Installing pre-commit hooks
- Dependency management
- Database migration tools
- Secrets management
- CI/CD



# Nix

Years ago I had been introduced to the concept of [infrastructure as code](https://en.wikipedia.org/wiki/Infrastructure_as_code)(IaC) to provision (cloud) resources in a declarative way (as code). Before IaC, developers were forced to master the art of [ClickOps](https://dev.to/terraformmonkey/what-is-clickops-27f9) and provision resources manually through a web interface. The latter is intuitive to those who have not been introduced to IaC, at least, this is how I experienced it. There is something that is very satisfying about maintaining your infrastructure with code that lives next to the application code that is running on it. At that point you can't really go back to the (AWS|GCP|Azure) developer console or shell scripts with complex CLI commands.

With this in mind, I had a look at my [dotfiles](https://github.com/danielsteman/.dotfiles) repository, where I maintain a bunch of config files that I use on my development machine, and noticed all the complex shell scripts and manual dependency management. Surely there must be a way to treat my machine the same way I treat my cloud infrastructure stack. I remember a conversation I had with a colleague who mentioned a fully declarative Linux distro called [NixOS](https://nixos.org/). Instead of `apt update && apt install ...` (or whatever package manager you'd use) with every fresh install, you'd describe your system as code, using the [Nix programming language](https://nix.dev/tutorials/nix-language.html). The code files can be kept in version control and outlive the OS installation. You can wipe your system, boot NixOS, clone your Nix configuration and rebuild your system. This enables full reproducability of the your OS. Pretty awesome right? 

I had a good run with NixOS on my machine at home but professionally I always work on macOS, mainly because the rest of the company does. Luckily, there is [nix-darwin](https://github.com/nix-darwin/nix-darwin) which is similar to NixOS but for macOS. It's fairly easy to install because of the installer built by [determinate](https://github.com/DeterminateSystems/nix-installer?tab=readme-ov-file#determinate-nix-installer), which is an interesting project on its own 🦀. 

## configuration.nix

So how does it work? This is a minimal example of `configuration.nix` which declares how your OS should be build. 

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

This is an example configuration file for NixOS, the Linux distro based on Nix package manager. You can see that the file takes some inputs, `pkgs` and `config`. We'll go over them one by one. `pkgs` is set by the [module system](https://nix.dev/tutorials/module-system/index.html) and usually points to [nixpkgs](https://search.nixos.org/packages), the biggest package collection. As you can see, `pkgs` includes software like `vim` and `git`. The other input, `config`, contains the current system configuration which can be read inside the function to potential apply changes. Even though `config` is not directly used inside the function, it's idiomatic to declare it as input because other modules might rely on it. NixOS also passes some other arguments that are not explicitly named, these are captured by the `...` spread operator. This example module declares options and their values. When you run `sudo nixos-rebuild switch`, the module system gathers all modules and merges them. You can leverage this mechanism by splitting up your configuration in more modules, which can be useful when it grows over time. 


## Flakes

An experimental, but widely used feature of Nix is [flakes](https://nixos.wiki/wiki/Flakes). It's good to know that Nix is not only providing a declarative solution to configure your OS, it also provides [development environments](https://zero-to-nix.com/start/nix-develop/). This is kind of like a Python `virtualenv`, which you might know, on steroids. Such development environments are often declared as a flake, using a tool like [devshell](https://numtide.github.io/devshell/getting_started.html). But what are flakes and how does it differ from the `configuration.nix` we discussed earlier? I like this quote from the [zero-to-nix](https://zero-to-nix.com/concepts/flakes/) tutorial:

"It may be helpful to think of flakes as processors of Nix code. They take Nix expressions as input and output things that Nix can use, like package definitions, development environments, or NixOS configurations."



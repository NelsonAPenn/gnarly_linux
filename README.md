# Gnarly Linux

Note: not yet implemented!

The goal of this project is to create an automated Arch Linux installer that:

- Produces a functional, secure GNOME desktop environment
- (stretch goal) aids with selecting the right set of driver and firmware packages
- Is accessible to everyone

If you:

- are someone looking to try Arch Linux
- want a basic desktop experience with most of the details worked out

OR you

- Love the result you get from installing Arch Linux with GNOME
- Are reconsidering your life choices after installing Arch for the nth time
- Want *basic* functionality and *strong* security out of the box
- You used to like Ubuntu before snapcraft divided the package management ecosystem and ruined your life, or some other supervillian backstory.

You may like this project.

## Scope

What is this project's definition of a functional, secure GNOME desktop? What is basic software and what is bloat?

This project makes the following main assumptions:

- You want a basic graphical experience on a desktop or laptop
- You want to use Arch Linux
- You want to use GNOME
- You want strong security but don't necessarily know what that entails

Things that are generally considered in scope

- Basic security-related configuration including:
  - Full disk encryption
  - Secure DNS
  - EFISTUB, secure boot signing
  - firewall rules suitable for a laptop
- Configuration that your grandma might not understand but makes the system work seamlessly:
  - mDNS resolution
  - Setup required for printers to work
- Minimal QoL features
  - Plymouth, quiet + splash (don't want to confuse grandma)
  
Things that are generally considered out of scope:

- Software that you want to use but many others may not (e.x. LibreOffice, VSCode, OBS studio, blender)
- Your elite haxxor keyboard shortcuts

## Motivation

This project aims to be a second embassador for Arch Linux. This is inspired by
Omarchy, but aims to provide an experience that is approachable by all as
opposed to a distribution that only elite-haxxors enjoy.

Arch Linux is:

- highly customizable
- bloat free by definition (the only bloat is what you added yourself)
- a distribution with excellent, up-to-date, package management
- customized to every detail by necessity, but it doesn't have to be (hence this!)

I'd like to champion all of GNOME's work on an smooth and accessible user
experience combined with Arch's approach to package management.

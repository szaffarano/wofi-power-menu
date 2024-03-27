# Wofi power menu

![GitHub Release](https://img.shields.io/github/v/release/szaffarano/wofi-power-menu?sort=date)
![GitHub License](https://img.shields.io/github/license/szaffarano/wofi-power-menu)
![CI](https://github.com/szaffarano/wofi-power-menu/actions/workflows/ci.yml/badge.svg)
![Release](https://github.com/szaffarano/wofi-power-menu/actions/workflows/release.yml/badge.svg)
[![pre-commit](https://img.shields.io/badge/pre--commit-enabled-brightgreen?logo=pre-commit)](https://github.com/pre-commit/pre-commit)

Implements a power menu using the [wofi](https://sr.ht/~scoopta/wofi/) launcher.

## Usage

Just run the program to show the power menu:

![wofi-power-menu](./img/wpm.png)

## CLI configuration

```bash
$ wofi-power-menu --help

Shows a highly configurable power menu using wofi

Usage: wofi-power-menu [OPTIONS]

Options:
  -v, --verbose                Print additional information
  -w, --wofi-path <WOFI_PATH>  Path to the wofi binary
  -d, --disable <DISABLE>      Menu item to disable (accepts multiple values)
  -D, --dry-run                Simulate the command without executing it
  -c, --confirm <CONFIRM>      Menu item to force confirmation
  -l, --list-items             Show the menu items and exit
  -h, --help                   Print help
  -V, --version                Print version
```

example

```bash
# list items
./wofi-power-menu -l
No config file found, using default values
Available items:
----------------
shutdown: Shut down [disabled: false, confirmation: true]
reboot: Reboot [disabled: false, confirmation: true]
suspend: Suspend [disabled: false, confirmation: true]
hibernate: Hibernate [disabled: false, confirmation: false]
logout: Logout [disabled: false, confirmation: false]
lock-screen: Lock screen [disabled: false, confirmation: false]

# enables confirmation for lock screen and suspend
./wofi-power-menu --confirm lock-screen --confirm reboot

# "hides" hibernate
./wofi-power-menu --disable hibernate
```

## Configuration File

Optionally you can create `$XDG_CONFIG_HOME/wofi-power-menu.toml` to customize
the app:

```toml
[wofi]
  path = "/alternative/path/to/wofi"
  extra_args = "--allow-markup --columns=1 --hide-scroll"

[menu.shutdown]
  title = "Apagar"

[menu.reboot]
  title = "Reiniciar"

[menu.suspend]
  title = "Suspender"
  enabled = "false"

[menu.hibernate]
  title = "Hibernar"

[menu.logout]
  title = "Salir"

[menu.lock-screen]
    title = "Bloquear pantalla"
    requires_confirmation = "false"

[menu.ls]
  title = "Listar directorio"
  cmd = "ls -l --color"
  icon = "L"
```

You can configure a custom wofi location as well as change which wofi extra
flags to use.

Also, you can customize the menue either:

1. Overriding default values in any existing menu entry. The above example
   translates the titles to Spanish, disables the `requires_confirmation` flag
   for the `lock-screen` item, and hides (i.e. set `enabled=false` the
   `suspend` item.
1. Adding new entries, like `ls`. Notice that the only optional field are
   `requires_confirmation` (defaults to `false`) and `enabled` (defaults to
   `true`), you have to set `title`, `cmd` and `icon`.

## Related tools

Highly inspired by his cousin [rofi-power-menu](https://github.com/jluttine/rofi-power-menu).

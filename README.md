# Wofi power menu

## Configuration

Optionally you can create `$XDG_CONFIG_HOME/wofi-power-menu.toml` to customize the app:
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

[menu.hibernate]
  title = "Hibernar"

[menu.logout]
  title = "Salir"

[menu.lock-screen]
    title = "Bloquear pantalla"

[menu.ls]
  title = "Listar directorio"
  cmd = "ls -l --color"
  icon = "L"
```

You can configure a custom wofi location as well as change which wofi extra flags to use.

Also, you can customize the menue either:

1. Overriding default values in any existing menu entry. The above example translates the titles to Spanish, or
1. Adding new entries, like `ls`. Notice that the only optional field is `requires_confirmation`, you have to set `title`, `cmd` and `icon`.

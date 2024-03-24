use anyhow::anyhow;

use wofi_power_menu::{cmd::run, wofi::{Menu, Wofi}};

fn main() -> anyhow::Result<()> {
    let menu = Menu::default();
    let wofi = Wofi::default();

    let selection = wofi.spawn(&menu)?;

    if selection.is_empty() {
        return Ok(());
    }

    let item_selected = menu
        .nth(selection.parse::<usize>()?)
        .ok_or(anyhow!(format!("Invalid selection: {}", selection)))?;

    let cmd = if item_selected.requires_confirmation {
        let confirmation = Menu::new_confirmation(&item_selected);
        let response = wofi.spawn(&confirmation)?;
        if response.is_empty() {
            return Ok(());
        }

        let option = confirmation
            .nth(response.parse::<usize>()?)
            .ok_or(anyhow!(format!("Invalid response: {}", selection)))?;
        option.cmd.to_owned()
    } else {
        item_selected.cmd.to_owned()
    };

    run(cmd)?;

    Ok(())
}

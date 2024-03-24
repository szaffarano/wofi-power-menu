use anyhow::anyhow;

use wofi_power_menu::{run, Menu, Wofi};

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

        println!("Response: {}", response);
        let option = confirmation
            .nth(response.parse::<usize>()?)
            .ok_or(anyhow!(format!("Invalid response: {}", selection)))?;
        String::from(option.cmd.clone())
    } else {
        item_selected.cmd.clone()
    };

    println!("About to execute: '{}'", cmd);

    run(cmd)?;

    Ok(())
}

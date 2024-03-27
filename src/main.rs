use std::process::exit;

use anyhow::{anyhow, Result};
use clap::Parser;
use wofi_power_menu::{
    power_menu, utils,
    wofi::{self, Menu},
};

fn main() -> Result<()> {
    let args = power_menu::CliArgs::parse();

    let mut menu = power_menu::default_menu();
    let mut wofi = power_menu::default_wofi();

    power_menu::merge_config(
        &mut menu,
        &mut wofi,
        wofi::get_config(env!("CARGO_BIN_NAME"))?,
    )?;
    power_menu::merge_cli_args(&mut menu, &mut wofi, &args)?;

    if args.list_items {
        println!("Available items:");
        println!("----------------");
        for item in menu.iter() {
            println!("{}", item);
        }
        exit(0);
    }

    let selection = wofi.spawn(&menu)?;

    if selection.is_empty() {
        return Ok(());
    }

    let item_selected = menu
        .nth(selection.parse::<usize>()?)
        .ok_or(anyhow!(format!("Invalid selection: {}", selection)))?;

    let cmd = if item_selected.requires_confirmation() {
        let confirmation = Menu::new_confirmation(item_selected);
        let response = wofi.spawn(&confirmation)?;
        if response.is_empty() {
            return Ok(());
        }

        let option = confirmation
            .nth(response.parse::<usize>()?)
            .ok_or(anyhow!(format!("Invalid response: {}", selection)))?;
        option.cmd()
    } else {
        item_selected.cmd()
    };

    utils::run(cmd)?;

    Ok(())
}

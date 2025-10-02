use bevy::{prelude::*, ui};
use bevy_egui::{egui, EguiContexts, EguiPlugin, EguiPrimaryContextPass};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

pub struct UIPlugin;
impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(UIWalletState {
            maximum_pool_size: 100,
            current_gold: 100,
            current_swap_rate: 0,
            gold_pool: 0,
        })
        .add_systems(EguiPrimaryContextPass, (ui_wallet, ui_msgbox, ui_toystash));
    }
}
#[derive(Resource)]
pub struct UIWalletState {
    current_gold: u64,
    current_swap_rate: u64,
    maximum_pool_size: u64,
    gold_pool: u64,
}

pub fn ui_wallet(mut contexts: EguiContexts, mut wallet: ResMut<UIWalletState>) -> Result {
    egui::Window::new("Wallet").show(contexts.ctx_mut()?, |ui| {
        ui.label(format!("{} Gold", wallet.current_gold));
        ui.label(format!("{} GoldPool", wallet.gold_pool));

        ui.add(egui::Slider::new(&mut wallet.current_swap_rate, 0..=100).text("Swap Rate"));

        if ui.button("Withdraw").clicked() {
            wallet.gold_pool = wallet.gold_pool.saturating_sub(wallet.current_swap_rate);
            wallet.current_gold = wallet.current_gold.saturating_add(wallet.current_swap_rate);
        }

        if ui.button("Deposit").clicked() {
            wallet.current_gold = wallet.current_gold.saturating_sub(wallet.current_swap_rate);
            wallet.gold_pool = wallet.gold_pool.saturating_add(wallet.current_swap_rate);
        }

        ui.add(egui::Slider::new(&mut wallet.maximum_pool_size, 0..=100).text("GoldPool Cap"));
    });
    Ok(())
}

pub fn ui_msgbox(mut contexts: EguiContexts) -> Result {
    egui::Window::new("MsgBox").show(contexts.ctx_mut()?, |ui| {
        let mut text = "Hello World".to_string();
        ui.text_edit_singleline(&mut text);
    });
    Ok(())
}

pub fn ui_toystash(mut contexts: EguiContexts) -> Result {
    egui::Window::new("Toy Stash").show(contexts.ctx_mut()?, |ui| {
        let things = vec!["Palm Tree", "Shovel"];
        for x in things {
            ui.horizontal(|ui| {
                if ui.button("Drop").clicked() {
                    error!("Unsupported action.");
                }

                ui.label(x);
            });
        }
    });
    Ok(())
}

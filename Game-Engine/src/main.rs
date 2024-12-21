use std::ops::Index;

use eframe::{App, Frame, NativeOptions};
use egui::Context;
use eframe::egui;
mod cell;


fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([500.0, 500.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Cell Schermish",
        options,
        Box::new(|cc| {
            // This gives us image support:
            egui_extras::install_image_loaders(&cc.egui_ctx);

            Ok(Box::<Game>::default())
        }),
    )
}

enum GameState {
    Menu,
    Local_Game,
    Public_Game,
    Private_Game
}

struct Game {
    //All variables

    //Set before game
    current_state: GameState,
    player_1_name: String,
    player_2_name: String,
    player_3_name: String,
    player_4_name: String,
    player_count: u16,
    grid_size: u32,
    grid_size_string: String,
    
    //set during game
    cell_grid: Vec<Vec<cell::Cell>>,
}

impl Default for Game {
    fn default() -> Self {
        Self {
            current_state: GameState::Menu,
            player_1_name: "P1".to_owned(),
            player_2_name: "P2".to_owned(),
            player_3_name: "P3".to_owned(),
            player_4_name: "P4".to_owned(),
            player_count: 4,
            grid_size: 13,
            grid_size_string: "13".to_owned(),
            cell_grid: Vec::new(),
        }
    }
}

impl eframe::App for Game {
    //Big ass update function :p
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            match self.current_state {
                GameState::Menu => {
                    ui.heading("Cell Schermish");
                    ui.vertical(|ui|
                        {
                            ui.label("Player 1 Name");
                            ui.text_edit_singleline( &mut self.player_1_name);

                            ui.label("Player 2 Name");
                            ui.text_edit_singleline( &mut self.player_2_name);

                            ui.label("Player 3 Name");
                            ui.text_edit_singleline( &mut self.player_3_name);
                            
                            ui.label("Player 4 Name");
                            ui.text_edit_singleline( &mut self.player_4_name);
                        }
                    );

                    ui.heading("Game Settings");
                    ui.label(format!("Player Count: {}",self.player_count));
                    ui.horizontal(|ui| 
                        {
                            if ui.button("2").clicked() {self.player_count = 2}
                            if ui.button("3").clicked() {self.player_count = 3}
                            if ui.button("4").clicked() {self.player_count = 4}
                        }
                    );
                    ui.label(format!("Grid Size: {}",self.grid_size));
                    let grid_size_edited = ui.text_edit_singleline( &mut self.grid_size_string);
                    if grid_size_edited.lost_focus() {
                        let is_number:Result<u32, _> = self.grid_size_string.parse();
                        if is_number.is_ok() {
                            self.grid_size = is_number.unwrap()
                        }
                    }
                    //spacer
                    ui.add_space(50.0);

                    if ui.button("Start Game").clicked() {
                        self.current_state = GameState::Local_Game;
                    }
                    
                }
                GameState::Local_Game => {
                    
                    ui.horizontal(|ui|
                        {
                            ui.heading("Local Game");
                            if ui.button("Back to menu").clicked() {
                                self.current_state = GameState::Menu;
                            }
                        }
                    );

                    match self.player_count {
                        2 => {
                            ui.label(format!("{} vs {}", self.player_1_name, self.player_2_name));
                        },
                        3 => {
                            ui.label(format!("{} vs {} vs {}", self.player_1_name, self.player_2_name, self.player_3_name));
                        },
                        4 => {
                            ui.label(format!("{} vs {} vs {} vs {}", self.player_1_name, self.player_2_name, self.player_3_name, self.player_4_name));
                        },
                        _ => {
                            ui.label(format!("How did you manage this one? Player_count: {}",self.player_count));
                        }
                    }

                    //NxN grid,

                    ui.horizontal(|ui|
                        {
                            for x in &self.cell_grid {
                                ui.vertical(|ui| 
                                    {
                                        for y in x {
                                            ui.image(y.clone().get_cell_image());
                                        }
                                    }
                                );
                            }
                        }
                    );
                    
                }
                _ => {
                    ui.heading("?????????");
                }
            }
            

            
        });
    }
}
 
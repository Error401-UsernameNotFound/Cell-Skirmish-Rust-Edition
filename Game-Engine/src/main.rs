use arraystring::{ArrayString, typenum::U200};
use base_game_engine::cell::{Cell, CellType};
use base_game_engine::{GameEngine, GameState,};
use eframe::{egui, Frame};
use egui::{Sense, Context};


mod base_game_engine;

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
            Ok(Box::<GameUI>::default())
        }),
    )
}

#[derive(Clone)]
struct GameUI {
    game_manager: GameEngine,
    grid_size:i32,
    local_cell_grid:Vec<Vec<Cell>>
}
impl Default for GameUI {
    fn default() -> Self {
        Self {
            game_manager: GameEngine::default(),
            grid_size: 0,
            local_cell_grid: vec![],
        }
    }
}

impl eframe::App for GameUI {
    //Big ass update function :p
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            match self.game_manager.clone().get_current_state() {
                GameState::Menu => {
                    ui.heading("Cell Schermish");
                    ui.vertical(|ui|
                        {
                            ui.label("Player 1 Name");
                            ui.horizontal(|ui| {
                                ui.add_sized([50.0, 10.0], egui::TextEdit::singleline(self.game_manager.get_player_string(0)));
                            });

                            ui.label("Player 2 Name");
                            ui.horizontal(|ui| {
                                ui.add_sized([50.0, 10.0], egui::TextEdit::singleline(self.game_manager.get_player_string(1)));
                            });

                            ui.label("Player 3 Name");
                            ui.horizontal(|ui| {
                                ui.add_sized([50.0, 10.0], egui::TextEdit::singleline(self.game_manager.get_player_string(2)));
                            });
                            
                            ui.label("Player 4 Name");
                            ui.horizontal(|ui| {
                                ui.add_sized([50.0, 10.0], egui::TextEdit::singleline(self.game_manager.get_player_string(3)));
                            });
                        }
                    );

                    ui.heading("Game Settings");
                    ui.label(format!("Player Count: {}", self.game_manager.clone().get_player_count()));
                    ui.horizontal(|ui| 
                        {
                            if ui.button("2").clicked() {self.game_manager.set_player_count(2);}
                            if ui.button("3").clicked() {self.game_manager.set_player_count(3)}
                            if ui.button("4").clicked() {self.game_manager.set_player_count(4)}
                        }
                    );
                    ui.checkbox(self.game_manager.is_debug(), "Activate Debug Options");
                    
                    ui.label(format!("Grid Size: {}",self.game_manager.get_grid_size_string()));
                    let grid_size_edited = ui.text_edit_singleline( self.game_manager.get_grid_size_string());
                    if grid_size_edited.lost_focus() {
                        let is_number:Result<i32, _> = self.game_manager.get_grid_size_string().parse();
                        if is_number.is_ok() {
                            self.grid_size = is_number.unwrap();
                            self.game_manager.set_grid_size(self.grid_size);
                        }
                    }
                    
                    ui.label(format!("Cell Size: {}", self.game_manager.clone().get_cell_size()));
                    if ui.text_edit_singleline( self.game_manager.get_cell_size_string()).lost_focus() {
                        let is_number:Result<f32, _> = self.game_manager.get_cell_size_string().parse();
                        if is_number.is_ok() {
                            self.game_manager.set_cell_size(is_number.unwrap());
                        }
                    }
                    //spacer
                    ui.add_space(50.0);

                    if ui.button("Start Game").clicked() {
                        self.game_manager.start_game();
                        self.grid_size = self.game_manager.get_grid_size()
                    }
                    
                }
                GameState::LocalGame => {
                    // Headding info
                    ui.horizontal(|ui|
                        {
                            ui.heading("Local Game");
                            if ui.button("Back to menu").clicked() {
                                self.game_manager.set_current_state(GameState::Menu);
                            }
                        }
                    );

                    ui.label(self.game_manager.clone().battle_vs_title());
                    

                    
                    ui.label(format!("Grid Size: {} x {}", self.grid_size, self.grid_size));
                    ui.horizontal(|ui|
                        {
                            ui.heading(format!("Current turn: {}", self.game_manager.get_current_turn_string()));
                            if ui.button("Next turn").clicked() {
                                self.game_manager.start_next_turn();
                            }
                            if *self.game_manager.is_debug(){
                                ui.label(self.game_manager.clone().debug_get_last_action());
                            }
                        }
                    );

                    //NxN grid,
                    // split off grid from manager
                    self.local_cell_grid = self.game_manager.clone().get_cell_grid();
                    ui.horizontal(|ui|
                        {
                            for x in 0..self.grid_size { ui.vertical(|ui| 
                            {for y in 0..self.grid_size {
                                let current_cell = self.local_cell_grid[x as usize].get_mut(y as usize).unwrap();
                                
                                
                                let cell_image_location:ArrayString<U200> = current_cell.get_cell_image();
                                let cell_responce = ui.add(
                                    egui::Image::new(egui::Image::from_uri(cell_image_location.as_str()).source(ctx))
                                        .maintain_aspect_ratio(true)
                                        .fit_to_exact_size(egui::vec2(self.game_manager.clone().get_cell_size(), self.game_manager.clone().get_cell_size()))
                                        .rounding(5.0)
                                        ,
                                );
                                

                                if cell_responce.interact(Sense::click()).clicked() && self.game_manager.clone().check_placable(*current_cell){
                                    self.game_manager.place_cell(vec![x,y]);
                                }

                                cell_responce.context_menu(|ui|{
                                    ui.label(format!("Cell Type: {}", current_cell.get_cell_type()));
                                    ui.label(format!("Prev Cell Type: {}", current_cell.get_previous_cell_type()));
                                    ui.label(format!("Cell Player: {}", current_cell.get_cell_player()));
                                    ui.label(format!("Cell Player id: {}", current_cell.get_cell_player_number()));
                                    ui.label(format!("Cell hp: {}", current_cell.get_cell_hp()));
                                    ui.label(format!("Cell apt: {}", current_cell.get_cell_atp()));
                                    if *self.game_manager.is_debug() {
                                        if ui.button("Replace with empty").clicked() {current_cell.replace_cell(CellType::EmptyCell);}
                                        if ui.button("Replace with stem").clicked() {current_cell.replace_cell(CellType::StemCell);}
                                        if ui.button("Replace with worker").clicked() {current_cell.replace_cell(CellType::WorkerCell);}
                                        if ui.button("Replace with attacker").clicked() {current_cell.replace_cell(CellType::AttackingCell);}
                                        if ui.button("Replace with placable").clicked() {current_cell.replace_cell(CellType::PlacableCell);}
                                        if ui.button("increase hp").clicked() { current_cell.increase_hp(1);}
                                        if ui.button("decrease hp").clicked() { current_cell.decrease_hp(1);}
                                        if ui.button("increase apt").clicked() { current_cell.increase_apt(1);}
                                        if ui.button("decrease apt").clicked() { current_cell.decrease_apt(1);}
                                        if ui.button("Change player id").clicked() {
                                            self.game_manager.clone().debug_change_player_id(current_cell);
                                        }
                                    }
                                    ui.heading("Actons");
                                    if ui.button("Move").clicked() && self.game_manager.clone().check_movable(*current_cell){
                                        self.game_manager.move_cell(vec![x,y], current_cell);
                                        ui.close_menu();
                                    }
                                    if ui.button("Duplicate").clicked() && self.game_manager.clone().check_duplicate(*current_cell){
                                        self.game_manager.duplicate_cell(vec![x,y], current_cell);
                                        ui.close_menu();
                                    }
                                    ui.horizontal(|ui| { 
                                    if ui.button("transfer atp").clicked() && self.game_manager.clone().check_transfer(*current_cell){
                                        self.game_manager.transfer_atp(current_cell, vec![x,y]);
                                        ui.close_menu();
                                    }
                                    ui.add_sized([50.0, 8.0], egui::TextEdit::singleline(self.game_manager.get_transfer_apt_amount_string()));
                                    });
                                    if ui.button("Specailize Worker").clicked() && self.game_manager.clone().check_specailize(*current_cell,CellType::WorkerCell){
                                        self.game_manager.specailize_cell(current_cell, CellType::WorkerCell);
                                    }
                                    if ui.button("Specailize Attacker").clicked() && self.game_manager.clone().check_specailize(*current_cell, CellType::AttackingCell){
                                        self.game_manager.specailize_cell(current_cell, CellType::AttackingCell);
                                        ui.close_menu();
                                    }
                                    if ui.button("Attack").clicked() && self.game_manager.clone().check_attack(*current_cell){
                                        self.game_manager.attack_cell(current_cell, vec![x,y]);
                                        ui.close_menu();
                                    }
                                });
                            }});}
                        }
                    );
                    // resinc grid and manager
                    self.game_manager.sync_cell_grid(self.local_cell_grid.clone());

                    // frame handeling
                    self.game_manager.handle_game_action();
                }
            }
            
        });
    }
}
 
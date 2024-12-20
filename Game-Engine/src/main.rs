
use eframe::egui;

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

struct Cell {
    player: String,
    cell_type: i32,
    cell_image: String
}

impl Cell {
    fn new(new_player:String,new_cell_type:i32) -> Cell {
        Self {
            player: new_player,
            cell_type: new_cell_type,
            cell_image: Self::cell_type_to_image(Some(new_cell_type)),
        }        
        
    }
    fn cell_type_to_image(new_cell_type:Option<i32>) -> String {
        match new_cell_type {
            Some(1) => return String::from("Base Stem Cell.jpg"),
            Some(2) => return String::from("Base Attacking Cell.jpg"),
            Some(3) => return String::from("Base Worker Cell.jpg"),
            None => return String::from("Base Stem Cell.jpg"),
            _ => return String::from("Not_implemented")
        }
    }

}

enum GameState {
    Menu,
    Local_Game,
    Public_Game,
    Private_Game
}

struct Game {
    current_state: GameState,
    player_1_name: String,
    player_2_name: String,
    player_3_name: String,
    player_4_name: String,
}

impl Default for Game {
    fn default() -> Self {
        Self {
            current_state: GameState::Menu,
            player_1_name: "P1".to_owned(),
            player_2_name: "P1".to_owned(),
            player_3_name: "P3".to_owned(),
            player_4_name: "P4".to_owned(),
        }
    }
}

impl eframe::App for Game {
    //Big ass update function :p
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
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
                    ui.horizontal(|ui| 
                        {
                            ui.label(self.player_1_name.clone());
                            ui.label(self.player_2_name.clone());
                            ui.label(self.player_3_name.clone());
                            ui.label(self.player_4_name.clone());
                        }
                    );
                }
                GameState::Local_Game => {
                    ui.heading("Local Game");
                }
                _ => {
                    ui.heading("?????????");
                }
            }
            

            
        });
    }
}
 
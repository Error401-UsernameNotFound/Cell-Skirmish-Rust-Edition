use arraystring::{ArrayString, typenum::U200};
use cell::Cell;
use cell::CellType;
use eframe::Frame;
use egui::Sense;
use egui::{Color32, Context};
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
    LocalGame,
    PublicGame,
    PrivateGame
}

enum FrameAction {
    MoveCellStartUp,
    MoveCellEnd,
    PlaceCell,
    RemovePlacables,
    NoAction,
}

struct Game {
    //All variables

    //Set before game
    current_state: GameState,

    player_1_name: String,
    player_1_color: Vec<u8>,
    player_1_color_string: Vec<String>,

    player_2_name: String,
    player_2_color: Vec<u8>,
    player_2_color_string: Vec<String>,

    player_3_name: String,
    player_3_color: Vec<u8>,
    player_3_color_string: Vec<String>,

    player_4_name: String,
    player_4_color: Vec<u8>,
    player_4_color_string: Vec<String>,

    player_count: u8,
    grid_size: u32,
    grid_size_string: String,
    cell_size: f32,
    cell_size_string: String,
    debug_menu_options: bool,
    last_game_action: String,

    move_cell_cost:u8,
    duplicate_cell_cost:u8,
    transport_apt_cost:u8,
    attack_cell_cost:u8,

    //set/changed during game
    //game action things
    game_actions: Vec<FrameAction>,

    move_selectable_cells:Vec<Vec<i8>>,
    move_current_position:Vec<u8>,

    place_cell_position:Vec<usize>,
    place_cell_type:CellType,

    duplicate_selectable_cells:Vec<Vec<i8>>,
    duplicate_current_position:Vec<u8>,

    transfer_apt_amount:u8,
    transfer_apt_amount_string:String,

    cell_grid: Vec<Vec<cell::Cell>>,
    first_round: bool,
    

    //current turn
    current_turn_string: String,
    current_turn_id: u8,
}

impl Default for Game {
    fn default() -> Self {
        Self {
            current_state: GameState::Menu,
            
            player_1_name: "P1".to_owned(),
            player_1_color: vec![200,200,200],
            player_1_color_string: vec!["200".to_owned(),"200".to_owned(),"200".to_owned()],

            player_2_name: "P2".to_owned(),
            player_2_color: vec![200,200,200],
            player_2_color_string: vec!["200".to_owned(),"200".to_owned(),"200".to_owned()],

            player_3_name: "P3".to_owned(),
            player_3_color: vec![200,200,200],
            player_3_color_string: vec!["200".to_owned(),"200".to_owned(),"200".to_owned()],

            player_4_name: "P4".to_owned(),
            player_4_color: vec![200,200,200],
            player_4_color_string: vec!["200".to_owned(),"200".to_owned(),"200".to_owned()],

            player_count: 4,
            grid_size: 3,
            grid_size_string: "3".to_owned(),
            cell_grid: Vec::new(),
            cell_size: 45.0,
            cell_size_string: "45.0".to_owned(),
            debug_menu_options: false,
            current_turn_string: "P1".to_owned(),
            current_turn_id: 0,
            first_round: true,
            game_actions: vec![],

            move_selectable_cells: vec![],
            move_current_position: vec![],
            
            duplicate_selectable_cells: vec![],
            duplicate_current_position: vec![],

            move_cell_cost: 1,
            duplicate_cell_cost: 3,
            transport_apt_cost: 1,
            attack_cell_cost: 1,
            transfer_apt_amount: 0,
            transfer_apt_amount_string: "0".to_owned(),
            place_cell_position: vec![0,0],
            place_cell_type: CellType::Null,
            last_game_action: "".to_owned(),
        }
    }
}

impl Game {
    pub fn get_string_by_player_id(&mut self, player_id:u8) -> String {
        match player_id {
            0 => return self.player_1_name.clone(),
            1 => return self.player_2_name.clone(),
            2 => return self.player_3_name.clone(),
            3 => return self.player_4_name.clone(),
            _ => return "Null".to_owned()
        }
    }
    pub fn replace_all_empty_with_placable(&mut self){

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
                            ui.horizontal(|ui| {
                                ui.add_sized([50.0, 10.0], egui::TextEdit::singleline(&mut self.player_1_name));
                                ui.label("Color (r,g,b)");
                                ui.add_sized([50.0, 10.0], egui::TextEdit::singleline(&mut self.player_1_color_string[0]));
                                ui.add_sized([50.0, 10.0], egui::TextEdit::singleline(&mut self.player_1_color_string[1]));
                                ui.add_sized([50.0, 10.0], egui::TextEdit::singleline(&mut self.player_1_color_string[2]));
                            });

                            ui.label("Player 2 Name");
                            ui.horizontal(|ui| {
                                ui.add_sized([50.0, 10.0], egui::TextEdit::singleline(&mut self.player_2_name));
                                ui.label("Color (r,g,b)");
                                ui.add_sized([50.0, 10.0], egui::TextEdit::singleline(&mut self.player_2_color_string[0]));
                                ui.add_sized([50.0, 10.0], egui::TextEdit::singleline(&mut self.player_2_color_string[1]));
                                ui.add_sized([50.0, 10.0], egui::TextEdit::singleline(&mut self.player_2_color_string[2]));
                            });

                            ui.label("Player 3 Name");
                            ui.horizontal(|ui| {
                                ui.add_sized([50.0, 10.0], egui::TextEdit::singleline(&mut self.player_3_name));
                                ui.label("Color (r,g,b)");
                                ui.add_sized([50.0, 10.0], egui::TextEdit::singleline(&mut self.player_3_color_string[0]));
                                ui.add_sized([50.0, 10.0], egui::TextEdit::singleline(&mut self.player_3_color_string[1]));
                                ui.add_sized([50.0, 10.0], egui::TextEdit::singleline(&mut self.player_3_color_string[2]));
                            });
                            
                            ui.label("Player 4 Name");
                            ui.horizontal(|ui| {
                                ui.add_sized([50.0, 10.0], egui::TextEdit::singleline(&mut self.player_4_name));
                                ui.label("Color (r,g,b)");
                                ui.add_sized([50.0, 10.0], egui::TextEdit::singleline(&mut self.player_4_color_string[0]));
                                ui.add_sized([50.0, 10.0], egui::TextEdit::singleline(&mut self.player_4_color_string[1]));
                                ui.add_sized([50.0, 10.0], egui::TextEdit::singleline(&mut self.player_4_color_string[2]));
                            });
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
                    ui.checkbox(&mut self.debug_menu_options, "Activate Debug Options");
                    
                    ui.label(format!("Grid Size: {}",self.grid_size));
                    let grid_size_edited = ui.text_edit_singleline( &mut self.grid_size_string);
                    if grid_size_edited.lost_focus() {
                        let is_number:Result<u32, _> = self.grid_size_string.parse();
                        if is_number.is_ok() {
                            self.grid_size = is_number.unwrap()
                        }
                    }
                    
                    ui.label(format!("Cell Size: {}", self.cell_size));
                    if ui.text_edit_singleline( &mut self.cell_size_string).lost_focus() {
                        let is_number:Result<f32, _> = self.cell_size_string.parse();
                        if is_number.is_ok() {
                            self.cell_size = is_number.unwrap()
                        }
                    }
                    //spacer
                    ui.add_space(50.0);

                    if ui.button("Start Game").clicked() {
                        self.current_state = GameState::LocalGame;
                        self.place_cell_type = CellType::StemCell;

                        // Things to do once at game start
                        self.cell_grid = vec![
                            vec![
                                Cell::new();
                                self.grid_size as usize
                            ];
                            self.grid_size as usize
                        ];

                        self.current_turn_string = self.player_1_name.clone();
                        self.player_1_color = vec![
                            self.player_1_color_string[0].parse().unwrap_or(100),
                            self.player_1_color_string[0].parse().unwrap_or(100), 
                            self.player_1_color_string[0].parse().unwrap_or(100)
                        ];

                        self.player_2_color = vec![
                            self.player_2_color_string[0].parse().unwrap_or(100),
                            self.player_2_color_string[0].parse().unwrap_or(100), 
                            self.player_2_color_string[0].parse().unwrap_or(100)
                        ];

                        self.player_3_color = vec![
                            self.player_3_color_string[0].parse().unwrap_or(100),
                            self.player_3_color_string[0].parse().unwrap_or(100), 
                            self.player_3_color_string[0].parse().unwrap_or(100)
                        ];

                        self.player_4_color = vec![
                            self.player_4_color_string[0].parse().unwrap_or(100),
                            self.player_4_color_string[0].parse().unwrap_or(100), 
                            self.player_4_color_string[0].parse().unwrap_or(100)
                        ];
                        self.first_round = true;
                    }
                    
                }
                GameState::LocalGame => {
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
                    ui.label(format!("Grid Size: {} x {}", self.cell_grid.len(), self.cell_grid[0].len()));
                    ui.horizontal(|ui|
                        {
                            ui.heading(format!("Current turn: {}", self.current_turn_string));
                            if ui.button("Next turn").clicked() {
                                self.current_turn_id = (self.current_turn_id + 1) % (self.player_count);
                                self.current_turn_string = self.get_string_by_player_id(self.current_turn_id);
                                if self.first_round && self.current_turn_id == 0 {
                                    self.first_round = false;
                                }
                            }
                            if self.debug_menu_options{
                                ui.label(format!("Last action taken: {}", self.last_game_action));
                            }
                        }
                    );
                    
                    ui.horizontal(|ui|
                        {
                            for x in 0..self.cell_grid.len() { ui.vertical(|ui| 
                            {for y in 0..self.cell_grid.len() {
                                let current_cell = self.cell_grid[x].get_mut(y).unwrap();
                                
                                let mut cell_color:Vec<u8> = vec![255,255,255];
                                if current_cell.get_cell_player_number() != 15 {
                                    match current_cell.get_cell_player_number() {
                                        0 => cell_color = self.player_1_color.clone(),
                                        1 => cell_color = self.player_2_color.clone(),
                                        2 => cell_color = self.player_3_color.clone(),
                                        3 => cell_color = self.player_4_color.clone(),
                                        _ => cell_color = vec![0,0,0],
                                    }
                                }
                                let cell_image_location:ArrayString<U200> = current_cell.get_cell_image();
                                let cell_responce = ui.add(
                                    egui::Image::new(egui::Image::from_uri(cell_image_location.as_str()).source(ctx))
                                        .maintain_aspect_ratio(true)
                                        .fit_to_exact_size(egui::vec2(self.cell_size, self.cell_size))
                                        .rounding(5.0)
                                        .tint(Color32::from_rgb(cell_color[0],cell_color[1], cell_color[2]))
                                        ,
                                );
                                

                                if cell_responce.interact(Sense::click()).clicked(){
                                    self.place_cell_position = vec![x,y];
                                    self.game_actions.push(FrameAction::PlaceCell);
                                    self.last_game_action = "Place".to_owned();
                                }

                                cell_responce.context_menu(|ui|{
                                    ui.label(format!("Cell Type: {}", current_cell.get_cell_type()));
                                    ui.label(format!("Prev Cell Type: {}", current_cell.get_previous_cell_type()));
                                    ui.label(format!("Cell Player: {}", current_cell.get_cell_player()));
                                    ui.label(format!("Cell Player id: {}", current_cell.get_cell_player_number()));
                                    ui.label(format!("Cell hp: {}", current_cell.get_cell_hp()));
                                    ui.label(format!("Cell apt: {}", current_cell.get_cell_apt()));
                                    if self.debug_menu_options {
                                        if ui.button("Replace with empty").clicked() {current_cell.replace_cell(CellType::EmptyCell);}
                                        if ui.button("Replace with stem").clicked() {current_cell.replace_cell(CellType::StemCell);}
                                        if ui.button("Replace with worker").clicked() {current_cell.replace_cell(CellType::WorkerCell);}
                                        if ui.button("Replace with attacker").clicked() {current_cell.replace_cell(CellType::AttackingCell);}
                                        if ui.button("Replace with placable").clicked() {current_cell.replace_cell(CellType::PlacableCell);}
                                        if ui.button("increase hp").clicked() { current_cell.increase_hp(1);}
                                        if ui.button("decrease hp").clicked() { current_cell.decrease_hp(1);}
                                        if ui.button("Change player id").clicked() {
                                            let next_player_id = (current_cell.get_cell_player_number() + 1) % (self.player_count);
                                            current_cell.change_player("new_player".to_owned(), next_player_id);
                                        }
                                    }
                                    ui.heading("Actons");
                                    if ui.button("Move").clicked() && current_cell.get_cell_player_number() == self.current_turn_id && current_cell.get_cell_apt() >= self.move_cell_cost{
                                        self.move_selectable_cells = Cell::get_surrounding_cells(self.grid_size, vec![x.clone(),y.clone()]);
                                        self.move_current_position = vec![x as u8, y as u8];
                                        self.game_actions.push(FrameAction::MoveCellStartUp);
                                        current_cell.decrease_apt(1);
                                        self.last_game_action = "Move".to_owned();
                                    }
                                    if ui.button("Duplicate").clicked() && current_cell.get_cell_player_number() == self.current_turn_id{
                                        
                                    }
                                    ui.horizontal(|ui| { 
                                    if ui.button("transfer atp").clicked() && current_cell.get_cell_player_number() == self.current_turn_id && current_cell.get_cell_apt() >= self.transport_apt_cost + 1{
                                        
                                    }
                                    ui.add_sized([50.0, 10.0], egui::TextEdit::singleline(&mut self.transfer_apt_amount_string));
                                    });
                                    if ui.button("Duplicate").clicked() && current_cell.get_cell_player_number() == self.current_turn_id{
                                        
                                    }
                                });
                            }});}
                        }
                    );
                    // frame handeling
                    let number_of_actions = self.game_actions.len();
                    for _ in 0..number_of_actions {
                        let frame_action = self.game_actions.pop().unwrap_or(FrameAction::NoAction);
                        match frame_action {
                            FrameAction::MoveCellStartUp => {
                                // get cells arround slected cell and replace them with placables
                                for positions in &self.move_selectable_cells{
                                    let current_x = (self.move_current_position[0] as i8) + positions[0];
                                    let current_y = (self.move_current_position[1] as i8) + positions[1];
                                    let current_cell = self.cell_grid[current_x as usize].get_mut(current_y as usize).unwrap();
                                    if !current_cell.is_alive(){
                                        current_cell.replace_cell(CellType::PlacableCell);
                                        current_cell.change_player(self.current_turn_string.clone(), self.current_turn_id);
                                        current_cell.set_hp(0);
                                    }
                                }
                            },
                            FrameAction::PlaceCell => {
                                //change placable into real cell, remove all other placables
                                let current_cell = self.cell_grid[self.place_cell_position[0]].get_mut(self.place_cell_position[1]).unwrap();
                                current_cell.change_player(self.current_turn_string.clone(), self.current_turn_id);
                                current_cell.replace_cell(CellType::StemCell);
                                current_cell.set_hp(5);
                                current_cell.set_apt(1);
                                //*current_cell;
                                for x in 0..self.cell_grid.len() {
                                for y in 0..self.cell_grid.len() {
                                    let test_cell = self.cell_grid[x].get_mut(y).unwrap();
                                    if test_cell.get_raw_cell_type() == CellType::PlacableCell {
                                        test_cell.reset_cell();
                                    }
                                }
                                }

                            },
                            FrameAction::MoveCellEnd => {},
                            FrameAction::RemovePlacables => {},
                            FrameAction::NoAction => {},
                        }
                    }
                }
                GameState::PublicGame => {
                    ui.heading("?????????");
                }
                GameState::PrivateGame => {
                    ui.heading("?????????");
                }
            }
            

            
        });
    }
}
 
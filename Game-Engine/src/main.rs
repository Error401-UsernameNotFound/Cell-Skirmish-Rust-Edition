use arraystring::{ArrayString, typenum::U200};
use cell::{Cell, CellAction, CellType};
use eframe::{egui, Frame};
use egui::{Sense,Color32, Context};

mod cell;
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
            Ok(Box::<Game>::default())
        }),
    )
}

enum GameState {
    Menu,
    LocalGame,
}

enum FrameAction {
    MoveCell,
    DuplicateCell,
    PlaceCell,
    TransferAPTStart,
    AttackCell,
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

    
    //set/changed during game
    //game action things
    game_actions: Vec<FrameAction>,

    move_cell_cost:u8,
    move_selectable_cells:Vec<Vec<i8>>,
    move_current_position:Vec<u8>,

    place_cell_position:Vec<usize>,
    place_cell_type:CellType,

    duplicate_cell_cost:u8,
    duplicate_selectable_cells:Vec<Vec<i8>>,
    duplicate_current_position:Vec<u8>,

    specalilize_into_attacker_cost:u8,
    specalilize_into_worker_cost:u8,

    transfer_apt_cost:u8,
    transfer_apt_amount:u8,
    transfer_apt_amount_string:String,
    transfer_current_position:Vec<u8>,
    transfer_surrounding_cells:Vec<Vec<i8>>,
    
    attack_cell_cost:u8,
    attack_current_position:Vec<u8>,
    attack_surrounding_cells:Vec<Vec<i8>>,
    attack_base_damage: u8,
    attack_extra_damage: u8,
        



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
            grid_size: 7,
            grid_size_string: "7".to_owned(),
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
            transfer_apt_cost: 1,
            attack_cell_cost: 1,
            transfer_apt_amount: 0,
            transfer_apt_amount_string: "0".to_owned(),
            place_cell_position: vec![0,0],
            place_cell_type: CellType::Null,
            last_game_action: "".to_owned(),
            specalilize_into_attacker_cost: 3,
            specalilize_into_worker_cost: 3,

            transfer_current_position: vec![],
            transfer_surrounding_cells: vec![],

            attack_current_position: vec![],
            attack_surrounding_cells: vec![],
            attack_base_damage: 1,
            attack_extra_damage: 1,
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
    pub fn add_player_to_board(&mut self, player_to_add:u8){
        // 2 parts
        // 1: find all non-empty cell, record them and all positions arround them,
        // 2: replace all non-standard positions into placables

        let mut removed_positions:Vec<Vec<usize>> = vec![];   
        let grid_size = self.cell_grid.len();     
        for x in 0..grid_size {
        for y in 0..grid_size {
            let test_cell = self.cell_grid[x].get_mut(y).unwrap();
            if test_cell.get_raw_cell_type() != CellType::EmptyCell {
                let local_surounding: Vec<Vec<i8>> = Cell::get_surrounding_cells(grid_size as u32, vec![x,y]);
                for local_pos in local_surounding{
                    let local_vec = vec![(((x as i8) + local_pos[0]) as usize),(((y as i8) + local_pos[1]) as usize)];
                    removed_positions.push(local_vec);
                }   
            }
        }
        }
        self.last_game_action = format!("Add player, removed cells {:?}", removed_positions);

        let player_to_add_string = self.get_string_by_player_id(player_to_add);
        for x in 0..grid_size {
        for y in 0..grid_size {
            let test_cell = self.cell_grid[x].get_mut(y).unwrap();
            if !removed_positions.contains(&vec![x.clone(),y.clone()]) {
                test_cell.replace_cell(CellType::PlacableCell);
                test_cell.change_player(player_to_add_string.clone(), player_to_add);
                test_cell.set_cell_action(CellAction::StartingPlace);
            }
        }
        }
    }
    pub fn remove_all_placables(&mut self){
        for x in 0..self.cell_grid.len() {
        for y in 0..self.cell_grid.len() {
            let test_cell = self.cell_grid[x].get_mut(y).unwrap();
            if test_cell.get_raw_cell_type() == CellType::PlacableCell {
                test_cell.reset_cell();
            }
        }
        }
    }
    pub fn revert_all_placables(&mut self){
        for x in 0..self.cell_grid.len() {
        for y in 0..self.cell_grid.len() {
            let test_cell = self.cell_grid[x].get_mut(y).unwrap();
            if test_cell.get_raw_cell_type() == CellType::PlacableCell {
                test_cell.revert_cell();
            }
        }
        }
    }
    pub fn increase_current_player_atp(&mut self){
        for x in 0..self.cell_grid.len() {
        for y in 0..self.cell_grid.len() {
            let test_cell = self.cell_grid[x].get_mut(y).unwrap();
            if test_cell.get_cell_player_number() == self.current_turn_id {
                test_cell.increase_apt(1);

                // one extra for workers :3
                if test_cell.get_raw_cell_type() == CellType::WorkerCell{
                    test_cell.increase_apt(1);
                }
            }
        }
        }
    }
    pub fn check_dead_cells(&mut self){
        for x in 0..self.cell_grid.len() {
        for y in 0..self.cell_grid.len() {
            let test_cell = self.cell_grid[x].get_mut(y).unwrap();
            if test_cell.get_cell_hp() <= 0  && test_cell.get_cell_player_number() != 15{
                test_cell.reset_cell();
            }
        }
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
                                Cell::new(self.player_1_name.clone());
                                self.grid_size as usize
                            ];
                            self.grid_size as usize
                        ];
                        self.current_turn_string = self.player_1_name.clone();
                        self.first_round = true;
                    }
                    
                }
                GameState::LocalGame => {
                    // Headding info
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

                    
                    ui.label(format!("Grid Size: {} x {}", self.cell_grid.len(), self.cell_grid[0].len()));
                    ui.horizontal(|ui|
                        {
                            ui.heading(format!("Current turn: {}", self.current_turn_string));
                            if ui.button("Next turn").clicked() {
                                self.current_turn_id = (self.current_turn_id + 1) % (self.player_count);
                                self.current_turn_string = self.get_string_by_player_id(self.current_turn_id);
                                self.game_actions.push(FrameAction::MoveCell);
                                self.increase_current_player_atp();
                                if self.first_round && self.current_turn_id == 0 {
                                    self.first_round = false;
                                }
                                else if self.first_round {
                                    self.add_player_to_board(self.current_turn_id);
                                }
                            }
                            if self.debug_menu_options{
                                ui.label(format!("Last action taken: {}", self.last_game_action));
                            }
                        }
                    );

                    //NxN grid,
                    ui.horizontal(|ui|
                        {
                            for x in 0..self.cell_grid.len() { ui.vertical(|ui| 
                            {for y in 0..self.cell_grid.len() {
                                let current_cell = self.cell_grid[x].get_mut(y).unwrap();
                                
                                
                                let cell_image_location:ArrayString<U200> = current_cell.get_cell_image();
                                let cell_responce = ui.add(
                                    egui::Image::new(egui::Image::from_uri(cell_image_location.as_str()).source(ctx))
                                        .maintain_aspect_ratio(true)
                                        .fit_to_exact_size(egui::vec2(self.cell_size, self.cell_size))
                                        .rounding(5.0)
                                        ,
                                );
                                

                                if cell_responce.interact(Sense::click()).clicked() && current_cell.get_raw_cell_type() == CellType::PlacableCell && (current_cell.get_cell_player_number() == self.current_turn_id || current_cell.get_cell_action() == CellAction::Attacked){
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
                                    ui.label(format!("Cell apt: {}", current_cell.get_cell_atp()));
                                    if self.debug_menu_options {
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
                                            let next_player_id = (current_cell.get_cell_player_number() + 1) % (self.player_count);
                                            current_cell.change_player("new_player".to_owned(), next_player_id);
                                        }
                                    }
                                    ui.heading("Actons");
                                    if ui.button("Move").clicked() && current_cell.get_cell_player_number() == self.current_turn_id && current_cell.get_cell_atp() >= self.move_cell_cost{
                                        self.move_selectable_cells = Cell::get_surrounding_cells(self.grid_size, vec![x.clone(),y.clone()]);
                                        self.move_current_position = vec![x as u8, y as u8];
                                        self.game_actions.push(FrameAction::MoveCell);
                                        current_cell.decrease_apt(self.move_cell_cost);
                                        self.last_game_action = "Move".to_owned();
                                        ui.close_menu();
                                    }
                                    if ui.button("Duplicate").clicked() && current_cell.get_cell_player_number() == self.current_turn_id  && current_cell.get_cell_atp() >= self.duplicate_cell_cost{
                                        self.duplicate_selectable_cells = Cell::get_surrounding_cells(self.grid_size, vec![x.clone(),y.clone()]);
                                        self.duplicate_current_position = vec![x.clone() as u8, y.clone() as u8];
                                        self.game_actions.push(FrameAction::DuplicateCell);
                                        current_cell.decrease_apt(self.duplicate_cell_cost);
                                        self.last_game_action = "Duplicated".to_owned();
                                        ui.close_menu();
                                    }
                                    ui.horizontal(|ui| { 
                                    if ui.button("transfer atp").clicked() && current_cell.get_cell_player_number() == self.current_turn_id && current_cell.get_cell_atp() >= self.transfer_apt_cost{
                                        self.transfer_apt_amount = self.transfer_apt_amount_string.parse().unwrap_or(0);
                                        if current_cell.get_cell_atp() >= self.transfer_apt_cost + self.transfer_apt_amount {
                                            self.transfer_surrounding_cells = Cell::get_surrounding_cells(self.grid_size, vec![x.clone(),y.clone()]);
                                            self.transfer_current_position = vec![x as u8,y as u8];
                                            self.game_actions.push(FrameAction::TransferAPTStart);
                                            ui.close_menu();
                                        }
                                    }
                                    ui.add_sized([50.0, 8.0], egui::TextEdit::singleline(&mut self.transfer_apt_amount_string));
                                    });
                                    if ui.button("Specailize Worker").clicked() && current_cell.get_cell_player_number() == self.current_turn_id && current_cell.get_cell_atp() >= self.specalilize_into_worker_cost{
                                        current_cell.replace_cell(CellType::WorkerCell);
                                        current_cell.decrease_apt(self.specalilize_into_worker_cost);
                                        self.last_game_action = "Specailize Worker".to_owned();
                                        ui.close_menu();
                                    }
                                    if ui.button("Specailize Attacker").clicked() && current_cell.get_cell_player_number() == self.current_turn_id && current_cell.get_cell_atp() >= self.specalilize_into_attacker_cost{
                                        current_cell.replace_cell(CellType::AttackingCell);
                                        current_cell.decrease_apt(self.specalilize_into_worker_cost);
                                        self.last_game_action = "Specailize Attacker".to_owned();
                                        ui.close_menu();
                                    }
                                    if ui.button("Attack").clicked() && current_cell.get_cell_player_number() == self.current_turn_id && current_cell.get_cell_atp() >= self.attack_cell_cost{
                                        self.attack_surrounding_cells = current_cell.get_attack_range(self.grid_size as i32, vec![x as i8, y as i8]);
                                        self.attack_current_position = vec![x.clone() as u8, y.clone() as u8];
                                        self.game_actions.push(FrameAction::AttackCell);
                                        current_cell.decrease_apt(self.attack_cell_cost);
                                        self.last_game_action = "Attack Started".to_owned();
                                        ui.close_menu();
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
                            FrameAction::MoveCell => {
                                // get cells arround slected cell and replace them with placables
                                for positions in &self.move_selectable_cells{
                                    let current_x = (self.move_current_position[0] as i8) + positions[0];
                                    let current_y = (self.move_current_position[1] as i8) + positions[1];
                                    let current_cell = self.cell_grid[current_x as usize].get_mut(current_y as usize).unwrap();
                                    if !current_cell.is_alive(){
                                        current_cell.replace_cell(CellType::PlacableCell);
                                        current_cell.change_player(self.current_turn_string.clone(), self.current_turn_id);
                                        current_cell.set_cell_action(CellAction::MoveCell);
                                    }
                                }
                            },
                            FrameAction::DuplicateCell => {
                                for positions in &self.duplicate_selectable_cells{
                                    let current_x = (self.duplicate_current_position[0] as i8) + positions[0];
                                    let current_y = (self.duplicate_current_position[1] as i8) + positions[1];
                                    let current_cell = self.cell_grid[current_x as usize].get_mut(current_y as usize).unwrap();
                                    if !current_cell.is_alive(){
                                        current_cell.replace_cell(CellType::PlacableCell);
                                        current_cell.change_player(self.current_turn_string.clone(), self.current_turn_id);
                                        current_cell.set_cell_action(CellAction::DuplicateCell);
                                    }
                                }
                            },
                            FrameAction::PlaceCell => {
                                //change placable into real cell, remove all other placables
                                let static_grid_refrence = self.cell_grid.clone();
                                let current_cell = self.cell_grid[self.place_cell_position[0]].get_mut(self.place_cell_position[1]).unwrap();
                                let current_cell_action = current_cell.get_cell_action();
                                match current_cell_action {
                                    CellAction::NoAction => {
                                        // Should Never happen
                                    },
                                    CellAction::MoveCell => {
                                        let moved_cell = static_grid_refrence[self.move_current_position[0] as usize][self.move_current_position[1] as usize];
                                        current_cell.replace_with_cell(moved_cell);
                                        current_cell.set_cell_action(CellAction::NoAction);
                                        let _ = *current_cell;
                                        let refrenced_moved_cell = self.cell_grid[self.move_current_position[0]as usize].get_mut(self.move_current_position[1] as usize).unwrap();
                                        refrenced_moved_cell.reset_cell();

                                    },
                                    CellAction::DuplicateCell => {
                                        let og_cell = static_grid_refrence[self.duplicate_current_position[0] as usize][self.duplicate_current_position[1] as usize];
                                        current_cell.replace_cell(og_cell.get_raw_cell_type());
                                        current_cell.set_cell_action(CellAction::NoAction);
                                        current_cell.set_hp(5);
                                        current_cell.set_apt(0);
                                    },
                                    CellAction::StartingPlace => {
                                        current_cell.change_player(self.current_turn_string.clone(), self.current_turn_id);
                                        current_cell.replace_cell(CellType::StemCell);
                                        current_cell.set_hp(5);
                                        current_cell.set_apt(0);
                                    },
                                    CellAction::TransferTo => {
                                        current_cell.increase_apt(self.transfer_apt_amount);
                                        self.revert_all_placables();
                                    },
                                    CellAction::Attacked => {
                                        let cur_x = self.attack_current_position[0] as usize;
                                        let cur_y = self.attack_current_position[1] as usize;
                                        let attack_starting_cell = static_grid_refrence[cur_x][cur_y];

                                        current_cell.decrease_hp(self.attack_base_damage);
                                        if attack_starting_cell.get_raw_cell_type() == CellType::AttackingCell{
                                            let melee_range_modifier = Cell::get_surrounding_cells(self.grid_size, vec![cur_x, cur_y]);
                                            let mut melee_range_cells:Vec<Vec<usize>> = vec![];
                                            for modifier in melee_range_modifier{
                                                melee_range_cells.push(vec![cur_x + modifier[0] as usize, cur_y + modifier[1] as usize]);
                                            }
                                            if melee_range_cells.contains(&vec![self.place_cell_position[0],self.place_cell_position[1]]){
                                                current_cell.decrease_hp(self.attack_extra_damage);
                                            }
                                        }
                                        self.revert_all_placables();
                                        self.check_dead_cells();
                                        
                                    },
                                }
                                self.last_game_action = format!("Placed at {}, {}; Action: {}", self.place_cell_position[0], self.place_cell_position[1], current_cell_action).to_owned();
                                
                                // remove all placables from board

                                self.remove_all_placables();

                            },
                            FrameAction::TransferAPTStart => {
                                let cur_x = self.transfer_current_position[0] as i8;
                                let cur_y = self.transfer_current_position[1] as i8;
                                let transfer_starting_cell = self.cell_grid[cur_x as usize].get_mut(cur_y as usize).unwrap();
                                transfer_starting_cell.decrease_apt(self.transfer_apt_amount + self.transfer_apt_cost);
                                for local_position in self.transfer_surrounding_cells.clone(){
                                    if local_position[0] == 0 && local_position[1] == 0 {continue};
                                    let absolute_position = vec![cur_x + local_position[0], cur_y+local_position[1]];
                                    // if the player owns a surrounding cell
                                    let test_cell = self.cell_grid[absolute_position[0] as usize].get_mut(absolute_position[1] as usize).unwrap();
                                    if test_cell.get_cell_player_number() == self.current_turn_id{
                                        test_cell.replace_cell(CellType::PlacableCell);
                                        test_cell.set_cell_action(CellAction::TransferTo);
                                        
                                    }

                                }
                            },
                            FrameAction::AttackCell => {
                                for local_position in self.attack_surrounding_cells.clone(){
                                    if local_position[0] == 0 && local_position[1] == 0 {continue};
                                    let absolute_position = vec![local_position[0], local_position[1]];
                                    //print!("{:?}", absolute_position);
                                    // if the player owns a surrounding cell
                                    let test_cell = self.cell_grid[absolute_position[0] as usize].get_mut(absolute_position[1] as usize).unwrap();
                                    if test_cell.get_cell_player_number() != self.current_turn_id && test_cell.is_alive(){
                                        test_cell.replace_cell(CellType::PlacableCell);
                                        test_cell.set_cell_action(CellAction::Attacked);
                                        
                                    }

                                }
                            },
                            FrameAction::NoAction => {},
                        }
                    }
                }
            }
            

            
        });
    }
}
 
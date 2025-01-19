use cell::{Cell, CellAction, CellType};
pub(crate) mod cell;

#[derive(Clone, Copy)]
pub enum GameState {
    Menu,
    LocalGame,
}

#[derive(Clone, Copy)]
pub enum FrameAction {
    MoveCell,
    DuplicateCell,
    PlaceCell,
    TransferAPTStart,
    AttackCell,
    NoAction,
}

#[derive(Clone)]
pub struct GameEngine {
    // State
    current_state: GameState,
    debug_menu_options: bool,
    current_turn_string: String,
    current_turn_id: i32,
    first_round: bool,
    game_actions: Vec<FrameAction>,
    last_game_action: String,

    // Players
    player_1_name: String,
    player_2_name: String,
    player_3_name: String,
    player_4_name: String,
    player_count: i32,

    // Grid
    grid_size: i32,
    grid_size_string: String,
    cell_grid: Vec<Vec<cell::Cell>>,

    // Cells
    cell_size: f32,
    cell_size_string: String,
    cell_apt_cap:i32,

    // Move
    move_cell_cost:i32,
    move_selectable_cells:Vec<Vec<i32>>,
    move_current_position:Vec<i32>,

    // Duplicate
    duplicate_cell_cost:i32,
    duplicate_selectable_cells:Vec<Vec<i32>>,
    duplicate_current_position:Vec<i32>,

    // Transfer
    transfer_apt_cost:i32,
    transfer_apt_amount:i32,
    transfer_apt_amount_string:String,
    transfer_current_position:Vec<i32>,
    transfer_surrounding_cells:Vec<Vec<i32>>,

    // Attack
    attack_cell_cost:i32,
    attack_current_position:Vec<i32>,
    attack_surrounding_cells:Vec<Vec<i32>>,
    attack_base_damage: i32,
    attack_extra_damage: i32,

    // Place
    place_cell_position:Vec<i32>,
    place_cell_type:CellType,

    // Specalize
    specalilize_into_attacker_cost:i32,
    specalilize_into_worker_cost:i32,
}

impl Default for GameEngine {
    fn default() -> Self {
        Self {
            // State
            current_state: GameState::Menu,
            debug_menu_options: false,
            current_turn_string: "P1".to_owned(),
            current_turn_id: 0,
            first_round: true,
            game_actions: vec![],
            last_game_action: "".to_owned(),
            
            // Players
            player_1_name: "P1".to_owned(),
            player_2_name: "P2".to_owned(),
            player_3_name: "P3".to_owned(),
            player_4_name: "P4".to_owned(),
            player_count: 4,

            // Grid
            grid_size: 9,
            grid_size_string: "9".to_owned(),
            cell_grid: vec![],

            // Cells
            cell_size: 45.0,
            cell_size_string: "45.0".to_owned(),
            cell_apt_cap: 5,

            // Move
            move_selectable_cells: vec![],
            move_current_position: vec![],
            move_cell_cost: 1,
            
            // Duplicate
            duplicate_selectable_cells: vec![],
            duplicate_current_position: vec![],
            duplicate_cell_cost: 3,

            // Transfer
            transfer_current_position: vec![],
            transfer_surrounding_cells: vec![],
            transfer_apt_amount: 0,
            transfer_apt_amount_string: "0".to_owned(),
            transfer_apt_cost: 1,

            // Attack
            attack_cell_cost: 1,
            attack_current_position: vec![],
            attack_surrounding_cells: vec![],
            attack_base_damage: 1,
            attack_extra_damage: 1,

            // Place
            place_cell_position: vec![0,0],
            place_cell_type: CellType::Null,
            
            // specalize
            specalilize_into_attacker_cost: 3,
            specalilize_into_worker_cost: 3,

            
        }
    }
}


// Helper Game Functions
impl GameEngine {
    fn get_string_by_player_id(&mut self, player_id:i32) -> String {
        match player_id {
            0 => return self.player_1_name.clone(),
            1 => return self.player_2_name.clone(),
            2 => return self.player_3_name.clone(),
            3 => return self.player_4_name.clone(),
            _ => return "Null".to_owned()
        }
    }
    fn add_player_to_board(&mut self, player_to_add:i32){
        // 2 parts
        // 1: find all non-empty cell, record them and all positions arround them,
        // 2: replace all non-standard positions into placables

        let mut removed_positions:Vec<Vec<usize>> = vec![];   
        let grid_size = self.cell_grid.len();     
        for x in 0..grid_size {
        for y in 0..grid_size {
            let test_cell = self.cell_grid[x].get_mut(y).unwrap();
            if test_cell.get_raw_cell_type() != CellType::EmptyCell {
                let local_surounding: Vec<Vec<i32>> = Cell::get_surrounding_cells(grid_size as i32, vec![x,y]);
                for local_pos in local_surounding{
                    let local_vec = vec![(((x as i32) + local_pos[0]) as usize),(((y as i32) + local_pos[1]) as usize)];
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
    fn remove_all_placables(&mut self){
        for x in 0..self.cell_grid.len() {
        for y in 0..self.cell_grid.len() {
            let test_cell = self.cell_grid[x].get_mut(y).unwrap();
            if test_cell.get_raw_cell_type() == CellType::PlacableCell {
                test_cell.reset_cell();
            }
        }
        }
    }
    fn revert_all_placables(&mut self){
        for x in 0..self.cell_grid.len() {
        for y in 0..self.cell_grid.len() {
            let test_cell = self.cell_grid[x].get_mut(y).unwrap();
            if test_cell.get_raw_cell_type() == CellType::PlacableCell {
                test_cell.revert_cell();
            }
        }
        }
    }
    fn increase_current_player_atp(&mut self){
        for x in 0..self.cell_grid.len() {
        for y in 0..self.cell_grid.len() {
            let test_cell = self.cell_grid[x].get_mut(y).unwrap();
            if test_cell.get_cell_player_number() == self.current_turn_id {
                test_cell.increase_apt(1);

                // one extra for workers :3
                if test_cell.get_raw_cell_type() == CellType::WorkerCell{
                    test_cell.increase_apt(1);
                }
                if test_cell.get_cell_atp() > self.cell_apt_cap {
                    test_cell.set_apt(self.cell_apt_cap);
                }
            }
        }
        }
    }
    fn check_dead_cells(&mut self){
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

// Interactable game functions
impl GameEngine {
    // Before game start
    pub fn reset_game(&mut self){
        self.debug_menu_options = false;
        self.current_turn_string = self.player_1_name.clone();
        self.current_turn_id = 0;
        self.first_round = true;
        self.game_actions = vec![];
        self.last_game_action = "".to_owned();
    }
    pub fn reset_grid(&mut self){
        self.grid_size;
        self.cell_grid = vec![
            vec![
                Cell::new(self.player_1_name.clone());
                self.grid_size as usize
            ];
            self.grid_size as usize
        ];
    }
    pub fn start_game(&mut self){
        self.current_state = GameState::LocalGame;
        self.place_cell_type = CellType::StemCell;
        self.reset_game();
        self.reset_grid();
    }
    
    // Game -> Ui Interaction
    pub fn battle_vs_title(self) -> String{
        match self.player_count {
            2 => {
                return format!("{} vs {}", self.player_1_name, self.player_2_name);
            },
            3 => {
                return format!("{} vs {} vs {}", self.player_1_name, self.player_2_name, self.player_3_name);
            },
            4 => {
                return format!("{} vs {} vs {} vs {}", self.player_1_name, self.player_2_name, self.player_3_name, self.player_4_name);
            },
            _ => {
                return format!("How did you manage this one? Player_count: {}",self.player_count);
            }
        }
    }
    
    // Durring game
    pub fn start_next_turn(&mut self){
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
    pub fn handle_game_action(&mut self){
        let number_of_actions = self.game_actions.len();
        for _ in 0..number_of_actions {
            let frame_action = self.game_actions.pop().unwrap_or(FrameAction::NoAction);
            match frame_action {
                FrameAction::MoveCell => {
                    // get cells arround slected cell and replace them with placables
                    for positions in &self.move_selectable_cells{
                        let current_x = self.move_current_position[0] + positions[0];
                        let current_y = self.move_current_position[1] + positions[1];
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
                        let current_x = self.duplicate_current_position[0]  + positions[0];
                        let current_y = self.duplicate_current_position[1] + positions[1];
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
                    let current_cell = self.cell_grid[self.place_cell_position[0] as usize].get_mut(self.place_cell_position[1]  as usize).unwrap();
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
                            let cur_x = self.attack_current_position[0];
                            let cur_y = self.attack_current_position[1];
                            let attack_starting_cell = static_grid_refrence[cur_x as usize][cur_y as usize];

                            current_cell.decrease_hp(self.attack_base_damage);
                            if attack_starting_cell.get_raw_cell_type() == CellType::AttackingCell{
                                let melee_range_modifier = Cell::get_surrounding_cells(self.grid_size, vec![cur_x as usize, cur_y as usize]);
                                let mut melee_range_cells:Vec<Vec<i32>> = vec![];
                                for modifier in melee_range_modifier{
                                    melee_range_cells.push(vec![cur_x + modifier[0], cur_y + modifier[1]]);
                                }
                                if melee_range_cells.contains(&vec![self.place_cell_position[0], self.place_cell_position[1]]){
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
                    let cur_x = self.transfer_current_position[0];
                    let cur_y = self.transfer_current_position[1];
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
    
    //   Button handeling
    pub fn check_placable(self, current_cell:Cell) -> bool{
        return  current_cell.get_raw_cell_type() == CellType::PlacableCell && (current_cell.get_cell_player_number() == self.current_turn_id || current_cell.get_cell_action() == CellAction::Attacked)
    }
    pub fn place_cell(&mut self, position:Vec<i32>){
        self.place_cell_position = position;
        self.game_actions.push(FrameAction::PlaceCell);
        self.last_game_action = "Place".to_owned();
    }

    pub fn check_movable(self,current_cell:Cell) -> bool{
        return current_cell.get_cell_player_number() == self.current_turn_id && current_cell.get_cell_atp() >= self.move_cell_cost;
    }
    pub fn move_cell(&mut self, position:Vec<i32>, current_cell:&mut Cell){
        self.move_selectable_cells = Cell::get_surrounding_cells(self.grid_size, vec![position[0] as usize, position[1] as usize]);
        self.move_current_position = position;
        self.game_actions.push(FrameAction::MoveCell);
        current_cell.decrease_apt(self.move_cell_cost);
        self.last_game_action = "Move".to_owned();
    }
    
    pub fn check_duplicate(self, current_cell:Cell) -> bool{
        return current_cell.get_cell_player_number() == self.current_turn_id  && current_cell.get_cell_atp() >= self.duplicate_cell_cost;
    }
    pub fn duplicate_cell(&mut self, position:Vec<i32>, current_cell:&mut Cell){
        self.duplicate_selectable_cells = Cell::get_surrounding_cells(self.grid_size, vec![position[0] as usize, position[1] as usize]);
        self.duplicate_current_position = position.clone();
        self.game_actions.push(FrameAction::DuplicateCell);
        current_cell.decrease_apt(self.duplicate_cell_cost);
        self.last_game_action = "Duplicated".to_owned();
    }

    pub fn check_transfer(self,current_cell:Cell) -> bool {
        return current_cell.get_cell_player_number() == self.current_turn_id && current_cell.get_cell_atp() >= self.transfer_apt_cost;
    }
    pub fn transfer_atp(&mut self,current_cell:&mut Cell,position:Vec<i32>){
        self.transfer_apt_amount = self.transfer_apt_amount_string.parse().unwrap_or(0);
        if current_cell.get_cell_atp() >= self.transfer_apt_cost + self.transfer_apt_amount {
            self.transfer_surrounding_cells = Cell::get_surrounding_cells(self.grid_size, vec![position[0] as usize, position[1] as usize]);
            self.transfer_current_position = position.clone();
            self.game_actions.push(FrameAction::TransferAPTStart);
        }
    }


    pub fn check_specailize(self, current_cell:Cell, desired_type:CellType) -> bool{
        if current_cell.get_cell_player_number() != self.current_turn_id || current_cell.get_raw_cell_type() != CellType::StemCell {return false;}
        match desired_type {
            CellType::WorkerCell => current_cell.get_cell_atp() >= self.specalilize_into_worker_cost,
            CellType::AttackingCell => current_cell.get_cell_atp() >= self.specalilize_into_attacker_cost,
            _ => false,
        }
    }
    pub fn specailize_cell(&mut self, current_cell:&mut Cell, desired_type:CellType){
        match desired_type {
            CellType::WorkerCell => {
                current_cell.replace_cell(CellType::WorkerCell);
                current_cell.decrease_apt(self.specalilize_into_worker_cost);
                self.last_game_action = "Specailize Worker".to_owned();
            },
            CellType::AttackingCell => {
                current_cell.replace_cell(CellType::AttackingCell);
                current_cell.decrease_apt(self.specalilize_into_worker_cost);
                self.last_game_action = "Specailize Attacker".to_owned();
            },
            _ => {},
        }
    }
    
    pub fn check_attack(self, current_cell:Cell) -> bool {
        return current_cell.get_cell_player_number() == self.current_turn_id && current_cell.get_cell_atp() >= self.attack_cell_cost;
    }
    pub fn attack_cell(&mut self, current_cell:&mut Cell, position:Vec<i32>){
        self.attack_surrounding_cells = current_cell.get_attack_range(self.grid_size, vec![position[0] as usize, position[1] as usize]);
        self.attack_current_position = position.clone();
        self.game_actions.push(FrameAction::AttackCell);
        current_cell.decrease_apt(self.attack_cell_cost);
        self.last_game_action = "Attack Started".to_owned();
    }
    
    // debug mode specific functions
    pub fn debug_get_last_action(self) -> String{
        return format!("Last action taken: {}", self.last_game_action);
    }
    pub fn debug_change_player_id(self, current_cell:&mut Cell){
        let next_player_id = (current_cell.get_cell_player_number() + 1) % (self.player_count);
        current_cell.change_player("new_player".to_owned(), next_player_id);
    }
    

}

// Getter methods
impl GameEngine {
    pub fn get_current_state(self) -> GameState {
        return self.current_state;
    }
    pub fn set_current_state(&mut self, next_state:GameState) {
        self.current_state = next_state;
    }
    pub fn get_player_string(&mut self, player_id:i32) -> &mut String{
        match player_id {
            0 => {return &mut self.player_1_name;}
            1 => {return &mut self.player_2_name;}
            2 => {return &mut self.player_3_name;}
            3 => {return &mut self.player_4_name;}
            _ => {return &mut self.player_4_name;}
        }
    }
    pub fn is_debug(&mut self,) -> &mut bool {
        return &mut self.debug_menu_options;
    }
    pub fn get_player_count(self) -> i32 {
        return self.player_count;
    }
    pub fn set_player_count(&mut self, count:i32) {
        self.player_count = count;
    }
    pub fn get_grid_size(&mut self) -> i32{
        return self.grid_size;
    }
    pub fn set_grid_size(&mut self, new_size:i32){
        self.grid_size = new_size;
    }
    pub fn get_grid_size_string(&mut self) -> &mut String {
        return &mut self.grid_size_string;
    }
    pub fn get_cell_size(self) -> f32 {
        return self.cell_size;
    }
    pub fn set_cell_size(&mut self, new_size:f32) {
        self.cell_size = new_size;
    }
    pub fn get_cell_size_string(&mut self) -> &mut String {
        return &mut self.cell_size_string;
    }
    pub fn get_current_turn_string(&mut self) -> &mut String {
        return &mut self.current_turn_string;
    }
    pub fn get_cell_by_position(&mut self, position:Vec<i32>) -> &mut Cell {
        return self.cell_grid[position[0] as usize].get_mut(position[1] as usize).unwrap();
    }
    pub fn get_transfer_apt_amount_string(&mut self) -> &mut String{
        return &mut self.transfer_apt_amount_string;
    }
    pub fn get_cell_grid(self) -> Vec<Vec<Cell>> {
        return self.cell_grid;
    }
    pub fn sync_cell_grid(&mut self, local_cell_grid:Vec<Vec<Cell>>) {
        self.cell_grid = local_cell_grid;
    }
}
use std::{env, fmt::Display};

use arraystring::{ArrayString, typenum::U200};

type PlayerName = ArrayString<U200>;
type CellImage = ArrayString<U200>;

#[derive(Clone)]
pub struct Cell {
    player: PlayerName,
    player_number: i32,
    previous_cell_type: CellType,
    cell_type: CellType,
    cell_image: CellImage,
    cell_action: CellAction,
    cell_hp: i32,
    cell_atp: i32,
    cell_attack_range: i32,
}
impl Default for Cell {
    fn default() -> Self {
        Cell::new("Nobody".to_owned())
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum CellType {
    EmptyCell,
    PlacableCell,
    StemCell,
    WorkerCell,
    AttackingCell,
    Null,
    SelectedCell
}
impl TryFrom<i32> for CellType {
    type Error = ();

    fn try_from(v: i32) -> Result<Self, Self::Error> {
        match v {
            0 if v == CellType::EmptyCell as i32 => Ok(CellType::EmptyCell),
            1 if v == CellType::PlacableCell as i32 => Ok(CellType::PlacableCell),
            2 if v == CellType::StemCell as i32 => Ok(CellType::StemCell),
            3 if v == CellType::WorkerCell as i32 => Ok(CellType::WorkerCell),
            4 if v == CellType::AttackingCell as i32 => Ok(CellType::AttackingCell),
            5 if v == CellType::Null as i32 => Ok(CellType::Null),
            6 if v == CellType::SelectedCell as i32 => Ok(CellType::SelectedCell),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum CellAction {
    NoAction,
    MoveCell,
    DuplicateCell,
    Attacked,
    StartingPlace,
    TransferTo,
}
impl Display for CellAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let action_string: &str;
        match self {
            CellAction::NoAction => action_string = "No Action",
            CellAction::MoveCell => action_string = "Move Cell",
            CellAction::DuplicateCell => action_string = "Duplicate Cell",
            CellAction::Attacked => action_string = "Attacked",
            CellAction::StartingPlace => action_string = "Starting Place",
            CellAction::TransferTo => action_string = "Transfer To",
        }
        write!(f, "{}", action_string)
    }
}

impl Cell {
    //static methods
    pub fn new(player_1_name:String) -> Cell {
        Self {
            player: PlayerName::try_from_str(player_1_name).unwrap(),
            player_number: 0,
            cell_action: CellAction::StartingPlace,
            previous_cell_type: CellType::Null,
            cell_type: CellType::PlacableCell,
            cell_image: CellImage::try_from_str(Self::cell_type_to_image(CellType::PlacableCell,).as_str()).unwrap_or(CellImage::new()),
            cell_hp: 0,
            cell_atp: 0,
            cell_attack_range:0
        }
    }

    fn cell_type_to_image(new_cell_type:CellType) -> String {
        let current_dir = env::current_dir();
        if current_dir.is_err(){
            return String::from("Null");
        }
        let binding = current_dir.unwrap();
        let dir_string = binding.as_os_str().to_str().unwrap().to_owned();
        match new_cell_type {
            CellType::StemCell => return String::from("file://".to_owned() + &dir_string + "/src/assets/Base_Stem_Cell.jpg"),
            CellType::AttackingCell => return String::from("file://".to_owned() + &dir_string + "/src/assets/Base_Attacking_Cell.jpg"),
            CellType::WorkerCell => return String::from("file://".to_owned() + &dir_string + "/src/assets/Base_Worker_Cell.jpg"),
            CellType::EmptyCell => return String::from("file://".to_owned() + &dir_string + "/src/assets/white.png"),
            CellType::PlacableCell => return String::from("file://".to_owned() + &dir_string + "/src/assets/Placable.jpg"),
            _ => return String::from("Not_implemented"),
        }
    }
  
    pub fn get_surrounding_cells(cell_grid_size:i32,position:Vec<usize>) -> Vec<Vec<i32>>{
        let all_positions:Vec<Vec<i32>> = vec![
            vec![-1,-1],vec![0,-1],vec![1,-1],    // 0, 1 ,2
            vec![-1,0], vec![0,0], vec![1,0],    // 3, 4 ,5
            vec![-1,1], vec![0,1], vec![1,1], // 6, 7 ,8
        ];
        let mut removed_positions:Vec<i8> = vec![];
        if position[0] == 0 {
            removed_positions.push(0);
            removed_positions.push(3);
            removed_positions.push(6);
        }
        if position[0] == (cell_grid_size as usize)-1 {
            removed_positions.push(2);
            removed_positions.push(5);
            removed_positions.push(8);
        }
        if position[1] == 0 {
            removed_positions.push(0);
            removed_positions.push(1);
            removed_positions.push(2);
        }
        if position[1] == (cell_grid_size as usize)-1 {
            removed_positions.push(6);
            removed_positions.push(7);
            removed_positions.push(8);
        }

        let mut final_positions:Vec<Vec<i32>> = vec![];
        for ind in 0..9 {
            if !removed_positions.contains(&ind) {
                final_positions.push(all_positions[ind as usize].clone());
            }
        }
        return final_positions;
    }
    

    //Cell specific methods

    //changer methods
    pub fn replace_cell(&mut self, new_cell_type:CellType) {
        self.previous_cell_type.clone_from(&self.cell_type);
        self.cell_type = new_cell_type;
        self.cell_image = CellImage::try_from_str(Self::cell_type_to_image(new_cell_type)).unwrap_or(CellImage::new());
        match new_cell_type{
            CellType::EmptyCell => self.cell_attack_range = 0,
            CellType::PlacableCell => self.cell_attack_range = 0,
            CellType::StemCell => self.cell_attack_range = 2,
            CellType::WorkerCell => self.cell_attack_range = 0,
            CellType::AttackingCell => self.cell_attack_range = 3,
            CellType::Null => self.cell_attack_range = 0,
            CellType::SelectedCell => self.cell_attack_range = 0,
        }
    }
    pub fn set_hp(&mut self, hp:i32){
        self.cell_hp = hp;
    }
    pub fn decrease_hp(&mut self, amount:i32) {
        self.cell_hp -= amount;
    }
    pub fn increase_hp(&mut self, amount:i32) {
        self.cell_atp += amount;
    }
    pub fn set_apt(&mut self, apt:i32){
        self.cell_atp = apt;
    }
    pub fn decrease_apt(&mut self, amount:i32) {
        self.cell_atp -= amount;
    }
    pub fn increase_apt(&mut self, amount:i32) {
        self.cell_atp += amount;
    }
    pub fn change_player(&mut self, new_player:String, player_id:i32){
        self.player = PlayerName::try_from_str(new_player).unwrap_or(PlayerName::from_str_truncate("Player Name Change Failure"));
        self.player_number = player_id;
    }
    pub fn reset_cell(&mut self){
        self.player = PlayerName::try_from_str("Nobody").unwrap();
        self.cell_action = CellAction::NoAction;
        self.player_number = 15;
        self.cell_hp = 0;
        self.cell_atp = 0;
        self.replace_cell(CellType::EmptyCell);
    }
    pub fn revert_cell(&mut self){
        self.cell_action = CellAction::NoAction;
        self.replace_cell(self.previous_cell_type.clone());
    }
    pub fn set_cell_action(&mut self, new_action:CellAction){
        self.cell_action = new_action;
    }
    pub fn replace_with_cell(&mut self, cell_to_copy:Cell){
        self.player = cell_to_copy.player.clone();
        self.player_number = cell_to_copy.get_cell_player_number();
        self.replace_cell(cell_to_copy.cell_type.clone());
        self.cell_action = CellAction::NoAction;
        self.set_hp(cell_to_copy.get_cell_hp());
        self.set_apt(cell_to_copy.get_cell_atp());
    }



    //get methods
    pub fn get_attack_range(self, cell_grid_size:i32, position:Vec<usize>) -> Vec<Vec<i32>>{
        let mut valid_positions = vec![];
        let left_limit:i32 = if (position[0] - self.cell_attack_range as usize) >= 0 
        {position[0] as i32 - self.cell_attack_range} else {0};
        let right_limit:i32 = if (position[0] + self.cell_attack_range as usize) <= (cell_grid_size-1) as usize 
        {position[0] as i32 + self.cell_attack_range} else {cell_grid_size-1};
        
        let upper_limit:i32 = if (position[1] + self.cell_attack_range as usize) <= (cell_grid_size-1) as usize 
        {position[1] as i32 + self.cell_attack_range} else {cell_grid_size-1};
        let lower_limit:i32 = if (position[1] - self.cell_attack_range as usize) >= 0
        {position[1] as i32 - self.cell_attack_range} else {0};


        for x in left_limit..right_limit+1 {
        for y in lower_limit..upper_limit+1 {
            valid_positions.push(vec![x,y]);
        }
        }

        return  valid_positions;
    }

    pub fn get_raw_cell_type(self) -> CellType {
        return self.cell_type;
    }
    pub fn get_cell_type(self) -> String {
        match self.cell_type {
            CellType::EmptyCell => "Empty Cell".to_owned(),
            CellType::PlacableCell => "Placable Cell".to_owned(),
            CellType::StemCell => "Stem Cell".to_owned(),
            CellType::WorkerCell => "Worker Cell".to_owned(),
            CellType::AttackingCell => "Attacking Cell".to_owned(),
            CellType::Null => "Null Cell".to_owned(),
            CellType::SelectedCell => "Slected Cell Cell".to_owned(),
        }
    }
    pub fn get_previous_cell_type(self) -> String {
        match self.previous_cell_type {
            CellType::EmptyCell => "Empty Cell".to_owned(),
            CellType::PlacableCell => "Placable Cell".to_owned(),
            CellType::StemCell => "Stem Cell".to_owned(),
            CellType::WorkerCell => "Worker Cell".to_owned(),
            CellType::AttackingCell => "Attacking Cell".to_owned(),
            CellType::Null => "Null Cell".to_owned(),
            CellType::SelectedCell => "Slected Cell Cell".to_owned(),
        }
    }
    pub fn get_cell_player(self) -> String {
        return self.player.as_str().to_owned();
    }
    pub fn get_cell_player_number(self) -> i32 {
        return self.player_number;
    }
    pub fn get_cell_hp(self) -> i32 {
        return self.cell_hp;
    }
    pub fn get_cell_image(self) -> CellImage{
        return self.cell_image;
    }
    pub fn get_cell_atp(self) -> i32{
        return self.cell_atp;
    }
    pub fn get_cell_action(self) -> CellAction{
        return self.cell_action;
    }
    pub fn is_alive(self) -> bool {
        if self.cell_type == CellType::EmptyCell {return false;}
        if self.cell_type == CellType::PlacableCell {return false;}
        if self.cell_hp <= 0 {return  false;}
        return  true;
    }
}


impl Copy for Cell {}

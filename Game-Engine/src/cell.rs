use std::env;

use arraystring::{ArrayString, typenum::U200};

type PlayerName = ArrayString<U200>;
type CellImage = ArrayString<U200>;

#[derive(Clone)]
pub struct Cell {
    player: PlayerName,
    player_number: u8,
    previous_cell_type: CellType,
    cell_type: CellType,
    cell_image: CellImage,
    cell_hp:u8,
    cell_apt:u8,
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


impl Cell {
    //static methods
    pub fn new() -> Cell {
        Self {
            player: PlayerName::try_from_str("Nobody").unwrap(),
            player_number: 15,
            previous_cell_type: CellType::Null,
            cell_type: CellType::EmptyCell,
            cell_image: CellImage::try_from_str(Self::cell_type_to_image(CellType::EmptyCell,).as_str()).unwrap_or(CellImage::new()),
            cell_hp: 5,
            cell_apt: 1,
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

    pub fn get_surrounding_cells(cell_grid_size:u32,position:Vec<usize>) -> Vec<Vec<i8>>{
        let all_positions:Vec<Vec<i8>> = vec![
            vec![-1,1],vec![0,1],vec![1,1],    // 0, 1 ,2
            vec![-1,0],vec![0,0],vec![1,0],    // 3, 4 ,5
            vec![-1,-1],vec![0,-1],vec![1,-1], // 6, 7 ,8
        ];
        let mut removed_positions:Vec<u8> = vec![];
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

        let mut final_positions:Vec<Vec<i8>> = vec![];
        for ind in 0..9 {
            if !removed_positions.contains(&(ind as u8)) {
                final_positions.push(all_positions[ind].clone());
            }
        }
        return final_positions;
    }


    //Cell specific methods

    //changer methods
    pub fn replace_cell(&mut self, new_cell_type:CellType) {
        self.previous_cell_type.clone_from(&self.cell_type);
        self.cell_image = CellImage::try_from_str(Self::cell_type_to_image(new_cell_type)).unwrap_or(CellImage::new());
    }
    pub fn set_hp(&mut self, hp:u8){
        self.cell_hp = hp;
    }
    pub fn decrease_hp(&mut self, amount:u8) {
        self.cell_apt -= amount;
    }
    pub fn increase_hp(&mut self, amount:u8) {
        self.cell_apt += amount;
    }
    pub fn set_apt(&mut self, hp:u8){
        self.cell_apt = hp;
    }
    pub fn decrease_apt(&mut self, amount:u8) {
        self.cell_hp -= amount;
    }
    pub fn increase_apt(&mut self, amount:u8) {
        self.cell_hp += amount;
    }
    pub fn change_player(&mut self, new_player:String, player_id:u8){
        self.player = PlayerName::try_from_str(new_player).unwrap_or(PlayerName::from_str_truncate("Player Name Change Failure"));
        self.player_number = player_id;
    }
    pub fn reset_cell(&mut self){
        self.player = PlayerName::try_from_str("Nobody").unwrap();
        self.player_number = 15;
        self.cell_hp = 0;
        self.cell_apt = 0;
        self.replace_cell(CellType::EmptyCell);
    }
    



    //get methods
    
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
    pub fn get_cell_player_number(self) -> u8 {
        return self.player_number;
    }
    pub fn get_cell_hp(self) -> u8 {
        return self.cell_hp;
    }
    pub fn get_cell_image(self) -> CellImage{
        return self.cell_image;
    }
    pub fn get_cell_apt(self) -> u8{
        return self.cell_apt;
    }
    pub fn is_alive(self) -> bool {
        if self.cell_type == CellType::EmptyCell {return false;}
        if self.cell_type == CellType::PlacableCell {return false;}
        if self.cell_hp <= 0 {return  false;}
        return  true;
    }
}


impl Copy for Cell {}

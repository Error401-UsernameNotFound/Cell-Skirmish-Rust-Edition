use egui::Widget;


pub struct Cell {
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
            None => return String::from("white.jpg"),
            _ => return String::from("Not_implemented")
        }
    }
    pub fn get_cell_image(self) -> String{
        return self.cell_image;
    }
}



impl<T> Clone for &Cell {
    fn clone(&self) -> Self {
        return &Cell {
            player: self.player.clone(),
            cell_type: self.cell_type,
            cell_image: self.cell_image,
        };
    }
    
    fn clone_from(&mut self, source: &Self) {
        *self = source.clone()
    }
}

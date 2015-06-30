use traits::{Describable};
use items;
use mazepath;

pub struct Player<'a: 'b, 'b> {
    name: String,
    keys: Vec<items::Key>,
    items: Vec<items::Item>,
    pub pos: Option<&'b mut mazepath::MazePath<'a>>,
}

impl<'a: 'b, 'b> Player<'a, 'b> {
    pub fn new(name: String ) -> Player<'a, 'b> {
        Player{ name: name, keys: vec![], items: vec![], pos: Option::None }
    }

   pub fn traverse(&'a mut self, next_room: Option<&'a mut mazepath::MazePath<'a>>) {
       // CURRENTLY NOT MAINTAINING PREVIOUS ROOM
       /*match self.pos {
           None => {
               self.previous_room = None;
           }
           Some(ref mut r) => {
               self.previous_room = Some(r);
           }
       }*/
       self.pos = next_room;

    }

    pub fn add_key(&mut self, key: items::Key) {
        self.keys.push(key); 
    }

    pub fn add_item(&mut self, item: items::Item) {
        self.items.push(item);
    }

    pub fn list_keys(&self) {
        let _: Vec<_> = self.keys.iter().map(|k| k.print_name()).collect();        
    }

    pub fn list_items(&self) {
        let _: Vec<_> = self.items.iter().map(|i| i.print_name()).collect();
    }

    // Lists both items and keys, may be useful for the UI
    pub fn list_inventory(&self) {
        println!("Keys held: ");
        self.list_keys();
        println!("Items in inventory: ");
        self.list_items();
    }
}

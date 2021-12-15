pub mod item;
use item::Item;

#[derive(Debug)]
pub struct ShoppingCart {
  items: Vec<Item>
}

impl ShoppingCart {
  pub fn new() -> Self {
    ShoppingCart {
      items: Vec::new(),
    }
  }

  pub fn cart_length(&self) -> usize {
    self.items.len()
  }

  pub fn add_item(&mut self, item: Item) {
    self.items.push(item);
  }

  pub fn remove_item(&mut self, id: String) {
    let mut index = 0;
    let mut found = false;
    for i in 0..self.items.len() {
      if self.items[i].get_id() == id {
        index = i;
        found = true;
        break;
      }
    }

    if found {
      self.items.remove(index);
    } else {
      println!("Não foi possível encontrar um item com esse id!")
    }
  }

  pub fn calculate_price(&mut self) -> f32 {
    let mut total = 0.0;
    for item in &self.items {
      total += item.get_price();
    }
    total
  }

  pub fn show_cart(&self) {
    println!("{}", format!("{:=^60}", " CART ITEMS "));
    println!("{}", format!("{:<10} | {:<10} | {:<30}", "Nome", "Preco", "Id"));
    println!("{}", format!("{:-^60}", ""));
    for item in &self.items {
      println!("{}", format!("{:<10} | R${:<8.2} | {:>30}", item.get_name(), item.get_price(), item.get_id()));
    }
    println!("{}", format!("{:=^60}", ""));
  }

  
  
}
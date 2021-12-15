use uuid::Uuid;

#[derive(Debug)]
pub struct Item {
  id: String,
  name: String,
  price: f32,
}

impl Item {
  pub fn new(name: String, price: f32) -> Self {
    Item {
      id: Uuid::new_v4().to_string(),
      name,
      price
    }
  }

  pub fn get_id(&self) -> String {
    self.id.clone()
  }

  pub fn get_name(&self) -> String {
    self.name.clone()
  }

  pub fn get_price(&self) -> f32 {
    self.price.clone()
  }

  pub fn copy(&self) -> Self {
    Item {
      id: self.get_id(),
      name: self.get_name(),
      price: self.get_price(),
    }
  }
}
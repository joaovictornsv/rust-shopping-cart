mod cart;
use cart::item::Item;
use cart::ShoppingCart;

fn main() {
    let mut cart = ShoppingCart::new();
    let item = Item::new(String::from("Arroz"), 10.0);
    let item2 = Item::new(String::from("Carne"), 40.0);
    let item3 = Item::new(String::from("Feijão"), 12.0);

    cart.add_item(item.copy());
    cart.add_item(item2.copy());
    cart.add_item(item3.copy());
    cart.remove_item(item2.get_id());
    cart.show_cart();
    
    println!("Total de items: {:?}", cart.cart_length());
    println!("Total: R${:?}", cart.calculate_price());
}

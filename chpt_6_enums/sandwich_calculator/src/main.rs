struct Sandwich {
    bread: Option<Bread>,
    meat: Option<Meat>,
    veggie: Option<Veggie>,
    cheese: Option<Cheese>,
}
impl Sandwich {
    fn get_price(&self) -> f64 {
        let price = match self.bread {
            Some(Bread::Bagel) => 2.00,
            Some(Bread::White) |
            Some(Bread::WholeWheat) |
            Some(Bread::Rye) => 1.5,
            None => 0.0
        } + match self.meat {
            Some(Meat::Beef) => 0.5,
            Some(Meat::Ham) |
            Some(Meat::Turkey) => 0.25,
            Some(Meat::Bacon) => 1.00,
            None => 0.0
        } + match self.cheese {
            Some(Cheese::American) |
            Some(Cheese::Cheddar) => 0.25,
            Some(Cheese::Provolone) |
            Some(Cheese::Swiss) => 0.5,
            None => 0.0
        } + match self.veggie {
            Some(_) => 0.5,
            None => 0.0
        };
        return price;
    } 
}

enum Meat {
    Ham,
    Turkey,
    Beef,
    Bacon,
}

enum Bread {
    White,
    WholeWheat,
    Rye,
    Bagel,
}

enum Veggie {
    Lettuce,
    Tomato,
    Onion,
}

enum Cheese {
    Swiss,
    Provolone,
    Cheddar,
    American,
}

fn main() {
    let sandwich1 = Sandwich {
        bread: Some(Bread::Rye),
        meat: Some(Meat::Turkey),
        cheese: Some(Cheese::Swiss),
        veggie: Some(Veggie::Lettuce),
    };

    let sandwich2 = Sandwich {
        bread: Some(Bread::Bagel),
        meat: Some(Meat::Ham),
        cheese: Some(Cheese::Provolone),
        veggie: Some(Veggie::Tomato),
    };

    println!(
        "Sandwich 1 costs ${}",
        sandwich1.get_price()
    );

    println!(
        "Sandwich 2 costs ${}",
        sandwich2.get_price()
    );
}

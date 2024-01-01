use std::{ops::{Mul, Add}, iter::Sum, cmp::max};

#[derive(Copy,Clone,PartialEq,Eq,Debug)]
enum IngredientName {
    Sprinkles,
    Butterscotch,
    Chocolate,
    Candy,
    Cinnamon,
    None
}

#[derive(Copy,Clone,PartialEq,Eq,Debug)]
struct Ingredient {
    name: IngredientName,
    capacity: i64,
    durability: i64,
    flavor: i64,
    texture: i64,
    calories: i64
}

impl Ingredient {
    fn zero() -> Self {
        Self {
            name: IngredientName::None,
            capacity: 0,
            durability: 0,
            flavor: 0,
            texture: 0,
            calories: 0,
        }
    }

    fn score(&self) -> (i64, i64) {
        (max(0, self.capacity) * max(0, self.durability) * max(0, self.flavor) * max(0, self.texture)
            , self.calories)
    }

}

impl Mul<i64> for Ingredient {
    type Output = Ingredient;

    fn mul(self, rhs: i64) -> Self::Output {
        Ingredient {
            name: self.name,
            capacity: self.capacity * rhs,
            durability: self.durability * rhs,
            flavor: self.flavor * rhs,
            texture: self.texture * rhs,
            calories: self.calories * rhs,
        }
    }
}

impl Add<Ingredient> for Ingredient {
    type Output = Ingredient;

    fn add(self, rhs: Ingredient) -> Self::Output {
        Ingredient {
            name: self.name,
            capacity: self.capacity + rhs.capacity,
            durability: self.durability + rhs.durability,
            flavor: self.flavor + rhs.flavor,
            texture: self.texture + rhs.texture,
            calories: self.calories + rhs.calories,
        }
    }
}

fn main() {
    use IngredientName::*;
    let mut testdata = [
        Ingredient { name: Butterscotch, capacity: -1, durability: -2, flavor: 6, texture: 3, calories: 8 },
        Ingredient { name: Cinnamon, capacity: 2, durability: 3, flavor: -2, texture: -1, calories: 3 }];

    let mut ingredients = [
        Ingredient { name: Sprinkles, capacity: 2, durability: 0, flavor: -2, texture: 0, calories: 3 },
        Ingredient { name: Butterscotch, capacity: 0, durability: 5, flavor: -3, texture: 0, calories: 3 },
        Ingredient { name: Chocolate, capacity: 0, durability: 0, flavor: 5, texture: -1, calories: 8 },
        Ingredient { name: Candy, capacity: 0, durability: -1, flavor: 0, texture: 5, calories: 8 }];

    let mut best1 = 0;
    let mut best2 = 0;
    for a in 0..=100 {
        for b in 0..=(100-a) {
            for c in 0..=(100-(a+b)) {
                let d = 100-(a+b+c);
                let amounts = [a,b,c,d];
                let (score,calories) = ingredients.iter().zip(amounts).map(|(i,a)| *i * a).fold(Ingredient::zero(), |a,b| a + b).score();
                // let (score, calories) = rate_recipe(&[a,b,c,d], &ingredients);
                if score > best1 {
                    best1 = score
                }
                if calories == 500 && score > best2 {
                    best2 = score
                }}
    }
        }
    println!("Part 1: {best1}");
    println!("Part 2: {best2}");
}

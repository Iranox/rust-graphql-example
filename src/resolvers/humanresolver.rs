use crate::entity::human::{Human};


pub fn to_human(_id: String) -> Human {
    Human::new()
}

pub fn get_all_human(n:i32) -> Vec<Human> {
    let mut result:Vec<Human> = Vec::new();
    for _i in 0..n{
        result.push(to_human(n.to_string()));
    }
    result
}
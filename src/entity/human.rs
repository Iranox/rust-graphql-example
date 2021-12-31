use juniper::{GraphQLEnum, GraphQLInputObject, GraphQLObject};


#[derive(GraphQLEnum)]
enum Episode {
    NewHope,
    Empire,
    Jedi,
}


#[derive(GraphQLObject)]
#[graphql(description = "A humanoid creature in the Star Wars universe")]
pub struct Human {
    id: String,
    name: String,
    appears_in: Vec<Episode>,
    home_planet: String,
}

impl Human {
    pub fn new()->Human{
        Human {
            id: "1234".to_owned(),
            name: "Luke".to_owned(),
            appears_in: vec![Episode::NewHope],
            home_planet: "Mars".to_owned(),
        }    }
}

#[derive(GraphQLInputObject)]
#[graphql(description = "A humanoid creature in the Star Wars universe")]
pub struct NewHuman {
    name: String,
    appears_in: Vec<Episode>,
    home_planet: String,
}

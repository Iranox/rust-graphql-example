use juniper::FieldResult;
use juniper::{EmptySubscription, RootNode};
use crate::resolvers::humanresolver;
use crate::entity::human::{NewHuman, Human};


pub struct QueryRoot;

#[juniper::graphql_object]
impl QueryRoot {
    fn human(_id: String) -> FieldResult<Human> {
        Ok(humanresolver::to_human(_id))
    }
    fn all_human(n:i32) -> FieldResult<Vec<Human>>{
        Ok(humanresolver::get_all_human(n))
    }
}

pub struct MutationRoot;

#[juniper::graphql_object]
impl MutationRoot {
    fn create_human(_new_human: NewHuman) -> FieldResult<Human> {
        Ok(humanresolver::to_human("1".to_owned()))
    }

}

pub type Schema = RootNode<'static, QueryRoot, MutationRoot, EmptySubscription>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, MutationRoot, EmptySubscription::new())
}
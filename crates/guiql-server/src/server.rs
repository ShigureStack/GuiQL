use guiql::{
    helper::{Query, QueryResult},
    lang::parser::Parser,
};
use guiql::{Context, Event, HandleEvent, HasContext};

pub struct CatcherServer {
    root: Context,
}

impl CatcherServer {
    pub fn new<T>(app: T) -> Self
    where
        T: HasContext + HandleEvent,
    {
        CatcherServer {
            root: app.context(),
        }
    }

    fn eval_global(&self, query: &Query) -> QueryResult {
        let ast = Parser::from_str(query.0).parse_all();
    }

    pub fn run() {

    }
}

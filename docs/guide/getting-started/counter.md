# Implementing a counter app

main.rs
```rs
use guiql::{Event, Context};
use catcher::CatcherServer;

struct Counter {
    count: i32,
    context: Context,
}

impl Counter {
    pub fn new() -> Self {
        let context = Context::new();

        context.query().anchor("@").create("""
            Div@container {
                stack = horizontial,

                Label@label '0'
                Button@count-button '+'
            }
        """);
        context.query().anchor("count-button").subscribe("click", "count");

        Counter { count: 0, context }
    }

    pub fn count(&mut self) {
        self.count += 1;
        self.context.query().anchor("label").replace_content(fortmat!("{self.count}"));
    }
}

impl HasContext for CounterApp {
    fn context(&mut self) -> Context {
        self.context
    }
}

impl HandleEvent for Counter {
    fn handle_event(&mut self, event: Event) {
        match event {
            Event("count") => self.count(),
            _ => {},
        }
    }
}

struct CounterApp {
    counters: Vec<Counter>,
    context: Countext,
}

impl CounterApp {
    pub fn add_counter(&mut self, counter: Counter) {
        self.counters.push(counter);
        self.context.query().anchor("container").push(counter);
    }

    pub fn new() -> Self {
        let context = Context::new();

        context.query().anchor("@").create("Div@container {}");
        context.query().anchor("container").create("Button@add-button 'Add counter'");
        context.query().anchor("add-button").subscribe("click", "add");

        CounterApp {
            counters: vec![],
            context,
        }
    }
}

impl HasContext for CounterApp {
    fn context(&mut self) -> Context {
        self.context
    }
}

impl HandleEvent for CounterApp {
    fn handle_event(&mut self, event: Event) {
        match event {
            Event("add") => self.add_counter(Counter::new()),
            _ => {}
        }
    }
}

fn main() {
    let mut app = CounterApp::new();

    CatcherServer::new(app).run();
}
```

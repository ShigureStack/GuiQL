pub struct Event(&str);

pub trait HandleEvent {
    fn handle_event(&mut self, event: Event);
}

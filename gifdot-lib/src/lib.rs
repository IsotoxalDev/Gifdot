mod encoder;

use gdnative::prelude::*;
use encoder::GifdotEncoder;

#[derive(NativeClass)]
#[user_data(user_data::RwLockData<Gifdot>)]
#[inherit(Node)]
pub struct Gifdot;

impl Gifdot {
    fn new(_owner: &Node) -> Self {
        Gifdot
    }
}

#[methods]
impl Gifdot {
    #[export]
    fn get_encoder(&self, _owner: &Node, width: u16, height: u16) -> Instance<GifdotEncoder, Unique> {
        GifdotEncoder::new(width, height).emplace()
    }
}

// Init Stuff

fn init(handle: InitHandle) {
    handle.add_class::<Gifdot>();
    handle.add_class::<GifdotEncoder>();
}


godot_init!(init);
use godot::prelude::*;

use crate::print_thread;

#[derive(GodotClass)]
#[class(base=Resource, init)]
pub struct MockResource;

#[derive(GodotClass)]
#[class(base=Resource)]
pub struct WithNested {
    #[export]
    nested: Gd<Nested>,
}

#[godot_api]
impl IResource for WithNested {
    fn init(_base: godot::obj::Base<Self::Base>) -> Self {
        print_thread("WithNested", "init");
        Self {
            nested: Gd::<Nested>::default(),
        }
    }
}

#[derive(GodotClass)]
#[class(base=Resource)]
pub struct Nested;

#[godot_api]
impl IResource for Nested {
    fn init(_base: godot::obj::Base<Self::Base>) -> Self {
        print_thread("Nested", "init");
        Self {}
    }
}

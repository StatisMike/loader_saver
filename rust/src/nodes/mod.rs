use godot::{engine::DirAccess, prelude::*};

use crate::{print_thread, print_thread_custom, resources::{MockResource, WithNested}};

#[derive(GodotClass)]
#[class(base=Node,init,tool)]
pub struct InEditorNode {
  base: Base<Node>
}

#[godot_api]
impl INode for InEditorNode {
    fn ready(&mut self,) {
        print_thread("ToolNode", "enter_tree");

        print_thread_custom("Saving MockResource from EDITOR node");
        godot::engine::save(Gd::<MockResource>::default(), "res://test_resource_editor.mock");

        print_thread_custom("Loading MockResource from EDITOR node");
        let _res = godot::engine::load::<MockResource>("res://test_resource_editor.mock");

        DirAccess::open("res://".into()).unwrap().remove("test_resource_editor.mock".into());

        // self.base_mut().queue_free();
    }
}

#[derive(GodotClass)]
#[class(base=Node)]
pub struct TestNode {
  base: Base<Node>,
  #[export]
  mock: Gd<MockResource>,
  #[export]
  nested: Gd<WithNested>
}

#[godot_api]
impl INode for TestNode {
  fn init(base: godot::obj::Base < Self::Base >) -> Self {
      Self {
        base,
        mock: Gd::<MockResource>::default(),
        nested: Gd::<WithNested>::default()
      }
  }

  fn ready(&mut self,) {
    print_thread("TestNode", "enter_tree");

    print_thread_custom("Saving MockResource from SCENE node");
    godot::engine::save(Gd::<MockResource>::default(), "res://test_resource_scene.mock");

    print_thread_custom("Loading MockResource from SCENE node");
    let _res = godot::engine::load::<MockResource>("res://test_resource_scene.mock");

    DirAccess::open("res://".into()).unwrap().remove("test_resource_scene.mock".into());

    self.base().get_tree().unwrap().quit();
  }
}
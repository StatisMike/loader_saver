use godot::{
    engine::{Engine, Os},
    prelude::*,
};
use saver_loader::MySingleton;

pub(crate) mod resources;
mod saver_loader;
mod nodes;

// GDEXTENSION
struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {
    fn on_level_init(level: InitLevel) {
        if level == InitLevel::Scene {
            let singleton = MySingleton::new_alloc();
            singleton.bind().register_saver_loader();

            Engine::singleton()
                .register_singleton(MySingleton::SINGLETON_NAME.into(), singleton.upcast());
        }
    }

    fn on_level_deinit(level: InitLevel) {
        if level == InitLevel::Scene {
            let mut engine = Engine::singleton();

            let singleton = engine
                .get_singleton(MySingleton::SINGLETON_NAME.into())
                .unwrap();

            engine.unregister_singleton(MySingleton::SINGLETON_NAME.into());

            singleton
                .clone()
                .cast::<MySingleton>()
                .bind()
                .unregister_saver_loader();

            singleton.free();
        }
    }
}

// UTILS

pub(crate) fn print_thread(class: &str, method: &str) {
    let thread_id = Os::singleton().get_thread_caller_id();

    godot_print!("Thread: {thread_id}: {class}::{method}");
}

pub(crate) fn print_thread_custom(message: &str) {
    let thread_id = Os::singleton().get_thread_caller_id();

    godot_print!("Thread: {thread_id}: {message}");
}


use godot::{
    engine::{
        file_access::ModeFlags,
        global::Error,
        Engine, IResourceFormatLoader, IResourceFormatSaver, Os, ResourceLoader,
        ResourceSaver, ResourceUid,
    },
    prelude::*,
};

#[derive(GodotClass)]
#[class(base=Resource, init)]
pub struct MockResource;

#[derive(GodotClass)]
#[class(base=ResourceFormatLoader, init, tool)]
pub struct MyLoader;

#[godot_api]
impl IResourceFormatLoader for MyLoader {
    fn get_recognized_extensions(&self) -> PackedStringArray {
        print_thread("ResourceFormatLoader", "get_recognized_extensions");
        PackedStringArray::from(&[GString::from("mock")])
    }
    fn handles_type(&self, type_: StringName) -> bool {
        print_thread("ResourceFormatLoader", "handles_type");
        type_.to_string().eq("MockResource")
    }
    fn get_resource_type(&self, path: GString) -> GString {
        print_thread("ResourceFormatLoader", "get_resource_type");
        if path.to_string().ends_with("mock") {
            return GString::from("MockResource");
        }
        GString::new()
    }
    fn get_resource_uid(&self, path: GString) -> i64 {
        print_thread("ResourceFormatLoader", "get_resource_uid");
        if !&path.to_string().ends_with("mock") {
            return -1;
        }
        let mut gfile = GFile::open(path, ModeFlags::READ).unwrap();
        gfile
            .read_pascal_string()
            .unwrap()
            .to_string()
            .parse::<i64>()
            .unwrap()
    }
    fn get_dependencies(&self, _path: GString, _add_types: bool) -> PackedStringArray {
        print_thread("ResourceFormatLoader", "get_dependencies");
        PackedStringArray::new()
    }
    fn rename_dependencies(&self, _path: GString, _renames: Dictionary) -> Error {
        print_thread("ResourceFormatLoader", "rename_dependencies");
        Error::OK
    }
    fn load(
        &self,
        path: GString,
        _original_path: GString,
        _use_sub_threads: bool,
        _cache_mode: i32,
    ) -> Variant {
        print_thread("ResourceFormatLoader", "load");
        if !&path.to_string().ends_with("mock") {
            return Variant::nil();
        }
        MockResource::new_gd().to_variant()
    }
}

#[derive(GodotClass)]
#[class(base=ResourceFormatSaver, init, tool)]
pub struct MySaver;

#[godot_api]
impl IResourceFormatSaver for MySaver {
    fn save(&mut self, _resource: Gd<Resource>, path: GString, _flags: u32) -> Error {
        print_thread("ResourceFormatSaver", "save");
        let mut resource_uid = ResourceUid::singleton();

        let uid = resource_uid.create_id();

        GFile::open(path, ModeFlags::WRITE)
            .unwrap()
            .write_pascal_string(uid.to_string())
            .unwrap();

        Error::OK
    }
    fn set_uid(&mut self, path: GString, uid: i64) -> Error {
        print_thread("ResourceFormatSaver", "set_uid");
        GFile::open(path, ModeFlags::WRITE)
            .unwrap()
            .write_pascal_string(uid.to_string())
            .unwrap();

        Error::OK
    }
    fn recognize(&self, resource: Gd<Resource>) -> bool {
        print_thread("ResourceFormatSaver", "recognize");
        resource.get_class().eq(&GString::from("MockResource"))
    }
    fn get_recognized_extensions(&self, resource: Gd<Resource>) -> PackedStringArray {
        print_thread("ResourceFormatSaver", "get_recognized_extensions");
        if resource.get_class().eq(&GString::from("MockResource")) {
            PackedStringArray::from(&[GString::from("mock")])
        } else {
            PackedStringArray::new()
        }
    }
}

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

// SINGLETON (for unregistering)

#[derive(GodotClass)]
#[class(base=Object, init)]
pub struct MySingleton {
    loader: Gd<MyLoader>,
    saver: Gd<MySaver>,
}

impl MySingleton {
    const SINGLETON_NAME: &str = "MySingleton";

    fn register_saver_loader(&self) {
        ResourceLoader::singleton().add_resource_format_loader(self.loader.clone().upcast());

        ResourceSaver::singleton().add_resource_format_saver(self.saver.clone().upcast());
    }

    fn unregister_saver_loader(&self) {
        ResourceLoader::singleton().remove_resource_format_loader(self.loader.clone().upcast());

        ResourceSaver::singleton().remove_resource_format_saver(self.saver.clone().upcast());
    }
}

// UTIL

fn print_thread(class: &str, method: &str) {
  let thread_id = Os::singleton().get_thread_caller_id();

  godot_print!("Thread: {thread_id}: {class}::{method}")
}
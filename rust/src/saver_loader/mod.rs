use godot::{
    engine::{
        file_access::ModeFlags, global::Error, IResourceFormatLoader, IResourceFormatSaver,
        ResourceLoader, ResourceSaver, ResourceUid,
    },
    prelude::*,
};

use crate::{print_thread, resources::MockResource};

#[derive(GodotClass)]
#[class(base=ResourceFormatLoader, init, tool)]
pub struct MyLoader;

#[godot_api]
impl IResourceFormatLoader for MyLoader {
    fn get_recognized_extensions(&self) -> PackedStringArray {
      // This gets called from other threads ALWAYS when running Godot project on Editor level.
      // $godot --headless -e --quit
      // $godot -e
        print_thread("ResourceFormatLoader", "get_recognized_extensions");
        PackedStringArray::from(&[GString::from("mock")])
    }
    fn handles_type(&self, type_: StringName) -> bool {
        print_thread("ResourceFormatLoader", "handles_type");
        type_.to_string().eq("MockResource")
    }
    fn get_resource_type(&self, path: GString) -> GString {
      // This gets called from other thread ONLY when running directly from editor
      // $godot -e
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

// Singleton - to handily unregister Loader and Saver

#[derive(GodotClass)]
#[class(base=Object, init)]
pub struct MySingleton {
    loader: Gd<MyLoader>,
    saver: Gd<MySaver>,
}

impl MySingleton {
    pub(crate) const SINGLETON_NAME: &str = "MySingleton";

    pub(crate) fn register_saver_loader(&self) {
        ResourceLoader::singleton().add_resource_format_loader(self.loader.clone().upcast());
        ResourceSaver::singleton().add_resource_format_saver(self.saver.clone().upcast());
    }

    pub(crate) fn unregister_saver_loader(&self) {
        ResourceLoader::singleton().remove_resource_format_loader(self.loader.clone().upcast());
        ResourceSaver::singleton().remove_resource_format_saver(self.saver.clone().upcast());
    }
}


# Resources _creepy_ user-independent multi-thread handling

So this little mock project aims to pinpoint the issues that arise after `gdext` introduced additional
checks to [cut off calls to `Gd` from other threads](https://github.com/godot-rust/gdext/pull/581).

- Main course is the `ResourceFormatSaver` and `ResourceFormatLoader` created in `gdext`
  - at the very least, `ResourceFormatLoader`, after registering it in `ResourceLoader` gets some calls
  from secondary threads via Godot Editor itself - it seems to be non-opt out in Godot.
  - methods in question are: 
    - ResourceFormatLoader::get_recognized_extensions (rust/src/saver_loader::17)
    - ResourceFormatLoader::get_resource_type (rust/src/saver_loader::28)
  - `ResourceFormatSaver` has been tried in-editor (via `InEditorNode` tool singleton), don't seem to be
  called from another threads but I cannot guarantee it won't. 
  - [Issue](https://github.com/godot-rust/gdext/issues/597)
- Secondary course is the `WithNested` and `Nested` custom Resources - I've tried to check for them being
  initialized within other threads, but I cannot reproduce [this panic](https://discord.com/channels/723850269347283004/1206624719931969536)

## Booting up the project without Editor

All methods are called from the main thread, as signified with `Thread: 1`

```bash
$godot --headless
Initialize godot-rust (API v4.2.stable.official, runtime v4.2.stable.official)
Godot Engine v4.2.stable.official.46dc27791 - https://godotengine.org

Thread: 1: ResourceFormatLoader::get_recognized_extensions
Thread: 1: ResourceFormatLoader::handles_type
Thread: 1: ResourceFormatLoader::handles_type
Thread: 1: ResourceFormatLoader::get_recognized_extensions
Thread: 1: ResourceFormatLoader::load
Thread: 1: Nested::init
Thread: 1: WithNested::init
Thread: 1: Nested::init
Thread: 1: WithNested::init
Thread: 1: Nested::init
# @tool node is added as an autoload - can't test the saving from Editor level without it.
# Probably causes segfault at the end of Editor run (dunno why?). It is also running on Scene
# (@tool being additive - can't cause a node to NOT run in Scene)
Thread: 1: ToolNode::enter_tree
Thread: 1: Saving MockResource from EDITOR node
Thread: 1: ResourceFormatSaver::recognize
Thread: 1: ResourceFormatSaver::get_recognized_extensions
Thread: 1: ResourceFormatSaver::save
Thread: 1: Loading MockResource from EDITOR node
Thread: 1: ResourceFormatLoader::handles_type
Thread: 1: ResourceFormatLoader::get_recognized_extensions
Thread: 1: ResourceFormatLoader::load
# Test node at the main scene. To test the Saving and Loading using `save()` and `load()`
Thread: 1: TestNode::enter_tree
Thread: 1: Saving MockResource from SCENE node
Thread: 1: ResourceFormatSaver::recognize
Thread: 1: ResourceFormatSaver::get_recognized_extensions
Thread: 1: ResourceFormatSaver::save
Thread: 1: Loading MockResource from SCENE node
Thread: 1: ResourceFormatLoader::handles_type
Thread: 1: ResourceFormatLoader::get_recognized_extensions
Thread: 1: ResourceFormatLoader::load
```

## Booting the project in Editor mode but without Editor

A lot more is going on there, even if the main scene is not being ran. Segfault probably
caused by the InEditorNode.

The only method ran there from another thread is `ResourceFormatLoader::get_recognized_extensions`

```bash
$godot --headless -e --quit
Initialize godot-rust (API v4.2.stable.official, runtime v4.2.stable.official)
Godot Engine v4.2.stable.official.46dc27791 - https://godotengine.org
 
Thread: 1: ResourceFormatLoader::get_recognized_extensions
Thread: 1: ResourceFormatLoader::handles_type
WARNING: Custom cursor shape not supported by this display server.
     at: cursor_set_custom_image (servers/display_server.cpp:505)
Thread: 1: ResourceFormatSaver::get_recognized_extensions
Thread: 1: ResourceFormatLoader::handles_type
Thread: 1: ResourceFormatLoader::handles_type
Thread: 1: ResourceFormatLoader::handles_type
Thread: 1: ResourceFormatLoader::handles_type
Thread: 1: ResourceFormatLoader::handles_type
Thread: 1: ResourceFormatLoader::handles_type
Thread: 1: ResourceFormatLoader::handles_type
Thread: 1: ResourceFormatLoader::handles_type
Thread: 1: ResourceFormatLoader::handles_type
Thread: 1: ResourceFormatLoader::handles_type
Thread: 1: ResourceFormatLoader::get_recognized_extensions
Thread: 1: Nested::init
Thread: 1: WithNested::init
Thread: 1: Nested::init
WARNING: Instantiated MockResource used as default value for TestNode's "mock" property.
     at: class_get_default_property_value (core/object/class_db.cpp:1743)
WARNING: Instantiated WithNested used as default value for TestNode's "nested" property.
     at: class_get_default_property_value (core/object/class_db.cpp:1743)
Thread: 1: WithNested::init
Thread: 1: Nested::init
WARNING: Instantiated Nested used as default value for WithNested's "nested" property.
     at: class_get_default_property_value (core/object/class_db.cpp:1743)

# The only appearance of `ResourceFormatLoader` method from other thread in this run mode!

Thread: 14: ResourceFormatLoader::get_recognized_extensions
Thread: 1: ResourceFormatLoader::handles_type
Thread: 1: ResourceFormatLoader::get_recognized_extensions
Thread: 1: ResourceFormatLoader::load
Thread: 1: Nested::init
Thread: 1: WithNested::init
Thread: 1: Nested::init
Thread: 1: WithNested::init
Thread: 1: Nested::init
Thread: 1: ToolNode::enter_tree
Thread: 1: Saving MockResource from EDITOR node
Thread: 1: ResourceFormatSaver::recognize
Thread: 1: ResourceFormatSaver::get_recognized_extensions
Thread: 1: ResourceFormatSaver::save
Thread: 1: ResourceFormatLoader::get_resource_type
Thread: 1: ResourceFormatLoader::get_resource_uid
Thread: 1: ResourceFormatLoader::get_recognized_extensions
Thread: 1: ResourceFormatLoader::get_recognized_extensions
Thread: 1: ResourceFormatLoader::get_recognized_extensions
Thread: 1: ResourceFormatLoader::get_dependencies
Thread: 1: ResourceFormatLoader::get_recognized_extensions
Thread: 1: Loading MockResource from EDITOR node
Thread: 1: ResourceFormatLoader::handles_type
Thread: 1: ResourceFormatLoader::get_recognized_extensions
Thread: 1: ResourceFormatLoader::load
WARNING: Instantiated MockResource used as default value for TestNode's "mock" property.
     at: class_get_default_property_value (core/object/class_db.cpp:1743)
WARNING: Instantiated MockResource used as default value for TestNode's "mock" property.
     at: class_get_default_property_value (core/object/class_db.cpp:1743)
WARNING: Instantiated WithNested used as default value for TestNode's "nested" property.
     at: class_get_default_property_value (core/object/class_db.cpp:1743)
WARNING: Instantiated WithNested used as default value for TestNode's "nested" property.
     at: class_get_default_property_value (core/object/class_db.cpp:1743)
Thread: 1: ResourceFormatLoader::handles_type
Thread: 1: ResourceFormatLoader::handles_type
Segmentation fault (core dumped)
```

## Booting the project with Editor

Both methods are called there from another threads
    - ResourceFormatLoader::get_recognized_extensions
    - ResourceFormatLoader::get_resource_type

```bash
$godot -e
Initialize godot-rust (API v4.2.stable.official, runtime v4.2.stable.official)
Godot Engine v4.2.stable.official.46dc27791 - https://godotengine.org
/lib/x86_64-linux-gnu/libxkbcommon.so.0: undefined symbol: xkb_utf32_to_keysym
/lib/x86_64-linux-gnu/libxkbcommon.so.0: undefined symbol: xkb_keymap_key_get_mods_for_level
OpenGL API 4.6 (Core Profile) Mesa 21.2.6 - Compatibility - Using Device: Intel - Mesa Intel(R) HD Graphics 520 (SKL GT2)
 
Thread: 1: ResourceFormatLoader::get_recognized_extensions
Thread: 1: ResourceFormatLoader::handles_type
Thread: 1: ResourceFormatSaver::get_recognized_extensions
Thread: 1: ResourceFormatLoader::handles_type
Thread: 1: ResourceFormatLoader::handles_type
Thread: 1: ResourceFormatLoader::handles_type
Thread: 1: ResourceFormatLoader::handles_type
Thread: 1: ResourceFormatLoader::handles_type
Thread: 1: ResourceFormatLoader::handles_type
Thread: 1: ResourceFormatLoader::handles_type
Thread: 1: ResourceFormatLoader::handles_type
Thread: 1: ResourceFormatLoader::handles_type
Thread: 1: ResourceFormatLoader::handles_type
Thread: 1: Nested::init
Thread: 1: WithNested::init
Thread: 1: Nested::init
WARNING: Instantiated MockResource used as default value for TestNode's "mock" property.
     at: class_get_default_property_value (core/object/class_db.cpp:1743)
WARNING: Instantiated WithNested used as default value for TestNode's "nested" property.
     at: class_get_default_property_value (core/object/class_db.cpp:1743)
Thread: 1: WithNested::init
Thread: 1: Nested::init
WARNING: Instantiated Nested used as default value for WithNested's "nested" property.
     at: class_get_default_property_value (core/object/class_db.cpp:1743)
Thread: 1: ResourceFormatLoader::get_recognized_extensions

# Get recognized extensions as in above is run there

Thread: 16: ResourceFormatLoader::get_recognized_extensions
Thread: 1: ResourceFormatLoader::handles_type
Thread: 1: ResourceFormatLoader::get_recognized_extensions
Thread: 1: ResourceFormatLoader::load
Thread: 1: Nested::init
Thread: 1: WithNested::init
Thread: 1: Nested::init
Thread: 1: WithNested::init
Thread: 1: Nested::init
Thread: 1: ToolNode::enter_tree
Thread: 1: Saving MockResource from EDITOR node
Thread: 1: ResourceFormatSaver::recognize
Thread: 1: ResourceFormatSaver::get_recognized_extensions
Thread: 1: ResourceFormatSaver::save
Thread: 1: ResourceFormatLoader::get_resource_type
Thread: 1: ResourceFormatLoader::get_resource_uid
Thread: 1: ResourceFormatLoader::get_recognized_extensions
Thread: 1: ResourceFormatLoader::get_recognized_extensions
Thread: 1: ResourceFormatLoader::get_recognized_extensions
Thread: 1: ResourceFormatLoader::get_dependencies
Thread: 1: ResourceFormatLoader::get_recognized_extensions
Thread: 1: Loading MockResource from EDITOR node
Thread: 1: ResourceFormatLoader::handles_type
Thread: 1: ResourceFormatLoader::get_recognized_extensions
Thread: 1: ResourceFormatLoader::load
WARNING: Instantiated MockResource used as default value for TestNode's "mock" property.
     at: class_get_default_property_value (core/object/class_db.cpp:1743)
WARNING: Instantiated MockResource used as default value for TestNode's "mock" property.
     at: class_get_default_property_value (core/object/class_db.cpp:1743)
WARNING: Instantiated WithNested used as default value for TestNode's "nested" property.
     at: class_get_default_property_value (core/object/class_db.cpp:1743)
WARNING: Instantiated WithNested used as default value for TestNode's "nested" property.
     at: class_get_default_property_value (core/object/class_db.cpp:1743)
Thread: 1: ResourceFormatLoader::handles_type
Thread: 1: ResourceFormatLoader::handles_type

# Additionally, these are also ran (possibly caused by resources appearing in the
# Godot Editor file browser)
Thread: 17: ResourceFormatLoader::get_resource_type
Thread: 17: ResourceFormatLoader::get_resource_type
Thread: 1: ResourceFormatLoader::get_recognized_extensions
Thread: 1: ResourceFormatLoader::get_recognized_extensions
Segmentation fault (core dumped)
```
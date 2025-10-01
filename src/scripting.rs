use bevy::prelude::*;
use bevy_scriptum::prelude::*;
use bevy_scriptum::runtimes::lua::prelude::*;

pub struct CoreScriptApiPlugin;

impl Plugin for CoreScriptApiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, call_lua_on_update_from_rust)
            .add_scripting::<LuaRuntime>(|runtime| {
                runtime
                    .add_function(String::from("spawn_entity"), spawn_entity)
                    .add_function(String::from("spawn_entity_named"), spawn_entity_named)
                    .add_function(
                        String::from("spawn_entity_scripted"),
                        spawn_entity_named_scripted,
                    );
            });
    }
}

// A spawn request struct to encapsulate parameters for spawning objects from lua.
#[warn(dead_code)]
pub struct SpawnRequest {
    pub toy_name: String,
    // A prototype is a template for an object, e.g., a specific type of toy. Cars, action figures, etc.
    pub prototype_name: String,
}

fn inner_spawn_entity(mut commands: Commands, request: SpawnRequest) {
    let toy_name = request.toy_name;

    commands.spawn(Name::new(toy_name));
}

pub fn spawn_entity(commands: Commands) {
    let request = SpawnRequest {
        toy_name: "SpawnedEntity".to_string(),
        prototype_name: "".to_string(),
    };
    inner_spawn_entity(commands, request);
}

pub fn spawn_entity_named(In((name,)): In<(String,)>, commands: Commands) {
    let request = SpawnRequest {
        toy_name: name,
        prototype_name: "".to_string(),
    };
    inner_spawn_entity(commands, request);
}

pub fn spawn_entity_named_scripted(
    In((name, script_name)): In<(String, String)>,
    mut commands: Commands,
    assets_server: Res<AssetServer>,
) {
    let request = SpawnRequest {
        toy_name: name,
        prototype_name: "".to_string(),
    };
    // inner_spawn_entity(commands, request);

    commands.spawn((
        Script::<LuaScript>::new(assets_server.load("entity_script.lua")),
        Name::new(request.toy_name),
    ));
}

fn call_lua_on_update_from_rust(
    mut scripted_entities: Query<(Entity, &mut LuaScriptData)>,
    scripting_runtime: ResMut<LuaRuntime>,
) {
    for (entity, mut script_data) in &mut scripted_entities {
        // calling function named `on_update` defined in lua script
        scripting_runtime
            .call_fn("on_tick", &mut script_data, entity, ())
            .unwrap();
    }
}

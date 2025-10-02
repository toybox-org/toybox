use bevy::prelude::*;
use bevy_scriptum::prelude::*;
use bevy_scriptum::runtimes::lua::prelude::*;

pub type PlayerName = String;

pub struct CoreScriptApiPlugin;

impl Plugin for CoreScriptApiPlugin {
    fn build(&self, app: &mut App) {
        println!("SCRIPTING");
        app.add_systems(Update, call_lua_on_update_from_rust)
            .add_scripting::<LuaRuntime>(|runtime| {
                runtime
                    .add_function(String::from("spawn_entity"), spawn_entity)
                    .add_function(String::from("spawn_entity_named"), spawn_entity_named)
                    .add_function(
                        String::from("spawn_entity_scripted"),
                        spawn_entity_named_scripted,
                    )
                    .add_function(String::from("set_postition"), set_postition)
                    .add_function(
                        String::from("pass_to_rust"),
                        |In((entity,)): In<(BevyEntity,)>| {
                            println!("pass_to_rust called with entity: {:?}", entity);
                        },
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

    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let request = SpawnRequest {
        toy_name: name,
        prototype_name: "".to_string(),
    };
    // inner_spawn_entity(commands, request);

    commands.spawn((
        Script::<LuaScript>::new(assets_server.load("entity_script.lua")),
        Name::new(request.toy_name),
        Transform::from_xyz(0., 0., 0.),
        Mesh2d(meshes.add(Circle::new(25.))),
        MeshMaterial2d(materials.add(Color::srgb(0.25, 0.4, 0.1))),
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

fn set_postition(
    In((entity, translation)): In<(BevyEntity, BevyVec3)>,
    mut entities: Query<&mut Transform>,
) {
    let mut transform = entities.get_mut(entity.0).unwrap();
    transform.translation = translation.0;
}

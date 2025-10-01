function launch()
 -- TODO: Expand on this spawn_entity function to take in parameters for different entity types
 -- like Toy Name, Part Name, etc.
 game_settings = {
  name = "value",
 }

 spawn_entity();
 spawn_entity_named("MyToy");
 spawn_entity_scripted("MyOtherToy", "entity_script.lua");
end

function on_tick()
 print("A")
end

function on_player_join(player_name)
 -- TODO: Spawn entity that can be puppeted
 ent = spawn_entity("Player-"+player_name, "PlayerCharacter")
 -- TODO: make Player control Character
 player_puppet(player_name, ent)
 print("Player joined...")
end

function on_player_leave()
 print("Player left...")
end


launch()
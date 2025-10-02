function on_spawn()
 local line = "Spawned as ID-(" .. entity.index .. ")"
 print(line)
 my_vec = Vec3(1, 200, 3)
 set_postition(entity, my_vec)

 pass_to_rust(entity)
end

-- A player has taken control of this entity. A car get puppeted by the player Driving the car.
function on_puppet(player_name)
 print("Puppeted by " .. player_name)
end

function on_unpuppet(player_name)
 print("Unpuppeted by " .. player_name)
end


function on_tick()
 -- print("Tick")
end

on_spawn()
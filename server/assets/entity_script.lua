function on_spawn()
 local line = "Spawned as ID-(" .. entity.index .. ")"
 print(line)


 pass_to_rust(entity)
end

-- A player has taken control of this entity. A car get puppeted by the player Driving the car.
function on_puppet(player_name)
 print("Puppeted by " .. player_name)
end

function on_unpuppet()
end


function on_tick()
 -- print("Tick")
end

on_spawn()
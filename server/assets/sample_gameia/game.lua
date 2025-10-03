-- The entrypoint script of the whole game.


function on_launch()
 print("Starting game.")
end


function on_player_join(player_name)
 ent = spawn_entity("Player-" .. player_name, "PlayerCharacter")
 set_puppetable(ent, true)
 set_player_puppet(player_name, ent)
end



function on_tick()

end
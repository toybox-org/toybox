-- A mana crystal.
function on_spawn()
 set_puppetable(entity, false)
 set_holdable(entity, true)
 set_usable(entity, true)
 set_pickupable(entity, true)

 set_varible_num("mana_current", 10)
 set_varible_num("mana_max", 10)
end

-- This is called once per minute.
function on_minute()
  local mv = get_varible_num("mana_value")
  mv = mv + 1
  set_varible_num("mana_value", mv)
end
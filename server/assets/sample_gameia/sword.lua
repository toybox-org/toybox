-- A sword item script.
-- Created from the perspective of a functioning lua api to host it.


-- TODO: Decide on a bigger and better list of WeaponTypes and FightStyles
-- This is just a starting point.
weapon_types = {
 -- A light one handed sword or long knife.
 "Dagger",
 -- A one handed sword.
 "Short Swords",
 -- A two handed sword.
 "Long Swords",

 -- A polearm. Like a Spear, Halberd or Glaive.
 "Polearms",
 
 -- A bow and arrow.
 "Bows",
 -- A crossbow and bolts.
 "Crossbows",

 -- A small shield. Like a buckler or round shield.
 "SmallShield",
 -- A large towering shield.
 "LargeShield",

 -- An arcane focus used to channel magic.
 -- Due to the nature of magic this can be a staff, wand, orb, or other magical implement. Like a book or even a tattoo.
 -- It is hard to define what an arcane focus is, so this is left open to interpretation.
 "Focus",
}

fight_styles = {
 -- An all purpose combat style, small damage buff. No weapon restrictions.
 "Brawler",
 -- A defensive combat style. Focused around a large shield and a small one handed weapon.
 -- Use: LargeShield + Dagger || Short Swords
 "Sword & Board",
 -- A dual melee and magic fighting style. Using the main hand for melee and off hand for magic.
 -- Use: One handed weapon + Unarmed
 "Spell Sword",
 "Archer",
}


-- Ran when the sword entity is created, typically when added to the world or a LootContainer.
function on_spawn()
 print("Sword spawned")
 -- TODO: Roll random stats for the sword.
 -- Set the sword to be non-puppetable, this way players can't puppet it directly.
 set_puppetable(entity, false)
 set_holdable(entity, true)
 set_usable(entity, true)
 -- Making an item pickupable means it can be picked and dropped. Setting this false causes the item to be dropped by the player instantly.
 -- Use set_quest_item(entity, true) to make it so the item can't be dropped until a quest is completed.
 set_pickupable(entity, true)
end


-- A character has begun holding this item in their hands.
function on_hold(player_name)
 print("Sword held by " .. player_name)
 -- Check player FightStyle to see if its applicable.
 fs = get_player_fight_style(player_name)

 -- If the fight style is not valid for this weapon, kick the player to the default FightStyle.
 if not validate_weapontype_for_fightstyle("Sword", fs) then
  print("Invalid FightStyle for Sword, changing to default.")
  set_player_fight_style(player_name, "Brawler")
 end
end

function on_unhold(player_name)
 print("Sword unheld by " .. player_name)
end

function on_use(player_name, target_entity)
 print("Sword used by " .. player_name .. " on " .. target_entity.index)
 -- TODO: Add a dice roll api in rust to do random damage calculations
 dmg = roll("1d4+2") -- Roll a 4 sided dice and add 2 to the result.
 -- deal_dmg(target_entity, dmg)
 deal_targeted_dmg(player_name, target_entity, dmg)
 -- Dagger 1d4+1
 -- Sword 1d4+2
 -- Greatsword 1d6+1
end

function on_pickup(player_name)
 print("Sword picked up by " .. player_name)
end
function on_drop(player_name)
 print("Sword dropped by " .. player_name)
end
struct PlayerAccount {
    // How much money a player has.
    wallet: u64,
}

// A GoldPool is a seperate wallet that is pulled from on Puppet death.
struct GoldPool {
    current: u64,
    max: u64,
}

enum LootDropRules {
    // All loot is dropped publicly.
    Public,
    // Loot is dropped only for the party.
    Party,
    // Loot is dropped only for the team.
    Team,
    // Loot is dropped only for the guild.
    Guild,
}

struct WorldSettings {
    // Some worlds charge gold per death. Pulled from a Gold Pool seperated from your wallet.
    death_charge: u64,
    // An amount of gold retained by the world on Puppet death, remaining is dropped according to lootdrop rules.
    death_tax_rate: u64,

    loot_drop_rules: LootDropRules,
}

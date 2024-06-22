#[repr(u8)]
#[derive(PartialEq)]
pub enum D2StringColorCodes {
    White       = 0,
    Red         = 1,
    LightGreen  = 2,
    Blue        = 3,
    DarkGold    = 4,
    Grey        = 5,
    Black       = 6,
    Tan         = 7,
    Orange      = 8,
    Yellow      = 9,
    DarkGreen   = 10,
    Purple      = 11,
    DarkGreen2  = 12,
    Invalid     = 255,
}

#[repr(i32)]
#[derive(PartialEq)]
pub enum D2UIvars {
    Game            = 0x00, // 0
    Inventory       = 0x01, // 1
    StatScreen      = 0x02, // 2
    MiniSkill       = 0x03, // 3
    SkillTree       = 0x04, // 4
    ChatBox         = 0x05, // 5
    NewStats        = 0x06, // 6
    NewSkills       = 0x07, // 7
    NpcMenu         = 0x08, // 8
    EscMenu         = 0x09, // 9
    AutoMap         = 0x0A, // 10
    Config          = 0x0B, // 11
    NpcShop         = 0x0C, // 12
    HoldAlt         = 0x0D, // 13
    Anvil           = 0x0E, // 14
    QuestScreen     = 0x0F, // 15
    IniScroll       = 0x10, // 16
    QuestLog        = 0x11, // 17
    Unknown18       = 0x12, // 18
    HiRicons        = 0x13, // 19
    Waypoint        = 0x14, // 20
    MiniPanel       = 0x15, // 21
    PartyScreen     = 0x16, // 22
    MpTrade         = 0x17, // 23
    MsgLog          = 0x18, // 24
    Stash           = 0x19, // 25
    Cube            = 0x1A, // 26
    SteegStone      = 0x1B, // 27
    GuildVault      = 0x1C, // 28
    Unknown29       = 0x1D, // 29
    Unknown30       = 0x1E, // 30
    BeltRows        = 0x1F, // 31
    Unknown32       = 0x20, // 32
    HelpScreen      = 0x21, // 33
    HelpButton      = 0x22, // 34
    HireIcons       = 0x23, // 35
    MercInv         = 0x24, // 36
    RecipeScroll    = 0x25, // 37
}


#[repr(i32)]
#[derive(PartialEq)]
pub enum D2ControlTypes {
    Editbox       = 1,
    Image         = 2,
    Animimage     = 3,
    Textbox       = 4,
    Scrollbar     = 5,
    Button        = 6,
    List          = 7,
    Timer         = 8,
    Smack         = 9,
    Progressbar   = 10,
    Popup         = 11,
    Accountlist   = 12,
    Image2        = 13,
}

#[repr(i32)]
#[derive(PartialEq)]
pub enum D2ItemStats {
    Invalid                           = -1,
    Strength                          = 0,
    Energy                            = 1,
    Dexterity                         = 2,
    Vitality                          = 3,
    StatPts                           = 4,
    SkillPts                          = 5,
    HitPoints                         = 6,
    MaxHp                             = 7,
    Mana                              = 8,
    MaxMana                           = 9,
    Stamina                           = 10,
    MaxStamina                        = 11,
    Level                             = 12,
    Experience                        = 13,
    Gold                              = 14,
    GoldBank                          = 15,
    Item_Armor_Percent                = 16,
    Item_MaxDamage_Percent            = 17,
    Item_MinDamage_Percent            = 18,
    ToHit                             = 19,
    ToBlock                           = 20,
    MinDamage                         = 21,
    MaxDamage                         = 22,
    Secondary_MinDamage               = 23,
    Secondary_MaxDamage               = 24,
    DamagePercent                     = 25,
    ManaRecovery                      = 26,
    ManaRecoveryBonus                 = 27,
    StaminaRecoveryBonus              = 28,
    LastExp                           = 29,
    NextExp                           = 30,
    ArmorClass                        = 31,
    ArmorClass_Vs_Missile             = 32,
    ArmorClass_Vs_Hth                 = 33,
    Normal_Damage_Reduction           = 34,
    Magic_Damage_Reduction            = 35,
    DamageResist                      = 36,
    MagicResist                       = 37,
    MaxMagicResist                    = 38,
    FireResist                        = 39,
    MaxFireResist                     = 40,
    LightResist                       = 41,
    MaxLightResist                    = 42,
    ColdResist                        = 43,
    MaxColdResist                     = 44,
    PoisonResist                      = 45,
    MaxPoisonResist                   = 46,
    DamageAura                        = 47,
    FireMinDam                        = 48,
    FireMaxDam                        = 49,
    LightMinDam                       = 50,
    LightMaxDam                       = 51,
    MagicMinDam                       = 52,
    MagicMaxDam                       = 53,
    ColdMinDam                        = 54,
    ColdMaxDam                        = 55,
    ColdLength                        = 56,
    PoisonMinDam                      = 57,
    PoisonMaxDam                      = 58,
    PoisonLength                      = 59,
    LifeDrainMinDam                   = 60,
    LifeDrainMaxDam                   = 61,
    ManaDrainMinDam                   = 62,
    ManaDrainMaxDam                   = 63,
    StamDrainMinDam                   = 64,
    StamDrainMaxDam                   = 65,
    StunLength                        = 66,
    VelocityPercent                   = 67,
    AttackRate                        = 68,
    Other_AnimRate                    = 69,
    Quantity                          = 70,
    Value                             = 71,
    Durability                        = 72,
    MaxDurability                     = 73,
    HpRegen                           = 74,
    Item_MaxDurability_Percent        = 75,
    Item_MaxHp_Percent                = 76,
    Item_MaxMana_Percent              = 77,
    Item_AttackerTakesDamage          = 78,
    Item_GoldBonus                    = 79,
    Item_MagicBonus                   = 80,
    Item_Knockback                    = 81,
    Item_TimeDuration                 = 82,
    Item_AddClassSkills               = 83,
    UnsentParam1                      = 84,
    Item_AddExperience                = 85,
    Item_HealAfterKill                = 86,
    Item_ReducedPrices                = 87,
    Item_DoubleHerbDuration           = 88,
    Item_LightRadius                  = 89,
    Item_LightColor                   = 90,
    Item_Req_Percent                  = 91,
    Item_LevelReq                     = 92,
    Item_FasterAttackRate             = 93,
    Item_LevelReqPct                  = 94,
    LastBlockFrame                    = 95,
    Item_FasterMoveVelocity           = 96,
    Item_NonClassSkill                = 97,
    State                             = 98,
    Item_FasterGetHitRate             = 99,
    Monster_PlayerCount               = 100,
    Skill_Poison_Override_Length      = 101,
    Item_FasterBlockRate              = 102,
    Skill_Bypass_Undead               = 103,
    Skill_Bypass_Demons               = 104,
    Item_FasterCastRate               = 105,
    Skill_Bypass_Beasts               = 106,
    Item_SingleSkill                  = 107,
    Item_RestInPeace                  = 108,
    Curse_Resistance                  = 109,
    Item_PoisonLengthResist           = 110,
    Item_NormalDamage                 = 111,
    Item_Howl                         = 112,
    Item_Stupidity                    = 113,
    Item_DamageToMana                 = 114,
    Item_IgnoreTargetAc               = 115,
    Item_FractionalTargetAc           = 116,
    Item_PreventHeal                  = 117,
    Item_HalfFreezeDuration           = 118,
    Item_ToHit_Percent                = 119,
    Item_DamageTargetAc               = 120,
    Item_DemonDamage_Percent          = 121,
    Item_UndeadDamage_Percent         = 122,
    Item_Demon_ToHit                  = 123,
    Item_Undead_ToHit                 = 124,
    Item_Throwable                    = 125,
    Item_ElemSkill                    = 126,
    Item_AllSkills                    = 127,
    Item_AttackerTakesLightDamage     = 128,
    IronMaiden_Level                  = 129,
    LifeTap_Level                     = 130,
    Thorns_Percent                    = 131,
    BoneArmor                         = 132,
    BoneArmorMax                      = 133,
    Item_Freeze                       = 134,
    Item_OpenWounds                   = 135,
    Item_CrushingBlow                 = 136,
    Item_KickDamage                   = 137,
    Item_ManaAfterKill                = 138,
    Item_HealAfterDemonKill           = 139,
    Item_ExtraBlood                   = 140,
    Item_DeadlyStrike                 = 141,
    Item_AbsorbFire_Percent           = 142,
    Item_AbsorbFire                   = 143,
    Item_AbsorbLight_Percent          = 144,
    Item_AbsorbLight                  = 145,
    Item_AbsorbMagic_Percent          = 146,
    Item_AbsorbMagic                  = 147,
    Item_AbsorbCold_Percent           = 148,
    Item_AbsorbCold                   = 149,
    Item_Slow                         = 150,
    Item_Aura                         = 151,
    Item_Indesctructible              = 152,
    Item_CannotBeFrozen               = 153,
    Item_StaminaDrainPct              = 154,
    Item_Reanimate                    = 155,
    Item_Pierce                       = 156,
    Item_MagicArrow                   = 157,
    Item_ExplosiveArrow               = 158,
    Item_Throw_MinDamage              = 159,
    Item_Throw_MaxDamage              = 160,
    Skill_HandOfAthena                = 161,
    Skill_StaminaPercent              = 162,
    Skill_Passive_StaminaPercent      = 163,
    Skill_Concentration               = 164,
    Skill_Enchant                     = 165,
    Skill_Pierce                      = 166,
    Skill_Conviction                  = 167,
    Skill_ChillingArmor               = 168,
    Skill_Frenzy                      = 169,
    Skill_Decrepify                   = 170,
    Skill_Armor_Percent               = 171,
    Alignment                         = 172,
    Target0                           = 173,
    Target1                           = 174,
    GoldLost                          = 175,
    Conversion_Level                  = 176,
    Conversion_MaxHp                  = 177,
    Unit_DoOverlay                    = 178,
    Attack_Vs_MonType                 = 179,
    Damage_Vs_MonType                 = 180,
    Fade                              = 181,
    Armor_Override_Percent            = 182,
    Unused183                         = 183,
    Unused184                         = 184,
    Unused185                         = 185,
    Unused186                         = 186,
    Unused187                         = 187,
    Item_AddSkill_Tab                 = 188,
    Unused189                         = 189,
    Unused190                         = 190,
    Unused191                         = 191,
    Unused192                         = 192,
    Unused193                         = 193,
    Item_NumSockets                   = 194,
    Item_SkillOnAttack                = 195,
    Item_SkillOnKill                  = 196,
    Item_SkillOnDeath                 = 197,
    Item_SkillOnHit                   = 198,
    Item_SkillOnLevelUp               = 199,
    Unused200                         = 200,
    Item_SkillOnGetHit                = 201,
    Unused202                         = 202,
    Unused203                         = 203,
    Item_Charged_Skill                = 204,
    Unused204                         = 205,
    Unused205                         = 206,
    Unused206                         = 207,
    Unused207                         = 208,
    Unused208                         = 209,
    Unused209                         = 210,
    Unused210                         = 211,
    Unused211                         = 212,
    Unused212                         = 213,
    Item_Armor_PerLevel               = 214,
    Item_ArmorPercent_PerLevel        = 215,
    Item_Hp_PerLevel                  = 216,
    Item_Mana_PerLevel                = 217,
    Item_MaxDamage_PerLevel           = 218,
    Item_MaxDamage_Percent_PerLevel   = 219,
    Item_Strength_PerLevel            = 220,
    Item_Dexterity_PerLevel           = 221,
    Item_Energy_PerLevel              = 222,
    Item_Vitality_PerLevel            = 223,
    Item_ToHit_PerLevel               = 224,
    Item_ToHitPercent_PerLevel        = 225,
    Item_Cold_DamageMax_PerLevel      = 226,
    Item_Fire_DamageMax_PerLevel      = 227,
    Item_Ltng_DamageMax_PerLevel      = 228,
    Item_Pois_DamageMax_PerLevel      = 229,
    Item_Resist_Cold_PerLevel         = 230,
    Item_Resist_Fire_PerLevel         = 231,
    Item_Resist_Ltng_PerLevel         = 232,
    Item_Resist_Pois_PerLevel         = 233,
    Item_Absorb_Cold_PerLevel         = 234,
    Item_Absorb_Fire_PerLevel         = 235,
    Item_Absorb_Ltng_PerLevel         = 236,
    Item_Absorb_Pois_PerLevel         = 237,
    Item_Thorns_PerLevel              = 238,
    Item_Find_Gold_PerLevel           = 239,
    Item_Find_Magic_PerLevel          = 240,
    Item_RegenStamina_PerLevel        = 241,
    Item_Stamina_PerLevel             = 242,
    Item_Damage_Demon_PerLevel        = 243,
    Item_Damage_Undead_PerLevel       = 244,
    Item_ToHit_Demon_PerLevel         = 245,
    Item_ToHit_Undead_PerLevel        = 246,
    Item_CrushingBlow_PerLevel        = 247,
    Item_OpenWounds_PerLevel          = 248,
    Item_Kick_Damage_PerLevel         = 249,
    Item_DeadlyStrike_PerLevel        = 250,
    Item_Find_Gems_PerLevel           = 251,
    Item_Replenish_Durability         = 252,
    Item_Replenish_Quantity           = 253,
    Item_Extra_Stack                  = 254,
    Item_Find_Item                    = 255,
    Item_Slash_Damage                 = 256,
    Item_Slash_Damage_Percent         = 257,
    Item_Crush_Damage                 = 258,
    Item_Crush_Damage_Percent         = 259,
    Item_Thrust_Damage                = 260,
    Item_Thrust_Damage_Percent        = 261,
    Item_Absorb_Slash                 = 262,
    Item_Absorb_Crush                 = 263,
    Item_Absorb_Thrust                = 264,
    Item_Absorb_Slash_Percent         = 265,
    Item_Absorb_Crush_Percent         = 266,
    Item_Absorb_Thrust_Percent        = 267,
    Item_Armor_ByTime                 = 268,
    Item_ArmorPercent_ByTime          = 269,
    Item_Hp_ByTime                    = 270,
    Item_Mana_ByTime                  = 271,
    Item_MaxDamage_ByTime             = 272,
    Item_MaxDamage_Percent_ByTime     = 273,
    Item_Strength_ByTime              = 274,
    Item_Dexterity_ByTime             = 275,
    Item_Energy_ByTime                = 276,
    Item_Vitality_ByTime              = 277,
    Item_ToHit_ByTime                 = 278,
    Item_ToHitPercent_ByTime          = 279,
    Item_Cold_DamageMax_ByTime        = 280,
    Item_Fire_DamageMax_ByTime        = 281,
    Item_Ltng_DamageMax_ByTime        = 282,
    Item_Pois_DamageMax_ByTime        = 283,
    Item_Resist_Cold_ByTime           = 284,
    Item_Resist_Fire_ByTime           = 285,
    Item_Resist_Ltng_ByTime           = 286,
    Item_Resist_Pois_ByTime           = 287,
    Item_Absorb_Cold_ByTime           = 288,
    Item_Absorb_Fire_ByTime           = 289,
    Item_Absorb_Ltng_ByTime           = 290,
    Item_Absorb_Pois_ByTime           = 291,
    Item_Find_Gold_ByTime             = 292,
    Item_Find_Magic_ByTime            = 293,
    Item_RegenStamina_ByTime          = 294,
    Item_Stamina_ByTime               = 295,
    Item_Damage_Demon_ByTime          = 296,
    Item_Damage_Undead_ByTime         = 297,
    Item_ToHit_Demon_ByTime           = 298,
    Item_ToHit_Undead_ByTime          = 299,
    Item_CrushingBlow_ByTime          = 300,
    Item_OpenWounds_ByTime            = 301,
    Item_Kick_Damage_ByTime           = 302,
    Item_DeadlyStrike_ByTime          = 303,
    Item_Find_Gems_ByTime             = 304,
    Item_Pierce_Cold                  = 305,
    Item_Pierce_Fire                  = 306,
    Item_Pierce_Ltng                  = 307,
    Item_Pierce_Pois                  = 308,
    Item_Damage_Vs_Monster            = 309,
    Item_Damage_Percent_Vs_Monster    = 310,
    Item_ToHit_Vs_Monster             = 311,
    Item_ToHit_Percent_Vs_Monster     = 312,
    Item_Ac_Vs_Monster                = 313,
    Item_Ac_Percent_Vs_Monster        = 314,
    FireLength                        = 315,
    BurningMin                        = 316,
    BurningMax                        = 317,
    Progressive_Damage                = 318,
    Progressive_Steal                 = 319,
    Progressive_Other                 = 320,
    Progressive_Fire                  = 321,
    Progressive_Cold                  = 322,
    Progressive_Lightning             = 323,
    Item_Extra_Charges                = 324,
    Progressive_ToHit                 = 325,
    Poison_Count                      = 326,
    Damage_FrameRate                  = 327,
    Pierce_Idx                        = 328,
    Passive_Fire_Mastery              = 329,
    Passive_Ltng_Mastery              = 330,
    Passive_Cold_Mastery              = 331,
    Passive_Pois_Mastery              = 332,
    Passive_Fire_Pierce               = 333,
    Passive_Ltng_Pierce               = 334,
    Passive_Cold_Pierce               = 335,
    Passive_Pois_Pierce               = 336,
    Passive_Critical_Strike           = 337,
    Passive_Dodge                     = 338,
    Passive_Avoid                     = 339,
    Passive_Evade                     = 340,
    Passive_Warmth                    = 341,
    Passive_Mastery_Melee_Th          = 342,
    Passive_Mastery_Melee_Dmg         = 343,
    Passive_Mastery_Melee_Crit        = 344,
    Passive_Mastery_Throw_Th          = 345,
    Passive_Mastery_Throw_Dmg         = 346,
    Passive_Mastery_Throw_Crit        = 347,
    Passive_WeaponBlock               = 348,
    Passive_Summon_Resist             = 349,
    ModifierList_Skill                = 350,
    ModifierList_Level                = 351,
    Last_Sent_Hp_Pct                  = 352,
    Source_Unit_Type                  = 353,
    Source_Unit_Id                    = 354,
    ShortParam1                       = 355,
    QuestItemDifficulty               = 356,
    Passive_Mag_Mastery               = 357,
    Passive_Mag_Pierce                = 358,
}

#[repr(i32)]
#[derive(PartialEq)]
pub enum D2DrawMode
{
  Trans25           = 0,
  Trans50           = 1,
  Trans75           = 2,
  Modulate          = 3,
  Burn              = 4,
  Normal            = 5,
  TransHighLight    = 6,
  HighLight         = 7,
}

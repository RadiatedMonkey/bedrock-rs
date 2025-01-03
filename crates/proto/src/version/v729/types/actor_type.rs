use bedrockrs_proto_core::ProtoCodec;
use bitflags::bitflags;
bitflags! {
    struct Flags: isize {
        const UNDEFINED = 1;
        const TYPE_MASK = 0x000000ff;
        const MOB = 0x00000100;
        const PATHFINDER_MOB = 0x00000200  | Self::MOB.bits();
        const MONSTER = 0x00000800 | Self::PATHFINDER_MOB.bits();
        const ANIMAL = 0x00001000 | Self::PATHFINDER_MOB.bits();
        const TAMABLE_ANIMAL = 0x00004000 | Self::ANIMAL.bits();
        const AMBIENT = 0x00008000 | Self::MOB.bits();
        const UNDEAD_MOB = 0x00010000 | Self::MONSTER.bits();
        const ZOMBIE_MONSTER = 0x00020000 | Self::UNDEAD_MOB.bits();
        const ARTHROPOD = 0x00040000 | Self::MONSTER.bits();
        const MINECART = 0x00080000;
        const SKELETONMONSTER = 0x00100000 | Self::UNDEAD_MOB.bits();
        const EQUINEANIMAL = 0x00200000 | Self::TAMABLE_ANIMAL.bits();
        const PROJECTILE = 0x00400000;
        const ABSTRACTARROW = 0x00800000;
        const WATERANIMAL = 0x00002000 | Self::PATHFINDER_MOB.bits();
        const VILLAGERBASE = 0x01000000 | Self::PATHFINDER_MOB.bits();
        const CHICKEN = 10 | Self::ANIMAL.bits();
        const COW = 11 | Self::ANIMAL.bits();
        const PIG = 12 | Self::ANIMAL.bits();
        const SHEEP = 13 | Self::ANIMAL.bits();
        const WOLF = 14 | Self::TAMABLE_ANIMAL.bits();
        const VILLAGER = 15 | Self::VILLAGERBASE.bits();
        const MUSHROOMCOW = 16 | Self::ANIMAL.bits();
        const SQUID = 17 | Self::WATERANIMAL.bits();
        const RABBIT = 18 | Self::ANIMAL.bits();
        const BAT = 19 | Self::AMBIENT.bits();
        const IRONGOLEM = 20 | Self::PATHFINDER_MOB.bits();
        const SNOWGOLEM = 21 | Self::PATHFINDER_MOB.bits();
        const OCELOT = 22 | Self::TAMABLE_ANIMAL.bits();
        const HORSE = 23 | Self::EQUINEANIMAL.bits();
        const POLARBEAR = 28 | Self::ANIMAL.bits();
        const LLAMA = 29 | Self::ANIMAL.bits();
        const PARROT = 30 | Self::TAMABLE_ANIMAL.bits();
        const DOLPHIN = 31 | Self::WATERANIMAL.bits();
        const DONKEY = 24 | Self::EQUINEANIMAL.bits();
        const MULE = 25 | Self::EQUINEANIMAL.bits();
        const SKELETONHORSE = 26 | Self::EQUINEANIMAL.bits() | Self::UNDEAD_MOB.bits();
        const ZOMBIEHORSE = 27 | Self::EQUINEANIMAL.bits() | Self::UNDEAD_MOB.bits();
        const ZOMBIE = 32 | Self::ZOMBIE_MONSTER.bits();
        const CREEPER = 33 | Self::MONSTER.bits();
        const SKELETON = 34 | Self::SKELETONMONSTER.bits();
        const SPIDER = 35 | Self::ARTHROPOD.bits();
        const PIGZOMBIE = 36 | Self::UNDEAD_MOB.bits();
        const SLIME = 37 | Self::MONSTER.bits();
        const ENDERMAN = 38 | Self::MONSTER.bits();
        const SILVERFISH = 39 | Self::ARTHROPOD.bits();
        const CAVESPIDER = 40 | Self::ARTHROPOD.bits();
        const GHAST = 41 | Self::MONSTER.bits();
        const LAVASLIME = 42 | Self::MONSTER.bits();
        const BLAZE = 43 | Self::MONSTER.bits();
        const ZOMBIEVILLAGER = 44 | Self::ZOMBIE_MONSTER.bits();
        const WITCH = 45 | Self::MONSTER.bits();
        const STRAY = 46 | Self::SKELETONMONSTER.bits();
        const HUSK = 47 | Self::ZOMBIE_MONSTER.bits();
        const WITHERSKELETON = 48 | Self::SKELETONMONSTER.bits();
        const GUARDIAN = 49 | Self::MONSTER.bits();
        const ELDERGUARDIAN = 50 | Self::MONSTER.bits();
        const NPC = 51 | Self::MOB.bits();
        const WITHERBOSS = 52 | Self::UNDEAD_MOB.bits();
        const DRAGON = 53 | Self::MONSTER.bits();
        const SHULKER = 54 | Self::MONSTER.bits();
        const ENDERMITE = 55 | Self::ARTHROPOD.bits();
        const AGENT = 56 | Self::MOB.bits();
        const VINDICATOR = 57 | Self::MONSTER.bits();
        const PHANTOM = 58 | Self::UNDEAD_MOB.bits();
        const ILLAGERBEAST = 59 | Self::MONSTER.bits();
        const ARMORSTAND = 61 | Self::MOB.bits();
        const TRIPODCAMERA = 62 | Self::MOB.bits();
        const PLAYER = 63 | Self::MOB.bits();
        const ITEMENTITY = 64;
        const PRIMEDTNT = 65;
        const FALLINGBLOCK = 66;
        const MOVINGBLOCK = 67;
        const EXPERIENCEPOTION = 68 | Self::PROJECTILE.bits();
        const EXPERIENCE = 69;
        const EYEOFENDER = 70;
        const ENDERCRYSTAL = 71;
        const FIREWORKSROCKET = 72;
        const TRIDENT = 73 | Self::PROJECTILE.bits() | Self::ABSTRACTARROW.bits();
        const TURTLE = 74 | Self::ANIMAL.bits();
        const CAT = 75 | Self::TAMABLE_ANIMAL.bits();
        const SHULKERBULLET = 76 | Self::PROJECTILE.bits();
        const FISHINGHOOK = 77;
        const CHALKBOARD = 78;
        const DRAGONFIREBALL = 79 | Self::PROJECTILE.bits();
        const ARROW = 80 | Self::PROJECTILE.bits() | Self::ABSTRACTARROW.bits();
        const SNOWBALL = 81 | Self::PROJECTILE.bits();
        const THROWNEGG = 82 | Self::PROJECTILE.bits();
        const PAINTING = 83;
        const LARGEFIREBALL = 85 | Self::PROJECTILE.bits();
        const THROWNPOTION = 86 | Self::PROJECTILE.bits();
        const ENDERPEARL = 87 | Self::PROJECTILE.bits();
        const LEASHKNOT = 88;
        const WITHERSKULL = 89 | Self::PROJECTILE.bits();
        const BOATRIDEABLE = 90;
        const WITHERSKULLDANGEROUS = 91 | Self::PROJECTILE.bits();
        const LIGHTNINGBOLT = 93;
        const SMALLFIREBALL = 94 | Self::PROJECTILE.bits();
        const AREAEFFECTCLOUD = 95;
        const LINGERINGPOTION = 101 | Self::PROJECTILE.bits();
        const LLAMASPIT = 102 | Self::PROJECTILE.bits();
        const EVOCATIONFANG = 103 | Self::PROJECTILE.bits();
        const EVOCATIONILLAGER = 104 | Self::MONSTER.bits();
        const VEX = 105 | Self::MONSTER.bits();
        const MINECARTRIDEABLE = 84 | Self::MINECART.bits();
        const MINECARTHOPPER = 96 | Self::MINECART.bits();
        const MINECARTTNT = 97 | Self::MINECART.bits();
        const MINECARTCHEST = 98 | Self::MINECART.bits();
        const MINECARTFURNACE = 99 | Self::MINECART.bits();
        const MINECARTCOMMANDBLOCK = 100 | Self::MINECART.bits();
        const ICEBOMB = 106 | Self::PROJECTILE.bits();
        const BALLOON = 107;
        const PUFFERFISH = 108 | Self::WATERANIMAL.bits();
        const SALMON = 109 | Self::WATERANIMAL.bits();
        const DROWNED = 110 | Self::ZOMBIE_MONSTER.bits();
        const TROPICALFISH = 111 | Self::WATERANIMAL.bits();
        const FISH = 112 | Self::WATERANIMAL.bits();
        const PANDA = 113 | Self::ANIMAL.bits();
        const PILLAGER = 114 | Self::MONSTER.bits();
        const VILLAGERV2 = 115 | Self::VILLAGERBASE.bits();
        const ZOMBIEVILLAGERV2 = 116 | Self::ZOMBIE_MONSTER.bits();
        const SHIELD = 117;
        const WANDERINGTRADER = 118 | Self::PATHFINDER_MOB.bits();
        const LECTERN = 119;
        const ELDERGUARDIANGHOST = 120 | Self::MONSTER.bits();
        const FOX = 121 | Self::ANIMAL.bits();
        const BEE = 122 | Self::MOB.bits();
        const PIGLIN = 123 | Self::MOB.bits();
        const HOGLIN = 124 | Self::ANIMAL.bits();
        const STRIDER = 125 | Self::ANIMAL.bits();
        const ZOGLIN = 126 | Self::UNDEAD_MOB.bits();
        const PIGLINBRUTE = 127 | Self::MOB.bits();
        const GOAT = 128 | Self::ANIMAL.bits();
        const GLOWSQUID = 129 | Self::WATERANIMAL.bits();
        const AXOLOTL = 130 | Self::ANIMAL.bits();
        const WARDEN = 131 | Self::MONSTER.bits();
        const FROG = 132 | Self::ANIMAL.bits();
        const TADPOLE = 133 | Self::WATERANIMAL.bits();
        const ALLAY = 134 | Self::MOB.bits();
        const CHESTBOATRIDEABLE = 136 |Self:: BOATRIDEABLE.bits();
        const TRADERLLAMA = 137 | Self::LLAMA.bits();
        const CAMEL = 138 | Self::ANIMAL.bits();
        const SNIFFER = 139 | Self::ANIMAL.bits();
        const BREEZE = 140 | Self::MONSTER.bits();
        const BREEZEWINDCHARGEPROJECTILE = 141 | Self::PROJECTILE.bits();
        const ARMADILLO = 142 | Self::ANIMAL.bits();
        const WINDCHARGEPROJECTILE = 143 |Self:: PROJECTILE.bits();
        const BOGGED = 144 | Self::SKELETONMONSTER.bits();
        const OMINOUSITEMSPAWNER = 145;
    }
}

#[derive(Debug, Clone)]
pub enum ActorType {
    Undefined = Flags::UNDEFINED.bits(),
    TypeMask = Flags::TYPE_MASK.bits(),
    Mob = Flags::MOB.bits(),
    PathfinderMob = Flags::PATHFINDER_MOB.bits(),
    Monster = Flags::MONSTER.bits(),
    Animal = Flags::ANIMAL.bits(),
    TamableAnimal = Flags::TAMABLE_ANIMAL.bits(),
    Ambient = Flags::AMBIENT.bits(),
    UndeadMob = Flags::UNDEAD_MOB.bits(),
    ZombieMonster = Flags::ZOMBIE_MONSTER.bits(),
    Arthropod = Flags::ARTHROPOD.bits(),
    Minecart = Flags::MINECART.bits(),
    SkeletonMonster = Flags::SKELETONMONSTER.bits(),
    EquineAnimal = Flags::EQUINEANIMAL.bits(),
    Projectile = Flags::PROJECTILE.bits(),
    AbstractArrow = Flags::ABSTRACTARROW.bits(),
    WaterAnimal = Flags::WATERANIMAL.bits(),
    VillagerBase = Flags::VILLAGERBASE.bits(),
    Chicken = Flags::CHICKEN.bits(),
    Cow = Flags::COW.bits(),
    Pig = Flags::PIG.bits(),
    Sheep = Flags::SHEEP.bits(),
    Wolf = Flags::WOLF.bits(),
    Villager = Flags::VILLAGER.bits(),
    MushroomCow = Flags::MUSHROOMCOW.bits(),
    Squid = Flags::SQUID.bits(),
    Rabbit = Flags::RABBIT.bits(),
    Bat = Flags::BAT.bits(),
    IronGolem = Flags::IRONGOLEM.bits(),
    SnowGolem = Flags::SNOWGOLEM.bits(),
    Ocelot = Flags::OCELOT.bits(),
    Horse = Flags::HORSE.bits(),
    PolarBear = Flags::POLARBEAR.bits(),
    Llama = Flags::LLAMA.bits(),
    Parrot = Flags::PARROT.bits(),
    Dolphin = Flags::DOLPHIN.bits(),
    Donkey = Flags::DONKEY.bits(),
    Mule = Flags::MULE.bits(),
    SkeletonHorse = Flags::SKELETONHORSE.bits(),
    ZombieHorse = Flags::ZOMBIEHORSE.bits(),
    Zombie = Flags::ZOMBIE.bits(),
    Creeper = Flags::CREEPER.bits(),
    Skeleton = Flags::SKELETON.bits(),
    Spider = Flags::SPIDER.bits(),
    PigZombie = Flags::PIGZOMBIE.bits(),
    Slime = Flags::SLIME.bits(),
    EnderMan = Flags::ENDERMAN.bits(),
    Silverfish = Flags::SILVERFISH.bits(),
    CaveSpider = Flags::CAVESPIDER.bits(),
    Ghast = Flags::GHAST.bits(),
    LavaSlime = Flags::LAVASLIME.bits(),
    Blaze = Flags::BLAZE.bits(),
    ZombieVillager = Flags::ZOMBIEVILLAGER.bits(),
    Witch = Flags::WITCH.bits(),
    Stray = Flags::STRAY.bits(),
    Husk = Flags::HUSK.bits(),
    WitherSkeleton = Flags::WITHERSKELETON.bits(),
    Guardian = Flags::GUARDIAN.bits(),
    ElderGuardian = Flags::ELDERGUARDIAN.bits(),
    Npc = Flags::NPC.bits(),
    WitherBoss = Flags::WITHERBOSS.bits(),
    Dragon = Flags::DRAGON.bits(),
    Shulker = Flags::SHULKER.bits(),
    Endermite = Flags::ENDERMITE.bits(),
    Agent = Flags::AGENT.bits(),
    Vindicator = Flags::VINDICATOR.bits(),
    Phantom = Flags::PHANTOM.bits(),
    IllagerBeast = Flags::ILLAGERBEAST.bits(),
    ArmorStand = Flags::ARMORSTAND.bits(),
    TripodCamera = Flags::TRIPODCAMERA.bits(),
    Player = Flags::PLAYER.bits(),
    ItemEntity = Flags::ITEMENTITY.bits(),
    PrimedTnt = Flags::PRIMEDTNT.bits(),
    FallingBlock = Flags::FALLINGBLOCK.bits(),
    MovingBlock = Flags::MOVINGBLOCK.bits(),
    ExperiencePotion = Flags::EXPERIENCEPOTION.bits(),
    Experience = Flags::EXPERIENCE.bits(),
    EyeOfEnder = Flags::EYEOFENDER.bits(),
    EnderCrystal = Flags::ENDERCRYSTAL.bits(),
    FireworksRocket = Flags::FIREWORKSROCKET.bits(),
    Trident = Flags::TRIDENT.bits(),
    Turtle = Flags::TURTLE.bits(),
    Cat = Flags::CAT.bits(),
    ShulkerBullet = Flags::SHULKERBULLET.bits(),
    FishingHook = Flags::FISHINGHOOK.bits(),
    Chalkboard = Flags::CHALKBOARD.bits(),
    DragonFireball = Flags::DRAGONFIREBALL.bits(),
    Arrow = Flags::ARROW.bits(),
    Snowball = Flags::SNOWBALL.bits(),
    ThrownEgg = Flags::THROWNEGG.bits(),
    Painting = Flags::PAINTING.bits(),
    LargeFireball = Flags::LARGEFIREBALL.bits(),
    ThrownPotion = Flags::THROWNPOTION.bits(),
    Enderpearl = Flags::ENDERPEARL.bits(),
    LeashKnot = Flags::LEASHKNOT.bits(),
    WitherSkull = Flags::WITHERSKULL.bits(),
    BoatRideable = Flags::BOATRIDEABLE.bits(),
    WitherSkullDangerous = Flags::WITHERSKULLDANGEROUS.bits(),
    LightningBolt = Flags::LIGHTNINGBOLT.bits(),
    SmallFireball = Flags::SMALLFIREBALL.bits(),
    AreaEffectCloud = Flags::AREAEFFECTCLOUD.bits(),
    LingeringPotion = Flags::LINGERINGPOTION.bits(),
    LlamaSpit = Flags::LLAMASPIT.bits(),
    EvocationFang = Flags::EVOCATIONFANG.bits(),
    EvocationIllager = Flags::EVOCATIONILLAGER.bits(),
    Vex = Flags::VEX.bits(),
    MinecartRideable = Flags::MINECARTRIDEABLE.bits(),
    MinecartHopper = Flags::MINECARTHOPPER.bits(),
    MinecartTNT = Flags::MINECARTTNT.bits(),
    MinecartChest = Flags::MINECARTCHEST.bits(),
    MinecartFurnace = Flags::MINECARTFURNACE.bits(),
    MinecartCommandBlock = Flags::MINECARTCOMMANDBLOCK.bits(),
    IceBomb = Flags::ICEBOMB.bits(),
    Balloon = Flags::BALLOON.bits(),
    Pufferfish = Flags::PUFFERFISH.bits(),
    Salmon = Flags::SALMON.bits(),
    Drowned = Flags::DROWNED.bits(),
    Tropicalfish = Flags::TROPICALFISH.bits(),
    Fish = Flags::FISH.bits(),
    Panda = Flags::PANDA.bits(),
    Pillager = Flags::PILLAGER.bits(),
    VillagerV2 = Flags::VILLAGERV2.bits(),
    ZombieVillagerV2 = Flags::ZOMBIEVILLAGERV2.bits(),
    Shield = Flags::SHIELD.bits(),
    WanderingTrader = Flags::WANDERINGTRADER.bits(),
    Lectern = Flags::LECTERN.bits(),
    ElderGuardianGhost = Flags::ELDERGUARDIANGHOST.bits(),
    Fox = Flags::FOX.bits(),
    Bee = Flags::BEE.bits(),
    Piglin = Flags::PIGLIN.bits(),
    Hoglin = Flags::HOGLIN.bits(),
    Strider = Flags::STRIDER.bits(),
    Zoglin = Flags::ZOGLIN.bits(),
    PiglinBrute = Flags::PIGLINBRUTE.bits(),
    Goat = Flags::GOAT.bits(),
    GlowSquid = Flags::GLOWSQUID.bits(),
    Axolotl = Flags::AXOLOTL.bits(),
    Warden = Flags::WARDEN.bits(),
    Frog = Flags::FROG.bits(),
    Tadpole = Flags::TADPOLE.bits(),
    Allay = Flags::ALLAY.bits(),
    ChestBoatRideable = Flags::CHESTBOATRIDEABLE.bits(),
    TraderLlama = Flags::TRADERLLAMA.bits(),
    Camel = Flags::CAMEL.bits(),
    Sniffer = Flags::SNIFFER.bits(),
    Breeze = Flags::BREEZE.bits(),
    BreezeWindChargeProjectile = Flags::BREEZEWINDCHARGEPROJECTILE.bits(),
    Armadillo = Flags::ARMADILLO.bits(),
    WindChargeProjectile = Flags::WINDCHARGEPROJECTILE.bits(),
    Bogged = Flags::BOGGED.bits(),
    OminousItemSpawner = Flags::OMINOUSITEMSPAWNER.bits(),
}

impl ActorType {
    pub fn to_string(self) -> String {
        match self {
            ActorType::Undefined => unimplemented!(),
            ActorType::TypeMask => unimplemented!(),
            ActorType::Mob => unimplemented!(),
            ActorType::PathfinderMob => unimplemented!(),
            ActorType::Monster => unimplemented!(),
            ActorType::Animal => unimplemented!(),
            ActorType::TamableAnimal => unimplemented!(),
            ActorType::Ambient => unimplemented!(),
            ActorType::UndeadMob => unimplemented!(),
            ActorType::ZombieMonster => unimplemented!(),
            ActorType::Arthropod => unimplemented!(),
            ActorType::Minecart => String::from("minecraft:minecart"),
            ActorType::SkeletonMonster => unimplemented!(),
            ActorType::EquineAnimal => unimplemented!(),
            ActorType::Projectile => unimplemented!(),
            ActorType::AbstractArrow => unimplemented!(),
            ActorType::WaterAnimal => unimplemented!(),
            ActorType::VillagerBase => unimplemented!(),
            ActorType::Chicken => String::from("minecraft:chicken"),
            ActorType::Cow => String::from("minecraft:cow"),
            ActorType::Pig => String::from("minecraft:pig"),
            ActorType::Sheep => String::from("minecraft:sheep"),
            ActorType::Wolf => String::from("minecraft:wolf"),
            ActorType::Villager => String::from("minecraft:villager"),
            ActorType::MushroomCow => unimplemented!(),
            ActorType::Squid => String::from("minecraft:squid"),
            ActorType::Rabbit => String::from("minecraft:rabbit"),
            ActorType::Bat => String::from("minecraft:bat"),
            ActorType::IronGolem => String::from("minecraft:iron_golem"),
            ActorType::SnowGolem => String::from("minecraft:snow_golem"),
            ActorType::Ocelot => String::from("minecraft:ocelot"),
            ActorType::Horse => String::from("minecraft:zombie_horse"),
            ActorType::PolarBear => String::from("minecraft:polar_bear"),
            ActorType::Llama => String::from("minecraft:llama_spit"),
            ActorType::Parrot => String::from("minecraft:parrot"),
            ActorType::Dolphin => String::from("minecraft:dolphin"),
            ActorType::Donkey => String::from("minecraft:donkey"),
            ActorType::Mule => String::from("minecraft:mule"),
            ActorType::SkeletonHorse => String::from("minecraft:skeleton_horse"),
            ActorType::ZombieHorse => String::from("minecraft:zombie_horse"),
            ActorType::Zombie => String::from("minecraft:zombie"),
            ActorType::Creeper => String::from("minecraft:creeper"),
            ActorType::Skeleton => String::from("minecraft:skeleton"),
            ActorType::Spider => String::from("minecraft:spider"),
            ActorType::PigZombie => unimplemented!(),
            ActorType::Slime => String::from("minecraft:slime"),
            ActorType::EnderMan => String::from("minecraft:enderman"),
            ActorType::Silverfish => String::from("minecraft:silverfish"),
            ActorType::CaveSpider => String::from("minecraft:cave_spider"),
            ActorType::Ghast => String::from("minecraft:ghast"),
            ActorType::LavaSlime => String::from("minecraft:magma_cube"),
            ActorType::Blaze => String::from("minecraft:blaze"),
            ActorType::ZombieVillager => String::from("minecraft:zombie_villager"),
            ActorType::Witch => String::from("minecraft:witch"),
            ActorType::Stray => String::from("minecraft:stray"),
            ActorType::Husk => String::from("minecraft:husk"),
            ActorType::WitherSkeleton => String::from("minecraft:wither_skeleton"),
            ActorType::Guardian => String::from("minecraft:guardian"),
            ActorType::ElderGuardian => String::from("minecraft:elder_guardian"),
            ActorType::Npc => String::from("minecraft:npc"),
            ActorType::WitherBoss => unimplemented!(),
            ActorType::Dragon => unimplemented!(),
            ActorType::Shulker => String::from("minecraft:shulker"),
            ActorType::Endermite => String::from("minecraft:endermite"),
            ActorType::Agent => String::from("minecraft:agent"),
            ActorType::Vindicator => String::from("minecraft:vindicator"),
            ActorType::Phantom => String::from("minecraft:phantom"),
            ActorType::IllagerBeast => unimplemented!(),
            ActorType::ArmorStand => String::from("minecraft:armor_stand"),
            ActorType::TripodCamera => unimplemented!(),
            ActorType::Player => String::from("minecraft:player"),
            ActorType::ItemEntity => unimplemented!(),
            ActorType::PrimedTnt => String::from("minecraft:tnt"),
            ActorType::FallingBlock => String::from("minecraft:falling_block"),
            ActorType::MovingBlock => unimplemented!(),
            ActorType::ExperiencePotion => unimplemented!(),
            ActorType::Experience => unimplemented!(),
            ActorType::EyeOfEnder => unimplemented!(),
            ActorType::EnderCrystal => unimplemented!(),
            ActorType::FireworksRocket => unimplemented!(),
            ActorType::Trident => unimplemented!(),
            ActorType::Turtle => String::from("minecraft:turtle"),
            ActorType::Cat => String::from("minecraft:cat"),
            ActorType::ShulkerBullet => String::from("minecraft:shulker_bullet"),
            ActorType::FishingHook => String::from("minecraft:fishing_hook"),
            ActorType::Chalkboard => unimplemented!(),
            ActorType::DragonFireball => unimplemented!(),
            ActorType::Arrow => String::from("minecraft:arrow"),
            ActorType::Snowball => String::from("minecraft:snowball"),
            ActorType::ThrownEgg => unimplemented!(),
            ActorType::Painting => String::from("minecraft:painting"),
            ActorType::LargeFireball => unimplemented!(),
            ActorType::ThrownPotion => unimplemented!(),
            ActorType::Enderpearl => String::from("minecraft:ender_pearl"),
            ActorType::LeashKnot => unimplemented!(),
            ActorType::WitherSkull => String::from("minecraft:wither_skull"),
            ActorType::BoatRideable => unimplemented!(),
            ActorType::WitherSkullDangerous => unimplemented!(),
            ActorType::LightningBolt => unimplemented!(),
            ActorType::SmallFireball => String::from("minecraft:small_fireball"),
            ActorType::AreaEffectCloud => String::from("minecraft:area_effect_cloud"),
            ActorType::LingeringPotion => unimplemented!(),
            ActorType::LlamaSpit => String::from("minecraft:llama_spit"),
            ActorType::EvocationFang => String::from("minecraft:evocation_fang"),
            ActorType::EvocationIllager => unimplemented!(),
            ActorType::Vex => String::from("minecraft:vex"),
            ActorType::MinecartRideable => unimplemented!(),
            ActorType::MinecartHopper => String::from("minecraft:hopper_minecart"),
            ActorType::MinecartTNT => String::from("minecraft:tnt_minecart"),
            ActorType::MinecartChest => String::from("minecraft:chest_minecart"),
            ActorType::MinecartFurnace => unimplemented!(),
            ActorType::MinecartCommandBlock => unimplemented!(),
            ActorType::IceBomb => unimplemented!(),
            ActorType::Balloon => String::from("minecraft:balloon"),
            ActorType::Pufferfish => String::from("minecraft:pufferfish"),
            ActorType::Salmon => String::from("minecraft:salmon"),
            ActorType::Drowned => String::from("minecraft:drowned"),
            ActorType::Tropicalfish => String::from("minecraft:tropicalfish"),
            ActorType::Fish => unimplemented!(),
            ActorType::Panda => String::from("minecraft:panda"),
            ActorType::Pillager => String::from("minecraft:pillager"),
            ActorType::VillagerV2 => String::from("minecraft:villager_v2"),
            ActorType::ZombieVillagerV2 => String::from("minecraft:zombie_villager_v2"),
            ActorType::Shield => unimplemented!(),
            ActorType::WanderingTrader => String::from("minecraft:wandering_trader"),
            ActorType::Lectern => unimplemented!(),
            ActorType::ElderGuardianGhost => unimplemented!(),
            ActorType::Fox => String::from("minecraft:fox"),
            ActorType::Bee => String::from("minecraft:bee"),
            ActorType::Piglin => String::from("minecraft:piglin"),
            ActorType::Hoglin => String::from("minecraft:hoglin"),
            ActorType::Strider => String::from("minecraft:strider"),
            ActorType::Zoglin => String::from("minecraft:zoglin"),
            ActorType::PiglinBrute => String::from("minecraft:piglin_brute"),
            ActorType::Goat => String::from("minecraft:goat"),
            ActorType::GlowSquid => String::from("minecraft:glow_squid"),
            ActorType::Axolotl => String::from("minecraft:axolotl"),
            ActorType::Warden => String::from("minecraft:warden"),
            ActorType::Frog => String::from("minecraft:frog"),
            ActorType::Tadpole => String::from("minecraft:tadpole"),
            ActorType::Allay => String::from("minecraft:allay"),
            ActorType::ChestBoatRideable => unimplemented!(),
            ActorType::TraderLlama => unimplemented!(),
            ActorType::Camel => String::from("minecraft:camel"),
            ActorType::Sniffer => unimplemented!(),
            ActorType::Breeze => unimplemented!(),
            ActorType::BreezeWindChargeProjectile => unimplemented!(),
            ActorType::Armadillo => unimplemented!(),
            ActorType::WindChargeProjectile => unimplemented!(),
            ActorType::Bogged => unimplemented!(),
            ActorType::OminousItemSpawner => unimplemented!(),
        }
    }
}

impl ProtoCodec for ActorType {
    fn proto_serialize(
        &self,
        _stream: &mut Vec<u8>,
    ) -> Result<(), bedrockrs_proto_core::error::ProtoCodecError> {
        unimplemented!()
    }

    fn proto_deserialize(
        _stream: &mut std::io::Cursor<&[u8]>,
    ) -> Result<Self, bedrockrs_proto_core::error::ProtoCodecError> {
        unimplemented!()
    }

    fn get_size_prediction(&self) -> usize {
        unimplemented!()
    }
}

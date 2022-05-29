use crate::BlockBehavior;
use block_macros::make_block_states;

pub trait Block {
    fn behavior(&self) -> BlockBehavior;
    fn id(&self) -> &'static str;
}

make_block_states! {
    Properties => {
        Face {
            Floor,
            Wall,
            Ceiling,
        },
        Facing {
            North,
            South,
            West,
            East,
        },
        Powered {
            True,
            False,
        },
        Half {
            Top,
            Bottom,
        },
        Hinge {
            Left,
            Right,
        },
        Open {
            True,
            False,
        },
        East {
            True,
            False,
        },
        North {
            True,
            False,
        },
        South {
            True,
            False,
        },
        Waterlogged {
            True,
            False,
        },
        West {
            True,
            False,
        },
        InWall {
            True,
            False,
        },
        Distance {
            _1,
            _2,
            _3,
            _4,
            _5,
            _6,
            _7,
        },
        Persistent {
            True,
            False,
        },
        Axis {
            X,
            Y,
            Z,
        },
        Stage {
            _0,
            _1,
        },
        Rotation {
            _0,
            _1,
            _2,
            _3,
            _4,
            _5,
            _6,
            _7,
            _8,
            _9,
            _10,
            _11,
            _12,
            _13,
            _14,
            _15,
        },
        Type {
            Top,
            Bottom,
            Double,
        },
        Shape {
            Straight,
            InnerLeft,
            InnerRight,
            OuterLeft,
            OuterRight,
        },
        Up {
            True,
            False,
        },
        Age {
            _0,
            _1,
            _2,
            _3,
            _4,
            _5,
            _6,
            _7,
        },
        Leaves {
            None,
            Small,
            Large,
        },
        HoneyLevel {
            _0,
            _1,
            _2,
            _3,
            _4,
            _5,
        },
        Attachment {
            Floor,
            Ceiling,
            SingleWall,
            DoubleWall,
        },
        Tilt {
            None,
            Unstable,
            Partial,
            Full,
        },
        Occupied {
            True,
            False,
        },
        Part {
            Head,
            Foot,
        },
        Candles {
            _1,
            _2,
            _3,
            _4,
        },
        Lit {
            True,
            False,
        },
        HasBottle_0 {
            True,
            False,
        },
        HasBottle_1 {
            True,
            False,
        },
        HasBottle_2 {
            True,
            False,
        },
        Down {
            True,
            False,
        },
        Drag {
            True,
            False,
        },
        Bites {
            _0,
            _1,
            _2,
            _3,
            _4,
            _5,
            _6,
        },
        SignalFire {
            True,
            False,
        },
        Berries {
            True,
            False,
        },
        Conditional {
            True,
            False,
        },
        Mode {
            Save,
            Load,
            Corner,
            Data,
        },
        Level {
            _1,
            _2,
            _3,
        },
        Inverted {
            True,
            False,
        },
        Power {
            _0,
            _1,
            _2,
            _3,
            _4,
            _5,
            _6,
            _7,
            _8,
            _9,
            _10,
            _11,
            _12,
            _13,
            _14,
            _15,
        },
        Triggered {
            True,
            False,
        },
        Eye {
            True,
            False,
        },
        Moisture {
            _0,
            _1,
            _2,
            _3,
            _4,
            _5,
            _6,
            _7,
        },
        Snowy {
            True,
            False,
        },
        Enabled {
            True,
            False,
        },
        Orientation {
            DownEast,
            DownNorth,
            DownSouth,
            DownWest,
            UpEast,
            UpNorth,
            UpSouth,
            UpWest,
            WestUp,
            EastUp,
            NorthUp,
            SouthUp,
        },
        HasRecord {
            True,
            False,
        },
        Hanging {
            True,
            False,
        },
        HasBook {
            True,
            False,
        },
        Instrument {
            Harp,
            Basedrum,
            Snare,
            Hat,
            Bass,
            Flute,
            Bell,
            Guitar,
            Chime,
            Xylophone,
            IronXylophone,
            CowBell,
            Didgeridoo,
            Bit,
            Banjo,
            Pling,
        },
        Note {
            _0,
            _1,
            _2,
            _3,
            _4,
            _5,
            _6,
            _7,
            _8,
            _9,
            _10,
            _11,
            _12,
            _13,
            _14,
            _15,
            _16,
            _17,
            _18,
            _19,
            _20,
            _21,
            _22,
            _23,
            _24,
        },
        Extended {
            True,
            False,
        },
        Short {
            True,
            False,
        },
        Thickness {
            TipMerge,
            Tip,
            Frustum,
            Middle,
            Base,
        },
        VerticalDirection {
            Up,
            Down,
        },
        Delay {
            _1,
            _2,
            _3,
            _4,
        },
        Locked {
            True,
            False,
        },
        Charges {
            _0,
            _1,
            _2,
            _3,
            _4,
        },
        Bottom {
            True,
            False,
        },
        Bloom {
            True,
            False,
        },
        SculkSensorPhase {
            Inactive,
            Active,
            Cooldown,
        },
        CanSummon {
            True,
            False,
        },
        Shrieking {
            True,
            False,
        },
        Pickles {
            _1,
            _2,
            _3,
            _4,
        },
        Layers {
            _1,
            _2,
            _3,
            _4,
            _5,
            _6,
            _7,
            _8,
        },
        Unstable {
            True,
            False,
        },
        Attached {
            True,
            False,
        },
        Disarmed {
            True,
            False,
        },
        Eggs {
            _1,
            _2,
            _3,
            _4,
        },
        Hatch {
            _0,
            _1,
            _2,
        },
    },
    Blocks => {
        acacia_button => BlockBehavior::default(), {
            Face=Wall,
            Facing=North,
            Powered=False,
        },
        acacia_door => BlockBehavior::default(), {
            Facing=North,
            Half=Lower,
            Hinge=Left,
            Open=False,
            Powered=False,
        },
        acacia_fence => BlockBehavior::default(), {
            East=False,
            North=False,
            South=False,
            Waterlogged=False,
            West=False,
        },
        acacia_fence_gate => BlockBehavior::default(), {
            Facing=North,
            InWall=False,
            Open=False,
            Powered=False,
        },
        acacia_leaves => BlockBehavior::default(), {
            Distance=_7,
            Persistent=False,
            Waterlogged=False,
        },
        acacia_log => BlockBehavior::default(), {
            Axis=Y,
        },
        acacia_planks => BlockBehavior::default(), {
        },
        acacia_pressure_plate => BlockBehavior::default(), {
            Powered=False,
        },
        acacia_sapling => BlockBehavior::default(), {
            Stage=_0,
        },
        acacia_sign => BlockBehavior::default(), {
            Rotation=_0,
            Waterlogged=False,
        },
        acacia_slab => BlockBehavior::default(), {
            Type=Bottom,
            Waterlogged=False,
        },
        acacia_stairs => BlockBehavior::default(), {
            Facing=North,
            Half=Bottom,
            Shape=Straight,
            Waterlogged=False,
        },
        acacia_trapdoor => BlockBehavior::default(), {
            Facing=North,
            Half=Bottom,
            Open=False,
            Powered=False,
            Waterlogged=False,
        },
        acacia_wall_sign => BlockBehavior::default(), {
            Facing=North,
            Waterlogged=False,
        },
        acacia_wood => BlockBehavior::default(), {
            Axis=Y,
        },
        activator_rail => BlockBehavior::default(), {
            Powered=False,
            Shape=NorthSouth,
            Waterlogged=False,
        },
        air => BlockBehavior::default(), {
        },
        allium => BlockBehavior::default(), {
        },
        amethyst_block => BlockBehavior::default(), {
        },
        amethyst_cluster => BlockBehavior::default(), {
            Facing=Up,
            Waterlogged=False,
        },
        ancient_debris => BlockBehavior::default(), {
        },
        andesite => BlockBehavior::default(), {
        },
        andesite_slab => BlockBehavior::default(), {
            Type=Bottom,
            Waterlogged=False,
        },
        andesite_stairs => BlockBehavior::default(), {
            Facing=North,
            Half=Bottom,
            Shape=Straight,
            Waterlogged=False,
        },
        andesite_wall => BlockBehavior::default(), {
            East=None,
            North=None,
            South=None,
            Up=True,
            Waterlogged=False,
            West=None,
        },
        anvil => BlockBehavior::default(), {
            Facing=North,
        },
        attached_melon_stem => BlockBehavior::default(), {
            Facing=North,
        },
        attached_pumpkin_stem => BlockBehavior::default(), {
            Facing=North,
        },
        azalea => BlockBehavior::default(), {
        },
        azalea_leaves => BlockBehavior::default(), {
            Distance=_7,
            Persistent=False,
            Waterlogged=False,
        },
        azure_bluet => BlockBehavior::default(), {
        },
        bamboo => BlockBehavior::default(), {
            Age=_0,
            Leaves=None,
            Stage=_0,
        },
        bamboo_sapling => BlockBehavior::default(), {
        },
        barrel => BlockBehavior::default(), {
            Facing=North,
            Open=False,
        },
        barrier => BlockBehavior::default(), {
        },
        basalt => BlockBehavior::default(), {
            Axis=Y,
        },
        beacon => BlockBehavior::default(), {
        },
        bedrock => BlockBehavior::default(), {
        },
        bee_nest => BlockBehavior::default(), {
            Facing=North,
            HoneyLevel=_0,
        },
        beehive => BlockBehavior::default(), {
            Facing=North,
            HoneyLevel=_0,
        },
        beetroots => BlockBehavior::default(), {
            Age=_0,
        },
        bell => BlockBehavior::default(), {
            Attachment=Floor,
            Facing=North,
            Powered=False,
        },
        big_dripleaf => BlockBehavior::default(), {
            Facing=North,
            Tilt=None,
            Waterlogged=False,
        },
        big_dripleaf_stem => BlockBehavior::default(), {
            Facing=North,
            Waterlogged=False,
        },
        birch_button => BlockBehavior::default(), {
            Face=Wall,
            Facing=North,
            Powered=False,
        },
        birch_door => BlockBehavior::default(), {
            Facing=North,
            Half=Lower,
            Hinge=Left,
            Open=False,
            Powered=False,
        },
        birch_fence => BlockBehavior::default(), {
            East=False,
            North=False,
            South=False,
            Waterlogged=False,
            West=False,
        },
        birch_fence_gate => BlockBehavior::default(), {
            Facing=North,
            InWall=False,
            Open=False,
            Powered=False,
        },
        birch_leaves => BlockBehavior::default(), {
            Distance=_7,
            Persistent=False,
            Waterlogged=False,
        },
        birch_log => BlockBehavior::default(), {
            Axis=Y,
        },
        birch_planks => BlockBehavior::default(), {
        },
        birch_pressure_plate => BlockBehavior::default(), {
            Powered=False,
        },
        birch_sapling => BlockBehavior::default(), {
            Stage=_0,
        },
        birch_sign => BlockBehavior::default(), {
            Rotation=_0,
            Waterlogged=False,
        },
        birch_slab => BlockBehavior::default(), {
            Type=Bottom,
            Waterlogged=False,
        },
        birch_stairs => BlockBehavior::default(), {
            Facing=North,
            Half=Bottom,
            Shape=Straight,
            Waterlogged=False,
        },
        birch_trapdoor => BlockBehavior::default(), {
            Facing=North,
            Half=Bottom,
            Open=False,
            Powered=False,
            Waterlogged=False,
        },
        birch_wall_sign => BlockBehavior::default(), {
            Facing=North,
            Waterlogged=False,
        },
        birch_wood => BlockBehavior::default(), {
            Axis=Y,
        },
        black_banner => BlockBehavior::default(), {
            Rotation=_0,
        },
        black_bed => BlockBehavior::default(), {
            Facing=North,
            Occupied=False,
            Part=Foot,
        },
        black_candle => BlockBehavior::default(), {
            Candles=_1,
            Lit=False,
            Waterlogged=False,
        },
        black_candle_cake => BlockBehavior::default(), {
            Lit=False,
        },
        black_carpet => BlockBehavior::default(), {
        },
        black_concrete => BlockBehavior::default(), {
        },
        black_concrete_powder => BlockBehavior::default(), {
        },
        black_glazed_terracotta => BlockBehavior::default(), {
            Facing=North,
        },
        black_shulker_box => BlockBehavior::default(), {
            Facing=Up,
        },
        black_stained_glass => BlockBehavior::default(), {
        },
        black_stained_glass_pane => BlockBehavior::default(), {
            East=False,
            North=False,
            South=False,
            Waterlogged=False,
            West=False,
        },
        black_terracotta => BlockBehavior::default(), {
        },
        black_wall_banner => BlockBehavior::default(), {
            Facing=North,
        },
        black_wool => BlockBehavior::default(), {
        },
        blackstone => BlockBehavior::default(), {
        },
        blackstone_slab => BlockBehavior::default(), {
            Type=Bottom,
            Waterlogged=False,
        },
        blackstone_stairs => BlockBehavior::default(), {
            Facing=North,
            Half=Bottom,
            Shape=Straight,
            Waterlogged=False,
        },
        blackstone_wall => BlockBehavior::default(), {
            East=None,
            North=None,
            South=None,
            Up=True,
            Waterlogged=False,
            West=None,
        },
        blast_furnace => BlockBehavior::default(), {
            Facing=North,
            Lit=False,
        },
        blue_banner => BlockBehavior::default(), {
            Rotation=_0,
        },
        blue_bed => BlockBehavior::default(), {
            Facing=North,
            Occupied=False,
            Part=Foot,
        },
        blue_candle => BlockBehavior::default(), {
            Candles=_1,
            Lit=False,
            Waterlogged=False,
        },
        blue_candle_cake => BlockBehavior::default(), {
            Lit=False,
        },
        blue_carpet => BlockBehavior::default(), {
        },
        blue_concrete => BlockBehavior::default(), {
        },
        blue_concrete_powder => BlockBehavior::default(), {
        },
        blue_glazed_terracotta => BlockBehavior::default(), {
            Facing=North,
        },
        blue_ice => BlockBehavior::default(), {
        },
        blue_orchid => BlockBehavior::default(), {
        },
        blue_shulker_box => BlockBehavior::default(), {
            Facing=Up,
        },
        blue_stained_glass => BlockBehavior::default(), {
        },
        blue_stained_glass_pane => BlockBehavior::default(), {
            East=False,
            North=False,
            South=False,
            Waterlogged=False,
            West=False,
        },
        blue_terracotta => BlockBehavior::default(), {
        },
        blue_wall_banner => BlockBehavior::default(), {
            Facing=North,
        },
        blue_wool => BlockBehavior::default(), {
        },
        bone_block => BlockBehavior::default(), {
            Axis=Y,
        },
        bookshelf => BlockBehavior::default(), {
        },
        brain_coral => BlockBehavior::default(), {
            Waterlogged=True,
        },
        brain_coral_block => BlockBehavior::default(), {
        },
        brain_coral_fan => BlockBehavior::default(), {
            Waterlogged=True,
        },
        brain_coral_wall_fan => BlockBehavior::default(), {
            Facing=North,
            Waterlogged=True,
        },
        brewing_stand => BlockBehavior::default(), {
            HasBottle_0=False,
            HasBottle_1=False,
            HasBottle_2=False,
        },
        brick_slab => BlockBehavior::default(), {
            Type=Bottom,
            Waterlogged=False,
        },
        brick_stairs => BlockBehavior::default(), {
            Facing=North,
            Half=Bottom,
            Shape=Straight,
            Waterlogged=False,
        },
        brick_wall => BlockBehavior::default(), {
            East=None,
            North=None,
            South=None,
            Up=True,
            Waterlogged=False,
            West=None,
        },
        bricks => BlockBehavior::default(), {
        },
        brown_banner => BlockBehavior::default(), {
            Rotation=_0,
        },
        brown_bed => BlockBehavior::default(), {
            Facing=North,
            Occupied=False,
            Part=Foot,
        },
        brown_candle => BlockBehavior::default(), {
            Candles=_1,
            Lit=False,
            Waterlogged=False,
        },
        brown_candle_cake => BlockBehavior::default(), {
            Lit=False,
        },
        brown_carpet => BlockBehavior::default(), {
        },
        brown_concrete => BlockBehavior::default(), {
        },
        brown_concrete_powder => BlockBehavior::default(), {
        },
        brown_glazed_terracotta => BlockBehavior::default(), {
            Facing=North,
        },
        brown_mushroom => BlockBehavior::default(), {
        },
        brown_mushroom_block => BlockBehavior::default(), {
            Down=True,
            East=True,
            North=True,
            South=True,
            Up=True,
            West=True,
        },
        brown_shulker_box => BlockBehavior::default(), {
            Facing=Up,
        },
        brown_stained_glass => BlockBehavior::default(), {
        },
        brown_stained_glass_pane => BlockBehavior::default(), {
            East=False,
            North=False,
            South=False,
            Waterlogged=False,
            West=False,
        },
        brown_terracotta => BlockBehavior::default(), {
        },
        brown_wall_banner => BlockBehavior::default(), {
            Facing=North,
        },
        brown_wool => BlockBehavior::default(), {
        },
        bubble_column => BlockBehavior::default(), {
            Drag=True,
        },
        bubble_coral => BlockBehavior::default(), {
            Waterlogged=True,
        },
        bubble_coral_block => BlockBehavior::default(), {
        },
        bubble_coral_fan => BlockBehavior::default(), {
            Waterlogged=True,
        },
        bubble_coral_wall_fan => BlockBehavior::default(), {
            Facing=North,
            Waterlogged=True,
        },
        budding_amethyst => BlockBehavior::default(), {
        },
        cactus => BlockBehavior::default(), {
            Age=_0,
        },
        cake => BlockBehavior::default(), {
            Bites=_0,
        },
        calcite => BlockBehavior::default(), {
        },
        campfire => BlockBehavior::default(), {
            Facing=North,
            Lit=True,
            SignalFire=False,
            Waterlogged=False,
        },
        candle => BlockBehavior::default(), {
            Candles=_1,
            Lit=False,
            Waterlogged=False,
        },
        candle_cake => BlockBehavior::default(), {
            Lit=False,
        },
        carrots => BlockBehavior::default(), {
            Age=_0,
        },
        cartography_table => BlockBehavior::default(), {
        },
        carved_pumpkin => BlockBehavior::default(), {
            Facing=North,
        },
        cauldron => BlockBehavior::default(), {
        },
        cave_air => BlockBehavior::default(), {
        },
        cave_vines => BlockBehavior::default(), {
            Age=_0,
            Berries=False,
        },
        cave_vines_plant => BlockBehavior::default(), {
            Berries=False,
        },
        chain => BlockBehavior::default(), {
            Axis=Y,
            Waterlogged=False,
        },
        chain_command_block => BlockBehavior::default(), {
            Conditional=False,
            Facing=North,
        },
        chest => BlockBehavior::default(), {
            Type=Single,
            Facing=North,
            Waterlogged=False,
        },
        chipped_anvil => BlockBehavior::default(), {
            Facing=North,
        },
        chiseled_deepslate => BlockBehavior::default(), {
        },
        chiseled_nether_bricks => BlockBehavior::default(), {
        },
        chiseled_polished_blackstone => BlockBehavior::default(), {
        },
        chiseled_quartz_block => BlockBehavior::default(), {
        },
        chiseled_red_sandstone => BlockBehavior::default(), {
        },
        chiseled_sandstone => BlockBehavior::default(), {
        },
        chiseled_stone_bricks => BlockBehavior::default(), {
        },
        chorus_flower => BlockBehavior::default(), {
            Age=_0,
        },
        chorus_plant => BlockBehavior::default(), {
            Down=False,
            East=False,
            North=False,
            South=False,
            Up=False,
            West=False,
        },
        clay => BlockBehavior::default(), {
        },
        coal_block => BlockBehavior::default(), {
        },
        coal_ore => BlockBehavior::default(), {
        },
        coarse_dirt => BlockBehavior::default(), {
        },
        cobbled_deepslate => BlockBehavior::default(), {
        },
        cobbled_deepslate_slab => BlockBehavior::default(), {
            Type=Bottom,
            Waterlogged=False,
        },
        cobbled_deepslate_stairs => BlockBehavior::default(), {
            Facing=North,
            Half=Bottom,
            Shape=Straight,
            Waterlogged=False,
        },
        cobbled_deepslate_wall => BlockBehavior::default(), {
            East=None,
            North=None,
            South=None,
            Up=True,
            Waterlogged=False,
            West=None,
        },
        cobblestone => BlockBehavior::default(), {
        },
        cobblestone_slab => BlockBehavior::default(), {
            Type=Bottom,
            Waterlogged=False,
        },
        cobblestone_stairs => BlockBehavior::default(), {
            Facing=North,
            Half=Bottom,
            Shape=Straight,
            Waterlogged=False,
        },
        cobblestone_wall => BlockBehavior::default(), {
            East=None,
            North=None,
            South=None,
            Up=True,
            Waterlogged=False,
            West=None,
        },
        cobweb => BlockBehavior::default(), {
        },
        cocoa => BlockBehavior::default(), {
            Age=_0,
            Facing=North,
        },
        command_block => BlockBehavior::default(), {
            Conditional=False,
            Facing=North,
        },
        comparator => BlockBehavior::default(), {
            Facing=North,
            Mode=Compare,
            Powered=False,
        },
        composter => BlockBehavior::default(), {
            Level=_0,
        },
        conduit => BlockBehavior::default(), {
            Waterlogged=True,
        },
        copper_block => BlockBehavior::default(), {
        },
        copper_ore => BlockBehavior::default(), {
        },
        cornflower => BlockBehavior::default(), {
        },
        cracked_deepslate_bricks => BlockBehavior::default(), {
        },
        cracked_deepslate_tiles => BlockBehavior::default(), {
        },
        cracked_nether_bricks => BlockBehavior::default(), {
        },
        cracked_polished_blackstone_bricks => BlockBehavior::default(), {
        },
        cracked_stone_bricks => BlockBehavior::default(), {
        },
        crafting_table => BlockBehavior::default(), {
        },
        creeper_head => BlockBehavior::default(), {
            Rotation=_0,
        },
        creeper_wall_head => BlockBehavior::default(), {
            Facing=North,
        },
        crimson_button => BlockBehavior::default(), {
            Face=Wall,
            Facing=North,
            Powered=False,
        },
        crimson_door => BlockBehavior::default(), {
            Facing=North,
            Half=Lower,
            Hinge=Left,
            Open=False,
            Powered=False,
        },
        crimson_fence => BlockBehavior::default(), {
            East=False,
            North=False,
            South=False,
            Waterlogged=False,
            West=False,
        },
        crimson_fence_gate => BlockBehavior::default(), {
            Facing=North,
            InWall=False,
            Open=False,
            Powered=False,
        },
        crimson_fungus => BlockBehavior::default(), {
        },
        crimson_hyphae => BlockBehavior::default(), {
            Axis=Y,
        },
        crimson_nylium => BlockBehavior::default(), {
        },
        crimson_planks => BlockBehavior::default(), {
        },
        crimson_pressure_plate => BlockBehavior::default(), {
            Powered=False,
        },
        crimson_roots => BlockBehavior::default(), {
        },
        crimson_sign => BlockBehavior::default(), {
            Rotation=_0,
            Waterlogged=False,
        },
        crimson_slab => BlockBehavior::default(), {
            Type=Bottom,
            Waterlogged=False,
        },
        crimson_stairs => BlockBehavior::default(), {
            Facing=North,
            Half=Bottom,
            Shape=Straight,
            Waterlogged=False,
        },
        crimson_stem => BlockBehavior::default(), {
            Axis=Y,
        },
        crimson_trapdoor => BlockBehavior::default(), {
            Facing=North,
            Half=Bottom,
            Open=False,
            Powered=False,
            Waterlogged=False,
        },
        crimson_wall_sign => BlockBehavior::default(), {
            Facing=North,
            Waterlogged=False,
        },
        crying_obsidian => BlockBehavior::default(), {
        },
        cut_copper => BlockBehavior::default(), {
        },
        cut_copper_slab => BlockBehavior::default(), {
            Type=Bottom,
            Waterlogged=False,
        },
        cut_copper_stairs => BlockBehavior::default(), {
            Facing=North,
            Half=Bottom,
            Shape=Straight,
            Waterlogged=False,
        },
        cut_red_sandstone => BlockBehavior::default(), {
        },
        cut_red_sandstone_slab => BlockBehavior::default(), {
            Type=Bottom,
            Waterlogged=False,
        },
        cut_sandstone => BlockBehavior::default(), {
        },
        cut_sandstone_slab => BlockBehavior::default(), {
            Type=Bottom,
            Waterlogged=False,
        },
        cyan_banner => BlockBehavior::default(), {
            Rotation=_0,
        },
        cyan_bed => BlockBehavior::default(), {
            Facing=North,
            Occupied=False,
            Part=Foot,
        },
        cyan_candle => BlockBehavior::default(), {
            Candles=_1,
            Lit=False,
            Waterlogged=False,
        },
        cyan_candle_cake => BlockBehavior::default(), {
            Lit=False,
        },
        cyan_carpet => BlockBehavior::default(), {
        },
        cyan_concrete => BlockBehavior::default(), {
        },
        cyan_concrete_powder => BlockBehavior::default(), {
        },
        cyan_glazed_terracotta => BlockBehavior::default(), {
            Facing=North,
        },
        cyan_shulker_box => BlockBehavior::default(), {
            Facing=Up,
        },
        cyan_stained_glass => BlockBehavior::default(), {
        },
        cyan_stained_glass_pane => BlockBehavior::default(), {
            East=False,
            North=False,
            South=False,
            Waterlogged=False,
            West=False,
        },
        cyan_terracotta => BlockBehavior::default(), {
        },
        cyan_wall_banner => BlockBehavior::default(), {
            Facing=North,
        },
        cyan_wool => BlockBehavior::default(), {
        },
        damaged_anvil => BlockBehavior::default(), {
            Facing=North,
        },
        dandelion => BlockBehavior::default(), {
        },
        dark_oak_button => BlockBehavior::default(), {
            Face=Wall,
            Facing=North,
            Powered=False,
        },
        dark_oak_door => BlockBehavior::default(), {
            Facing=North,
            Half=Lower,
            Hinge=Left,
            Open=False,
            Powered=False,
        },
        dark_oak_fence => BlockBehavior::default(), {
            East=False,
            North=False,
            South=False,
            Waterlogged=False,
            West=False,
        },
        dark_oak_fence_gate => BlockBehavior::default(), {
            Facing=North,
            InWall=False,
            Open=False,
            Powered=False,
        },
        dark_oak_leaves => BlockBehavior::default(), {
            Distance=_7,
            Persistent=False,
            Waterlogged=False,
        },
        dark_oak_log => BlockBehavior::default(), {
            Axis=Y,
        },
        dark_oak_planks => BlockBehavior::default(), {
        },
        dark_oak_pressure_plate => BlockBehavior::default(), {
            Powered=False,
        },
        dark_oak_sapling => BlockBehavior::default(), {
            Stage=_0,
        },
        dark_oak_sign => BlockBehavior::default(), {
            Rotation=_0,
            Waterlogged=False,
        },
        dark_oak_slab => BlockBehavior::default(), {
            Type=Bottom,
            Waterlogged=False,
        },
        dark_oak_stairs => BlockBehavior::default(), {
            Facing=North,
            Half=Bottom,
            Shape=Straight,
            Waterlogged=False,
        },
        dark_oak_trapdoor => BlockBehavior::default(), {
            Facing=North,
            Half=Bottom,
            Open=False,
            Powered=False,
            Waterlogged=False,
        },
        dark_oak_wall_sign => BlockBehavior::default(), {
            Facing=North,
            Waterlogged=False,
        },
        dark_oak_wood => BlockBehavior::default(), {
            Axis=Y,
        },
        dark_prismarine => BlockBehavior::default(), {
        },
        dark_prismarine_slab => BlockBehavior::default(), {
            Type=Bottom,
            Waterlogged=False,
        },
        dark_prismarine_stairs => BlockBehavior::default(), {
            Facing=North,
            Half=Bottom,
            Shape=Straight,
            Waterlogged=False,
        },
        daylight_detector => BlockBehavior::default(), {
            Inverted=False,
            Power=_0,
        },
        dead_brain_coral => BlockBehavior::default(), {
            Waterlogged=True,
        },
        dead_brain_coral_block => BlockBehavior::default(), {
        },
        dead_brain_coral_fan => BlockBehavior::default(), {
            Waterlogged=True,
        },
        dead_brain_coral_wall_fan => BlockBehavior::default(), {
            Facing=North,
            Waterlogged=True,
        },
        dead_bubble_coral => BlockBehavior::default(), {
            Waterlogged=True,
        },
        dead_bubble_coral_block => BlockBehavior::default(), {
        },
        dead_bubble_coral_fan => BlockBehavior::default(), {
            Waterlogged=True,
        },
        dead_bubble_coral_wall_fan => BlockBehavior::default(), {
            Facing=North,
            Waterlogged=True,
        },
        dead_bush => BlockBehavior::default(), {
        },
        dead_fire_coral => BlockBehavior::default(), {
            Waterlogged=True,
        },
        dead_fire_coral_block => BlockBehavior::default(), {
        },
        dead_fire_coral_fan => BlockBehavior::default(), {
            Waterlogged=True,
        },
        dead_fire_coral_wall_fan => BlockBehavior::default(), {
            Facing=North,
            Waterlogged=True,
        },
        dead_horn_coral => BlockBehavior::default(), {
            Waterlogged=True,
        },
        dead_horn_coral_block => BlockBehavior::default(), {
        },
        dead_horn_coral_fan => BlockBehavior::default(), {
            Waterlogged=True,
        },
        dead_horn_coral_wall_fan => BlockBehavior::default(), {
            Facing=North,
            Waterlogged=True,
        },
        dead_tube_coral => BlockBehavior::default(), {
            Waterlogged=True,
        },
        dead_tube_coral_block => BlockBehavior::default(), {
        },
        dead_tube_coral_fan => BlockBehavior::default(), {
            Waterlogged=True,
        },
        dead_tube_coral_wall_fan => BlockBehavior::default(), {
            Facing=North,
            Waterlogged=True,
        },
        deepslate => BlockBehavior::default(), {
            Axis=Y,
        },
        deepslate_brick_slab => BlockBehavior::default(), {
            Type=Bottom,
            Waterlogged=False,
        },
        deepslate_brick_stairs => BlockBehavior::default(), {
            Facing=North,
            Half=Bottom,
            Shape=Straight,
            Waterlogged=False,
        },
        deepslate_brick_wall => BlockBehavior::default(), {
            East=None,
            North=None,
            South=None,
            Up=True,
            Waterlogged=False,
            West=None,
        },
        deepslate_bricks => BlockBehavior::default(), {
        },
        deepslate_coal_ore => BlockBehavior::default(), {
        },
        deepslate_copper_ore => BlockBehavior::default(), {
        },
        deepslate_diamond_ore => BlockBehavior::default(), {
        },
        deepslate_emerald_ore => BlockBehavior::default(), {
        },
        deepslate_gold_ore => BlockBehavior::default(), {
        },
        deepslate_iron_ore => BlockBehavior::default(), {
        },
        deepslate_lapis_ore => BlockBehavior::default(), {
        },
        deepslate_redstone_ore => BlockBehavior::default(), {
            Lit=False,
        },
        deepslate_tile_slab => BlockBehavior::default(), {
            Type=Bottom,
            Waterlogged=False,
        },
        deepslate_tile_stairs => BlockBehavior::default(), {
            Facing=North,
            Half=Bottom,
            Shape=Straight,
            Waterlogged=False,
        },
        deepslate_tile_wall => BlockBehavior::default(), {
            East=None,
            North=None,
            South=None,
            Up=True,
            Waterlogged=False,
            West=None,
        },
        deepslate_tiles => BlockBehavior::default(), {
        },
        detector_rail => BlockBehavior::default(), {
            Powered=False,
            Shape=NorthSouth,
            Waterlogged=False,
        },
        diamond_block => BlockBehavior::default(), {
        },
        diamond_ore => BlockBehavior::default(), {
        },
        diorite => BlockBehavior::default(), {
        },
        diorite_slab => BlockBehavior::default(), {
            Type=Bottom,
            Waterlogged=False,
        },
        diorite_stairs => BlockBehavior::default(), {
            Facing=North,
            Half=Bottom,
            Shape=Straight,
            Waterlogged=False,
        },
        diorite_wall => BlockBehavior::default(), {
            East=None,
            North=None,
            South=None,
            Up=True,
            Waterlogged=False,
            West=None,
        },
        dirt => BlockBehavior::default(), {
        },
        dirt_path => BlockBehavior::default(), {
        },
        dispenser => BlockBehavior::default(), {
            Facing=North,
            Triggered=False,
        },
        dragon_egg => BlockBehavior::default(), {
        },
        dragon_head => BlockBehavior::default(), {
            Rotation=_0,
        },
        dragon_wall_head => BlockBehavior::default(), {
            Facing=North,
        },
        dried_kelp_block => BlockBehavior::default(), {
        },
        dripstone_block => BlockBehavior::default(), {
        },
        dropper => BlockBehavior::default(), {
            Facing=North,
            Triggered=False,
        },
        emerald_block => BlockBehavior::default(), {
        },
        emerald_ore => BlockBehavior::default(), {
        },
        enchanting_table => BlockBehavior::default(), {
        },
        end_gateway => BlockBehavior::default(), {
        },
        end_portal => BlockBehavior::default(), {
        },
        end_portal_frame => BlockBehavior::default(), {
            Eye=False,
            Facing=North,
        },
        end_rod => BlockBehavior::default(), {
            Facing=Up,
        },
        end_stone => BlockBehavior::default(), {
        },
        end_stone_brick_slab => BlockBehavior::default(), {
            Type=Bottom,
            Waterlogged=False,
        },
        end_stone_brick_stairs => BlockBehavior::default(), {
            Facing=North,
            Half=Bottom,
            Shape=Straight,
            Waterlogged=False,
        },
        end_stone_brick_wall => BlockBehavior::default(), {
            East=None,
            North=None,
            South=None,
            Up=True,
            Waterlogged=False,
            West=None,
        },
        end_stone_bricks => BlockBehavior::default(), {
        },
        ender_chest => BlockBehavior::default(), {
            Facing=North,
            Waterlogged=False,
        },
        exposed_copper => BlockBehavior::default(), {
        },
        exposed_cut_copper => BlockBehavior::default(), {
        },
        exposed_cut_copper_slab => BlockBehavior::default(), {
            Type=Bottom,
            Waterlogged=False,
        },
        exposed_cut_copper_stairs => BlockBehavior::default(), {
            Facing=North,
            Half=Bottom,
            Shape=Straight,
            Waterlogged=False,
        },
        farmland => BlockBehavior::default(), {
            Moisture=_0,
        },
        fern => BlockBehavior::default(), {
        },
        fire => BlockBehavior::default(), {
            Age=_0,
            East=False,
            North=False,
            South=False,
            Up=False,
            West=False,
        },
        fire_coral => BlockBehavior::default(), {
            Waterlogged=True,
        },
        fire_coral_block => BlockBehavior::default(), {
        },
        fire_coral_fan => BlockBehavior::default(), {
            Waterlogged=True,
        },
        fire_coral_wall_fan => BlockBehavior::default(), {
            Facing=North,
            Waterlogged=True,
        },
        fletching_table => BlockBehavior::default(), {
        },
        flower_pot => BlockBehavior::default(), {
        },
        flowering_azalea => BlockBehavior::default(), {
        },
        flowering_azalea_leaves => BlockBehavior::default(), {
            Distance=_7,
            Persistent=False,
            Waterlogged=False,
        },
        frogspawn => BlockBehavior::default(), {
        },
        frosted_ice => BlockBehavior::default(), {
            Age=_0,
        },
        furnace => BlockBehavior::default(), {
            Facing=North,
            Lit=False,
        },
        gilded_blackstone => BlockBehavior::default(), {
        },
        glass => BlockBehavior::default(), {
        },
        glass_pane => BlockBehavior::default(), {
            East=False,
            North=False,
            South=False,
            Waterlogged=False,
            West=False,
        },
        glow_lichen => BlockBehavior::default(), {
            Down=False,
            East=False,
            North=False,
            South=False,
            Up=False,
            Waterlogged=False,
            West=False,
        },
        glowstone => BlockBehavior::default(), {
        },
        gold_block => BlockBehavior::default(), {
        },
        gold_ore => BlockBehavior::default(), {
        },
        granite => BlockBehavior::default(), {
        },
        granite_slab => BlockBehavior::default(), {
            Type=Bottom,
            Waterlogged=False,
        },
        granite_stairs => BlockBehavior::default(), {
            Facing=North,
            Half=Bottom,
            Shape=Straight,
            Waterlogged=False,
        },
        granite_wall => BlockBehavior::default(), {
            East=None,
            North=None,
            South=None,
            Up=True,
            Waterlogged=False,
            West=None,
        },
        grass => BlockBehavior::default(), {
        },
        grass_block => BlockBehavior::default(), {
            Snowy=False,
        },
        gravel => BlockBehavior::default(), {
        },
        gray_banner => BlockBehavior::default(), {
            Rotation=_0,
        },
        gray_bed => BlockBehavior::default(), {
            Facing=North,
            Occupied=False,
            Part=Foot,
        },
        gray_candle => BlockBehavior::default(), {
            Candles=_1,
            Lit=False,
            Waterlogged=False,
        },
        gray_candle_cake => BlockBehavior::default(), {
            Lit=False,
        },
        gray_carpet => BlockBehavior::default(), {
        },
        gray_concrete => BlockBehavior::default(), {
        },
        gray_concrete_powder => BlockBehavior::default(), {
        },
        gray_glazed_terracotta => BlockBehavior::default(), {
            Facing=North,
        },
        gray_shulker_box => BlockBehavior::default(), {
            Facing=Up,
        },
        gray_stained_glass => BlockBehavior::default(), {
        },
        gray_stained_glass_pane => BlockBehavior::default(), {
            East=False,
            North=False,
            South=False,
            Waterlogged=False,
            West=False,
        },
        gray_terracotta => BlockBehavior::default(), {
        },
        gray_wall_banner => BlockBehavior::default(), {
            Facing=North,
        },
        gray_wool => BlockBehavior::default(), {
        },
        green_banner => BlockBehavior::default(), {
            Rotation=_0,
        },
        green_bed => BlockBehavior::default(), {
            Facing=North,
            Occupied=False,
            Part=Foot,
        },
        green_candle => BlockBehavior::default(), {
            Candles=_1,
            Lit=False,
            Waterlogged=False,
        },
        green_candle_cake => BlockBehavior::default(), {
            Lit=False,
        },
        green_carpet => BlockBehavior::default(), {
        },
        green_concrete => BlockBehavior::default(), {
        },
        green_concrete_powder => BlockBehavior::default(), {
        },
        green_glazed_terracotta => BlockBehavior::default(), {
            Facing=North,
        },
        green_shulker_box => BlockBehavior::default(), {
            Facing=Up,
        },
        green_stained_glass => BlockBehavior::default(), {
        },
        green_stained_glass_pane => BlockBehavior::default(), {
            East=False,
            North=False,
            South=False,
            Waterlogged=False,
            West=False,
        },
        green_terracotta => BlockBehavior::default(), {
        },
        green_wall_banner => BlockBehavior::default(), {
            Facing=North,
        },
        green_wool => BlockBehavior::default(), {
        },
        grindstone => BlockBehavior::default(), {
            Face=Wall,
            Facing=North,
        },
        hanging_roots => BlockBehavior::default(), {
            Waterlogged=False,
        },
        hay_block => BlockBehavior::default(), {
            Axis=Y,
        },
        heavy_weighted_pressure_plate => BlockBehavior::default(), {
            Power=_0,
        },
        honey_block => BlockBehavior::default(), {
        },
        honeycomb_block => BlockBehavior::default(), {
        },
        hopper => BlockBehavior::default(), {
            Enabled=True,
            Facing=Down,
        },
        horn_coral => BlockBehavior::default(), {
            Waterlogged=True,
        },
        horn_coral_block => BlockBehavior::default(), {
        },
        horn_coral_fan => BlockBehavior::default(), {
            Waterlogged=True,
        },
        horn_coral_wall_fan => BlockBehavior::default(), {
            Facing=North,
            Waterlogged=True,
        },
        ice => BlockBehavior::default(), {
        },
        infested_chiseled_stone_bricks => BlockBehavior::default(), {
        },
        infested_cobblestone => BlockBehavior::default(), {
        },
        infested_cracked_stone_bricks => BlockBehavior::default(), {
        },
        infested_deepslate => BlockBehavior::default(), {
            Axis=Y,
        },
        infested_mossy_stone_bricks => BlockBehavior::default(), {
        },
        infested_stone => BlockBehavior::default(), {
        },
        infested_stone_bricks => BlockBehavior::default(), {
        },
        iron_bars => BlockBehavior::default(), {
            East=False,
            North=False,
            South=False,
            Waterlogged=False,
            West=False,
        },
        iron_block => BlockBehavior::default(), {
        },
        iron_door => BlockBehavior::default(), {
            Facing=North,
            Half=Lower,
            Hinge=Left,
            Open=False,
            Powered=False,
        },
        iron_ore => BlockBehavior::default(), {
        },
        iron_trapdoor => BlockBehavior::default(), {
            Facing=North,
            Half=Bottom,
            Open=False,
            Powered=False,
            Waterlogged=False,
        },
        jack_o_lantern => BlockBehavior::default(), {
            Facing=North,
        },
        jigsaw => BlockBehavior::default(), {
            Orientation=NorthUp,
        },
        jukebox => BlockBehavior::default(), {
            HasRecord=False,
        },
        jungle_button => BlockBehavior::default(), {
            Face=Wall,
            Facing=North,
            Powered=False,
        },
        jungle_door => BlockBehavior::default(), {
            Facing=North,
            Half=Lower,
            Hinge=Left,
            Open=False,
            Powered=False,
        },
        jungle_fence => BlockBehavior::default(), {
            East=False,
            North=False,
            South=False,
            Waterlogged=False,
            West=False,
        },
        jungle_fence_gate => BlockBehavior::default(), {
            Facing=North,
            InWall=False,
            Open=False,
            Powered=False,
        },
        jungle_leaves => BlockBehavior::default(), {
            Distance=_7,
            Persistent=False,
            Waterlogged=False,
        },
        jungle_log => BlockBehavior::default(), {
            Axis=Y,
        },
        jungle_planks => BlockBehavior::default(), {
        },
        jungle_pressure_plate => BlockBehavior::default(), {
            Powered=False,
        },
        jungle_sapling => BlockBehavior::default(), {
            Stage=_0,
        },
        jungle_sign => BlockBehavior::default(), {
            Rotation=_0,
            Waterlogged=False,
        },
        jungle_slab => BlockBehavior::default(), {
            Type=Bottom,
            Waterlogged=False,
        },
        jungle_stairs => BlockBehavior::default(), {
            Facing=North,
            Half=Bottom,
            Shape=Straight,
            Waterlogged=False,
        },
        jungle_trapdoor => BlockBehavior::default(), {
            Facing=North,
            Half=Bottom,
            Open=False,
            Powered=False,
            Waterlogged=False,
        },
        jungle_wall_sign => BlockBehavior::default(), {
            Facing=North,
            Waterlogged=False,
        },
        jungle_wood => BlockBehavior::default(), {
            Axis=Y,
        },
        kelp => BlockBehavior::default(), {
            Age=_0,
        },
        kelp_plant => BlockBehavior::default(), {
        },
        ladder => BlockBehavior::default(), {
            Facing=North,
            Waterlogged=False,
        },
        lantern => BlockBehavior::default(), {
            Hanging=False,
            Waterlogged=False,
        },
        lapis_block => BlockBehavior::default(), {
        },
        lapis_ore => BlockBehavior::default(), {
        },
        large_amethyst_bud => BlockBehavior::default(), {
            Facing=Up,
            Waterlogged=False,
        },
        large_fern => BlockBehavior::default(), {
            Half=Lower,
        },
        lava => BlockBehavior::default(), {
            Level=_0,
        },
        lava_cauldron => BlockBehavior::default(), {
        },
        lectern => BlockBehavior::default(), {
            Facing=North,
            HasBook=False,
            Powered=False,
        },
        lever => BlockBehavior::default(), {
            Face=Wall,
            Facing=North,
            Powered=False,
        },
        light => BlockBehavior::default(), {
            Level=_15,
            Waterlogged=False,
        },
        light_blue_banner => BlockBehavior::default(), {
            Rotation=_0,
        },
        light_blue_bed => BlockBehavior::default(), {
            Facing=North,
            Occupied=False,
            Part=Foot,
        },
        light_blue_candle => BlockBehavior::default(), {
            Candles=_1,
            Lit=False,
            Waterlogged=False,
        },
        light_blue_candle_cake => BlockBehavior::default(), {
            Lit=False,
        },
        light_blue_carpet => BlockBehavior::default(), {
        },
        light_blue_concrete => BlockBehavior::default(), {
        },
        light_blue_concrete_powder => BlockBehavior::default(), {
        },
        light_blue_glazed_terracotta => BlockBehavior::default(), {
            Facing=North,
        },
        light_blue_shulker_box => BlockBehavior::default(), {
            Facing=Up,
        },
        light_blue_stained_glass => BlockBehavior::default(), {
        },
        light_blue_stained_glass_pane => BlockBehavior::default(), {
            East=False,
            North=False,
            South=False,
            Waterlogged=False,
            West=False,
        },
        light_blue_terracotta => BlockBehavior::default(), {
        },
        light_blue_wall_banner => BlockBehavior::default(), {
            Facing=North,
        },
        light_blue_wool => BlockBehavior::default(), {
        },
        light_gray_banner => BlockBehavior::default(), {
            Rotation=_0,
        },
        light_gray_bed => BlockBehavior::default(), {
            Facing=North,
            Occupied=False,
            Part=Foot,
        },
        light_gray_candle => BlockBehavior::default(), {
            Candles=_1,
            Lit=False,
            Waterlogged=False,
        },
        light_gray_candle_cake => BlockBehavior::default(), {
            Lit=False,
        },
        light_gray_carpet => BlockBehavior::default(), {
        },
        light_gray_concrete => BlockBehavior::default(), {
        },
        light_gray_concrete_powder => BlockBehavior::default(), {
        },
        light_gray_glazed_terracotta => BlockBehavior::default(), {
            Facing=North,
        },
        light_gray_shulker_box => BlockBehavior::default(), {
            Facing=Up,
        },
        light_gray_stained_glass => BlockBehavior::default(), {
        },
        light_gray_stained_glass_pane => BlockBehavior::default(), {
            East=False,
            North=False,
            South=False,
            Waterlogged=False,
            West=False,
        },
        light_gray_terracotta => BlockBehavior::default(), {
        },
        light_gray_wall_banner => BlockBehavior::default(), {
            Facing=North,
        },
        light_gray_wool => BlockBehavior::default(), {
        },
        light_weighted_pressure_plate => BlockBehavior::default(), {
            Power=_0,
        },
        lightning_rod => BlockBehavior::default(), {
            Facing=Up,
            Powered=False,
            Waterlogged=False,
        },
        lilac => BlockBehavior::default(), {
            Half=Lower,
        },
        lily_of_the_valley => BlockBehavior::default(), {
        },
        lily_pad => BlockBehavior::default(), {
        },
        lime_banner => BlockBehavior::default(), {
            Rotation=_0,
        },
        lime_bed => BlockBehavior::default(), {
            Facing=North,
            Occupied=False,
            Part=Foot,
        },
        lime_candle => BlockBehavior::default(), {
            Candles=_1,
            Lit=False,
            Waterlogged=False,
        },
        lime_candle_cake => BlockBehavior::default(), {
            Lit=False,
        },
        lime_carpet => BlockBehavior::default(), {
        },
        lime_concrete => BlockBehavior::default(), {
        },
        lime_concrete_powder => BlockBehavior::default(), {
        },
        lime_glazed_terracotta => BlockBehavior::default(), {
            Facing=North,
        },
        lime_shulker_box => BlockBehavior::default(), {
            Facing=Up,
        },
        lime_stained_glass => BlockBehavior::default(), {
        },
        lime_stained_glass_pane => BlockBehavior::default(), {
            East=False,
            North=False,
            South=False,
            Waterlogged=False,
            West=False,
        },
        lime_terracotta => BlockBehavior::default(), {
        },
        lime_wall_banner => BlockBehavior::default(), {
            Facing=North,
        },
        lime_wool => BlockBehavior::default(), {
        },
        lodestone => BlockBehavior::default(), {
        },
        loom => BlockBehavior::default(), {
            Facing=North,
        },
        magenta_banner => BlockBehavior::default(), {
            Rotation=_0,
        },
        magenta_bed => BlockBehavior::default(), {
            Facing=North,
            Occupied=False,
            Part=Foot,
        },
        magenta_candle => BlockBehavior::default(), {
            Candles=_1,
            Lit=False,
            Waterlogged=False,
        },
        magenta_candle_cake => BlockBehavior::default(), {
            Lit=False,
        },
        magenta_carpet => BlockBehavior::default(), {
        },
        magenta_concrete => BlockBehavior::default(), {
        },
        magenta_concrete_powder => BlockBehavior::default(), {
        },
        magenta_glazed_terracotta => BlockBehavior::default(), {
            Facing=North,
        },
        magenta_shulker_box => BlockBehavior::default(), {
            Facing=Up,
        },
        magenta_stained_glass => BlockBehavior::default(), {
        },
        magenta_stained_glass_pane => BlockBehavior::default(), {
            East=False,
            North=False,
            South=False,
            Waterlogged=False,
            West=False,
        },
        magenta_terracotta => BlockBehavior::default(), {
        },
        magenta_wall_banner => BlockBehavior::default(), {
            Facing=North,
        },
        magenta_wool => BlockBehavior::default(), {
        },
        magma_block => BlockBehavior::default(), {
        },
        mangrove_button => BlockBehavior::default(), {
            Face=Wall,
            Facing=North,
            Powered=False,
        },
        mangrove_door => BlockBehavior::default(), {
            Facing=North,
            Half=Lower,
            Hinge=Left,
            Open=False,
            Powered=False,
        },
        mangrove_fence => BlockBehavior::default(), {
            East=False,
            North=False,
            South=False,
            Waterlogged=False,
            West=False,
        },
        mangrove_fence_gate => BlockBehavior::default(), {
            Facing=North,
            InWall=False,
            Open=False,
            Powered=False,
        },
        mangrove_leaves => BlockBehavior::default(), {
            Distance=_7,
            Persistent=False,
            Waterlogged=False,
        },
        mangrove_log => BlockBehavior::default(), {
            Axis=Y,
        },
        mangrove_planks => BlockBehavior::default(), {
        },
        mangrove_pressure_plate => BlockBehavior::default(), {
            Powered=False,
        },
        mangrove_propagule => BlockBehavior::default(), {
            Age=_0,
            Hanging=False,
            Stage=_0,
            Waterlogged=False,
        },
        mangrove_roots => BlockBehavior::default(), {
            Waterlogged=False,
        },
        mangrove_sign => BlockBehavior::default(), {
            Rotation=_0,
            Waterlogged=False,
        },
        mangrove_slab => BlockBehavior::default(), {
            Type=Bottom,
            Waterlogged=False,
        },
        mangrove_stairs => BlockBehavior::default(), {
            Facing=North,
            Half=Bottom,
            Shape=Straight,
            Waterlogged=False,
        },
        mangrove_trapdoor => BlockBehavior::default(), {
            Facing=North,
            Half=Bottom,
            Open=False,
            Powered=False,
            Waterlogged=False,
        },
        mangrove_wall_sign => BlockBehavior::default(), {
            Facing=North,
            Waterlogged=False,
        },
        mangrove_wood => BlockBehavior::default(), {
            Axis=Y,
        },
        medium_amethyst_bud => BlockBehavior::default(), {
            Facing=Up,
            Waterlogged=False,
        },
        melon => BlockBehavior::default(), {
        },
        melon_stem => BlockBehavior::default(), {
            Age=_0,
        },
        moss_block => BlockBehavior::default(), {
        },
        moss_carpet => BlockBehavior::default(), {
        },
        mossy_cobblestone => BlockBehavior::default(), {
        },
        mossy_cobblestone_slab => BlockBehavior::default(), {
            Type=Bottom,
            Waterlogged=False,
        },
        mossy_cobblestone_stairs => BlockBehavior::default(), {
            Facing=North,
            Half=Bottom,
            Shape=Straight,
            Waterlogged=False,
        },
        mossy_cobblestone_wall => BlockBehavior::default(), {
            East=None,
            North=None,
            South=None,
            Up=True,
            Waterlogged=False,
            West=None,
        },
        mossy_stone_brick_slab => BlockBehavior::default(), {
            Type=Bottom,
            Waterlogged=False,
        },
        mossy_stone_brick_stairs => BlockBehavior::default(), {
            Facing=North,
            Half=Bottom,
            Shape=Straight,
            Waterlogged=False,
        },
        mossy_stone_brick_wall => BlockBehavior::default(), {
            East=None,
            North=None,
            South=None,
            Up=True,
            Waterlogged=False,
            West=None,
        },
        mossy_stone_bricks => BlockBehavior::default(), {
        },
        moving_piston => BlockBehavior::default(), {
            Type=Normal,
            Facing=North,
        },
        mud => BlockBehavior::default(), {
        },
        mud_brick_slab => BlockBehavior::default(), {
            Type=Bottom,
            Waterlogged=False,
        },
        mud_brick_stairs => BlockBehavior::default(), {
            Facing=North,
            Half=Bottom,
            Shape=Straight,
            Waterlogged=False,
        },
        mud_brick_wall => BlockBehavior::default(), {
            East=None,
            North=None,
            South=None,
            Up=True,
            Waterlogged=False,
            West=None,
        },
        mud_bricks => BlockBehavior::default(), {
        },
        muddy_mangrove_roots => BlockBehavior::default(), {
            Axis=Y,
        },
        mushroom_stem => BlockBehavior::default(), {
            Down=True,
            East=True,
            North=True,
            South=True,
            Up=True,
            West=True,
        },
        mycelium => BlockBehavior::default(), {
            Snowy=False,
        },
        nether_brick_fence => BlockBehavior::default(), {
            East=False,
            North=False,
            South=False,
            Waterlogged=False,
            West=False,
        },
        nether_brick_slab => BlockBehavior::default(), {
            Type=Bottom,
            Waterlogged=False,
        },
        nether_brick_stairs => BlockBehavior::default(), {
            Facing=North,
            Half=Bottom,
            Shape=Straight,
            Waterlogged=False,
        },
        nether_brick_wall => BlockBehavior::default(), {
            East=None,
            North=None,
            South=None,
            Up=True,
            Waterlogged=False,
            West=None,
        },
        nether_bricks => BlockBehavior::default(), {
        },
        nether_gold_ore => BlockBehavior::default(), {
        },
        nether_portal => BlockBehavior::default(), {
            Axis=X,
        },
        nether_quartz_ore => BlockBehavior::default(), {
        },
        nether_sprouts => BlockBehavior::default(), {
        },
        nether_wart => BlockBehavior::default(), {
            Age=_0,
        },
        nether_wart_block => BlockBehavior::default(), {
        },
        netherite_block => BlockBehavior::default(), {
        },
        netherrack => BlockBehavior::default(), {
        },
        note_block => BlockBehavior::default(), {
            Instrument=Harp,
            Note=_0,
            Powered=False,
        },
        oak_button => BlockBehavior::default(), {
            Face=Wall,
            Facing=North,
            Powered=False,
        },
        oak_door => BlockBehavior::default(), {
            Facing=North,
            Half=Lower,
            Hinge=Left,
            Open=False,
            Powered=False,
        },
        oak_fence => BlockBehavior::default(), {
            East=False,
            North=False,
            South=False,
            Waterlogged=False,
            West=False,
        },
        oak_fence_gate => BlockBehavior::default(), {
            Facing=North,
            InWall=False,
            Open=False,
            Powered=False,
        },
        oak_leaves => BlockBehavior::default(), {
            Distance=_7,
            Persistent=False,
            Waterlogged=False,
        },
        oak_log => BlockBehavior::default(), {
            Axis=Y,
        },
        oak_planks => BlockBehavior::default(), {
        },
        oak_pressure_plate => BlockBehavior::default(), {
            Powered=False,
        },
        oak_sapling => BlockBehavior::default(), {
            Stage=_0,
        },
        oak_sign => BlockBehavior::default(), {
            Rotation=_0,
            Waterlogged=False,
        },
        oak_slab => BlockBehavior::default(), {
            Type=Bottom,
            Waterlogged=False,
        },
        oak_stairs => BlockBehavior::default(), {
            Facing=North,
            Half=Bottom,
            Shape=Straight,
            Waterlogged=False,
        },
        oak_trapdoor => BlockBehavior::default(), {
            Facing=North,
            Half=Bottom,
            Open=False,
            Powered=False,
            Waterlogged=False,
        },
        oak_wall_sign => BlockBehavior::default(), {
            Facing=North,
            Waterlogged=False,
        },
        oak_wood => BlockBehavior::default(), {
            Axis=Y,
        },
        observer => BlockBehavior::default(), {
            Facing=South,
            Powered=False,
        },
        obsidian => BlockBehavior::default(), {
        },
        ochre_froglight => BlockBehavior::default(), {
            Axis=Y,
        },
        orange_banner => BlockBehavior::default(), {
            Rotation=_0,
        },
        orange_bed => BlockBehavior::default(), {
            Facing=North,
            Occupied=False,
            Part=Foot,
        },
        orange_candle => BlockBehavior::default(), {
            Candles=_1,
            Lit=False,
            Waterlogged=False,
        },
        orange_candle_cake => BlockBehavior::default(), {
            Lit=False,
        },
        orange_carpet => BlockBehavior::default(), {
        },
        orange_concrete => BlockBehavior::default(), {
        },
        orange_concrete_powder => BlockBehavior::default(), {
        },
        orange_glazed_terracotta => BlockBehavior::default(), {
            Facing=North,
        },
        orange_shulker_box => BlockBehavior::default(), {
            Facing=Up,
        },
        orange_stained_glass => BlockBehavior::default(), {
        },
        orange_stained_glass_pane => BlockBehavior::default(), {
            East=False,
            North=False,
            South=False,
            Waterlogged=False,
            West=False,
        },
        orange_terracotta => BlockBehavior::default(), {
        },
        orange_tulip => BlockBehavior::default(), {
        },
        orange_wall_banner => BlockBehavior::default(), {
            Facing=North,
        },
        orange_wool => BlockBehavior::default(), {
        },
        oxeye_daisy => BlockBehavior::default(), {
        },
        oxidized_copper => BlockBehavior::default(), {
        },
        oxidized_cut_copper => BlockBehavior::default(), {
        },
        oxidized_cut_copper_slab => BlockBehavior::default(), {
            Type=Bottom,
            Waterlogged=False,
        },
        oxidized_cut_copper_stairs => BlockBehavior::default(), {
            Facing=North,
            Half=Bottom,
            Shape=Straight,
            Waterlogged=False,
        },
        packed_ice => BlockBehavior::default(), {
        },
        packed_mud => BlockBehavior::default(), {
        },
        pearlescent_froglight => BlockBehavior::default(), {
            Axis=Y,
        },
        peony => BlockBehavior::default(), {
            Half=Lower,
        },
        petrified_oak_slab => BlockBehavior::default(), {
            Type=Bottom,
            Waterlogged=False,
        },
        pink_banner => BlockBehavior::default(), {
            Rotation=_0,
        },
        pink_bed => BlockBehavior::default(), {
            Facing=North,
            Occupied=False,
            Part=Foot,
        },
        pink_candle => BlockBehavior::default(), {
            Candles=_1,
            Lit=False,
            Waterlogged=False,
        },
        pink_candle_cake => BlockBehavior::default(), {
            Lit=False,
        },
        pink_carpet => BlockBehavior::default(), {
        },
        pink_concrete => BlockBehavior::default(), {
        },
        pink_concrete_powder => BlockBehavior::default(), {
        },
        pink_glazed_terracotta => BlockBehavior::default(), {
            Facing=North,
        },
        pink_shulker_box => BlockBehavior::default(), {
            Facing=Up,
        },
        pink_stained_glass => BlockBehavior::default(), {
        },
        pink_stained_glass_pane => BlockBehavior::default(), {
            East=False,
            North=False,
            South=False,
            Waterlogged=False,
            West=False,
        },
        pink_terracotta => BlockBehavior::default(), {
        },
        pink_tulip => BlockBehavior::default(), {
        },
        pink_wall_banner => BlockBehavior::default(), {
            Facing=North,
        },
        pink_wool => BlockBehavior::default(), {
        },
        piston => BlockBehavior::default(), {
            Extended=False,
            Facing=North,
        },
        piston_head => BlockBehavior::default(), {
            Type=Normal,
            Facing=North,
            Short=False,
        },
        player_head => BlockBehavior::default(), {
            Rotation=_0,
        },
        player_wall_head => BlockBehavior::default(), {
            Facing=North,
        },
        podzol => BlockBehavior::default(), {
            Snowy=False,
        },
        pointed_dripstone => BlockBehavior::default(), {
            Thickness=Tip,
            VerticalDirection=Up,
            Waterlogged=False,
        },
        polished_andesite => BlockBehavior::default(), {
        },
        polished_andesite_slab => BlockBehavior::default(), {
            Type=Bottom,
            Waterlogged=False,
        },
        polished_andesite_stairs => BlockBehavior::default(), {
            Facing=North,
            Half=Bottom,
            Shape=Straight,
            Waterlogged=False,
        },
        polished_basalt => BlockBehavior::default(), {
            Axis=Y,
        },
        polished_blackstone => BlockBehavior::default(), {
        },
        polished_blackstone_brick_slab => BlockBehavior::default(), {
            Type=Bottom,
            Waterlogged=False,
        },
        polished_blackstone_brick_stairs => BlockBehavior::default(), {
            Facing=North,
            Half=Bottom,
            Shape=Straight,
            Waterlogged=False,
        },
        polished_blackstone_brick_wall => BlockBehavior::default(), {
            East=None,
            North=None,
            South=None,
            Up=True,
            Waterlogged=False,
            West=None,
        },
        polished_blackstone_bricks => BlockBehavior::default(), {
        },
        polished_blackstone_button => BlockBehavior::default(), {
            Face=Wall,
            Facing=North,
            Powered=False,
        },
        polished_blackstone_pressure_plate => BlockBehavior::default(), {
            Powered=False,
        },
        polished_blackstone_slab => BlockBehavior::default(), {
            Type=Bottom,
            Waterlogged=False,
        },
        polished_blackstone_stairs => BlockBehavior::default(), {
            Facing=North,
            Half=Bottom,
            Shape=Straight,
            Waterlogged=False,
        },
        polished_blackstone_wall => BlockBehavior::default(), {
            East=None,
            North=None,
            South=None,
            Up=True,
            Waterlogged=False,
            West=None,
        },
        polished_deepslate => BlockBehavior::default(), {
        },
        polished_deepslate_slab => BlockBehavior::default(), {
            Type=Bottom,
            Waterlogged=False,
        },
        polished_deepslate_stairs => BlockBehavior::default(), {
            Facing=North,
            Half=Bottom,
            Shape=Straight,
            Waterlogged=False,
        },
        polished_deepslate_wall => BlockBehavior::default(), {
            East=None,
            North=None,
            South=None,
            Up=True,
            Waterlogged=False,
            West=None,
        },
        polished_diorite => BlockBehavior::default(), {
        },
        polished_diorite_slab => BlockBehavior::default(), {
            Type=Bottom,
            Waterlogged=False,
        },
        polished_diorite_stairs => BlockBehavior::default(), {
            Facing=North,
            Half=Bottom,
            Shape=Straight,
            Waterlogged=False,
        },
        polished_granite => BlockBehavior::default(), {
        },
        polished_granite_slab => BlockBehavior::default(), {
            Type=Bottom,
            Waterlogged=False,
        },
        polished_granite_stairs => BlockBehavior::default(), {
            Facing=North,
            Half=Bottom,
            Shape=Straight,
            Waterlogged=False,
        },
        poppy => BlockBehavior::default(), {
        },
        potatoes => BlockBehavior::default(), {
            Age=_0,
        },
        potted_acacia_sapling => BlockBehavior::default(), {
        },
        potted_allium => BlockBehavior::default(), {
        },
        potted_azalea_bush => BlockBehavior::default(), {
        },
        potted_azure_bluet => BlockBehavior::default(), {
        },
        potted_bamboo => BlockBehavior::default(), {
        },
        potted_birch_sapling => BlockBehavior::default(), {
        },
        potted_blue_orchid => BlockBehavior::default(), {
        },
        potted_brown_mushroom => BlockBehavior::default(), {
        },
        potted_cactus => BlockBehavior::default(), {
        },
        potted_cornflower => BlockBehavior::default(), {
        },
        potted_crimson_fungus => BlockBehavior::default(), {
        },
        potted_crimson_roots => BlockBehavior::default(), {
        },
        potted_dandelion => BlockBehavior::default(), {
        },
        potted_dark_oak_sapling => BlockBehavior::default(), {
        },
        potted_dead_bush => BlockBehavior::default(), {
        },
        potted_fern => BlockBehavior::default(), {
        },
        potted_flowering_azalea_bush => BlockBehavior::default(), {
        },
        potted_jungle_sapling => BlockBehavior::default(), {
        },
        potted_lily_of_the_valley => BlockBehavior::default(), {
        },
        potted_mangrove_propagule => BlockBehavior::default(), {
        },
        potted_oak_sapling => BlockBehavior::default(), {
        },
        potted_orange_tulip => BlockBehavior::default(), {
        },
        potted_oxeye_daisy => BlockBehavior::default(), {
        },
        potted_pink_tulip => BlockBehavior::default(), {
        },
        potted_poppy => BlockBehavior::default(), {
        },
        potted_red_mushroom => BlockBehavior::default(), {
        },
        potted_red_tulip => BlockBehavior::default(), {
        },
        potted_spruce_sapling => BlockBehavior::default(), {
        },
        potted_warped_fungus => BlockBehavior::default(), {
        },
        potted_warped_roots => BlockBehavior::default(), {
        },
        potted_white_tulip => BlockBehavior::default(), {
        },
        potted_wither_rose => BlockBehavior::default(), {
        },
        powder_snow => BlockBehavior::default(), {
        },
        powder_snow_cauldron => BlockBehavior::default(), {
            Level=_1,
        },
        powered_rail => BlockBehavior::default(), {
            Powered=False,
            Shape=NorthSouth,
            Waterlogged=False,
        },
        prismarine => BlockBehavior::default(), {
        },
        prismarine_brick_slab => BlockBehavior::default(), {
            Type=Bottom,
            Waterlogged=False,
        },
        prismarine_brick_stairs => BlockBehavior::default(), {
            Facing=North,
            Half=Bottom,
            Shape=Straight,
            Waterlogged=False,
        },
        prismarine_bricks => BlockBehavior::default(), {
        },
        prismarine_slab => BlockBehavior::default(), {
            Type=Bottom,
            Waterlogged=False,
        },
        prismarine_stairs => BlockBehavior::default(), {
            Facing=North,
            Half=Bottom,
            Shape=Straight,
            Waterlogged=False,
        },
        prismarine_wall => BlockBehavior::default(), {
            East=None,
            North=None,
            South=None,
            Up=True,
            Waterlogged=False,
            West=None,
        },
        pumpkin => BlockBehavior::default(), {
        },
        pumpkin_stem => BlockBehavior::default(), {
            Age=_0,
        },
        purple_banner => BlockBehavior::default(), {
            Rotation=_0,
        },
        purple_bed => BlockBehavior::default(), {
            Facing=North,
            Occupied=False,
            Part=Foot,
        },
        purple_candle => BlockBehavior::default(), {
            Candles=_1,
            Lit=False,
            Waterlogged=False,
        },
        purple_candle_cake => BlockBehavior::default(), {
            Lit=False,
        },
        purple_carpet => BlockBehavior::default(), {
        },
        purple_concrete => BlockBehavior::default(), {
        },
        purple_concrete_powder => BlockBehavior::default(), {
        },
        purple_glazed_terracotta => BlockBehavior::default(), {
            Facing=North,
        },
        purple_shulker_box => BlockBehavior::default(), {
            Facing=Up,
        },
        purple_stained_glass => BlockBehavior::default(), {
        },
        purple_stained_glass_pane => BlockBehavior::default(), {
            East=False,
            North=False,
            South=False,
            Waterlogged=False,
            West=False,
        },
        purple_terracotta => BlockBehavior::default(), {
        },
        purple_wall_banner => BlockBehavior::default(), {
            Facing=North,
        },
        purple_wool => BlockBehavior::default(), {
        },
        purpur_block => BlockBehavior::default(), {
        },
        purpur_pillar => BlockBehavior::default(), {
            Axis=Y,
        },
        purpur_slab => BlockBehavior::default(), {
            Type=Bottom,
            Waterlogged=False,
        },
        purpur_stairs => BlockBehavior::default(), {
            Facing=North,
            Half=Bottom,
            Shape=Straight,
            Waterlogged=False,
        },
        quartz_block => BlockBehavior::default(), {
        },
        quartz_bricks => BlockBehavior::default(), {
        },
        quartz_pillar => BlockBehavior::default(), {
            Axis=Y,
        },
        quartz_slab => BlockBehavior::default(), {
            Type=Bottom,
            Waterlogged=False,
        },
        quartz_stairs => BlockBehavior::default(), {
            Facing=North,
            Half=Bottom,
            Shape=Straight,
            Waterlogged=False,
        },
        rail => BlockBehavior::default(), {
            Shape=NorthSouth,
            Waterlogged=False,
        },
        raw_copper_block => BlockBehavior::default(), {
        },
        raw_gold_block => BlockBehavior::default(), {
        },
        raw_iron_block => BlockBehavior::default(), {
        },
        red_banner => BlockBehavior::default(), {
            Rotation=_0,
        },
        red_bed => BlockBehavior::default(), {
            Facing=North,
            Occupied=False,
            Part=Foot,
        },
        red_candle => BlockBehavior::default(), {
            Candles=_1,
            Lit=False,
            Waterlogged=False,
        },
        red_candle_cake => BlockBehavior::default(), {
            Lit=False,
        },
        red_carpet => BlockBehavior::default(), {
        },
        red_concrete => BlockBehavior::default(), {
        },
        red_concrete_powder => BlockBehavior::default(), {
        },
        red_glazed_terracotta => BlockBehavior::default(), {
            Facing=North,
        },
        red_mushroom => BlockBehavior::default(), {
        },
        red_mushroom_block => BlockBehavior::default(), {
            Down=True,
            East=True,
            North=True,
            South=True,
            Up=True,
            West=True,
        },
        red_nether_brick_slab => BlockBehavior::default(), {
            Type=Bottom,
            Waterlogged=False,
        },
        red_nether_brick_stairs => BlockBehavior::default(), {
            Facing=North,
            Half=Bottom,
            Shape=Straight,
            Waterlogged=False,
        },
        red_nether_brick_wall => BlockBehavior::default(), {
            East=None,
            North=None,
            South=None,
            Up=True,
            Waterlogged=False,
            West=None,
        },
        red_nether_bricks => BlockBehavior::default(), {
        },
        red_sand => BlockBehavior::default(), {
        },
        red_sandstone => BlockBehavior::default(), {
        },
        red_sandstone_slab => BlockBehavior::default(), {
            Type=Bottom,
            Waterlogged=False,
        },
        red_sandstone_stairs => BlockBehavior::default(), {
            Facing=North,
            Half=Bottom,
            Shape=Straight,
            Waterlogged=False,
        },
        red_sandstone_wall => BlockBehavior::default(), {
            East=None,
            North=None,
            South=None,
            Up=True,
            Waterlogged=False,
            West=None,
        },
        red_shulker_box => BlockBehavior::default(), {
            Facing=Up,
        },
        red_stained_glass => BlockBehavior::default(), {
        },
        red_stained_glass_pane => BlockBehavior::default(), {
            East=False,
            North=False,
            South=False,
            Waterlogged=False,
            West=False,
        },
        red_terracotta => BlockBehavior::default(), {
        },
        red_tulip => BlockBehavior::default(), {
        },
        red_wall_banner => BlockBehavior::default(), {
            Facing=North,
        },
        red_wool => BlockBehavior::default(), {
        },
        redstone_block => BlockBehavior::default(), {
        },
        redstone_lamp => BlockBehavior::default(), {
            Lit=False,
        },
        redstone_ore => BlockBehavior::default(), {
            Lit=False,
        },
        redstone_torch => BlockBehavior::default(), {
            Lit=True,
        },
        redstone_wall_torch => BlockBehavior::default(), {
            Facing=North,
            Lit=True,
        },
        redstone_wire => BlockBehavior::default(), {
            East=None,
            North=None,
            Power=_0,
            South=None,
            West=None,
        },
        reinforced_deepslate => BlockBehavior::default(), {
        },
        repeater => BlockBehavior::default(), {
            Delay=_1,
            Facing=North,
            Locked=False,
            Powered=False,
        },
        repeating_command_block => BlockBehavior::default(), {
            Conditional=False,
            Facing=North,
        },
        respawn_anchor => BlockBehavior::default(), {
            Charges=_0,
        },
        rooted_dirt => BlockBehavior::default(), {
        },
        rose_bush => BlockBehavior::default(), {
            Half=Lower,
        },
        sand => BlockBehavior::default(), {
        },
        sandstone => BlockBehavior::default(), {
        },
        sandstone_slab => BlockBehavior::default(), {
            Type=Bottom,
            Waterlogged=False,
        },
        sandstone_stairs => BlockBehavior::default(), {
            Facing=North,
            Half=Bottom,
            Shape=Straight,
            Waterlogged=False,
        },
        sandstone_wall => BlockBehavior::default(), {
            East=None,
            North=None,
            South=None,
            Up=True,
            Waterlogged=False,
            West=None,
        },
        scaffolding => BlockBehavior::default(), {
            Bottom=False,
            Distance=_7,
            Waterlogged=False,
        },
        sculk => BlockBehavior::default(), {
        },
        sculk_catalyst => BlockBehavior::default(), {
            Bloom=False,
        },
        sculk_sensor => BlockBehavior::default(), {
            Power=_0,
            SculkSensorPhase=Inactive,
            Waterlogged=False,
        },
        sculk_shrieker => BlockBehavior::default(), {
            CanSummon=False,
            Shrieking=False,
            Waterlogged=False,
        },
        sculk_vein => BlockBehavior::default(), {
            Down=False,
            East=False,
            North=False,
            South=False,
            Up=False,
            Waterlogged=False,
            West=False,
        },
        sea_lantern => BlockBehavior::default(), {
        },
        sea_pickle => BlockBehavior::default(), {
            Pickles=_1,
            Waterlogged=True,
        },
        seagrass => BlockBehavior::default(), {
        },
        shroomlight => BlockBehavior::default(), {
        },
        shulker_box => BlockBehavior::default(), {
            Facing=Up,
        },
        skeleton_skull => BlockBehavior::default(), {
            Rotation=_0,
        },
        skeleton_wall_skull => BlockBehavior::default(), {
            Facing=North,
        },
        slime_block => BlockBehavior::default(), {
        },
        small_amethyst_bud => BlockBehavior::default(), {
            Facing=Up,
            Waterlogged=False,
        },
        small_dripleaf => BlockBehavior::default(), {
            Facing=North,
            Half=Lower,
            Waterlogged=False,
        },
        smithing_table => BlockBehavior::default(), {
        },
        smoker => BlockBehavior::default(), {
            Facing=North,
            Lit=False,
        },
        smooth_basalt => BlockBehavior::default(), {
        },
        smooth_quartz => BlockBehavior::default(), {
        },
        smooth_quartz_slab => BlockBehavior::default(), {
            Type=Bottom,
            Waterlogged=False,
        },
        smooth_quartz_stairs => BlockBehavior::default(), {
            Facing=North,
            Half=Bottom,
            Shape=Straight,
            Waterlogged=False,
        },
        smooth_red_sandstone => BlockBehavior::default(), {
        },
        smooth_red_sandstone_slab => BlockBehavior::default(), {
            Type=Bottom,
            Waterlogged=False,
        },
        smooth_red_sandstone_stairs => BlockBehavior::default(), {
            Facing=North,
            Half=Bottom,
            Shape=Straight,
            Waterlogged=False,
        },
        smooth_sandstone => BlockBehavior::default(), {
        },
        smooth_sandstone_slab => BlockBehavior::default(), {
            Type=Bottom,
            Waterlogged=False,
        },
        smooth_sandstone_stairs => BlockBehavior::default(), {
            Facing=North,
            Half=Bottom,
            Shape=Straight,
            Waterlogged=False,
        },
        smooth_stone => BlockBehavior::default(), {
        },
        smooth_stone_slab => BlockBehavior::default(), {
            Type=Bottom,
            Waterlogged=False,
        },
        snow => BlockBehavior::default(), {
            Layers=_1,
        },
        snow_block => BlockBehavior::default(), {
        },
        soul_campfire => BlockBehavior::default(), {
            Facing=North,
            Lit=True,
            SignalFire=False,
            Waterlogged=False,
        },
        soul_fire => BlockBehavior::default(), {
        },
        soul_lantern => BlockBehavior::default(), {
            Hanging=False,
            Waterlogged=False,
        },
        soul_sand => BlockBehavior::default(), {
        },
        soul_soil => BlockBehavior::default(), {
        },
        soul_torch => BlockBehavior::default(), {
        },
        soul_wall_torch => BlockBehavior::default(), {
            Facing=North,
        },
        spawner => BlockBehavior::default(), {
        },
        sponge => BlockBehavior::default(), {
        },
        spore_blossom => BlockBehavior::default(), {
        },
        spruce_button => BlockBehavior::default(), {
            Face=Wall,
            Facing=North,
            Powered=False,
        },
        spruce_door => BlockBehavior::default(), {
            Facing=North,
            Half=Lower,
            Hinge=Left,
            Open=False,
            Powered=False,
        },
        spruce_fence => BlockBehavior::default(), {
            East=False,
            North=False,
            South=False,
            Waterlogged=False,
            West=False,
        },
        spruce_fence_gate => BlockBehavior::default(), {
            Facing=North,
            InWall=False,
            Open=False,
            Powered=False,
        },
        spruce_leaves => BlockBehavior::default(), {
            Distance=_7,
            Persistent=False,
            Waterlogged=False,
        },
        spruce_log => BlockBehavior::default(), {
            Axis=Y,
        },
        spruce_planks => BlockBehavior::default(), {
        },
        spruce_pressure_plate => BlockBehavior::default(), {
            Powered=False,
        },
        spruce_sapling => BlockBehavior::default(), {
            Stage=_0,
        },
        spruce_sign => BlockBehavior::default(), {
            Rotation=_0,
            Waterlogged=False,
        },
        spruce_slab => BlockBehavior::default(), {
            Type=Bottom,
            Waterlogged=False,
        },
        spruce_stairs => BlockBehavior::default(), {
            Facing=North,
            Half=Bottom,
            Shape=Straight,
            Waterlogged=False,
        },
        spruce_trapdoor => BlockBehavior::default(), {
            Facing=North,
            Half=Bottom,
            Open=False,
            Powered=False,
            Waterlogged=False,
        },
        spruce_wall_sign => BlockBehavior::default(), {
            Facing=North,
            Waterlogged=False,
        },
        spruce_wood => BlockBehavior::default(), {
            Axis=Y,
        },
        sticky_piston => BlockBehavior::default(), {
            Extended=False,
            Facing=North,
        },
        stone => BlockBehavior::default(), {
        },
        stone_brick_slab => BlockBehavior::default(), {
            Type=Bottom,
            Waterlogged=False,
        },
        stone_brick_stairs => BlockBehavior::default(), {
            Facing=North,
            Half=Bottom,
            Shape=Straight,
            Waterlogged=False,
        },
        stone_brick_wall => BlockBehavior::default(), {
            East=None,
            North=None,
            South=None,
            Up=True,
            Waterlogged=False,
            West=None,
        },
        stone_bricks => BlockBehavior::default(), {
        },
        stone_button => BlockBehavior::default(), {
            Face=Wall,
            Facing=North,
            Powered=False,
        },
        stone_pressure_plate => BlockBehavior::default(), {
            Powered=False,
        },
        stone_slab => BlockBehavior::default(), {
            Type=Bottom,
            Waterlogged=False,
        },
        stone_stairs => BlockBehavior::default(), {
            Facing=North,
            Half=Bottom,
            Shape=Straight,
            Waterlogged=False,
        },
        stonecutter => BlockBehavior::default(), {
            Facing=North,
        },
        stripped_acacia_log => BlockBehavior::default(), {
            Axis=Y,
        },
        stripped_acacia_wood => BlockBehavior::default(), {
            Axis=Y,
        },
        stripped_birch_log => BlockBehavior::default(), {
            Axis=Y,
        },
        stripped_birch_wood => BlockBehavior::default(), {
            Axis=Y,
        },
        stripped_crimson_hyphae => BlockBehavior::default(), {
            Axis=Y,
        },
        stripped_crimson_stem => BlockBehavior::default(), {
            Axis=Y,
        },
        stripped_dark_oak_log => BlockBehavior::default(), {
            Axis=Y,
        },
        stripped_dark_oak_wood => BlockBehavior::default(), {
            Axis=Y,
        },
        stripped_jungle_log => BlockBehavior::default(), {
            Axis=Y,
        },
        stripped_jungle_wood => BlockBehavior::default(), {
            Axis=Y,
        },
        stripped_mangrove_log => BlockBehavior::default(), {
            Axis=Y,
        },
        stripped_mangrove_wood => BlockBehavior::default(), {
            Axis=Y,
        },
        stripped_oak_log => BlockBehavior::default(), {
            Axis=Y,
        },
        stripped_oak_wood => BlockBehavior::default(), {
            Axis=Y,
        },
        stripped_spruce_log => BlockBehavior::default(), {
            Axis=Y,
        },
        stripped_spruce_wood => BlockBehavior::default(), {
            Axis=Y,
        },
        stripped_warped_hyphae => BlockBehavior::default(), {
            Axis=Y,
        },
        stripped_warped_stem => BlockBehavior::default(), {
            Axis=Y,
        },
        structure_block => BlockBehavior::default(), {
            Mode=Load,
        },
        structure_void => BlockBehavior::default(), {
        },
        sugar_cane => BlockBehavior::default(), {
            Age=_0,
        },
        sunflower => BlockBehavior::default(), {
            Half=Lower,
        },
        sweet_berry_bush => BlockBehavior::default(), {
            Age=_0,
        },
        tall_grass => BlockBehavior::default(), {
            Half=Lower,
        },
        tall_seagrass => BlockBehavior::default(), {
            Half=Lower,
        },
        target => BlockBehavior::default(), {
            Power=_0,
        },
        terracotta => BlockBehavior::default(), {
        },
        tinted_glass => BlockBehavior::default(), {
        },
        tnt => BlockBehavior::default(), {
            Unstable=False,
        },
        torch => BlockBehavior::default(), {
        },
        trapped_chest => BlockBehavior::default(), {
            Type=Single,
            Facing=North,
            Waterlogged=False,
        },
        tripwire => BlockBehavior::default(), {
            Attached=False,
            Disarmed=False,
            East=False,
            North=False,
            Powered=False,
            South=False,
            West=False,
        },
        tripwire_hook => BlockBehavior::default(), {
            Attached=False,
            Facing=North,
            Powered=False,
        },
        tube_coral => BlockBehavior::default(), {
            Waterlogged=True,
        },
        tube_coral_block => BlockBehavior::default(), {
        },
        tube_coral_fan => BlockBehavior::default(), {
            Waterlogged=True,
        },
        tube_coral_wall_fan => BlockBehavior::default(), {
            Facing=North,
            Waterlogged=True,
        },
        tuff => BlockBehavior::default(), {
        },
        turtle_egg => BlockBehavior::default(), {
            Eggs=_1,
            Hatch=_0,
        },
        twisting_vines => BlockBehavior::default(), {
            Age=_0,
        },
        twisting_vines_plant => BlockBehavior::default(), {
        },
        verdant_froglight => BlockBehavior::default(), {
            Axis=Y,
        },
        vine => BlockBehavior::default(), {
            East=False,
            North=False,
            South=False,
            Up=False,
            West=False,
        },
        void_air => BlockBehavior::default(), {
        },
        wall_torch => BlockBehavior::default(), {
            Facing=North,
        },
        warped_button => BlockBehavior::default(), {
            Face=Wall,
            Facing=North,
            Powered=False,
        },
        warped_door => BlockBehavior::default(), {
            Facing=North,
            Half=Lower,
            Hinge=Left,
            Open=False,
            Powered=False,
        },
        warped_fence => BlockBehavior::default(), {
            East=False,
            North=False,
            South=False,
            Waterlogged=False,
            West=False,
        },
        warped_fence_gate => BlockBehavior::default(), {
            Facing=North,
            InWall=False,
            Open=False,
            Powered=False,
        },
        warped_fungus => BlockBehavior::default(), {
        },
        warped_hyphae => BlockBehavior::default(), {
            Axis=Y,
        },
        warped_nylium => BlockBehavior::default(), {
        },
        warped_planks => BlockBehavior::default(), {
        },
        warped_pressure_plate => BlockBehavior::default(), {
            Powered=False,
        },
        warped_roots => BlockBehavior::default(), {
        },
        warped_sign => BlockBehavior::default(), {
            Rotation=_0,
            Waterlogged=False,
        },
        warped_slab => BlockBehavior::default(), {
            Type=Bottom,
            Waterlogged=False,
        },
        warped_stairs => BlockBehavior::default(), {
            Facing=North,
            Half=Bottom,
            Shape=Straight,
            Waterlogged=False,
        },
        warped_stem => BlockBehavior::default(), {
            Axis=Y,
        },
        warped_trapdoor => BlockBehavior::default(), {
            Facing=North,
            Half=Bottom,
            Open=False,
            Powered=False,
            Waterlogged=False,
        },
        warped_wall_sign => BlockBehavior::default(), {
            Facing=North,
            Waterlogged=False,
        },
        warped_wart_block => BlockBehavior::default(), {
        },
        water => BlockBehavior::default(), {
            Level=_0,
        },
        water_cauldron => BlockBehavior::default(), {
            Level=_1,
        },
        waxed_copper_block => BlockBehavior::default(), {
        },
        waxed_cut_copper => BlockBehavior::default(), {
        },
        waxed_cut_copper_slab => BlockBehavior::default(), {
            Type=Bottom,
            Waterlogged=False,
        },
        waxed_cut_copper_stairs => BlockBehavior::default(), {
            Facing=North,
            Half=Bottom,
            Shape=Straight,
            Waterlogged=False,
        },
        waxed_exposed_copper => BlockBehavior::default(), {
        },
        waxed_exposed_cut_copper => BlockBehavior::default(), {
        },
        waxed_exposed_cut_copper_slab => BlockBehavior::default(), {
            Type=Bottom,
            Waterlogged=False,
        },
        waxed_exposed_cut_copper_stairs => BlockBehavior::default(), {
            Facing=North,
            Half=Bottom,
            Shape=Straight,
            Waterlogged=False,
        },
        waxed_oxidized_copper => BlockBehavior::default(), {
        },
        waxed_oxidized_cut_copper => BlockBehavior::default(), {
        },
        waxed_oxidized_cut_copper_slab => BlockBehavior::default(), {
            Type=Bottom,
            Waterlogged=False,
        },
        waxed_oxidized_cut_copper_stairs => BlockBehavior::default(), {
            Facing=North,
            Half=Bottom,
            Shape=Straight,
            Waterlogged=False,
        },
        waxed_weathered_copper => BlockBehavior::default(), {
        },
        waxed_weathered_cut_copper => BlockBehavior::default(), {
        },
        waxed_weathered_cut_copper_slab => BlockBehavior::default(), {
            Type=Bottom,
            Waterlogged=False,
        },
        waxed_weathered_cut_copper_stairs => BlockBehavior::default(), {
            Facing=North,
            Half=Bottom,
            Shape=Straight,
            Waterlogged=False,
        },
        weathered_copper => BlockBehavior::default(), {
        },
        weathered_cut_copper => BlockBehavior::default(), {
        },
        weathered_cut_copper_slab => BlockBehavior::default(), {
            Type=Bottom,
            Waterlogged=False,
        },
        weathered_cut_copper_stairs => BlockBehavior::default(), {
            Facing=North,
            Half=Bottom,
            Shape=Straight,
            Waterlogged=False,
        },
        weeping_vines => BlockBehavior::default(), {
            Age=_0,
        },
        weeping_vines_plant => BlockBehavior::default(), {
        },
        wet_sponge => BlockBehavior::default(), {
        },
        wheat => BlockBehavior::default(), {
            Age=_0,
        },
        white_banner => BlockBehavior::default(), {
            Rotation=_0,
        },
        white_bed => BlockBehavior::default(), {
            Facing=North,
            Occupied=False,
            Part=Foot,
        },
        white_candle => BlockBehavior::default(), {
            Candles=_1,
            Lit=False,
            Waterlogged=False,
        },
        white_candle_cake => BlockBehavior::default(), {
            Lit=False,
        },
        white_carpet => BlockBehavior::default(), {
        },
        white_concrete => BlockBehavior::default(), {
        },
        white_concrete_powder => BlockBehavior::default(), {
        },
        white_glazed_terracotta => BlockBehavior::default(), {
            Facing=North,
        },
        white_shulker_box => BlockBehavior::default(), {
            Facing=Up,
        },
        white_stained_glass => BlockBehavior::default(), {
        },
        white_stained_glass_pane => BlockBehavior::default(), {
            East=False,
            North=False,
            South=False,
            Waterlogged=False,
            West=False,
        },
        white_terracotta => BlockBehavior::default(), {
        },
        white_tulip => BlockBehavior::default(), {
        },
        white_wall_banner => BlockBehavior::default(), {
            Facing=North,
        },
        white_wool => BlockBehavior::default(), {
        },
        wither_rose => BlockBehavior::default(), {
        },
        wither_skeleton_skull => BlockBehavior::default(), {
            Rotation=_0,
        },
        wither_skeleton_wall_skull => BlockBehavior::default(), {
            Facing=North,
        },
        yellow_banner => BlockBehavior::default(), {
            Rotation=_0,
        },
        yellow_bed => BlockBehavior::default(), {
            Facing=North,
            Occupied=False,
            Part=Foot,
        },
        yellow_candle => BlockBehavior::default(), {
            Candles=_1,
            Lit=False,
            Waterlogged=False,
        },
        yellow_candle_cake => BlockBehavior::default(), {
            Lit=False,
        },
        yellow_carpet => BlockBehavior::default(), {
        },
        yellow_concrete => BlockBehavior::default(), {
        },
        yellow_concrete_powder => BlockBehavior::default(), {
        },
        yellow_glazed_terracotta => BlockBehavior::default(), {
            Facing=North,
        },
        yellow_shulker_box => BlockBehavior::default(), {
            Facing=Up,
        },
        yellow_stained_glass => BlockBehavior::default(), {
        },
        yellow_stained_glass_pane => BlockBehavior::default(), {
            East=False,
            North=False,
            South=False,
            Waterlogged=False,
            West=False,
        },
        yellow_terracotta => BlockBehavior::default(), {
        },
        yellow_wall_banner => BlockBehavior::default(), {
            Facing=North,
        },
        yellow_wool => BlockBehavior::default(), {
        },
        zombie_head => BlockBehavior::default(), {
            Rotation=_0,
        },
        zombie_wall_head => BlockBehavior::default(), {
            Facing=North,
        },
    }
}

// #[derive(Debug, Clone, Copy)]
// pub enum Face {
//     Floor,
//     Wall,
//     Ceiling,
// }

// #[derive(Debug, Clone, Copy)]
// pub enum Facing {
//     North,
//     South,
//     West,
//     East,
// }

// #[derive(Debug, Clone, Copy)]
// pub enum Powered {
//     True,
//     False,
// }

// // the underscore makes it more readable, so i think it's fine to allow it
// #[allow(non_camel_case_types)]
// pub enum BlockState {
//     AcaciaButton_FloorNorthTrue,
//     AcaciaButton_WallNorthTrue,
//     AcaciaButton_CeilingNorthTrue,
// }

// pub trait Block {
//     fn behavior(&self) -> BlockBehavior;
// }

// #[derive(Debug)]
// pub struct AcaciaButtonBlock {
//     pub face: properties::Face,
//     pub facing: properties::Facing,
//     pub powered: properties::Powered,
// }

// impl Block for AcaciaButtonBlock {
//     fn behavior(&self) -> BlockBehavior {
//         BlockBehavior {
//             has_collision: false,
//         }
//     }
// }

// pub struct AcaciaDoorBlock {
//     pub facing: properties::Facing,
//     // pub half: properties::Half,
//     // pub hinge: properties::Hinge,
//     // pub open: properties::Open,
//     pub powered: properties::Powered,
// }

// impl From<BlockState> for &dyn Block {
//     fn from(b: BlockState) -> Self {
//         match b {
//             BlockState::AcaciaButton_FloorNorthTrue => &AcaciaButtonBlock {
//                 face: properties::Face::Floor,
//                 facing: properties::Facing::North,
//                 powered: properties::Powered::True,
//             },
//             // BlockState::AcaciaButton_WallNorthTrue => todo!(),
//             // BlockState::AcaciaButton_CeilingNorthTrue => todo!(),
//             _ => todo!(),
//         }
//     }
// }
// impl From<AcaciaButtonBlock> for BlockState {
//     fn from(b: AcaciaButtonBlock) -> Self {
//         match b {
//             AcaciaButtonBlock {
//                 face: properties::Face::Floor,
//                 facing: properties::Facing::North,
//                 powered: properties::Powered::True,
//             } => BlockState::AcaciaButton_FloorNorthTrue,
//             // AcaciaButtonBlock {
//             //     face: properties::Face::Wall,
//             //     facing: properties::Facing::North,
//             //     powered: properties::Powered::True,
//             // } => todo!(),
//             // AcaciaButtonBlock {
//             //     face: properties::Face::Ceiling,
//             //     facing: properties::Facing::North,
//             //     powered: properties::Powered::True,
//             // } => todo!(),
//             _ => todo!(),
//         }
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     fn test_from_state_to_block() {
//         let state = BlockState::AcaciaButton_CeilingSouthFalse;
//         let block_state = BlockState::from(state);
//         let block: Box<dyn Block> = block_state.into();
//         assert_eq!(block.id(), "acacia_button");
//         // downcast block to AcaciaButtonBlock
//         // let acacia_button_block = block.try_into::<AcaciaButtonBlock>().unwrap();
//         // assert_eq!(acacia_button_block.face, Face::Ceiling);
//         // assert_eq!(acacia_button_block.facing, Facing::South);
//         // assert_eq!(acacia_button_block.powered, Powered::False);
//     }

//     fn test_from_state_to_block_bottom_edge() {
//         let state = BlockState::AcaciaButton_FloorNorthTrue;
//         let block_state = BlockState::from(state);
//         let block: Box<dyn Block> = block_state.into();
//         assert_eq!(block.id(), "acacia_button");
//     }
// }
// }
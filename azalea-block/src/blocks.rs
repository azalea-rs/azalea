use crate::BlockBehavior;
use block_macros::make_block_states;

pub trait Block {
    fn behavior(&self) -> BlockBehavior;
    fn id(&self) -> &'static str;
}

make_block_states! {
    Properties => {
        "snowy" => Snowy {
            True,
            False,
        },
        "stage" => OakSaplingStage {
            _0,
            _1,
        },
        "stage" => SpruceSaplingStage {
            _0,
            _1,
        },
        "stage" => BirchSaplingStage {
            _0,
            _1,
        },
        "stage" => JungleSaplingStage {
            _0,
            _1,
        },
        "stage" => AcaciaSaplingStage {
            _0,
            _1,
        },
        "stage" => DarkOakSaplingStage {
            _0,
            _1,
        },
        "age" => MangrovePropaguleAge {
            _0,
            _1,
            _2,
            _3,
            _4,
        },
        "hanging" => Hanging {
            True,
            False,
        },
        "stage" => MangrovePropaguleStage {
            _0,
            _1,
        },
        "waterlogged" => Waterlogged {
            True,
            False,
        },
        "level" => WaterLevel {
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
        "level" => LavaLevel {
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
        "axis" => Axis {
            X,
            Y,
            Z,
        },
        "distance" => OakLeavesDistance {
            _1,
            _2,
            _3,
            _4,
            _5,
            _6,
            _7,
        },
        "persistent" => Persistent {
            True,
            False,
        },
        "distance" => SpruceLeavesDistance {
            _1,
            _2,
            _3,
            _4,
            _5,
            _6,
            _7,
        },
        "distance" => BirchLeavesDistance {
            _1,
            _2,
            _3,
            _4,
            _5,
            _6,
            _7,
        },
        "distance" => JungleLeavesDistance {
            _1,
            _2,
            _3,
            _4,
            _5,
            _6,
            _7,
        },
        "distance" => AcaciaLeavesDistance {
            _1,
            _2,
            _3,
            _4,
            _5,
            _6,
            _7,
        },
        "distance" => DarkOakLeavesDistance {
            _1,
            _2,
            _3,
            _4,
            _5,
            _6,
            _7,
        },
        "distance" => MangroveLeavesDistance {
            _1,
            _2,
            _3,
            _4,
            _5,
            _6,
            _7,
        },
        "distance" => AzaleaLeavesDistance {
            _1,
            _2,
            _3,
            _4,
            _5,
            _6,
            _7,
        },
        "distance" => FloweringAzaleaLeavesDistance {
            _1,
            _2,
            _3,
            _4,
            _5,
            _6,
            _7,
        },
        "facing" => FacingCubic {
            North,
            East,
            South,
            West,
            Up,
            Down,
        },
        "triggered" => Triggered {
            True,
            False,
        },
        "instrument" => Instrument {
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
        "note" => NoteBlockNote {
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
        "powered" => Powered {
            True,
            False,
        },
        "facing" => FacingCardinal {
            North,
            South,
            West,
            East,
        },
        "occupied" => Occupied {
            True,
            False,
        },
        "part" => Part {
            Head,
            Foot,
        },
        "shape" => RailShape {
            NorthSouth,
            EastWest,
            AscendingEast,
            AscendingWest,
            AscendingNorth,
            AscendingSouth,
        },
        "extended" => Extended {
            True,
            False,
        },
        "half" => Half {
            Upper,
            Lower,
        },
        "kind" => PistonType {
            Normal,
            Sticky,
        },
        "short" => Short {
            True,
            False,
        },
        "unstable" => Unstable {
            True,
            False,
        },
        "age" => FireAge {
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
        "east" => East {
            True,
            False,
        },
        "north" => North {
            True,
            False,
        },
        "south" => South {
            True,
            False,
        },
        "up" => Up {
            True,
            False,
        },
        "west" => West {
            True,
            False,
        },
        "half" => TopBottom {
            Top,
            Bottom,
        },
        "shape" => StairShape {
            Straight,
            InnerLeft,
            InnerRight,
            OuterLeft,
            OuterRight,
        },
        "kind" => ChestType {
            Single,
            Left,
            Right,
        },
        "east" => WireEast {
            Up,
            Side,
            None,
        },
        "north" => WireNorth {
            Up,
            Side,
            None,
        },
        "power" => RedstoneWirePower {
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
        "south" => WireSouth {
            Up,
            Side,
            None,
        },
        "west" => WireWest {
            Up,
            Side,
            None,
        },
        "age" => WheatAge {
            _0,
            _1,
            _2,
            _3,
            _4,
            _5,
            _6,
            _7,
        },
        "moisture" => FarmlandMoisture {
            _0,
            _1,
            _2,
            _3,
            _4,
            _5,
            _6,
            _7,
        },
        "lit" => Lit {
            True,
            False,
        },
        "rotation" => OakSignRotation {
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
        "rotation" => SpruceSignRotation {
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
        "rotation" => BirchSignRotation {
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
        "rotation" => AcaciaSignRotation {
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
        "rotation" => JungleSignRotation {
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
        "rotation" => DarkOakSignRotation {
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
        "rotation" => MangroveSignRotation {
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
        "hinge" => Hinge {
            Left,
            Right,
        },
        "open" => Open {
            True,
            False,
        },
        "shape" => Shape {
            NorthSouth,
            EastWest,
            AscendingEast,
            AscendingWest,
            AscendingNorth,
            AscendingSouth,
            SouthEast,
            SouthWest,
            NorthWest,
            NorthEast,
        },
        "face" => Face {
            Floor,
            Wall,
            Ceiling,
        },
        "layers" => SnowLayers {
            _1,
            _2,
            _3,
            _4,
            _5,
            _6,
            _7,
            _8,
        },
        "age" => CactusAge {
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
        "age" => SugarCaneAge {
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
        "has_record" => HasRecord {
            True,
            False,
        },
        "axis" => AxisXZ {
            X,
            Z,
        },
        "bites" => CakeBites {
            _0,
            _1,
            _2,
            _3,
            _4,
            _5,
            _6,
        },
        "delay" => RepeaterDelay {
            _1,
            _2,
            _3,
            _4,
        },
        "locked" => Locked {
            True,
            False,
        },
        "down" => Down {
            True,
            False,
        },
        "age" => PumpkinStemAge {
            _0,
            _1,
            _2,
            _3,
            _4,
            _5,
            _6,
            _7,
        },
        "age" => MelonStemAge {
            _0,
            _1,
            _2,
            _3,
            _4,
            _5,
            _6,
            _7,
        },
        "berries" => TrueFalse {
            True,
            False,
        },
        "in_wall" => InWall {
            True,
            False,
        },
        "age" => NetherWartAge {
            _0,
            _1,
            _2,
            _3,
        },
        "has_bottle" => HasBottle {
            True,
            False,
        },
        "level" => WaterCauldronLevel {
            _1,
            _2,
            _3,
        },
        "level" => PowderSnowCauldronLevel {
            _1,
            _2,
            _3,
        },
        "eye" => HasEye {
            True,
            False,
        },
        "age" => CocoaAge {
            _0,
            _1,
            _2,
        },
        "attached" => Attached {
            True,
            False,
        },
        "disarmed" => Disarmed {
            True,
            False,
        },
        "conditional" => Conditional {
            True,
            False,
        },
        "east" => EastWall {
            None,
            Low,
            Tall,
        },
        "north" => NorthWall {
            None,
            Low,
            Tall,
        },
        "south" => SouthWall {
            None,
            Low,
            Tall,
        },
        "west" => WestWall {
            None,
            Low,
            Tall,
        },
        "age" => CarrotsAge {
            _0,
            _1,
            _2,
            _3,
            _4,
            _5,
            _6,
            _7,
        },
        "age" => PotatoesAge {
            _0,
            _1,
            _2,
            _3,
            _4,
            _5,
            _6,
            _7,
        },
        "rotation" => SkeletonSkullRotation {
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
        "rotation" => WitherSkeletonSkullRotation {
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
        "rotation" => ZombieHeadRotation {
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
        "rotation" => PlayerHeadRotation {
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
        "rotation" => CreeperHeadRotation {
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
        "rotation" => DragonHeadRotation {
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
        "power" => LightWeightedPressurePlatePower {
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
        "power" => HeavyWeightedPressurePlatePower {
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
        "mode" => ComparatorType {
            Compare,
            Subtract,
        },
        "inverted" => Inverted {
            True,
            False,
        },
        "power" => DaylightDetectorPower {
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
        "enabled" => Enabled {
            True,
            False,
        },
        "facing" => Facing {
            Down,
            North,
            South,
            West,
            East,
        },
        "level" => LightLevel {
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
        "kind" => Type {
            Top,
            Bottom,
            Double,
        },
        "rotation" => WhiteBannerRotation {
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
        "rotation" => OrangeBannerRotation {
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
        "rotation" => MagentaBannerRotation {
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
        "rotation" => LightBlueBannerRotation {
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
        "rotation" => YellowBannerRotation {
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
        "rotation" => LimeBannerRotation {
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
        "rotation" => PinkBannerRotation {
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
        "rotation" => GrayBannerRotation {
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
        "rotation" => LightGrayBannerRotation {
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
        "rotation" => CyanBannerRotation {
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
        "rotation" => PurpleBannerRotation {
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
        "rotation" => BlueBannerRotation {
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
        "rotation" => BrownBannerRotation {
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
        "rotation" => GreenBannerRotation {
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
        "rotation" => RedBannerRotation {
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
        "rotation" => BlackBannerRotation {
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
        "age" => ChorusFlowerAge {
            _0,
            _1,
            _2,
            _3,
            _4,
            _5,
        },
        "age" => BeetrootsAge {
            _0,
            _1,
            _2,
            _3,
        },
        "age" => FrostedIceAge {
            _0,
            _1,
            _2,
            _3,
        },
        "age" => KelpAge {
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
            _25,
        },
        "eggs" => TurtleEggEggs {
            _1,
            _2,
            _3,
            _4,
        },
        "hatch" => TurtleEggHatch {
            _0,
            _1,
            _2,
        },
        "pickles" => SeaPicklePickles {
            _1,
            _2,
            _3,
            _4,
        },
        "age" => BambooAge {
            _0,
            _1,
        },
        "leaves" => Leaves {
            None,
            Small,
            Large,
        },
        "stage" => BambooStage {
            _0,
            _1,
        },
        "drag" => DragDown {
            True,
            False,
        },
        "bottom" => Bottom {
            True,
            False,
        },
        "distance" => ScaffoldingDistance {
            _0,
            _1,
            _2,
            _3,
            _4,
            _5,
            _6,
            _7,
        },
        "has_book" => HasBook {
            True,
            False,
        },
        "attachment" => Attachment {
            Floor,
            Ceiling,
            SingleWall,
            DoubleWall,
        },
        "signal_fire" => SignalFire {
            True,
            False,
        },
        "age" => SweetBerryBushAge {
            _0,
            _1,
            _2,
            _3,
        },
        "age" => WeepingVinesAge {
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
            _25,
        },
        "age" => TwistingVinesAge {
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
            _25,
        },
        "rotation" => CrimsonSignRotation {
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
        "rotation" => WarpedSignRotation {
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
        "mode" => Mode {
            Save,
            Load,
            Corner,
            Data,
        },
        "orientation" => Orientation {
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
        "level" => ComposterLevel {
            _0,
            _1,
            _2,
            _3,
            _4,
            _5,
            _6,
            _7,
            _8,
        },
        "power" => TargetOutputPower {
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
        "honey_level" => BeeNestHoneyLevel {
            _0,
            _1,
            _2,
            _3,
            _4,
            _5,
        },
        "honey_level" => BeehiveHoneyLevel {
            _0,
            _1,
            _2,
            _3,
            _4,
            _5,
        },
        "charges" => RespawnAnchorCharge {
            _0,
            _1,
            _2,
            _3,
            _4,
        },
        "candles" => CandleCandles {
            _1,
            _2,
            _3,
            _4,
        },
        "candles" => WhiteCandleCandles {
            _1,
            _2,
            _3,
            _4,
        },
        "candles" => OrangeCandleCandles {
            _1,
            _2,
            _3,
            _4,
        },
        "candles" => MagentaCandleCandles {
            _1,
            _2,
            _3,
            _4,
        },
        "candles" => LightBlueCandleCandles {
            _1,
            _2,
            _3,
            _4,
        },
        "candles" => YellowCandleCandles {
            _1,
            _2,
            _3,
            _4,
        },
        "candles" => LimeCandleCandles {
            _1,
            _2,
            _3,
            _4,
        },
        "candles" => PinkCandleCandles {
            _1,
            _2,
            _3,
            _4,
        },
        "candles" => GrayCandleCandles {
            _1,
            _2,
            _3,
            _4,
        },
        "candles" => LightGrayCandleCandles {
            _1,
            _2,
            _3,
            _4,
        },
        "candles" => CyanCandleCandles {
            _1,
            _2,
            _3,
            _4,
        },
        "candles" => PurpleCandleCandles {
            _1,
            _2,
            _3,
            _4,
        },
        "candles" => BlueCandleCandles {
            _1,
            _2,
            _3,
            _4,
        },
        "candles" => BrownCandleCandles {
            _1,
            _2,
            _3,
            _4,
        },
        "candles" => GreenCandleCandles {
            _1,
            _2,
            _3,
            _4,
        },
        "candles" => RedCandleCandles {
            _1,
            _2,
            _3,
            _4,
        },
        "candles" => BlackCandleCandles {
            _1,
            _2,
            _3,
            _4,
        },
        "power" => SculkSensorPower {
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
        "sculk_sensor_phase" => Phase {
            Inactive,
            Active,
            Cooldown,
        },
        "bloom" => Pulse {
            True,
            False,
        },
        "can_summon" => CanSummon {
            True,
            False,
        },
        "shrieking" => Shrieking {
            True,
            False,
        },
        "thickness" => Thickness {
            TipMerge,
            Tip,
            Frustum,
            Middle,
            Base,
        },
        "vertical_direction" => TipDirection {
            Up,
            Down,
        },
        "age" => _0_1_2_3_4_5_6_7_8_9_10_11_12_13_14_15_16_17_18_19_20_21_22_23_24_25 {
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
            _25,
        },
        "tilt" => Tilt {
            None,
            Unstable,
            Partial,
            Full,
        },
        "axis" => XYZ {
            X,
            Y,
            Z,
        },
    },
    Blocks => {
        air => BlockBehavior::default(), {
        },
        stone => BlockBehavior::default(), {
        },
        granite => BlockBehavior::default(), {
        },
        polished_granite => BlockBehavior::default(), {
        },
        diorite => BlockBehavior::default(), {
        },
        polished_diorite => BlockBehavior::default(), {
        },
        andesite => BlockBehavior::default(), {
        },
        polished_andesite => BlockBehavior::default(), {
        },
        grass_block => BlockBehavior::default(), {
            Snowy=False,
        },
        dirt => BlockBehavior::default(), {
        },
        coarse_dirt => BlockBehavior::default(), {
        },
        podzol => BlockBehavior::default(), {
            Snowy=False,
        },
        cobblestone => BlockBehavior::default(), {
        },
        oak_planks => BlockBehavior::default(), {
        },
        spruce_planks => BlockBehavior::default(), {
        },
        birch_planks => BlockBehavior::default(), {
        },
        jungle_planks => BlockBehavior::default(), {
        },
        acacia_planks => BlockBehavior::default(), {
        },
        dark_oak_planks => BlockBehavior::default(), {
        },
        mangrove_planks => BlockBehavior::default(), {
        },
        oak_sapling => BlockBehavior::default(), {
            OakSaplingStage=_0,
        },
        spruce_sapling => BlockBehavior::default(), {
            SpruceSaplingStage=_0,
        },
        birch_sapling => BlockBehavior::default(), {
            BirchSaplingStage=_0,
        },
        jungle_sapling => BlockBehavior::default(), {
            JungleSaplingStage=_0,
        },
        acacia_sapling => BlockBehavior::default(), {
            AcaciaSaplingStage=_0,
        },
        dark_oak_sapling => BlockBehavior::default(), {
            DarkOakSaplingStage=_0,
        },
        mangrove_propagule => BlockBehavior::default(), {
            MangrovePropaguleAge=_0,
            Hanging=False,
            MangrovePropaguleStage=_0,
            Waterlogged=False,
        },
        bedrock => BlockBehavior::default(), {
        },
        water => BlockBehavior::default(), {
            WaterLevel=_0,
        },
        lava => BlockBehavior::default(), {
            LavaLevel=_0,
        },
        sand => BlockBehavior::default(), {
        },
        red_sand => BlockBehavior::default(), {
        },
        gravel => BlockBehavior::default(), {
        },
        gold_ore => BlockBehavior::default(), {
        },
        deepslate_gold_ore => BlockBehavior::default(), {
        },
        iron_ore => BlockBehavior::default(), {
        },
        deepslate_iron_ore => BlockBehavior::default(), {
        },
        coal_ore => BlockBehavior::default(), {
        },
        deepslate_coal_ore => BlockBehavior::default(), {
        },
        nether_gold_ore => BlockBehavior::default(), {
        },
        oak_log => BlockBehavior::default(), {
            Axis=Y,
        },
        spruce_log => BlockBehavior::default(), {
            Axis=Y,
        },
        birch_log => BlockBehavior::default(), {
            Axis=Y,
        },
        jungle_log => BlockBehavior::default(), {
            Axis=Y,
        },
        acacia_log => BlockBehavior::default(), {
            Axis=Y,
        },
        dark_oak_log => BlockBehavior::default(), {
            Axis=Y,
        },
        mangrove_log => BlockBehavior::default(), {
            Axis=Y,
        },
        mangrove_roots => BlockBehavior::default(), {
            Waterlogged=False,
        },
        muddy_mangrove_roots => BlockBehavior::default(), {
            Axis=Y,
        },
        stripped_spruce_log => BlockBehavior::default(), {
            Axis=Y,
        },
        stripped_birch_log => BlockBehavior::default(), {
            Axis=Y,
        },
        stripped_jungle_log => BlockBehavior::default(), {
            Axis=Y,
        },
        stripped_acacia_log => BlockBehavior::default(), {
            Axis=Y,
        },
        stripped_dark_oak_log => BlockBehavior::default(), {
            Axis=Y,
        },
        stripped_oak_log => BlockBehavior::default(), {
            Axis=Y,
        },
        stripped_mangrove_log => BlockBehavior::default(), {
            Axis=Y,
        },
        oak_wood => BlockBehavior::default(), {
            Axis=Y,
        },
        spruce_wood => BlockBehavior::default(), {
            Axis=Y,
        },
        birch_wood => BlockBehavior::default(), {
            Axis=Y,
        },
        jungle_wood => BlockBehavior::default(), {
            Axis=Y,
        },
        acacia_wood => BlockBehavior::default(), {
            Axis=Y,
        },
        dark_oak_wood => BlockBehavior::default(), {
            Axis=Y,
        },
        mangrove_wood => BlockBehavior::default(), {
            Axis=Y,
        },
        stripped_oak_wood => BlockBehavior::default(), {
            Axis=Y,
        },
        stripped_spruce_wood => BlockBehavior::default(), {
            Axis=Y,
        },
        stripped_birch_wood => BlockBehavior::default(), {
            Axis=Y,
        },
        stripped_jungle_wood => BlockBehavior::default(), {
            Axis=Y,
        },
        stripped_acacia_wood => BlockBehavior::default(), {
            Axis=Y,
        },
        stripped_dark_oak_wood => BlockBehavior::default(), {
            Axis=Y,
        },
        stripped_mangrove_wood => BlockBehavior::default(), {
            Axis=Y,
        },
        oak_leaves => BlockBehavior::default(), {
            OakLeavesDistance=_7,
            Persistent=False,
            Waterlogged=False,
        },
        spruce_leaves => BlockBehavior::default(), {
            SpruceLeavesDistance=_7,
            Persistent=False,
            Waterlogged=False,
        },
        birch_leaves => BlockBehavior::default(), {
            BirchLeavesDistance=_7,
            Persistent=False,
            Waterlogged=False,
        },
        jungle_leaves => BlockBehavior::default(), {
            JungleLeavesDistance=_7,
            Persistent=False,
            Waterlogged=False,
        },
        acacia_leaves => BlockBehavior::default(), {
            AcaciaLeavesDistance=_7,
            Persistent=False,
            Waterlogged=False,
        },
        dark_oak_leaves => BlockBehavior::default(), {
            DarkOakLeavesDistance=_7,
            Persistent=False,
            Waterlogged=False,
        },
        mangrove_leaves => BlockBehavior::default(), {
            MangroveLeavesDistance=_7,
            Persistent=False,
            Waterlogged=False,
        },
        azalea_leaves => BlockBehavior::default(), {
            AzaleaLeavesDistance=_7,
            Persistent=False,
            Waterlogged=False,
        },
        flowering_azalea_leaves => BlockBehavior::default(), {
            FloweringAzaleaLeavesDistance=_7,
            Persistent=False,
            Waterlogged=False,
        },
        sponge => BlockBehavior::default(), {
        },
        wet_sponge => BlockBehavior::default(), {
        },
        glass => BlockBehavior::default(), {
        },
        lapis_ore => BlockBehavior::default(), {
        },
        deepslate_lapis_ore => BlockBehavior::default(), {
        },
        lapis_block => BlockBehavior::default(), {
        },
        dispenser => BlockBehavior::default(), {
            FacingCubic=North,
            Triggered=False,
        },
        sandstone => BlockBehavior::default(), {
        },
        chiseled_sandstone => BlockBehavior::default(), {
        },
        cut_sandstone => BlockBehavior::default(), {
        },
        note_block => BlockBehavior::default(), {
            Instrument=Harp,
            NoteBlockNote=_0,
            Powered=False,
        },
        white_bed => BlockBehavior::default(), {
            FacingCardinal=North,
            Occupied=False,
            Part=Foot,
        },
        orange_bed => BlockBehavior::default(), {
            FacingCardinal=North,
            Occupied=False,
            Part=Foot,
        },
        magenta_bed => BlockBehavior::default(), {
            FacingCardinal=North,
            Occupied=False,
            Part=Foot,
        },
        light_blue_bed => BlockBehavior::default(), {
            FacingCardinal=North,
            Occupied=False,
            Part=Foot,
        },
        yellow_bed => BlockBehavior::default(), {
            FacingCardinal=North,
            Occupied=False,
            Part=Foot,
        },
        lime_bed => BlockBehavior::default(), {
            FacingCardinal=North,
            Occupied=False,
            Part=Foot,
        },
        pink_bed => BlockBehavior::default(), {
            FacingCardinal=North,
            Occupied=False,
            Part=Foot,
        },
        gray_bed => BlockBehavior::default(), {
            FacingCardinal=North,
            Occupied=False,
            Part=Foot,
        },
        light_gray_bed => BlockBehavior::default(), {
            FacingCardinal=North,
            Occupied=False,
            Part=Foot,
        },
        cyan_bed => BlockBehavior::default(), {
            FacingCardinal=North,
            Occupied=False,
            Part=Foot,
        },
        purple_bed => BlockBehavior::default(), {
            FacingCardinal=North,
            Occupied=False,
            Part=Foot,
        },
        blue_bed => BlockBehavior::default(), {
            FacingCardinal=North,
            Occupied=False,
            Part=Foot,
        },
        brown_bed => BlockBehavior::default(), {
            FacingCardinal=North,
            Occupied=False,
            Part=Foot,
        },
        green_bed => BlockBehavior::default(), {
            FacingCardinal=North,
            Occupied=False,
            Part=Foot,
        },
        red_bed => BlockBehavior::default(), {
            FacingCardinal=North,
            Occupied=False,
            Part=Foot,
        },
        black_bed => BlockBehavior::default(), {
            FacingCardinal=North,
            Occupied=False,
            Part=Foot,
        },
        powered_rail => BlockBehavior::default(), {
            Powered=False,
            RailShape=NorthSouth,
            Waterlogged=False,
        },
        detector_rail => BlockBehavior::default(), {
            Powered=False,
            RailShape=NorthSouth,
            Waterlogged=False,
        },
        sticky_piston => BlockBehavior::default(), {
            Extended=False,
            FacingCubic=North,
        },
        cobweb => BlockBehavior::default(), {
        },
        grass => BlockBehavior::default(), {
        },
        fern => BlockBehavior::default(), {
        },
        dead_bush => BlockBehavior::default(), {
        },
        seagrass => BlockBehavior::default(), {
        },
        tall_seagrass => BlockBehavior::default(), {
            Half=Lower,
        },
        piston => BlockBehavior::default(), {
            Extended=False,
            FacingCubic=North,
        },
        piston_head => BlockBehavior::default(), {
            PistonType=Normal,
            FacingCubic=North,
            Short=False,
        },
        white_wool => BlockBehavior::default(), {
        },
        orange_wool => BlockBehavior::default(), {
        },
        magenta_wool => BlockBehavior::default(), {
        },
        light_blue_wool => BlockBehavior::default(), {
        },
        yellow_wool => BlockBehavior::default(), {
        },
        lime_wool => BlockBehavior::default(), {
        },
        pink_wool => BlockBehavior::default(), {
        },
        gray_wool => BlockBehavior::default(), {
        },
        light_gray_wool => BlockBehavior::default(), {
        },
        cyan_wool => BlockBehavior::default(), {
        },
        purple_wool => BlockBehavior::default(), {
        },
        blue_wool => BlockBehavior::default(), {
        },
        brown_wool => BlockBehavior::default(), {
        },
        green_wool => BlockBehavior::default(), {
        },
        red_wool => BlockBehavior::default(), {
        },
        black_wool => BlockBehavior::default(), {
        },
        moving_piston => BlockBehavior::default(), {
            PistonType=Normal,
            FacingCubic=North,
        },
        dandelion => BlockBehavior::default(), {
        },
        poppy => BlockBehavior::default(), {
        },
        blue_orchid => BlockBehavior::default(), {
        },
        allium => BlockBehavior::default(), {
        },
        azure_bluet => BlockBehavior::default(), {
        },
        red_tulip => BlockBehavior::default(), {
        },
        orange_tulip => BlockBehavior::default(), {
        },
        white_tulip => BlockBehavior::default(), {
        },
        pink_tulip => BlockBehavior::default(), {
        },
        oxeye_daisy => BlockBehavior::default(), {
        },
        cornflower => BlockBehavior::default(), {
        },
        wither_rose => BlockBehavior::default(), {
        },
        lily_of_the_valley => BlockBehavior::default(), {
        },
        brown_mushroom => BlockBehavior::default(), {
        },
        red_mushroom => BlockBehavior::default(), {
        },
        gold_block => BlockBehavior::default(), {
        },
        iron_block => BlockBehavior::default(), {
        },
        bricks => BlockBehavior::default(), {
        },
        tnt => BlockBehavior::default(), {
            Unstable=False,
        },
        bookshelf => BlockBehavior::default(), {
        },
        mossy_cobblestone => BlockBehavior::default(), {
        },
        obsidian => BlockBehavior::default(), {
        },
        torch => BlockBehavior::default(), {
        },
        wall_torch => BlockBehavior::default(), {
            FacingCardinal=North,
        },
        fire => BlockBehavior::default(), {
            FireAge=_0,
            East=False,
            North=False,
            South=False,
            Up=False,
            West=False,
        },
        soul_fire => BlockBehavior::default(), {
        },
        spawner => BlockBehavior::default(), {
        },
        oak_stairs => BlockBehavior::default(), {
            FacingCardinal=North,
            TopBottom=Bottom,
            StairShape=Straight,
            Waterlogged=False,
        },
        chest => BlockBehavior::default(), {
            ChestType=Single,
            FacingCardinal=North,
            Waterlogged=False,
        },
        redstone_wire => BlockBehavior::default(), {
            WireEast=None,
            WireNorth=None,
            RedstoneWirePower=_0,
            WireSouth=None,
            WireWest=None,
        },
        diamond_ore => BlockBehavior::default(), {
        },
        deepslate_diamond_ore => BlockBehavior::default(), {
        },
        diamond_block => BlockBehavior::default(), {
        },
        crafting_table => BlockBehavior::default(), {
        },
        wheat => BlockBehavior::default(), {
            WheatAge=_0,
        },
        farmland => BlockBehavior::default(), {
            FarmlandMoisture=_0,
        },
        furnace => BlockBehavior::default(), {
            FacingCardinal=North,
            Lit=False,
        },
        oak_sign => BlockBehavior::default(), {
            OakSignRotation=_0,
            Waterlogged=False,
        },
        spruce_sign => BlockBehavior::default(), {
            SpruceSignRotation=_0,
            Waterlogged=False,
        },
        birch_sign => BlockBehavior::default(), {
            BirchSignRotation=_0,
            Waterlogged=False,
        },
        acacia_sign => BlockBehavior::default(), {
            AcaciaSignRotation=_0,
            Waterlogged=False,
        },
        jungle_sign => BlockBehavior::default(), {
            JungleSignRotation=_0,
            Waterlogged=False,
        },
        dark_oak_sign => BlockBehavior::default(), {
            DarkOakSignRotation=_0,
            Waterlogged=False,
        },
        mangrove_sign => BlockBehavior::default(), {
            MangroveSignRotation=_0,
            Waterlogged=False,
        },
        oak_door => BlockBehavior::default(), {
            FacingCardinal=North,
            Half=Lower,
            Hinge=Left,
            Open=False,
            Powered=False,
        },
        ladder => BlockBehavior::default(), {
            FacingCardinal=North,
            Waterlogged=False,
        },
        rail => BlockBehavior::default(), {
            Shape=NorthSouth,
            Waterlogged=False,
        },
        cobblestone_stairs => BlockBehavior::default(), {
            FacingCardinal=North,
            TopBottom=Bottom,
            StairShape=Straight,
            Waterlogged=False,
        },
        oak_wall_sign => BlockBehavior::default(), {
            FacingCardinal=North,
            Waterlogged=False,
        },
        spruce_wall_sign => BlockBehavior::default(), {
            FacingCardinal=North,
            Waterlogged=False,
        },
        birch_wall_sign => BlockBehavior::default(), {
            FacingCardinal=North,
            Waterlogged=False,
        },
        acacia_wall_sign => BlockBehavior::default(), {
            FacingCardinal=North,
            Waterlogged=False,
        },
        jungle_wall_sign => BlockBehavior::default(), {
            FacingCardinal=North,
            Waterlogged=False,
        },
        dark_oak_wall_sign => BlockBehavior::default(), {
            FacingCardinal=North,
            Waterlogged=False,
        },
        mangrove_wall_sign => BlockBehavior::default(), {
            FacingCardinal=North,
            Waterlogged=False,
        },
        lever => BlockBehavior::default(), {
            Face=Wall,
            FacingCardinal=North,
            Powered=False,
        },
        stone_pressure_plate => BlockBehavior::default(), {
            Powered=False,
        },
        iron_door => BlockBehavior::default(), {
            FacingCardinal=North,
            Half=Lower,
            Hinge=Left,
            Open=False,
            Powered=False,
        },
        oak_pressure_plate => BlockBehavior::default(), {
            Powered=False,
        },
        spruce_pressure_plate => BlockBehavior::default(), {
            Powered=False,
        },
        birch_pressure_plate => BlockBehavior::default(), {
            Powered=False,
        },
        jungle_pressure_plate => BlockBehavior::default(), {
            Powered=False,
        },
        acacia_pressure_plate => BlockBehavior::default(), {
            Powered=False,
        },
        dark_oak_pressure_plate => BlockBehavior::default(), {
            Powered=False,
        },
        mangrove_pressure_plate => BlockBehavior::default(), {
            Powered=False,
        },
        redstone_ore => BlockBehavior::default(), {
            Lit=False,
        },
        deepslate_redstone_ore => BlockBehavior::default(), {
            Lit=False,
        },
        redstone_torch => BlockBehavior::default(), {
            Lit=True,
        },
        redstone_wall_torch => BlockBehavior::default(), {
            FacingCardinal=North,
            Lit=True,
        },
        stone_button => BlockBehavior::default(), {
            Face=Wall,
            FacingCardinal=North,
            Powered=False,
        },
        snow => BlockBehavior::default(), {
            SnowLayers=_1,
        },
        ice => BlockBehavior::default(), {
        },
        snow_block => BlockBehavior::default(), {
        },
        cactus => BlockBehavior::default(), {
            CactusAge=_0,
        },
        clay => BlockBehavior::default(), {
        },
        sugar_cane => BlockBehavior::default(), {
            SugarCaneAge=_0,
        },
        jukebox => BlockBehavior::default(), {
            HasRecord=False,
        },
        oak_fence => BlockBehavior::default(), {
            East=False,
            North=False,
            South=False,
            Waterlogged=False,
            West=False,
        },
        pumpkin => BlockBehavior::default(), {
        },
        netherrack => BlockBehavior::default(), {
        },
        soul_sand => BlockBehavior::default(), {
        },
        soul_soil => BlockBehavior::default(), {
        },
        basalt => BlockBehavior::default(), {
            Axis=Y,
        },
        polished_basalt => BlockBehavior::default(), {
            Axis=Y,
        },
        soul_torch => BlockBehavior::default(), {
        },
        soul_wall_torch => BlockBehavior::default(), {
            FacingCardinal=North,
        },
        glowstone => BlockBehavior::default(), {
        },
        nether_portal => BlockBehavior::default(), {
            AxisXZ=X,
        },
        carved_pumpkin => BlockBehavior::default(), {
            FacingCardinal=North,
        },
        jack_o_lantern => BlockBehavior::default(), {
            FacingCardinal=North,
        },
        cake => BlockBehavior::default(), {
            CakeBites=_0,
        },
        repeater => BlockBehavior::default(), {
            RepeaterDelay=_1,
            FacingCardinal=North,
            Locked=False,
            Powered=False,
        },
        white_stained_glass => BlockBehavior::default(), {
        },
        orange_stained_glass => BlockBehavior::default(), {
        },
        magenta_stained_glass => BlockBehavior::default(), {
        },
        light_blue_stained_glass => BlockBehavior::default(), {
        },
        yellow_stained_glass => BlockBehavior::default(), {
        },
        lime_stained_glass => BlockBehavior::default(), {
        },
        pink_stained_glass => BlockBehavior::default(), {
        },
        gray_stained_glass => BlockBehavior::default(), {
        },
        light_gray_stained_glass => BlockBehavior::default(), {
        },
        cyan_stained_glass => BlockBehavior::default(), {
        },
        purple_stained_glass => BlockBehavior::default(), {
        },
        blue_stained_glass => BlockBehavior::default(), {
        },
        brown_stained_glass => BlockBehavior::default(), {
        },
        green_stained_glass => BlockBehavior::default(), {
        },
        red_stained_glass => BlockBehavior::default(), {
        },
        black_stained_glass => BlockBehavior::default(), {
        },
        oak_trapdoor => BlockBehavior::default(), {
            FacingCardinal=North,
            TopBottom=Bottom,
            Open=False,
            Powered=False,
            Waterlogged=False,
        },
        spruce_trapdoor => BlockBehavior::default(), {
            FacingCardinal=North,
            TopBottom=Bottom,
            Open=False,
            Powered=False,
            Waterlogged=False,
        },
        birch_trapdoor => BlockBehavior::default(), {
            FacingCardinal=North,
            TopBottom=Bottom,
            Open=False,
            Powered=False,
            Waterlogged=False,
        },
        jungle_trapdoor => BlockBehavior::default(), {
            FacingCardinal=North,
            TopBottom=Bottom,
            Open=False,
            Powered=False,
            Waterlogged=False,
        },
        acacia_trapdoor => BlockBehavior::default(), {
            FacingCardinal=North,
            TopBottom=Bottom,
            Open=False,
            Powered=False,
            Waterlogged=False,
        },
        dark_oak_trapdoor => BlockBehavior::default(), {
            FacingCardinal=North,
            TopBottom=Bottom,
            Open=False,
            Powered=False,
            Waterlogged=False,
        },
        mangrove_trapdoor => BlockBehavior::default(), {
            FacingCardinal=North,
            TopBottom=Bottom,
            Open=False,
            Powered=False,
            Waterlogged=False,
        },
        stone_bricks => BlockBehavior::default(), {
        },
        mossy_stone_bricks => BlockBehavior::default(), {
        },
        cracked_stone_bricks => BlockBehavior::default(), {
        },
        chiseled_stone_bricks => BlockBehavior::default(), {
        },
        packed_mud => BlockBehavior::default(), {
        },
        mud_bricks => BlockBehavior::default(), {
        },
        infested_stone => BlockBehavior::default(), {
        },
        infested_cobblestone => BlockBehavior::default(), {
        },
        infested_stone_bricks => BlockBehavior::default(), {
        },
        infested_mossy_stone_bricks => BlockBehavior::default(), {
        },
        infested_cracked_stone_bricks => BlockBehavior::default(), {
        },
        infested_chiseled_stone_bricks => BlockBehavior::default(), {
        },
        brown_mushroom_block => BlockBehavior::default(), {
            Down=True,
            East=True,
            North=True,
            South=True,
            Up=True,
            West=True,
        },
        red_mushroom_block => BlockBehavior::default(), {
            Down=True,
            East=True,
            North=True,
            South=True,
            Up=True,
            West=True,
        },
        mushroom_stem => BlockBehavior::default(), {
            Down=True,
            East=True,
            North=True,
            South=True,
            Up=True,
            West=True,
        },
        iron_bars => BlockBehavior::default(), {
            East=False,
            North=False,
            South=False,
            Waterlogged=False,
            West=False,
        },
        chain => BlockBehavior::default(), {
            Axis=Y,
            Waterlogged=False,
        },
        glass_pane => BlockBehavior::default(), {
            East=False,
            North=False,
            South=False,
            Waterlogged=False,
            West=False,
        },
        melon => BlockBehavior::default(), {
        },
        attached_pumpkin_stem => BlockBehavior::default(), {
            FacingCardinal=North,
        },
        attached_melon_stem => BlockBehavior::default(), {
            FacingCardinal=North,
        },
        pumpkin_stem => BlockBehavior::default(), {
            PumpkinStemAge=_0,
        },
        melon_stem => BlockBehavior::default(), {
            MelonStemAge=_0,
        },
        vine => BlockBehavior::default(), {
            East=False,
            North=False,
            South=False,
            Up=False,
            West=False,
        },
        glow_lichen => BlockBehavior::default(), {
            TrueFalse=False,
            TrueFalse=False,
            TrueFalse=False,
            TrueFalse=False,
            TrueFalse=False,
            TrueFalse=False,
            TrueFalse=False,
        },
        oak_fence_gate => BlockBehavior::default(), {
            FacingCardinal=North,
            InWall=False,
            Open=False,
            Powered=False,
        },
        brick_stairs => BlockBehavior::default(), {
            FacingCardinal=North,
            TopBottom=Bottom,
            StairShape=Straight,
            Waterlogged=False,
        },
        stone_brick_stairs => BlockBehavior::default(), {
            FacingCardinal=North,
            TopBottom=Bottom,
            StairShape=Straight,
            Waterlogged=False,
        },
        mud_brick_stairs => BlockBehavior::default(), {
            FacingCardinal=North,
            TopBottom=Bottom,
            StairShape=Straight,
            Waterlogged=False,
        },
        mycelium => BlockBehavior::default(), {
            Snowy=False,
        },
        lily_pad => BlockBehavior::default(), {
        },
        nether_bricks => BlockBehavior::default(), {
        },
        nether_brick_fence => BlockBehavior::default(), {
            East=False,
            North=False,
            South=False,
            Waterlogged=False,
            West=False,
        },
        nether_brick_stairs => BlockBehavior::default(), {
            FacingCardinal=North,
            TopBottom=Bottom,
            StairShape=Straight,
            Waterlogged=False,
        },
        nether_wart => BlockBehavior::default(), {
            NetherWartAge=_0,
        },
        enchanting_table => BlockBehavior::default(), {
        },
        brewing_stand => BlockBehavior::default(), {
            HasBottle=False,
            HasBottle=False,
            HasBottle=False,
        },
        cauldron => BlockBehavior::default(), {
        },
        water_cauldron => BlockBehavior::default(), {
            WaterCauldronLevel=_1,
        },
        lava_cauldron => BlockBehavior::default(), {
        },
        powder_snow_cauldron => BlockBehavior::default(), {
            PowderSnowCauldronLevel=_1,
        },
        end_portal => BlockBehavior::default(), {
        },
        end_portal_frame => BlockBehavior::default(), {
            HasEye=False,
            FacingCardinal=North,
        },
        end_stone => BlockBehavior::default(), {
        },
        dragon_egg => BlockBehavior::default(), {
        },
        redstone_lamp => BlockBehavior::default(), {
            Lit=False,
        },
        cocoa => BlockBehavior::default(), {
            CocoaAge=_0,
            FacingCardinal=North,
        },
        sandstone_stairs => BlockBehavior::default(), {
            FacingCardinal=North,
            TopBottom=Bottom,
            StairShape=Straight,
            Waterlogged=False,
        },
        emerald_ore => BlockBehavior::default(), {
        },
        deepslate_emerald_ore => BlockBehavior::default(), {
        },
        ender_chest => BlockBehavior::default(), {
            FacingCardinal=North,
            Waterlogged=False,
        },
        tripwire_hook => BlockBehavior::default(), {
            Attached=False,
            FacingCardinal=North,
            Powered=False,
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
        emerald_block => BlockBehavior::default(), {
        },
        spruce_stairs => BlockBehavior::default(), {
            FacingCardinal=North,
            TopBottom=Bottom,
            StairShape=Straight,
            Waterlogged=False,
        },
        birch_stairs => BlockBehavior::default(), {
            FacingCardinal=North,
            TopBottom=Bottom,
            StairShape=Straight,
            Waterlogged=False,
        },
        jungle_stairs => BlockBehavior::default(), {
            FacingCardinal=North,
            TopBottom=Bottom,
            StairShape=Straight,
            Waterlogged=False,
        },
        command_block => BlockBehavior::default(), {
            Conditional=False,
            FacingCubic=North,
        },
        beacon => BlockBehavior::default(), {
        },
        cobblestone_wall => BlockBehavior::default(), {
            EastWall=None,
            NorthWall=None,
            SouthWall=None,
            Up=True,
            Waterlogged=False,
            WestWall=None,
        },
        mossy_cobblestone_wall => BlockBehavior::default(), {
            EastWall=None,
            NorthWall=None,
            SouthWall=None,
            Up=True,
            Waterlogged=False,
            WestWall=None,
        },
        flower_pot => BlockBehavior::default(), {
        },
        potted_oak_sapling => BlockBehavior::default(), {
        },
        potted_spruce_sapling => BlockBehavior::default(), {
        },
        potted_birch_sapling => BlockBehavior::default(), {
        },
        potted_jungle_sapling => BlockBehavior::default(), {
        },
        potted_acacia_sapling => BlockBehavior::default(), {
        },
        potted_dark_oak_sapling => BlockBehavior::default(), {
        },
        potted_mangrove_propagule => BlockBehavior::default(), {
        },
        potted_fern => BlockBehavior::default(), {
        },
        potted_dandelion => BlockBehavior::default(), {
        },
        potted_poppy => BlockBehavior::default(), {
        },
        potted_blue_orchid => BlockBehavior::default(), {
        },
        potted_allium => BlockBehavior::default(), {
        },
        potted_azure_bluet => BlockBehavior::default(), {
        },
        potted_red_tulip => BlockBehavior::default(), {
        },
        potted_orange_tulip => BlockBehavior::default(), {
        },
        potted_white_tulip => BlockBehavior::default(), {
        },
        potted_pink_tulip => BlockBehavior::default(), {
        },
        potted_oxeye_daisy => BlockBehavior::default(), {
        },
        potted_cornflower => BlockBehavior::default(), {
        },
        potted_lily_of_the_valley => BlockBehavior::default(), {
        },
        potted_wither_rose => BlockBehavior::default(), {
        },
        potted_red_mushroom => BlockBehavior::default(), {
        },
        potted_brown_mushroom => BlockBehavior::default(), {
        },
        potted_dead_bush => BlockBehavior::default(), {
        },
        potted_cactus => BlockBehavior::default(), {
        },
        carrots => BlockBehavior::default(), {
            CarrotsAge=_0,
        },
        potatoes => BlockBehavior::default(), {
            PotatoesAge=_0,
        },
        oak_button => BlockBehavior::default(), {
            Face=Wall,
            FacingCardinal=North,
            Powered=False,
        },
        spruce_button => BlockBehavior::default(), {
            Face=Wall,
            FacingCardinal=North,
            Powered=False,
        },
        birch_button => BlockBehavior::default(), {
            Face=Wall,
            FacingCardinal=North,
            Powered=False,
        },
        jungle_button => BlockBehavior::default(), {
            Face=Wall,
            FacingCardinal=North,
            Powered=False,
        },
        acacia_button => BlockBehavior::default(), {
            Face=Wall,
            FacingCardinal=North,
            Powered=False,
        },
        dark_oak_button => BlockBehavior::default(), {
            Face=Wall,
            FacingCardinal=North,
            Powered=False,
        },
        mangrove_button => BlockBehavior::default(), {
            Face=Wall,
            FacingCardinal=North,
            Powered=False,
        },
        skeleton_skull => BlockBehavior::default(), {
            SkeletonSkullRotation=_0,
        },
        skeleton_wall_skull => BlockBehavior::default(), {
            FacingCardinal=North,
        },
        wither_skeleton_skull => BlockBehavior::default(), {
            WitherSkeletonSkullRotation=_0,
        },
        wither_skeleton_wall_skull => BlockBehavior::default(), {
            FacingCardinal=North,
        },
        zombie_head => BlockBehavior::default(), {
            ZombieHeadRotation=_0,
        },
        zombie_wall_head => BlockBehavior::default(), {
            FacingCardinal=North,
        },
        player_head => BlockBehavior::default(), {
            PlayerHeadRotation=_0,
        },
        player_wall_head => BlockBehavior::default(), {
            FacingCardinal=North,
        },
        creeper_head => BlockBehavior::default(), {
            CreeperHeadRotation=_0,
        },
        creeper_wall_head => BlockBehavior::default(), {
            FacingCardinal=North,
        },
        dragon_head => BlockBehavior::default(), {
            DragonHeadRotation=_0,
        },
        dragon_wall_head => BlockBehavior::default(), {
            FacingCardinal=North,
        },
        anvil => BlockBehavior::default(), {
            FacingCardinal=North,
        },
        chipped_anvil => BlockBehavior::default(), {
            FacingCardinal=North,
        },
        damaged_anvil => BlockBehavior::default(), {
            FacingCardinal=North,
        },
        trapped_chest => BlockBehavior::default(), {
            ChestType=Single,
            FacingCardinal=North,
            Waterlogged=False,
        },
        light_weighted_pressure_plate => BlockBehavior::default(), {
            LightWeightedPressurePlatePower=_0,
        },
        heavy_weighted_pressure_plate => BlockBehavior::default(), {
            HeavyWeightedPressurePlatePower=_0,
        },
        comparator => BlockBehavior::default(), {
            FacingCardinal=North,
            ComparatorType=Compare,
            Powered=False,
        },
        daylight_detector => BlockBehavior::default(), {
            Inverted=False,
            DaylightDetectorPower=_0,
        },
        redstone_block => BlockBehavior::default(), {
        },
        nether_quartz_ore => BlockBehavior::default(), {
        },
        hopper => BlockBehavior::default(), {
            Enabled=True,
            Facing=Down,
        },
        quartz_block => BlockBehavior::default(), {
        },
        chiseled_quartz_block => BlockBehavior::default(), {
        },
        quartz_pillar => BlockBehavior::default(), {
            Axis=Y,
        },
        quartz_stairs => BlockBehavior::default(), {
            FacingCardinal=North,
            TopBottom=Bottom,
            StairShape=Straight,
            Waterlogged=False,
        },
        activator_rail => BlockBehavior::default(), {
            Powered=False,
            RailShape=NorthSouth,
            Waterlogged=False,
        },
        dropper => BlockBehavior::default(), {
            FacingCubic=North,
            Triggered=False,
        },
        white_terracotta => BlockBehavior::default(), {
        },
        orange_terracotta => BlockBehavior::default(), {
        },
        magenta_terracotta => BlockBehavior::default(), {
        },
        light_blue_terracotta => BlockBehavior::default(), {
        },
        yellow_terracotta => BlockBehavior::default(), {
        },
        lime_terracotta => BlockBehavior::default(), {
        },
        pink_terracotta => BlockBehavior::default(), {
        },
        gray_terracotta => BlockBehavior::default(), {
        },
        light_gray_terracotta => BlockBehavior::default(), {
        },
        cyan_terracotta => BlockBehavior::default(), {
        },
        purple_terracotta => BlockBehavior::default(), {
        },
        blue_terracotta => BlockBehavior::default(), {
        },
        brown_terracotta => BlockBehavior::default(), {
        },
        green_terracotta => BlockBehavior::default(), {
        },
        red_terracotta => BlockBehavior::default(), {
        },
        black_terracotta => BlockBehavior::default(), {
        },
        white_stained_glass_pane => BlockBehavior::default(), {
            East=False,
            North=False,
            South=False,
            Waterlogged=False,
            West=False,
        },
        orange_stained_glass_pane => BlockBehavior::default(), {
            East=False,
            North=False,
            South=False,
            Waterlogged=False,
            West=False,
        },
        magenta_stained_glass_pane => BlockBehavior::default(), {
            East=False,
            North=False,
            South=False,
            Waterlogged=False,
            West=False,
        },
        light_blue_stained_glass_pane => BlockBehavior::default(), {
            East=False,
            North=False,
            South=False,
            Waterlogged=False,
            West=False,
        },
        yellow_stained_glass_pane => BlockBehavior::default(), {
            East=False,
            North=False,
            South=False,
            Waterlogged=False,
            West=False,
        },
        lime_stained_glass_pane => BlockBehavior::default(), {
            East=False,
            North=False,
            South=False,
            Waterlogged=False,
            West=False,
        },
        pink_stained_glass_pane => BlockBehavior::default(), {
            East=False,
            North=False,
            South=False,
            Waterlogged=False,
            West=False,
        },
        gray_stained_glass_pane => BlockBehavior::default(), {
            East=False,
            North=False,
            South=False,
            Waterlogged=False,
            West=False,
        },
        light_gray_stained_glass_pane => BlockBehavior::default(), {
            East=False,
            North=False,
            South=False,
            Waterlogged=False,
            West=False,
        },
        cyan_stained_glass_pane => BlockBehavior::default(), {
            East=False,
            North=False,
            South=False,
            Waterlogged=False,
            West=False,
        },
        purple_stained_glass_pane => BlockBehavior::default(), {
            East=False,
            North=False,
            South=False,
            Waterlogged=False,
            West=False,
        },
        blue_stained_glass_pane => BlockBehavior::default(), {
            East=False,
            North=False,
            South=False,
            Waterlogged=False,
            West=False,
        },
        brown_stained_glass_pane => BlockBehavior::default(), {
            East=False,
            North=False,
            South=False,
            Waterlogged=False,
            West=False,
        },
        green_stained_glass_pane => BlockBehavior::default(), {
            East=False,
            North=False,
            South=False,
            Waterlogged=False,
            West=False,
        },
        red_stained_glass_pane => BlockBehavior::default(), {
            East=False,
            North=False,
            South=False,
            Waterlogged=False,
            West=False,
        },
        black_stained_glass_pane => BlockBehavior::default(), {
            East=False,
            North=False,
            South=False,
            Waterlogged=False,
            West=False,
        },
        acacia_stairs => BlockBehavior::default(), {
            FacingCardinal=North,
            TopBottom=Bottom,
            StairShape=Straight,
            Waterlogged=False,
        },
        dark_oak_stairs => BlockBehavior::default(), {
            FacingCardinal=North,
            TopBottom=Bottom,
            StairShape=Straight,
            Waterlogged=False,
        },
        mangrove_stairs => BlockBehavior::default(), {
            FacingCardinal=North,
            TopBottom=Bottom,
            StairShape=Straight,
            Waterlogged=False,
        },
        slime_block => BlockBehavior::default(), {
        },
        barrier => BlockBehavior::default(), {
        },
        light => BlockBehavior::default(), {
            LightLevel=_15,
            Waterlogged=False,
        },
        iron_trapdoor => BlockBehavior::default(), {
            FacingCardinal=North,
            TopBottom=Bottom,
            Open=False,
            Powered=False,
            Waterlogged=False,
        },
        prismarine => BlockBehavior::default(), {
        },
        prismarine_bricks => BlockBehavior::default(), {
        },
        dark_prismarine => BlockBehavior::default(), {
        },
        prismarine_stairs => BlockBehavior::default(), {
            FacingCardinal=North,
            TopBottom=Bottom,
            StairShape=Straight,
            Waterlogged=False,
        },
        prismarine_brick_stairs => BlockBehavior::default(), {
            FacingCardinal=North,
            TopBottom=Bottom,
            StairShape=Straight,
            Waterlogged=False,
        },
        dark_prismarine_stairs => BlockBehavior::default(), {
            FacingCardinal=North,
            TopBottom=Bottom,
            StairShape=Straight,
            Waterlogged=False,
        },
        prismarine_slab => BlockBehavior::default(), {
            Type=Bottom,
            Waterlogged=False,
        },
        prismarine_brick_slab => BlockBehavior::default(), {
            Type=Bottom,
            Waterlogged=False,
        },
        dark_prismarine_slab => BlockBehavior::default(), {
            Type=Bottom,
            Waterlogged=False,
        },
        sea_lantern => BlockBehavior::default(), {
        },
        hay_block => BlockBehavior::default(), {
            Axis=Y,
        },
        white_carpet => BlockBehavior::default(), {
        },
        orange_carpet => BlockBehavior::default(), {
        },
        magenta_carpet => BlockBehavior::default(), {
        },
        light_blue_carpet => BlockBehavior::default(), {
        },
        yellow_carpet => BlockBehavior::default(), {
        },
        lime_carpet => BlockBehavior::default(), {
        },
        pink_carpet => BlockBehavior::default(), {
        },
        gray_carpet => BlockBehavior::default(), {
        },
        light_gray_carpet => BlockBehavior::default(), {
        },
        cyan_carpet => BlockBehavior::default(), {
        },
        purple_carpet => BlockBehavior::default(), {
        },
        blue_carpet => BlockBehavior::default(), {
        },
        brown_carpet => BlockBehavior::default(), {
        },
        green_carpet => BlockBehavior::default(), {
        },
        red_carpet => BlockBehavior::default(), {
        },
        black_carpet => BlockBehavior::default(), {
        },
        terracotta => BlockBehavior::default(), {
        },
        coal_block => BlockBehavior::default(), {
        },
        packed_ice => BlockBehavior::default(), {
        },
        sunflower => BlockBehavior::default(), {
            Half=Lower,
        },
        lilac => BlockBehavior::default(), {
            Half=Lower,
        },
        rose_bush => BlockBehavior::default(), {
            Half=Lower,
        },
        peony => BlockBehavior::default(), {
            Half=Lower,
        },
        tall_grass => BlockBehavior::default(), {
            Half=Lower,
        },
        large_fern => BlockBehavior::default(), {
            Half=Lower,
        },
        white_banner => BlockBehavior::default(), {
            WhiteBannerRotation=_0,
        },
        orange_banner => BlockBehavior::default(), {
            OrangeBannerRotation=_0,
        },
        magenta_banner => BlockBehavior::default(), {
            MagentaBannerRotation=_0,
        },
        light_blue_banner => BlockBehavior::default(), {
            LightBlueBannerRotation=_0,
        },
        yellow_banner => BlockBehavior::default(), {
            YellowBannerRotation=_0,
        },
        lime_banner => BlockBehavior::default(), {
            LimeBannerRotation=_0,
        },
        pink_banner => BlockBehavior::default(), {
            PinkBannerRotation=_0,
        },
        gray_banner => BlockBehavior::default(), {
            GrayBannerRotation=_0,
        },
        light_gray_banner => BlockBehavior::default(), {
            LightGrayBannerRotation=_0,
        },
        cyan_banner => BlockBehavior::default(), {
            CyanBannerRotation=_0,
        },
        purple_banner => BlockBehavior::default(), {
            PurpleBannerRotation=_0,
        },
        blue_banner => BlockBehavior::default(), {
            BlueBannerRotation=_0,
        },
        brown_banner => BlockBehavior::default(), {
            BrownBannerRotation=_0,
        },
        green_banner => BlockBehavior::default(), {
            GreenBannerRotation=_0,
        },
        red_banner => BlockBehavior::default(), {
            RedBannerRotation=_0,
        },
        black_banner => BlockBehavior::default(), {
            BlackBannerRotation=_0,
        },
        white_wall_banner => BlockBehavior::default(), {
            FacingCardinal=North,
        },
        orange_wall_banner => BlockBehavior::default(), {
            FacingCardinal=North,
        },
        magenta_wall_banner => BlockBehavior::default(), {
            FacingCardinal=North,
        },
        light_blue_wall_banner => BlockBehavior::default(), {
            FacingCardinal=North,
        },
        yellow_wall_banner => BlockBehavior::default(), {
            FacingCardinal=North,
        },
        lime_wall_banner => BlockBehavior::default(), {
            FacingCardinal=North,
        },
        pink_wall_banner => BlockBehavior::default(), {
            FacingCardinal=North,
        },
        gray_wall_banner => BlockBehavior::default(), {
            FacingCardinal=North,
        },
        light_gray_wall_banner => BlockBehavior::default(), {
            FacingCardinal=North,
        },
        cyan_wall_banner => BlockBehavior::default(), {
            FacingCardinal=North,
        },
        purple_wall_banner => BlockBehavior::default(), {
            FacingCardinal=North,
        },
        blue_wall_banner => BlockBehavior::default(), {
            FacingCardinal=North,
        },
        brown_wall_banner => BlockBehavior::default(), {
            FacingCardinal=North,
        },
        green_wall_banner => BlockBehavior::default(), {
            FacingCardinal=North,
        },
        red_wall_banner => BlockBehavior::default(), {
            FacingCardinal=North,
        },
        black_wall_banner => BlockBehavior::default(), {
            FacingCardinal=North,
        },
        red_sandstone => BlockBehavior::default(), {
        },
        chiseled_red_sandstone => BlockBehavior::default(), {
        },
        cut_red_sandstone => BlockBehavior::default(), {
        },
        red_sandstone_stairs => BlockBehavior::default(), {
            FacingCardinal=North,
            TopBottom=Bottom,
            StairShape=Straight,
            Waterlogged=False,
        },
        oak_slab => BlockBehavior::default(), {
            Type=Bottom,
            Waterlogged=False,
        },
        spruce_slab => BlockBehavior::default(), {
            Type=Bottom,
            Waterlogged=False,
        },
        birch_slab => BlockBehavior::default(), {
            Type=Bottom,
            Waterlogged=False,
        },
        jungle_slab => BlockBehavior::default(), {
            Type=Bottom,
            Waterlogged=False,
        },
        acacia_slab => BlockBehavior::default(), {
            Type=Bottom,
            Waterlogged=False,
        },
        dark_oak_slab => BlockBehavior::default(), {
            Type=Bottom,
            Waterlogged=False,
        },
        mangrove_slab => BlockBehavior::default(), {
            Type=Bottom,
            Waterlogged=False,
        },
        stone_slab => BlockBehavior::default(), {
            Type=Bottom,
            Waterlogged=False,
        },
        smooth_stone_slab => BlockBehavior::default(), {
            Type=Bottom,
            Waterlogged=False,
        },
        sandstone_slab => BlockBehavior::default(), {
            Type=Bottom,
            Waterlogged=False,
        },
        cut_sandstone_slab => BlockBehavior::default(), {
            Type=Bottom,
            Waterlogged=False,
        },
        petrified_oak_slab => BlockBehavior::default(), {
            Type=Bottom,
            Waterlogged=False,
        },
        cobblestone_slab => BlockBehavior::default(), {
            Type=Bottom,
            Waterlogged=False,
        },
        brick_slab => BlockBehavior::default(), {
            Type=Bottom,
            Waterlogged=False,
        },
        stone_brick_slab => BlockBehavior::default(), {
            Type=Bottom,
            Waterlogged=False,
        },
        mud_brick_slab => BlockBehavior::default(), {
            Type=Bottom,
            Waterlogged=False,
        },
        nether_brick_slab => BlockBehavior::default(), {
            Type=Bottom,
            Waterlogged=False,
        },
        quartz_slab => BlockBehavior::default(), {
            Type=Bottom,
            Waterlogged=False,
        },
        red_sandstone_slab => BlockBehavior::default(), {
            Type=Bottom,
            Waterlogged=False,
        },
        cut_red_sandstone_slab => BlockBehavior::default(), {
            Type=Bottom,
            Waterlogged=False,
        },
        purpur_slab => BlockBehavior::default(), {
            Type=Bottom,
            Waterlogged=False,
        },
        smooth_stone => BlockBehavior::default(), {
        },
        smooth_sandstone => BlockBehavior::default(), {
        },
        smooth_quartz => BlockBehavior::default(), {
        },
        smooth_red_sandstone => BlockBehavior::default(), {
        },
        spruce_fence_gate => BlockBehavior::default(), {
            FacingCardinal=North,
            InWall=False,
            Open=False,
            Powered=False,
        },
        birch_fence_gate => BlockBehavior::default(), {
            FacingCardinal=North,
            InWall=False,
            Open=False,
            Powered=False,
        },
        jungle_fence_gate => BlockBehavior::default(), {
            FacingCardinal=North,
            InWall=False,
            Open=False,
            Powered=False,
        },
        acacia_fence_gate => BlockBehavior::default(), {
            FacingCardinal=North,
            InWall=False,
            Open=False,
            Powered=False,
        },
        dark_oak_fence_gate => BlockBehavior::default(), {
            FacingCardinal=North,
            InWall=False,
            Open=False,
            Powered=False,
        },
        mangrove_fence_gate => BlockBehavior::default(), {
            FacingCardinal=North,
            InWall=False,
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
        birch_fence => BlockBehavior::default(), {
            East=False,
            North=False,
            South=False,
            Waterlogged=False,
            West=False,
        },
        jungle_fence => BlockBehavior::default(), {
            East=False,
            North=False,
            South=False,
            Waterlogged=False,
            West=False,
        },
        acacia_fence => BlockBehavior::default(), {
            East=False,
            North=False,
            South=False,
            Waterlogged=False,
            West=False,
        },
        dark_oak_fence => BlockBehavior::default(), {
            East=False,
            North=False,
            South=False,
            Waterlogged=False,
            West=False,
        },
        mangrove_fence => BlockBehavior::default(), {
            East=False,
            North=False,
            South=False,
            Waterlogged=False,
            West=False,
        },
        spruce_door => BlockBehavior::default(), {
            FacingCardinal=North,
            Half=Lower,
            Hinge=Left,
            Open=False,
            Powered=False,
        },
        birch_door => BlockBehavior::default(), {
            FacingCardinal=North,
            Half=Lower,
            Hinge=Left,
            Open=False,
            Powered=False,
        },
        jungle_door => BlockBehavior::default(), {
            FacingCardinal=North,
            Half=Lower,
            Hinge=Left,
            Open=False,
            Powered=False,
        },
        acacia_door => BlockBehavior::default(), {
            FacingCardinal=North,
            Half=Lower,
            Hinge=Left,
            Open=False,
            Powered=False,
        },
        dark_oak_door => BlockBehavior::default(), {
            FacingCardinal=North,
            Half=Lower,
            Hinge=Left,
            Open=False,
            Powered=False,
        },
        mangrove_door => BlockBehavior::default(), {
            FacingCardinal=North,
            Half=Lower,
            Hinge=Left,
            Open=False,
            Powered=False,
        },
        end_rod => BlockBehavior::default(), {
            FacingCubic=Up,
        },
        chorus_plant => BlockBehavior::default(), {
            Down=False,
            East=False,
            North=False,
            South=False,
            Up=False,
            West=False,
        },
        chorus_flower => BlockBehavior::default(), {
            ChorusFlowerAge=_0,
        },
        purpur_block => BlockBehavior::default(), {
        },
        purpur_pillar => BlockBehavior::default(), {
            Axis=Y,
        },
        purpur_stairs => BlockBehavior::default(), {
            FacingCardinal=North,
            TopBottom=Bottom,
            StairShape=Straight,
            Waterlogged=False,
        },
        end_stone_bricks => BlockBehavior::default(), {
        },
        beetroots => BlockBehavior::default(), {
            BeetrootsAge=_0,
        },
        dirt_path => BlockBehavior::default(), {
        },
        end_gateway => BlockBehavior::default(), {
        },
        repeating_command_block => BlockBehavior::default(), {
            Conditional=False,
            FacingCubic=North,
        },
        chain_command_block => BlockBehavior::default(), {
            Conditional=False,
            FacingCubic=North,
        },
        frosted_ice => BlockBehavior::default(), {
            FrostedIceAge=_0,
        },
        magma_block => BlockBehavior::default(), {
        },
        nether_wart_block => BlockBehavior::default(), {
        },
        red_nether_bricks => BlockBehavior::default(), {
        },
        bone_block => BlockBehavior::default(), {
            Axis=Y,
        },
        structure_void => BlockBehavior::default(), {
        },
        observer => BlockBehavior::default(), {
            FacingCubic=South,
            Powered=False,
        },
        shulker_box => BlockBehavior::default(), {
            FacingCubic=Up,
        },
        white_shulker_box => BlockBehavior::default(), {
            FacingCubic=Up,
        },
        orange_shulker_box => BlockBehavior::default(), {
            FacingCubic=Up,
        },
        magenta_shulker_box => BlockBehavior::default(), {
            FacingCubic=Up,
        },
        light_blue_shulker_box => BlockBehavior::default(), {
            FacingCubic=Up,
        },
        yellow_shulker_box => BlockBehavior::default(), {
            FacingCubic=Up,
        },
        lime_shulker_box => BlockBehavior::default(), {
            FacingCubic=Up,
        },
        pink_shulker_box => BlockBehavior::default(), {
            FacingCubic=Up,
        },
        gray_shulker_box => BlockBehavior::default(), {
            FacingCubic=Up,
        },
        light_gray_shulker_box => BlockBehavior::default(), {
            FacingCubic=Up,
        },
        cyan_shulker_box => BlockBehavior::default(), {
            FacingCubic=Up,
        },
        purple_shulker_box => BlockBehavior::default(), {
            FacingCubic=Up,
        },
        blue_shulker_box => BlockBehavior::default(), {
            FacingCubic=Up,
        },
        brown_shulker_box => BlockBehavior::default(), {
            FacingCubic=Up,
        },
        green_shulker_box => BlockBehavior::default(), {
            FacingCubic=Up,
        },
        red_shulker_box => BlockBehavior::default(), {
            FacingCubic=Up,
        },
        black_shulker_box => BlockBehavior::default(), {
            FacingCubic=Up,
        },
        white_glazed_terracotta => BlockBehavior::default(), {
            FacingCardinal=North,
        },
        orange_glazed_terracotta => BlockBehavior::default(), {
            FacingCardinal=North,
        },
        magenta_glazed_terracotta => BlockBehavior::default(), {
            FacingCardinal=North,
        },
        light_blue_glazed_terracotta => BlockBehavior::default(), {
            FacingCardinal=North,
        },
        yellow_glazed_terracotta => BlockBehavior::default(), {
            FacingCardinal=North,
        },
        lime_glazed_terracotta => BlockBehavior::default(), {
            FacingCardinal=North,
        },
        pink_glazed_terracotta => BlockBehavior::default(), {
            FacingCardinal=North,
        },
        gray_glazed_terracotta => BlockBehavior::default(), {
            FacingCardinal=North,
        },
        light_gray_glazed_terracotta => BlockBehavior::default(), {
            FacingCardinal=North,
        },
        cyan_glazed_terracotta => BlockBehavior::default(), {
            FacingCardinal=North,
        },
        purple_glazed_terracotta => BlockBehavior::default(), {
            FacingCardinal=North,
        },
        blue_glazed_terracotta => BlockBehavior::default(), {
            FacingCardinal=North,
        },
        brown_glazed_terracotta => BlockBehavior::default(), {
            FacingCardinal=North,
        },
        green_glazed_terracotta => BlockBehavior::default(), {
            FacingCardinal=North,
        },
        red_glazed_terracotta => BlockBehavior::default(), {
            FacingCardinal=North,
        },
        black_glazed_terracotta => BlockBehavior::default(), {
            FacingCardinal=North,
        },
        white_concrete => BlockBehavior::default(), {
        },
        orange_concrete => BlockBehavior::default(), {
        },
        magenta_concrete => BlockBehavior::default(), {
        },
        light_blue_concrete => BlockBehavior::default(), {
        },
        yellow_concrete => BlockBehavior::default(), {
        },
        lime_concrete => BlockBehavior::default(), {
        },
        pink_concrete => BlockBehavior::default(), {
        },
        gray_concrete => BlockBehavior::default(), {
        },
        light_gray_concrete => BlockBehavior::default(), {
        },
        cyan_concrete => BlockBehavior::default(), {
        },
        purple_concrete => BlockBehavior::default(), {
        },
        blue_concrete => BlockBehavior::default(), {
        },
        brown_concrete => BlockBehavior::default(), {
        },
        green_concrete => BlockBehavior::default(), {
        },
        red_concrete => BlockBehavior::default(), {
        },
        black_concrete => BlockBehavior::default(), {
        },
        white_concrete_powder => BlockBehavior::default(), {
        },
        orange_concrete_powder => BlockBehavior::default(), {
        },
        magenta_concrete_powder => BlockBehavior::default(), {
        },
        light_blue_concrete_powder => BlockBehavior::default(), {
        },
        yellow_concrete_powder => BlockBehavior::default(), {
        },
        lime_concrete_powder => BlockBehavior::default(), {
        },
        pink_concrete_powder => BlockBehavior::default(), {
        },
        gray_concrete_powder => BlockBehavior::default(), {
        },
        light_gray_concrete_powder => BlockBehavior::default(), {
        },
        cyan_concrete_powder => BlockBehavior::default(), {
        },
        purple_concrete_powder => BlockBehavior::default(), {
        },
        blue_concrete_powder => BlockBehavior::default(), {
        },
        brown_concrete_powder => BlockBehavior::default(), {
        },
        green_concrete_powder => BlockBehavior::default(), {
        },
        red_concrete_powder => BlockBehavior::default(), {
        },
        black_concrete_powder => BlockBehavior::default(), {
        },
        kelp => BlockBehavior::default(), {
            KelpAge=_0,
        },
        kelp_plant => BlockBehavior::default(), {
        },
        dried_kelp_block => BlockBehavior::default(), {
        },
        turtle_egg => BlockBehavior::default(), {
            TurtleEggEggs=_1,
            TurtleEggHatch=_0,
        },
        dead_tube_coral_block => BlockBehavior::default(), {
        },
        dead_brain_coral_block => BlockBehavior::default(), {
        },
        dead_bubble_coral_block => BlockBehavior::default(), {
        },
        dead_fire_coral_block => BlockBehavior::default(), {
        },
        dead_horn_coral_block => BlockBehavior::default(), {
        },
        tube_coral_block => BlockBehavior::default(), {
        },
        brain_coral_block => BlockBehavior::default(), {
        },
        bubble_coral_block => BlockBehavior::default(), {
        },
        fire_coral_block => BlockBehavior::default(), {
        },
        horn_coral_block => BlockBehavior::default(), {
        },
        dead_tube_coral => BlockBehavior::default(), {
            Waterlogged=True,
        },
        dead_brain_coral => BlockBehavior::default(), {
            Waterlogged=True,
        },
        dead_bubble_coral => BlockBehavior::default(), {
            Waterlogged=True,
        },
        dead_fire_coral => BlockBehavior::default(), {
            Waterlogged=True,
        },
        dead_horn_coral => BlockBehavior::default(), {
            Waterlogged=True,
        },
        tube_coral => BlockBehavior::default(), {
            Waterlogged=True,
        },
        brain_coral => BlockBehavior::default(), {
            Waterlogged=True,
        },
        bubble_coral => BlockBehavior::default(), {
            Waterlogged=True,
        },
        fire_coral => BlockBehavior::default(), {
            Waterlogged=True,
        },
        horn_coral => BlockBehavior::default(), {
            Waterlogged=True,
        },
        dead_tube_coral_fan => BlockBehavior::default(), {
            Waterlogged=True,
        },
        dead_brain_coral_fan => BlockBehavior::default(), {
            Waterlogged=True,
        },
        dead_bubble_coral_fan => BlockBehavior::default(), {
            Waterlogged=True,
        },
        dead_fire_coral_fan => BlockBehavior::default(), {
            Waterlogged=True,
        },
        dead_horn_coral_fan => BlockBehavior::default(), {
            Waterlogged=True,
        },
        tube_coral_fan => BlockBehavior::default(), {
            Waterlogged=True,
        },
        brain_coral_fan => BlockBehavior::default(), {
            Waterlogged=True,
        },
        bubble_coral_fan => BlockBehavior::default(), {
            Waterlogged=True,
        },
        fire_coral_fan => BlockBehavior::default(), {
            Waterlogged=True,
        },
        horn_coral_fan => BlockBehavior::default(), {
            Waterlogged=True,
        },
        dead_tube_coral_wall_fan => BlockBehavior::default(), {
            FacingCardinal=North,
            Waterlogged=True,
        },
        dead_brain_coral_wall_fan => BlockBehavior::default(), {
            FacingCardinal=North,
            Waterlogged=True,
        },
        dead_bubble_coral_wall_fan => BlockBehavior::default(), {
            FacingCardinal=North,
            Waterlogged=True,
        },
        dead_fire_coral_wall_fan => BlockBehavior::default(), {
            FacingCardinal=North,
            Waterlogged=True,
        },
        dead_horn_coral_wall_fan => BlockBehavior::default(), {
            FacingCardinal=North,
            Waterlogged=True,
        },
        tube_coral_wall_fan => BlockBehavior::default(), {
            FacingCardinal=North,
            Waterlogged=True,
        },
        brain_coral_wall_fan => BlockBehavior::default(), {
            FacingCardinal=North,
            Waterlogged=True,
        },
        bubble_coral_wall_fan => BlockBehavior::default(), {
            FacingCardinal=North,
            Waterlogged=True,
        },
        fire_coral_wall_fan => BlockBehavior::default(), {
            FacingCardinal=North,
            Waterlogged=True,
        },
        horn_coral_wall_fan => BlockBehavior::default(), {
            FacingCardinal=North,
            Waterlogged=True,
        },
        sea_pickle => BlockBehavior::default(), {
            SeaPicklePickles=_1,
            Waterlogged=True,
        },
        blue_ice => BlockBehavior::default(), {
        },
        conduit => BlockBehavior::default(), {
            Waterlogged=True,
        },
        bamboo_sapling => BlockBehavior::default(), {
        },
        bamboo => BlockBehavior::default(), {
            BambooAge=_0,
            Leaves=None,
            BambooStage=_0,
        },
        potted_bamboo => BlockBehavior::default(), {
        },
        void_air => BlockBehavior::default(), {
        },
        cave_air => BlockBehavior::default(), {
        },
        bubble_column => BlockBehavior::default(), {
            DragDown=True,
        },
        polished_granite_stairs => BlockBehavior::default(), {
            FacingCardinal=North,
            TopBottom=Bottom,
            StairShape=Straight,
            Waterlogged=False,
        },
        smooth_red_sandstone_stairs => BlockBehavior::default(), {
            FacingCardinal=North,
            TopBottom=Bottom,
            StairShape=Straight,
            Waterlogged=False,
        },
        mossy_stone_brick_stairs => BlockBehavior::default(), {
            FacingCardinal=North,
            TopBottom=Bottom,
            StairShape=Straight,
            Waterlogged=False,
        },
        polished_diorite_stairs => BlockBehavior::default(), {
            FacingCardinal=North,
            TopBottom=Bottom,
            StairShape=Straight,
            Waterlogged=False,
        },
        mossy_cobblestone_stairs => BlockBehavior::default(), {
            FacingCardinal=North,
            TopBottom=Bottom,
            StairShape=Straight,
            Waterlogged=False,
        },
        end_stone_brick_stairs => BlockBehavior::default(), {
            FacingCardinal=North,
            TopBottom=Bottom,
            StairShape=Straight,
            Waterlogged=False,
        },
        stone_stairs => BlockBehavior::default(), {
            FacingCardinal=North,
            TopBottom=Bottom,
            StairShape=Straight,
            Waterlogged=False,
        },
        smooth_sandstone_stairs => BlockBehavior::default(), {
            FacingCardinal=North,
            TopBottom=Bottom,
            StairShape=Straight,
            Waterlogged=False,
        },
        smooth_quartz_stairs => BlockBehavior::default(), {
            FacingCardinal=North,
            TopBottom=Bottom,
            StairShape=Straight,
            Waterlogged=False,
        },
        granite_stairs => BlockBehavior::default(), {
            FacingCardinal=North,
            TopBottom=Bottom,
            StairShape=Straight,
            Waterlogged=False,
        },
        andesite_stairs => BlockBehavior::default(), {
            FacingCardinal=North,
            TopBottom=Bottom,
            StairShape=Straight,
            Waterlogged=False,
        },
        red_nether_brick_stairs => BlockBehavior::default(), {
            FacingCardinal=North,
            TopBottom=Bottom,
            StairShape=Straight,
            Waterlogged=False,
        },
        polished_andesite_stairs => BlockBehavior::default(), {
            FacingCardinal=North,
            TopBottom=Bottom,
            StairShape=Straight,
            Waterlogged=False,
        },
        diorite_stairs => BlockBehavior::default(), {
            FacingCardinal=North,
            TopBottom=Bottom,
            StairShape=Straight,
            Waterlogged=False,
        },
        polished_granite_slab => BlockBehavior::default(), {
            Type=Bottom,
            Waterlogged=False,
        },
        smooth_red_sandstone_slab => BlockBehavior::default(), {
            Type=Bottom,
            Waterlogged=False,
        },
        mossy_stone_brick_slab => BlockBehavior::default(), {
            Type=Bottom,
            Waterlogged=False,
        },
        polished_diorite_slab => BlockBehavior::default(), {
            Type=Bottom,
            Waterlogged=False,
        },
        mossy_cobblestone_slab => BlockBehavior::default(), {
            Type=Bottom,
            Waterlogged=False,
        },
        end_stone_brick_slab => BlockBehavior::default(), {
            Type=Bottom,
            Waterlogged=False,
        },
        smooth_sandstone_slab => BlockBehavior::default(), {
            Type=Bottom,
            Waterlogged=False,
        },
        smooth_quartz_slab => BlockBehavior::default(), {
            Type=Bottom,
            Waterlogged=False,
        },
        granite_slab => BlockBehavior::default(), {
            Type=Bottom,
            Waterlogged=False,
        },
        andesite_slab => BlockBehavior::default(), {
            Type=Bottom,
            Waterlogged=False,
        },
        red_nether_brick_slab => BlockBehavior::default(), {
            Type=Bottom,
            Waterlogged=False,
        },
        polished_andesite_slab => BlockBehavior::default(), {
            Type=Bottom,
            Waterlogged=False,
        },
        diorite_slab => BlockBehavior::default(), {
            Type=Bottom,
            Waterlogged=False,
        },
        brick_wall => BlockBehavior::default(), {
            EastWall=None,
            NorthWall=None,
            SouthWall=None,
            Up=True,
            Waterlogged=False,
            WestWall=None,
        },
        prismarine_wall => BlockBehavior::default(), {
            EastWall=None,
            NorthWall=None,
            SouthWall=None,
            Up=True,
            Waterlogged=False,
            WestWall=None,
        },
        red_sandstone_wall => BlockBehavior::default(), {
            EastWall=None,
            NorthWall=None,
            SouthWall=None,
            Up=True,
            Waterlogged=False,
            WestWall=None,
        },
        mossy_stone_brick_wall => BlockBehavior::default(), {
            EastWall=None,
            NorthWall=None,
            SouthWall=None,
            Up=True,
            Waterlogged=False,
            WestWall=None,
        },
        granite_wall => BlockBehavior::default(), {
            EastWall=None,
            NorthWall=None,
            SouthWall=None,
            Up=True,
            Waterlogged=False,
            WestWall=None,
        },
        stone_brick_wall => BlockBehavior::default(), {
            EastWall=None,
            NorthWall=None,
            SouthWall=None,
            Up=True,
            Waterlogged=False,
            WestWall=None,
        },
        mud_brick_wall => BlockBehavior::default(), {
            EastWall=None,
            NorthWall=None,
            SouthWall=None,
            Up=True,
            Waterlogged=False,
            WestWall=None,
        },
        nether_brick_wall => BlockBehavior::default(), {
            EastWall=None,
            NorthWall=None,
            SouthWall=None,
            Up=True,
            Waterlogged=False,
            WestWall=None,
        },
        andesite_wall => BlockBehavior::default(), {
            EastWall=None,
            NorthWall=None,
            SouthWall=None,
            Up=True,
            Waterlogged=False,
            WestWall=None,
        },
        red_nether_brick_wall => BlockBehavior::default(), {
            EastWall=None,
            NorthWall=None,
            SouthWall=None,
            Up=True,
            Waterlogged=False,
            WestWall=None,
        },
        sandstone_wall => BlockBehavior::default(), {
            EastWall=None,
            NorthWall=None,
            SouthWall=None,
            Up=True,
            Waterlogged=False,
            WestWall=None,
        },
        end_stone_brick_wall => BlockBehavior::default(), {
            EastWall=None,
            NorthWall=None,
            SouthWall=None,
            Up=True,
            Waterlogged=False,
            WestWall=None,
        },
        diorite_wall => BlockBehavior::default(), {
            EastWall=None,
            NorthWall=None,
            SouthWall=None,
            Up=True,
            Waterlogged=False,
            WestWall=None,
        },
        scaffolding => BlockBehavior::default(), {
            Bottom=False,
            ScaffoldingDistance=_7,
            Waterlogged=False,
        },
        loom => BlockBehavior::default(), {
            FacingCardinal=North,
        },
        barrel => BlockBehavior::default(), {
            FacingCubic=North,
            Open=False,
        },
        smoker => BlockBehavior::default(), {
            FacingCardinal=North,
            Lit=False,
        },
        blast_furnace => BlockBehavior::default(), {
            FacingCardinal=North,
            Lit=False,
        },
        cartography_table => BlockBehavior::default(), {
        },
        fletching_table => BlockBehavior::default(), {
        },
        grindstone => BlockBehavior::default(), {
            Face=Wall,
            FacingCardinal=North,
        },
        lectern => BlockBehavior::default(), {
            FacingCardinal=North,
            HasBook=False,
            Powered=False,
        },
        smithing_table => BlockBehavior::default(), {
        },
        stonecutter => BlockBehavior::default(), {
            FacingCardinal=North,
        },
        bell => BlockBehavior::default(), {
            Attachment=Floor,
            FacingCardinal=North,
            Powered=False,
        },
        lantern => BlockBehavior::default(), {
            Hanging=False,
            Waterlogged=False,
        },
        soul_lantern => BlockBehavior::default(), {
            Hanging=False,
            Waterlogged=False,
        },
        campfire => BlockBehavior::default(), {
            FacingCardinal=North,
            Lit=True,
            SignalFire=False,
            Waterlogged=False,
        },
        soul_campfire => BlockBehavior::default(), {
            FacingCardinal=North,
            Lit=True,
            SignalFire=False,
            Waterlogged=False,
        },
        sweet_berry_bush => BlockBehavior::default(), {
            SweetBerryBushAge=_0,
        },
        warped_stem => BlockBehavior::default(), {
            Axis=Y,
        },
        stripped_warped_stem => BlockBehavior::default(), {
            Axis=Y,
        },
        warped_hyphae => BlockBehavior::default(), {
            Axis=Y,
        },
        stripped_warped_hyphae => BlockBehavior::default(), {
            Axis=Y,
        },
        warped_nylium => BlockBehavior::default(), {
        },
        warped_fungus => BlockBehavior::default(), {
        },
        warped_wart_block => BlockBehavior::default(), {
        },
        warped_roots => BlockBehavior::default(), {
        },
        nether_sprouts => BlockBehavior::default(), {
        },
        crimson_stem => BlockBehavior::default(), {
            Axis=Y,
        },
        stripped_crimson_stem => BlockBehavior::default(), {
            Axis=Y,
        },
        crimson_hyphae => BlockBehavior::default(), {
            Axis=Y,
        },
        stripped_crimson_hyphae => BlockBehavior::default(), {
            Axis=Y,
        },
        crimson_nylium => BlockBehavior::default(), {
        },
        crimson_fungus => BlockBehavior::default(), {
        },
        shroomlight => BlockBehavior::default(), {
        },
        weeping_vines => BlockBehavior::default(), {
            WeepingVinesAge=_0,
        },
        weeping_vines_plant => BlockBehavior::default(), {
        },
        twisting_vines => BlockBehavior::default(), {
            TwistingVinesAge=_0,
        },
        twisting_vines_plant => BlockBehavior::default(), {
        },
        crimson_roots => BlockBehavior::default(), {
        },
        crimson_planks => BlockBehavior::default(), {
        },
        warped_planks => BlockBehavior::default(), {
        },
        crimson_slab => BlockBehavior::default(), {
            Type=Bottom,
            Waterlogged=False,
        },
        warped_slab => BlockBehavior::default(), {
            Type=Bottom,
            Waterlogged=False,
        },
        crimson_pressure_plate => BlockBehavior::default(), {
            Powered=False,
        },
        warped_pressure_plate => BlockBehavior::default(), {
            Powered=False,
        },
        crimson_fence => BlockBehavior::default(), {
            East=False,
            North=False,
            South=False,
            Waterlogged=False,
            West=False,
        },
        warped_fence => BlockBehavior::default(), {
            East=False,
            North=False,
            South=False,
            Waterlogged=False,
            West=False,
        },
        crimson_trapdoor => BlockBehavior::default(), {
            FacingCardinal=North,
            TopBottom=Bottom,
            Open=False,
            Powered=False,
            Waterlogged=False,
        },
        warped_trapdoor => BlockBehavior::default(), {
            FacingCardinal=North,
            TopBottom=Bottom,
            Open=False,
            Powered=False,
            Waterlogged=False,
        },
        crimson_fence_gate => BlockBehavior::default(), {
            FacingCardinal=North,
            InWall=False,
            Open=False,
            Powered=False,
        },
        warped_fence_gate => BlockBehavior::default(), {
            FacingCardinal=North,
            InWall=False,
            Open=False,
            Powered=False,
        },
        crimson_stairs => BlockBehavior::default(), {
            FacingCardinal=North,
            TopBottom=Bottom,
            StairShape=Straight,
            Waterlogged=False,
        },
        warped_stairs => BlockBehavior::default(), {
            FacingCardinal=North,
            TopBottom=Bottom,
            StairShape=Straight,
            Waterlogged=False,
        },
        crimson_button => BlockBehavior::default(), {
            Face=Wall,
            FacingCardinal=North,
            Powered=False,
        },
        warped_button => BlockBehavior::default(), {
            Face=Wall,
            FacingCardinal=North,
            Powered=False,
        },
        crimson_door => BlockBehavior::default(), {
            FacingCardinal=North,
            Half=Lower,
            Hinge=Left,
            Open=False,
            Powered=False,
        },
        warped_door => BlockBehavior::default(), {
            FacingCardinal=North,
            Half=Lower,
            Hinge=Left,
            Open=False,
            Powered=False,
        },
        crimson_sign => BlockBehavior::default(), {
            CrimsonSignRotation=_0,
            Waterlogged=False,
        },
        warped_sign => BlockBehavior::default(), {
            WarpedSignRotation=_0,
            Waterlogged=False,
        },
        crimson_wall_sign => BlockBehavior::default(), {
            FacingCardinal=North,
            Waterlogged=False,
        },
        warped_wall_sign => BlockBehavior::default(), {
            FacingCardinal=North,
            Waterlogged=False,
        },
        structure_block => BlockBehavior::default(), {
            Mode=Load,
        },
        jigsaw => BlockBehavior::default(), {
            Orientation=NorthUp,
        },
        composter => BlockBehavior::default(), {
            ComposterLevel=_0,
        },
        target => BlockBehavior::default(), {
            TargetOutputPower=_0,
        },
        bee_nest => BlockBehavior::default(), {
            FacingCardinal=North,
            BeeNestHoneyLevel=_0,
        },
        beehive => BlockBehavior::default(), {
            FacingCardinal=North,
            BeehiveHoneyLevel=_0,
        },
        honey_block => BlockBehavior::default(), {
        },
        honeycomb_block => BlockBehavior::default(), {
        },
        netherite_block => BlockBehavior::default(), {
        },
        ancient_debris => BlockBehavior::default(), {
        },
        crying_obsidian => BlockBehavior::default(), {
        },
        respawn_anchor => BlockBehavior::default(), {
            RespawnAnchorCharge=_0,
        },
        potted_crimson_fungus => BlockBehavior::default(), {
        },
        potted_warped_fungus => BlockBehavior::default(), {
        },
        potted_crimson_roots => BlockBehavior::default(), {
        },
        potted_warped_roots => BlockBehavior::default(), {
        },
        lodestone => BlockBehavior::default(), {
        },
        blackstone => BlockBehavior::default(), {
        },
        blackstone_stairs => BlockBehavior::default(), {
            FacingCardinal=North,
            TopBottom=Bottom,
            StairShape=Straight,
            Waterlogged=False,
        },
        blackstone_wall => BlockBehavior::default(), {
            EastWall=None,
            NorthWall=None,
            SouthWall=None,
            Up=True,
            Waterlogged=False,
            WestWall=None,
        },
        blackstone_slab => BlockBehavior::default(), {
            Type=Bottom,
            Waterlogged=False,
        },
        polished_blackstone => BlockBehavior::default(), {
        },
        polished_blackstone_bricks => BlockBehavior::default(), {
        },
        cracked_polished_blackstone_bricks => BlockBehavior::default(), {
        },
        chiseled_polished_blackstone => BlockBehavior::default(), {
        },
        polished_blackstone_brick_slab => BlockBehavior::default(), {
            Type=Bottom,
            Waterlogged=False,
        },
        polished_blackstone_brick_stairs => BlockBehavior::default(), {
            FacingCardinal=North,
            TopBottom=Bottom,
            StairShape=Straight,
            Waterlogged=False,
        },
        polished_blackstone_brick_wall => BlockBehavior::default(), {
            EastWall=None,
            NorthWall=None,
            SouthWall=None,
            Up=True,
            Waterlogged=False,
            WestWall=None,
        },
        gilded_blackstone => BlockBehavior::default(), {
        },
        polished_blackstone_stairs => BlockBehavior::default(), {
            FacingCardinal=North,
            TopBottom=Bottom,
            StairShape=Straight,
            Waterlogged=False,
        },
        polished_blackstone_slab => BlockBehavior::default(), {
            Type=Bottom,
            Waterlogged=False,
        },
        polished_blackstone_pressure_plate => BlockBehavior::default(), {
            Powered=False,
        },
        polished_blackstone_button => BlockBehavior::default(), {
            Face=Wall,
            FacingCardinal=North,
            Powered=False,
        },
        polished_blackstone_wall => BlockBehavior::default(), {
            EastWall=None,
            NorthWall=None,
            SouthWall=None,
            Up=True,
            Waterlogged=False,
            WestWall=None,
        },
        chiseled_nether_bricks => BlockBehavior::default(), {
        },
        cracked_nether_bricks => BlockBehavior::default(), {
        },
        quartz_bricks => BlockBehavior::default(), {
        },
        candle => BlockBehavior::default(), {
            CandleCandles=_1,
            Lit=False,
            Waterlogged=False,
        },
        white_candle => BlockBehavior::default(), {
            WhiteCandleCandles=_1,
            Lit=False,
            Waterlogged=False,
        },
        orange_candle => BlockBehavior::default(), {
            OrangeCandleCandles=_1,
            Lit=False,
            Waterlogged=False,
        },
        magenta_candle => BlockBehavior::default(), {
            MagentaCandleCandles=_1,
            Lit=False,
            Waterlogged=False,
        },
        light_blue_candle => BlockBehavior::default(), {
            LightBlueCandleCandles=_1,
            Lit=False,
            Waterlogged=False,
        },
        yellow_candle => BlockBehavior::default(), {
            YellowCandleCandles=_1,
            Lit=False,
            Waterlogged=False,
        },
        lime_candle => BlockBehavior::default(), {
            LimeCandleCandles=_1,
            Lit=False,
            Waterlogged=False,
        },
        pink_candle => BlockBehavior::default(), {
            PinkCandleCandles=_1,
            Lit=False,
            Waterlogged=False,
        },
        gray_candle => BlockBehavior::default(), {
            GrayCandleCandles=_1,
            Lit=False,
            Waterlogged=False,
        },
        light_gray_candle => BlockBehavior::default(), {
            LightGrayCandleCandles=_1,
            Lit=False,
            Waterlogged=False,
        },
        cyan_candle => BlockBehavior::default(), {
            CyanCandleCandles=_1,
            Lit=False,
            Waterlogged=False,
        },
        purple_candle => BlockBehavior::default(), {
            PurpleCandleCandles=_1,
            Lit=False,
            Waterlogged=False,
        },
        blue_candle => BlockBehavior::default(), {
            BlueCandleCandles=_1,
            Lit=False,
            Waterlogged=False,
        },
        brown_candle => BlockBehavior::default(), {
            BrownCandleCandles=_1,
            Lit=False,
            Waterlogged=False,
        },
        green_candle => BlockBehavior::default(), {
            GreenCandleCandles=_1,
            Lit=False,
            Waterlogged=False,
        },
        red_candle => BlockBehavior::default(), {
            RedCandleCandles=_1,
            Lit=False,
            Waterlogged=False,
        },
        black_candle => BlockBehavior::default(), {
            BlackCandleCandles=_1,
            Lit=False,
            Waterlogged=False,
        },
        candle_cake => BlockBehavior::default(), {
            Lit=False,
        },
        white_candle_cake => BlockBehavior::default(), {
            Lit=False,
        },
        orange_candle_cake => BlockBehavior::default(), {
            Lit=False,
        },
        magenta_candle_cake => BlockBehavior::default(), {
            Lit=False,
        },
        light_blue_candle_cake => BlockBehavior::default(), {
            Lit=False,
        },
        yellow_candle_cake => BlockBehavior::default(), {
            Lit=False,
        },
        lime_candle_cake => BlockBehavior::default(), {
            Lit=False,
        },
        pink_candle_cake => BlockBehavior::default(), {
            Lit=False,
        },
        gray_candle_cake => BlockBehavior::default(), {
            Lit=False,
        },
        light_gray_candle_cake => BlockBehavior::default(), {
            Lit=False,
        },
        cyan_candle_cake => BlockBehavior::default(), {
            Lit=False,
        },
        purple_candle_cake => BlockBehavior::default(), {
            Lit=False,
        },
        blue_candle_cake => BlockBehavior::default(), {
            Lit=False,
        },
        brown_candle_cake => BlockBehavior::default(), {
            Lit=False,
        },
        green_candle_cake => BlockBehavior::default(), {
            Lit=False,
        },
        red_candle_cake => BlockBehavior::default(), {
            Lit=False,
        },
        black_candle_cake => BlockBehavior::default(), {
            Lit=False,
        },
        amethyst_block => BlockBehavior::default(), {
        },
        budding_amethyst => BlockBehavior::default(), {
        },
        amethyst_cluster => BlockBehavior::default(), {
            FacingCubic=Up,
            Waterlogged=False,
        },
        large_amethyst_bud => BlockBehavior::default(), {
            FacingCubic=Up,
            Waterlogged=False,
        },
        medium_amethyst_bud => BlockBehavior::default(), {
            FacingCubic=Up,
            Waterlogged=False,
        },
        small_amethyst_bud => BlockBehavior::default(), {
            FacingCubic=Up,
            Waterlogged=False,
        },
        tuff => BlockBehavior::default(), {
        },
        calcite => BlockBehavior::default(), {
        },
        tinted_glass => BlockBehavior::default(), {
        },
        powder_snow => BlockBehavior::default(), {
        },
        sculk_sensor => BlockBehavior::default(), {
            SculkSensorPower=_0,
            Phase=Inactive,
            Waterlogged=False,
        },
        sculk => BlockBehavior::default(), {
        },
        sculk_vein => BlockBehavior::default(), {
            TrueFalse=False,
            TrueFalse=False,
            TrueFalse=False,
            TrueFalse=False,
            TrueFalse=False,
            TrueFalse=False,
            TrueFalse=False,
        },
        sculk_catalyst => BlockBehavior::default(), {
            Pulse=False,
        },
        sculk_shrieker => BlockBehavior::default(), {
            CanSummon=False,
            Shrieking=False,
            Waterlogged=False,
        },
        oxidized_copper => BlockBehavior::default(), {
        },
        weathered_copper => BlockBehavior::default(), {
        },
        exposed_copper => BlockBehavior::default(), {
        },
        copper_block => BlockBehavior::default(), {
        },
        copper_ore => BlockBehavior::default(), {
        },
        deepslate_copper_ore => BlockBehavior::default(), {
        },
        oxidized_cut_copper => BlockBehavior::default(), {
        },
        weathered_cut_copper => BlockBehavior::default(), {
        },
        exposed_cut_copper => BlockBehavior::default(), {
        },
        cut_copper => BlockBehavior::default(), {
        },
        oxidized_cut_copper_stairs => BlockBehavior::default(), {
            FacingCardinal=North,
            TopBottom=Bottom,
            StairShape=Straight,
            Waterlogged=False,
        },
        weathered_cut_copper_stairs => BlockBehavior::default(), {
            FacingCardinal=North,
            TopBottom=Bottom,
            StairShape=Straight,
            Waterlogged=False,
        },
        exposed_cut_copper_stairs => BlockBehavior::default(), {
            FacingCardinal=North,
            TopBottom=Bottom,
            StairShape=Straight,
            Waterlogged=False,
        },
        cut_copper_stairs => BlockBehavior::default(), {
            FacingCardinal=North,
            TopBottom=Bottom,
            StairShape=Straight,
            Waterlogged=False,
        },
        oxidized_cut_copper_slab => BlockBehavior::default(), {
            Type=Bottom,
            Waterlogged=False,
        },
        weathered_cut_copper_slab => BlockBehavior::default(), {
            Type=Bottom,
            Waterlogged=False,
        },
        exposed_cut_copper_slab => BlockBehavior::default(), {
            Type=Bottom,
            Waterlogged=False,
        },
        cut_copper_slab => BlockBehavior::default(), {
            Type=Bottom,
            Waterlogged=False,
        },
        waxed_copper_block => BlockBehavior::default(), {
        },
        waxed_weathered_copper => BlockBehavior::default(), {
        },
        waxed_exposed_copper => BlockBehavior::default(), {
        },
        waxed_oxidized_copper => BlockBehavior::default(), {
        },
        waxed_oxidized_cut_copper => BlockBehavior::default(), {
        },
        waxed_weathered_cut_copper => BlockBehavior::default(), {
        },
        waxed_exposed_cut_copper => BlockBehavior::default(), {
        },
        waxed_cut_copper => BlockBehavior::default(), {
        },
        waxed_oxidized_cut_copper_stairs => BlockBehavior::default(), {
            FacingCardinal=North,
            TopBottom=Bottom,
            StairShape=Straight,
            Waterlogged=False,
        },
        waxed_weathered_cut_copper_stairs => BlockBehavior::default(), {
            FacingCardinal=North,
            TopBottom=Bottom,
            StairShape=Straight,
            Waterlogged=False,
        },
        waxed_exposed_cut_copper_stairs => BlockBehavior::default(), {
            FacingCardinal=North,
            TopBottom=Bottom,
            StairShape=Straight,
            Waterlogged=False,
        },
        waxed_cut_copper_stairs => BlockBehavior::default(), {
            FacingCardinal=North,
            TopBottom=Bottom,
            StairShape=Straight,
            Waterlogged=False,
        },
        waxed_oxidized_cut_copper_slab => BlockBehavior::default(), {
            Type=Bottom,
            Waterlogged=False,
        },
        waxed_weathered_cut_copper_slab => BlockBehavior::default(), {
            Type=Bottom,
            Waterlogged=False,
        },
        waxed_exposed_cut_copper_slab => BlockBehavior::default(), {
            Type=Bottom,
            Waterlogged=False,
        },
        waxed_cut_copper_slab => BlockBehavior::default(), {
            Type=Bottom,
            Waterlogged=False,
        },
        lightning_rod => BlockBehavior::default(), {
            FacingCubic=Up,
            Powered=False,
            Waterlogged=False,
        },
        pointed_dripstone => BlockBehavior::default(), {
            Thickness=Tip,
            TipDirection=Up,
            Waterlogged=False,
        },
        dripstone_block => BlockBehavior::default(), {
        },
        cave_vines => BlockBehavior::default(), {
            _0_1_2_3_4_5_6_7_8_9_10_11_12_13_14_15_16_17_18_19_20_21_22_23_24_25=_0,
            TrueFalse=False,
        },
        cave_vines_plant => BlockBehavior::default(), {
            TrueFalse=False,
        },
        spore_blossom => BlockBehavior::default(), {
        },
        azalea => BlockBehavior::default(), {
        },
        flowering_azalea => BlockBehavior::default(), {
        },
        moss_carpet => BlockBehavior::default(), {
        },
        moss_block => BlockBehavior::default(), {
        },
        big_dripleaf => BlockBehavior::default(), {
            FacingCardinal=North,
            Tilt=None,
            Waterlogged=False,
        },
        big_dripleaf_stem => BlockBehavior::default(), {
            FacingCardinal=North,
            Waterlogged=False,
        },
        small_dripleaf => BlockBehavior::default(), {
            FacingCardinal=North,
            Half=Lower,
            Waterlogged=False,
        },
        hanging_roots => BlockBehavior::default(), {
            Waterlogged=False,
        },
        rooted_dirt => BlockBehavior::default(), {
        },
        mud => BlockBehavior::default(), {
        },
        deepslate => BlockBehavior::default(), {
            Axis=Y,
        },
        cobbled_deepslate => BlockBehavior::default(), {
        },
        cobbled_deepslate_stairs => BlockBehavior::default(), {
            FacingCardinal=North,
            TopBottom=Bottom,
            StairShape=Straight,
            Waterlogged=False,
        },
        cobbled_deepslate_slab => BlockBehavior::default(), {
            Type=Bottom,
            Waterlogged=False,
        },
        cobbled_deepslate_wall => BlockBehavior::default(), {
            EastWall=None,
            NorthWall=None,
            SouthWall=None,
            Up=True,
            Waterlogged=False,
            WestWall=None,
        },
        polished_deepslate => BlockBehavior::default(), {
        },
        polished_deepslate_stairs => BlockBehavior::default(), {
            FacingCardinal=North,
            TopBottom=Bottom,
            StairShape=Straight,
            Waterlogged=False,
        },
        polished_deepslate_slab => BlockBehavior::default(), {
            Type=Bottom,
            Waterlogged=False,
        },
        polished_deepslate_wall => BlockBehavior::default(), {
            EastWall=None,
            NorthWall=None,
            SouthWall=None,
            Up=True,
            Waterlogged=False,
            WestWall=None,
        },
        deepslate_tiles => BlockBehavior::default(), {
        },
        deepslate_tile_stairs => BlockBehavior::default(), {
            FacingCardinal=North,
            TopBottom=Bottom,
            StairShape=Straight,
            Waterlogged=False,
        },
        deepslate_tile_slab => BlockBehavior::default(), {
            Type=Bottom,
            Waterlogged=False,
        },
        deepslate_tile_wall => BlockBehavior::default(), {
            EastWall=None,
            NorthWall=None,
            SouthWall=None,
            Up=True,
            Waterlogged=False,
            WestWall=None,
        },
        deepslate_bricks => BlockBehavior::default(), {
        },
        deepslate_brick_stairs => BlockBehavior::default(), {
            FacingCardinal=North,
            TopBottom=Bottom,
            StairShape=Straight,
            Waterlogged=False,
        },
        deepslate_brick_slab => BlockBehavior::default(), {
            Type=Bottom,
            Waterlogged=False,
        },
        deepslate_brick_wall => BlockBehavior::default(), {
            EastWall=None,
            NorthWall=None,
            SouthWall=None,
            Up=True,
            Waterlogged=False,
            WestWall=None,
        },
        chiseled_deepslate => BlockBehavior::default(), {
        },
        cracked_deepslate_bricks => BlockBehavior::default(), {
        },
        cracked_deepslate_tiles => BlockBehavior::default(), {
        },
        infested_deepslate => BlockBehavior::default(), {
            XYZ=Y,
        },
        smooth_basalt => BlockBehavior::default(), {
        },
        raw_iron_block => BlockBehavior::default(), {
        },
        raw_copper_block => BlockBehavior::default(), {
        },
        raw_gold_block => BlockBehavior::default(), {
        },
        potted_azalea_bush => BlockBehavior::default(), {
        },
        potted_flowering_azalea_bush => BlockBehavior::default(), {
        },
        ochre_froglight => BlockBehavior::default(), {
            Axis=Y,
        },
        verdant_froglight => BlockBehavior::default(), {
            Axis=Y,
        },
        pearlescent_froglight => BlockBehavior::default(), {
            Axis=Y,
        },
        frogspawn => BlockBehavior::default(), {
        },
        reinforced_deepslate => BlockBehavior::default(), {
        },
    }
}
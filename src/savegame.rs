use encoding::Encoding;
use nom::bytes::complete::take_while_m_n;
use nom::{number::complete::*, IResult};
use num_traits::{FromPrimitive, ToPrimitive};

#[derive(Debug, Serialize, Deserialize)]
pub struct VCVector {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VCPreamble {
    pub last_mission_passed: String,
    pub w_year: u16,
    pub w_month: u16,
    pub w_day_of_week: u16,
    pub w_day: u16,
    pub w_hour: u16,
    pub w_minute: u16,
    pub w_second: u16,
    pub w_milliseconds: u16,
    pub magic: u32,
    pub current_level: u32,
    pub camera_coordinates: VCVector,
    pub dampf: Option<u32>,
    pub ms_per_game_minute: u32,
    pub game_timer: u32,
    pub game_hour: u8,
    // 3 bytes alignment
    pub game_minute: u8,
    // 3 bytes alignment
    pub pad_number: u32,
    pub game_timer_in_ms: u32,
    pub time_step: f32,
    pub time_step_not_clipped: f32,
    pub fps_since_start: u32,
    pub time_step_2: f32,
    pub fpu: f32,
    pub time_scale: f32,
    pub old_weather_type: u16,
    // 2 byte alignment: u32,
    pub new_weather_type: u16,
    // 2 byte alignment: u32,
    pub forced_weather_type: u16,
    // 2 byte alignment: u32,
    pub weather_interpolation_value: f32,
    pub weather_list: u32,
    pub vehicle_camera_view: f32,
    pub on_foot_camera_view: f32,
    pub current_interior: u32,
    pub taxi_boost_jump: u8,
    // 3 byte alignment
    pub invert_look: u8,
    // 3 byte alignment
    pub extra_color: u32,
    pub extra_color_on: u32,
    pub extra_color_interpolation: f32,
    pub current_radio_station_position: Vec<u32>,
    // scripts
}

#[derive(Debug, Serialize, Deserialize, Primitive)]
pub enum VCVehicle {
    Unknown = 0,
    LandStalker = 130,
    Idaho = 131,
    Stinger = 132,
    LineRunner = 133,
    Perennial = 134,
    Sentinel = 135,
    Rio = 136,
    FireTruck = 137,
    Trash = 138,
    Stretch = 139,
    Manana = 140,
    Infernus = 141,
    Voodoo = 142,
    Pony = 143,
    Mule = 144,
    Cheetah = 145,
    Ambulance = 146,
    FbiWashington = 147,
    MoonBeam = 148,
    Esperanto = 149,
    Taxi = 150,
    Washing = 151,
    BobCat = 152,
    MrWhoopee = 153,
    BfInjection = 154,
    Hunter = 155,
    Police = 156,
    Enforcer = 157,
    Securicar = 158,
    Banshee = 159,
    Predator = 160,
    Bus = 161,
    Rhino = 162,
    BarracksOl = 163,
    CubanHermes = 164,
    Chopper = 165,
    Angel = 166,
    Coach = 167,
    Cabbie = 168,
    Stallion = 169,
    Rumpo = 170,
    RcBandit = 171,
    RomerosHearse = 172,
    Packer = 173,
    SentinelXs = 174,
    Admiral = 175,
    Squalo = 176,
    SeaSparrow = 177,
    PizzaBoy = 178,
    GangBurrito = 179,
    Aeropl = 180,
    Dodo = 181,
    Speeder = 182,
    Reefer = 183,
    Tropic = 184,
    Flatbed = 185,
    Yankee = 186,
    Caddy = 187,
    ZebraCab = 188,
    TopFun = 189,
    Skimmer = 190,
    Pcj600 = 191,
    Faggio = 192,
    Freeway = 193,
    RcBaron = 194,
    RcRaider = 195,
    Glendale = 196,
    Oceanic = 197,
    Sanchez = 198,
    Wparrow = 199,
    Patriot = 200,
    LoveFist = 201,
    CoastGuard = 202,
    Dinghy = 203,
    Hermes = 204,
    Sabre = 205,
    SabreTurbo = 206,
    Phoenix = 207,
    Walton = 208,
    Regina = 209,
    Comet = 210,
    Deluxo = 211,
    Burrito = 212,
    SpandExpress = 213,
    Marquis = 214,
    BaggageHandler = 215,
    KaufmanCab = 216,
    Maverick = 217,
    VcnMaverick = 218,
    Rancher = 219,
    FbiRancher = 220,
    Virgo = 221,
    Greenwood = 222,
    CubanJetmax = 223,
    HotringRacer = 224,
    Sandking = 225,
    BlistaCompact = 226,
    PoliceMaverick = 227,
    Boxville = 228,
    Benson = 229,
    MesaGrande = 230,
    RcgGoblin = 231,
    HotringRacerA = 232,
    HotringRacerB = 233,
    BloodringBangerA = 234,
    BloodringBanderB = 235,
    ViceChee = 236,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VCCarGenerators {
    pub subblock_size: u32,
    pub magic: u32,
    pub rest_size: u32,
    pub subdata_size: u32,
    pub active_car_generators: u32,
    pub process_counter: u8,
    pub generate_even_if_player_is_close_counter: u8,
    pub sub_subblock_size: u32,
    // 2 byte align
    pub generators: Vec<VCCarGenerator>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VCCarGenerator {
    pub vehicle: VCVehicle,
    pub coordinates: VCVector,
    pub heading: f32,
    pub primary_color: u16,
    pub secondary_color: u16,
    pub force_spawn: bool,
    pub alarm: bool,
    pub lock: bool,
    // 1 byte align
    pub min_delay: u16,
    pub max_delay: u16,
    pub game_timer_when_car_is_generated: u32,
    pub vehicle_index: i32,
    pub is_on: bool,
    // word
    pub recently_stolen: bool,
    // one byte align
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VCSaveGame {
    pub preamble: VCPreamble,
    pub car_generators: VCCarGenerators,
}

fn parse_block(input: &[u8]) -> IResult<&[u8], &[u8]> {
    let (input, block_length) = le_u32(input)?;
    nom::bytes::complete::take(block_length)(input)
}

fn parse_checksum(input: &[u8]) -> IResult<&[u8], u32> {
    if input.len() != 4 {
        nom::bytes::complete::take(1 as usize)(&([])[..]).map(|_| (input, 0))
    } else {
        le_u32(input)
    }
}

fn parse_blocks(input: &[u8]) -> IResult<&[u8], (Vec<&[u8]>, u32)> {
    nom::multi::many_till(parse_block, parse_checksum)(input)
}

fn parse_vector(input: &[u8]) -> IResult<&[u8], VCVector> {
    let (input, x) = le_f32(input)?;
    let (input, y) = le_f32(input)?;
    let (input, z) = le_f32(input)?;
    Ok((input, VCVector { x, y, z }))
}

fn parse_preamble(input: &[u8]) -> IResult<&[u8], VCPreamble> {
    let (input, last_mission_passed) = nom::bytes::complete::take(48 as usize)(input).map(|b| {
        (
            b.0,
            encoding::all::UTF_16LE
                .decode(b.1, encoding::DecoderTrap::Ignore)
                .unwrap(),
        )
    })?;
    let (input, w_year) = le_u16(input)?;
    let (input, w_month) = le_u16(input)?;
    let (input, w_day_of_week) = le_u16(input)?;
    let (input, w_day) = le_u16(input)?;
    let (input, w_hour) = le_u16(input)?;
    let (input, w_minute) = le_u16(input)?;
    let (input, w_second) = le_u16(input)?;
    let (input, w_milliseconds) = le_u16(input)?;
    let (input, magic) = le_u32(input)?;
    let (input, current_level) = le_u32(input)?;
    let (input, camera_coordinates) = parse_vector(input)?;
    let (input, dampf) = le_u32(input)?; // TODO: check value
    let (input, ms_per_game_minute) = le_u32(input)?;
    let (input, game_timer) = le_u32(input)?;
    let (input, game_hour) = le_u8(input)?;
    // alignment
    let (input, _) = nom::bytes::complete::take(3 as usize)(input)?;
    let (input, game_minute) = le_u8(input)?;
    // alignment
    let (input, _) = nom::bytes::complete::take(3 as usize)(input)?;
    let (input, pad_number) = le_u32(input)?;
    let (input, game_timer_in_ms) = le_u32(input)?;
    let (input, time_step) = le_f32(input)?;
    let (input, time_step_not_clipped) = le_f32(input)?;
    let (input, fps_since_start) = le_u32(input)?;
    let (input, time_step_2) = le_f32(input)?;
    let (input, fpu) = le_f32(input)?;
    let (input, time_scale) = le_f32(input)?;
    let (input, old_weather_type) = le_u16(input)?;
    // alignment
    let (input, _) = nom::bytes::complete::take(2 as usize)(input)?;
    let (input, new_weather_type) = le_u16(input)?;
    // alignment
    let (input, _) = nom::bytes::complete::take(2 as usize)(input)?;
    let (input, forced_weather_type) = le_u16(input)?;
    // alignment
    let (input, _) = nom::bytes::complete::take(2 as usize)(input)?;
    let (input, weather_interpolation_value) = le_f32(input)?;
    let (input, weather_list) = le_u32(input)?;
    let (input, vehicle_camera_view) = le_f32(input)?;
    let (input, on_foot_camera_view) = le_f32(input)?;
    let (input, current_interior) = le_u32(input)?;
    let (input, taxi_boost_jump) = le_u8(input)?;
    // alignment
    let (input, _) = nom::bytes::complete::take(3 as usize)(input)?;
    let (input, invert_look) = le_u8(input)?;
    // alignment
    let (input, _) = nom::bytes::complete::take(3 as usize)(input)?;
    let (input, extra_color) = le_u32(input)?;
    let (input, extra_color_on) = le_u32(input)?;
    let (input, extra_color_interpolation) = le_f32(input)?;
    let (input, current_radio_station_position) = nom::multi::many_m_n(10, 10, le_u32)(input)?;

    Ok((
        input,
        VCPreamble {
            last_mission_passed,
            w_year,
            w_month,
            w_day_of_week,
            w_day,
            w_hour,
            w_minute,
            w_second,
            w_milliseconds,
            magic,
            current_level,
            camera_coordinates,
            dampf: Some(dampf),
            ms_per_game_minute,
            game_timer,
            game_hour,
            game_minute,
            pad_number,
            game_timer_in_ms,
            time_step,
            time_step_not_clipped,
            fps_since_start,
            time_step_2,
            fpu,
            time_scale,
            old_weather_type,
            new_weather_type,
            forced_weather_type,
            weather_interpolation_value,
            weather_list,
            vehicle_camera_view,
            on_foot_camera_view,
            current_interior,
            taxi_boost_jump,
            invert_look,
            extra_color,
            extra_color_on,
            extra_color_interpolation,
            current_radio_station_position,
        },
    ))
}

fn parse_car_generator(input: &[u8]) -> IResult<&[u8], VCCarGenerator> {
    let (input, vehicle) = le_u32(input).map(|vn| {
        (
            vn.0,
            VCVehicle::from_u32(vn.1).unwrap_or(VCVehicle::Unknown),
        )
    })?;
    let (input, coordinates) = parse_vector(input)?;
    let (input, heading) = le_f32(input)?;
    let (input, primary_color) = le_u16(input)?;
    let (input, secondary_color) = le_u16(input)?;
    let (input, force_spawn) = le_u8(input).map(|i| (i.0, i.1 != 0))?;
    let (input, alarm) = le_u8(input).map(|i| (i.0, i.1 != 0))?;
    let (input, lock) = le_u8(input).map(|i| (i.0, i.1 != 0))?;
    let (input, _align) = le_u8(input)?;
    let (input, min_delay) = le_u16(input)?;
    let (input, max_delay) = le_u16(input)?;
    let (input, game_timer_when_car_is_generated) = le_u32(input)?;
    let (input, vehicle_index) = le_i32(input)?;
    let (input, is_on) = le_i16(input).map(|i| (i.0, i.1 != 0))?;
    let (input, recently_stolen) = le_u8(input).map(|i| (i.0, i.1 != 0))?;
    let (input, _align) = le_u8(input)?;
    Ok((
        input,
        VCCarGenerator {
            vehicle,
            coordinates,
            heading,
            primary_color,
            secondary_color,
            force_spawn,
            alarm,
            lock,
            min_delay,
            max_delay,
            game_timer_when_car_is_generated,
            vehicle_index,
            is_on,
            recently_stolen,
        },
    ))
}

fn parse_gc(input: &[u8]) -> IResult<&[u8], VCCarGenerators> {
    let (input, subblock_size) = le_u32(input)?;
    let (input, magic) = le_u32(input)?;
    let (input, rest_size) = le_u32(input)?;
    let (input, subdata_size) = le_u32(input)?;
    let (input, num_car_generators) = le_u32(input)?;
    let (input, active_car_generators) = le_u32(input)?;
    let (input, process_counter) = le_u8(input)?;
    let (input, generate_even_if_player_is_close_counter) = le_u8(input)?;
    let (input, _) = nom::bytes::complete::take(2 as usize)(input)?;
    let (input, sub_subblock_size) = le_u32(input)?;
    let (_, generators) = nom::multi::many_m_n(
        num_car_generators as usize,
        num_car_generators as usize,
        parse_car_generator,
    )(input)?;
    let vcc = VCCarGenerators {
        subblock_size,
        magic,
        rest_size,
        subdata_size,
        active_car_generators,
        process_counter,
        generate_even_if_player_is_close_counter,
        sub_subblock_size,
        generators,
    };
    Ok((input, vcc))
}

pub fn parse_savegame(buf: &[u8]) -> IResult<&[u8], VCSaveGame> {
    let (input, (blocks, checksum)) = parse_blocks(buf)?;

    let (_, preamble) = parse_preamble(blocks.get(0).unwrap())?;
    let (_, car_generators) = parse_gc(blocks.get(14).unwrap())?;

    let computed_checksum = buf[0..buf.len() - 4]
        .iter()
        .fold(0 as u32, |acc, i| acc + (*i as u32));
    eprintln!(
        "Read checksum: {}; computed checksum: {}",
        checksum, computed_checksum
    );
    eprintln!("Num blocks: {}", blocks.len());

    Ok((
        input,
        VCSaveGame {
            preamble,
            car_generators,
        },
    ))
}

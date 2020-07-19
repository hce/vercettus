#[macro_use]
extern crate stdweb;

use stdweb::web::*;
use stdweb::js_export;

#[js_export]
pub fn tropxe(save_game: &ArrayBuffer) -> String {
    let ta: TypedArray<u8> = TypedArray::from(save_game);
    let tv = ta.to_vec();
    let sg = vercettus::savegame::parse_savegame(&tv);
    match sg {
        Ok((_, vc_save_game)) => {
            serde_yaml::to_string(&vc_save_game).unwrap()
        }
        Err(_e) => {
            let message = "Son, we got a problem here. Unable to parse the save game file.\
            It is probably from\
            an unsupported version. Do contact me at hc-vc@hcesperer.org\
            if you believe this is an error, ye hear, son?";
            js! {
            err_condition( @{message} );
            }
            return "".to_string();
        }
    }
}

#[js_export]
pub fn tichpat(save_game: &ArrayBuffer, patch: String) -> TypedArray<u8> {
    let ta: TypedArray<u8> = TypedArray::from(save_game);
    let tv = ta.to_vec();
    let save_game = match serde_yaml::from_str(&patch) {
        Ok(sg) => sg,
        Err(e) => {
            let message = format!("Son, that yaml file is invalid, can't parse it, no way: {:?}", e);
            js! {
            // hat the weck
            err_condition( @{message} );
            }
            return TypedArray::from(&[][..]);
        }
    };
    let pr = vercettus::savegame::patch_savegame(&tv, &save_game);
    match pr {
        Err(e) => {
            let message = format!("Son, I'm sorry, but there's a problem: {:?}", e);
            js! {
            err_condition( @{message} );
            }
            TypedArray::from(&[][..])
        },
        Ok(res) => {
            TypedArray::from(res.as_slice())
        }
    }
}

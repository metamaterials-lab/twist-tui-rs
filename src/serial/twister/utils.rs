use crate::update::{Action, StateAction};
use crate::serial::DeviceAction;
use crate::serial::setup::Mode;
use super::Twister;

pub fn control( twister : &mut Twister, buf : String ) -> Option<Action> {
    let res = parse_buffer(buf)?;
    let res = if !twister.dir && res.ang.abs() > twister.setup.lim as f32 {
        twister.dir = true;
        Action::Com(DeviceAction::ChangeDir)
    } else {
        let s : f32 = twister.setup.dir.sign();
        match twister.setup.mode {
            Mode::ZTOR if twister.dir && s * res.tor < 0.2 => {
                twister.dir = false;
                ztor_routine(twister)
            },
            Mode::ZANG if twister.dir && s * res.ang < 0.2 => {
                twister.dir = false;
                zang_routine(twister)
            },
            _ => {
                norm_routine(twister, res)
            }
        }
    };
    Some(res)
}

fn norm_routine( twister : &mut Twister, res : R ) -> Action {
    Action::State(
        if twister.record {StateAction::SerialRecord(res.ang, res.tor)}
        else {StateAction::SerialData(res.ang,res.ang_u,res.tor,res.tor_u)}
    )
}

fn zang_routine( twister : &mut Twister ) -> Action {
    twister.cyc += 1;
    if twister.cyc < twister.setup.cyc {
        Action::Com(DeviceAction::ChangeDir)
    } else {
        Action::Com(DeviceAction::Stop)
    }
}

fn ztor_routine( twister : &mut Twister ) -> Action {
    twister.cyc += 1;
    if twister.cyc < twister.setup.cyc {
        Action::Com(DeviceAction::ChangeDir)
    } else {
        Action::Com(DeviceAction::Stop)
    }
}

struct R { ang : f32, tor : f32, ang_u : String, tor_u : String }
fn parse_buffer( buf : String) -> Option<R> {
    let bufs : Vec<&str> = buf.split("\r\n").collect();
    if bufs.len() == 3 {
        let tor : Vec<&str> = bufs[0].split(' ').collect();
        let ang : Vec<&str> = bufs[1].split(' ').collect();
        if tor.len() == 2 && ang.len() == 3 {
            if let Ok( x ) = ang[1].parse::<f32>() {
                if let Ok( y ) = tor[0].parse::<f32>() {
                    return Some(R{
                        ang : x,
                        ang_u : ang[2].to_string(),
                        tor: y,
                        tor_u: tor[1].to_string()
                    });
                }
            }
        }
    }
    None
}

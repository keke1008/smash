use wasm_bindgen::prelude::*;

use super::predictions::{MovementPrediction, RotationPrediction};

#[wasm_bindgen]
#[derive(Debug)]
enum JsMovementPrediction {
    None = "None",
    Stay = "Stay",
    Run = "Run",
    LeftPunch = "Left Punch",
    RightPunch = "Right Punch",
}

impl JsMovementPrediction {
    fn unwrap_js(self) -> Option<MovementPrediction> {
        match self {
            Self::Stay => Some(MovementPrediction::Stay),
            Self::Run => Some(MovementPrediction::Run),
            Self::LeftPunch => Some(MovementPrediction::LeftPunch),
            Self::RightPunch => Some(MovementPrediction::RightPunch),
            _ => None,
        }
    }
}

#[wasm_bindgen]
#[derive(Debug)]
enum JsRotationPrediction {
    None = "None",
    Stay = "Stay",
    RotateLeft = "RotateLeft",
    RotateRight = "RotateRight",
}

impl JsRotationPrediction {
    fn unwrap_js(self) -> Option<RotationPrediction> {
        match self {
            Self::RotateLeft => Some(RotationPrediction::RotateLeft),
            Self::RotateRight => Some(RotationPrediction::RotateRight),
            Self::Stay => Some(RotationPrediction::Stay),
            _ => None,
        }
    }
}

#[wasm_bindgen(module = "/public/js/prediction.js")]
extern "C" {
    fn getMovementPrediction() -> JsMovementPrediction;
    fn getRotationPrediction() -> JsRotationPrediction;
}

pub(super) fn get_movement_prediction() -> Option<MovementPrediction> {
    #[allow(unused_unsafe)]
    unsafe { getMovementPrediction() }.unwrap_js()
}

pub(super) fn get_rotation_prediction() -> Option<RotationPrediction> {
    #[allow(unused_unsafe)]
    unsafe { getRotationPrediction() }.unwrap_js()
}

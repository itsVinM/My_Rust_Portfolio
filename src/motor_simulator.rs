use wasm_bindfen::prelude::*;

// The #[wasm_bindgen] attribute exposes Rust structs and functions to JavaScript.

/// A struct to hold the state of the DC motor simulation.

#[wasm_bindgen]
#[derive(Debug, Clone, Copy)]
pub struct MotorState {
    pub current: f64,
    pub omega: f64, // angular velocity (rad/s)
    pub theta: f64, // angular position (rad)
}

//// Implementation block for Motor State struct
#[wasm_bindgen]
impl MotorState {
    /// constructor to create a new 'Motor State' with initial values.
    #[wasm_bindgen(constructor)]
    pub fn new() -> MotorState{
        MotorState{
            current: 0.0,
            omega: 0.0,
            theta: 0.0,
        }
    }
}

/// Core function that will be called by JavaScript 
/// Updates motor's state based on the inout values and time 
#[wasm_bindgen]
pub fn update_motor_state(state: &mut MotorState, voltage: f64, dt:64){
    // Define the motor constants.
    const R: f64 = 1.5;     //Armature resistance (Ohms)
    const L: f64 = 0.5;     //Armature inductance (H)
    const KE: f64 = 0.05;   //Back-EMF constant (V-s/rad)
    const KT: f64 =0.05;    // Torque cst (N-m/A)
    const J: f64 = 0.01;    //Rotor inertia (kg-m^2)
    const B: f64 = 0.001;   // viscout friction coefficient (N-m-s/rad)

    // Calculate deriavatives of current and angular velocity
    let di_dt = (voltage - R*state.current - KE*state.omega) /L;
    let d_omega_dt = (KT * state.current - B*state.omega)/J;

    //Updating variales
    state.current += di_dt * dt;
    state.omega += d_omega_dt * dt;
    state.theta += state.omega * dt;
}
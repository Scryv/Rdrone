use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashMap;
use std::net::UdpSocket;
use std::str;

#[derive(Serialize, Deserialize, Debug, Clone)]
enum Payload {
    MedicalSupplies,
    HeShell,
    Grenade,
    MilitarySupplies,
    InfraRedCamera,
    SignalRelay,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
enum Weather {
    Sunny,
    Clear,
    Rainy,
    Freezing,
    Stormy,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
enum Role {
    Scout,
    Kamikaze,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct DroneData {
    name: String,
    role: Role,
    x_axis: f64,
    y_axis: f64,
    battery: f64,
    payload: Payload,
    weather: Weather,
    completed: bool,
}

fn main() {
    let socket = UdpSocket::bind("127.0.0.1:3232").unwrap();
    let mut drones: HashMap<String, DroneData> = HashMap::new();
    println!("Waiting for input");

    let mut buf = [0; 2048];
    loop {
        let (bts, src) = socket.recv_from(&mut buf).unwrap();

        let sbuf = &mut buf[..bts];
        let u: DroneData = serde_json::from_slice(sbuf).unwrap();
        drones.insert(u.name.clone(), u.clone());
        println!(
            "Name: {} | Role: {:?} | X:{:.3} Y:{:.3} | Batt: {:.1} |",
            u.name, u.role, u.x_axis, u.y_axis, u.battery,
        );
    }
}

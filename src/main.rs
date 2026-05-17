use rand::RngExt;
use serde::{Deserialize, Serialize};
use std::thread;
use std::time::Duration;

/*
KAMIKAZE DRONE
LiPo powered fpv drone
83mm artilliry shell = 4kg
3-20km
10-20min max

RECON DRONE
Li-ion powered fpv drone
camera/mapping cap
20-60km depending on signal and weather
20-60min max

NEED
battery cost per meter
acc speed
multible drones
udp-based dataTransfer
*/

#[derive(Serialize, Deserialize)]
enum Payload {
    MedicalSupplies,
    HeShell,
    Grenade,
    MilitarySupplies,
    InfraRedCamera,
    SignalRelay,
}

#[derive(Serialize, Deserialize)]
enum Role {
    Scout,
    Kamikaze,
}

#[derive(Serialize, Deserialize)]
struct DroneData {
    name: String,
    role: Role,
    x_axis: f64,
    y_axis: f64,
    battery: f64,
    payload: Payload,
}
impl Role {
    fn rolebatgo(&self) -> f64 {
        match self {
            Role::Scout => 0.2,
            Role::Kamikaze => 0.6,
        }
    }
}

impl Payload {
    fn weight(&self) -> f64 {
        match self {
            Payload::MedicalSupplies => 1.0,
            Payload::HeShell => 4.0,
            Payload::Grenade => 0.4,
            Payload::MilitarySupplies => 2.5,
            Payload::InfraRedCamera => 3.8,
            Payload::SignalRelay => 1.2,
        }
    }
    fn battery_multiplier(&self) -> f64 {
        self.weight() * 0.15
    }
}

fn main() {
    let mut rng = rand::rng();
    println!("Integer: {}", rng.random_range(1..10));
    let target_x: f64 = 3422.523;
    let target_y: f64 = 3566.1;

    let mut a_drone: DroneData = DroneData {
        name: String::from("drone_1"),
        role: Role::kamikaze,
        x_axis: 0.0,
        y_axis: 0.0,
        battery: 100.0,
        payload: Payload::HeShell,
    };
    loop {
        let mut randrange_x = target_x - a_drone.x_axis;
        let mut randrange_y = target_y - a_drone.y_axis;

        let distance: f64 =
            ((a_drone.x_axis - target_x).powf(2.0) + (a_drone.y_axis - target_y).powf(2.0)).sqrt();

        if distance <= 0.12 {
            println!("target been reached");
            break;
        }
        let lading = serde_json::to_string(&a_drone).unwrap();
        println!("{}", distance);
        println!("{}", lading);
        if randrange_x > 300.0 {
            a_drone.x_axis += rng.random_range(0.1..300.0);
        } else if randrange_x > 0.1 {
            a_drone.x_axis += rng.random_range(0.1..randrange_x);
        }
        if randrange_y > 300.0 {
            a_drone.y_axis += rng.random_range(0.1..300.0);
        } else if randrange_y > 0.1 {
            a_drone.y_axis += rng.random_range(0.1..randrange_y);
        }
        a_drone.battery -= a_drone.role.rolebatgo() + a_drone.payload.battery_multiplier();
        if a_drone.battery == 50.0 {
            println!("Drone battery has halfway drained!");
        } else if a_drone.battery == 25.0 {
            println!("25 procent remaining please recharge or hit target!")
        } else if a_drone.battery == 0.0 {
            println!("Drone battery has died");
            break;
        }
        thread::sleep(Duration::from_secs(1));
    }
}

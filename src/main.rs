use rand::RngExt;
use serde::{Deserialize, Serialize};
use std::net::UdpSocket;
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
#[derive(Serialize, Deserialize, Clone)]
enum Weather {
    Sunny,
    Clear,
    Rainy,
    Freezing,
    Stormy,
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
    weather: Weather,
    completed: bool,
    destroyed: bool,
}
impl Role {
    fn rolebatgo(&self) -> f64 {
        match self {
            Role::Scout => 0.2,
            Role::Kamikaze => 0.6,
        }
    }
}

impl Weather {
    fn weather_pen(&self) -> f64 {
        match self {
            Weather::Clear => 0.0,
            Weather::Sunny => 0.05,
            Weather::Rainy => 0.25,
            Weather::Freezing => 0.4,
            Weather::Stormy => 0.75,
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
    let socket = UdpSocket::bind("0.0.0.0:0").expect("couldn't bind to address");
    let weatherNum = rng.random_range(1..=5);
    let mut weather = Weather::Clear;

    match weatherNum {
        1 => weather = Weather::Clear,
        2 => weather = Weather::Sunny,
        3 => weather = Weather::Rainy,
        4 => weather = Weather::Freezing,
        5 => weather = Weather::Stormy,
        _ => println!("Not Valid"),
    }
    let target_x: f64 = 2030.23;
    let target_y: f64 = 1566.1;

    let mut dro: Vec<DroneData> = vec![
        DroneData {
            name: String::from("drone_1"),
            role: Role::Kamikaze,
            x_axis: 0.0,
            y_axis: 0.0,
            battery: 100.0,
            payload: Payload::HeShell,
            weather: weather.clone(),
            completed: false,
            destroyed: false,
        },
        DroneData {
            name: String::from("drone_2"),
            role: Role::Scout,
            x_axis: 0.0,
            y_axis: 0.0,
            battery: 100.0,
            payload: Payload::SignalRelay,
            weather: weather.clone(),
            completed: false,
            destroyed: false,
        },
    ];

    'outer: loop {
        let mut all_drone = true;
        for d in &dro {
            if !d.completed {
                all_drone = false;
                break;
            }
        }
        for drones in &mut dro {
            if drones.destroyed == false {
                let mut dx = target_x - drones.x_axis;
                let mut dy = target_y - drones.y_axis;

                let distance: f64 = ((dx).powf(2.0) + (dy).powf(2.0)).sqrt();

                if distance <= 1.0 {
                    println!("Drone: {} has reached target", drones.name);
                    let rt = "drone {} has reached the target";
                    socket
                        .send_to(&rt.as_bytes(), "127.0.0.1:3232")
                        .expect("couldn't send data");
                    drones.completed = true;
                } else {
                    let speed = 45.0;
                    let ux = dx / distance;
                    let uy = dy / distance;
                    drones.x_axis += ux * speed;
                    drones.y_axis += uy * speed;
                }

                let lading = serde_json::to_string(&drones).unwrap();
                socket
                    .send_to(&lading.as_bytes(), "127.0.0.1:3232")
                    .expect("couldn't send data");
                println!("{}", distance);
                println!("{}", lading);

                drones.battery -= drones.role.rolebatgo()
                    + drones.payload.battery_multiplier()
                    + drones.weather.weather_pen();
                println!("{}", drones.battery);
                if drones.battery < 50.0 && drones.battery > 47.0 {
                    println!("Drone battery has halfway drained!");
                } else if drones.battery < 25.0 && drones.battery > 22.0 {
                    println!("25 procent remaining please recharge or hit target!")
                } else if drones.battery <= 0.0 {
                    println!("Drone battery has died");
                    drones.destroyed = true;
                    drones.completed = true;
                }
            }
        }
        if all_drone == true {
            println!("All Drones have reached Target");
            socket
                .send_to(
                    String::from("All drones have reached target").as_bytes(),
                    "127.0.0.1:3232",
                )
                .expect("couldn't send data");
            break 'outer;
        }

        thread::sleep(Duration::from_secs(1));
    }
}

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

#[derive(Serialize, Deserialize, Clone)]
enum Weather {
    Sunny,
    Clear,
    Rainy,
    Freezing,
    Thunder,
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
    let socket = UdpSocket::bind("0.0.0.0:0").expect("couldn't bind to address");
    let weatherNum = rng.random_range(1..5);
    let mut weather = Weather::Clear;

    match weatherNum {
        1 => weather = Weather::Clear,
        2 => weather = Weather::Sunny,
        3 => weather = Weather::Rainy,
        4 => weather = Weather::Freezing,
        5 => weather = Weather::Thunder,
        _ => println!("Not Valid"),
    }
    let target_x: f64 = 3422.523;
    let target_y: f64 = 3566.1;

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
            let mut randrange_x = target_x - drones.x_axis;
            let mut randrange_y = target_y - drones.y_axis;

            let distance: f64 = ((drones.x_axis - target_x).powf(2.0)
                + (drones.y_axis - target_y).powf(2.0))
            .sqrt();

            if distance <= 0.12 {
                println!("Drone: {} has reached target", drones.name);
                let rt = "drone {} has reached the target";
                socket
                    .send_to(&rt.as_bytes(), "127.0.0.1:3232")
                    .expect("couldn't send data");
                drones.completed = true;
            }
            let lading = serde_json::to_string(&drones).unwrap();
            socket
                .send_to(&lading.as_bytes(), "127.0.0.1:3232")
                .expect("couldn't send data");
            println!("{}", distance);
            println!("{}", lading);
            if randrange_x > 300.0 {
                drones.x_axis += rng.random_range(0.1..300.0);
            } else if randrange_x > 0.1 {
                drones.x_axis += rng.random_range(0.1..randrange_x);
            }
            if randrange_y > 300.0 {
                drones.y_axis += rng.random_range(0.1..300.0);
            } else if randrange_y > 0.1 {
                drones.y_axis += rng.random_range(0.1..randrange_y);
            }
            drones.battery -= drones.role.rolebatgo() + drones.payload.battery_multiplier();
            if drones.battery == 50.0 {
                println!("Drone battery has halfway drained!");
            } else if drones.battery == 25.0 {
                println!("25 procent remaining please recharge or hit target!")
            } else if drones.battery <= 0.0 {
                println!("Drone battery has died");
                break;
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

use rand::RngExt;
use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::thread;
use std::time::Duration;

#[derive(Serialize, Deserialize)]
enum Payload {
    medical_supplies,
    he_shell,
    grenade,
    military_supplies,
}
#[derive(Serialize, Deserialize)]
struct drone_data {
    name: String,
    x_axis: f64,
    y_axis: f64,
    battery: u8,
    payload: Payload,
}

fn main() {
    let mut rng = rand::rng();
    println!("Integer: {}", rng.random_range(1..10));
    let target_x: f64 = 3422.523;
    let target_y: f64 = 3566.1;

    let mut a_drone: drone_data = drone_data {
        name: String::from("drone_1"),
        x_axis: 0.0,
        y_axis: 0.0,
        battery: 100,
        payload: Payload::he_shell,
    };
    loop {
        let mut randrange_x = target_x - a_drone.x_axis;
        let mut randrange_y = target_y - a_drone.y_axis;

        let distance: f64 =
            ((a_drone.x_axis - target_x).powf(2.0) + (a_drone.y_axis - target_y).powf(2.0)).sqrt();

        if distance <= 0.1 {
            println!("target been reached");
            break;
        }
        let lading = serde_json::to_string(&a_drone).unwrap();
        println!("{}", distance);
        println!("{}", lading);
        if randrange_x > 0.1 {
            a_drone.x_axis += rng.random_range(0.1..randrange_x);
        }
        if randrange_y > 0.1 {
            a_drone.y_axis += rng.random_range(0.1..randrange_y);
        }
        a_drone.battery -= 1;
        if a_drone.battery == 50 {
            println!("Drone battery has halfway drained!");
        } else if a_drone.battery == 25 {
            println!("25 procent remaining please recharge or hit target!")
        } else if a_drone.battery == 0 {
            println!("Drone battery has died");
            break;
        }
        thread::sleep(Duration::from_secs(1));
    }
}

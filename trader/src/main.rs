use std::{collections::VecDeque, sync::{Mutex, Arc}};

use color_eyre::Result;
use spacedust::{models::{Register201Response, register_request::Faction, RegisterRequest}, apis::{default_api::register, configuration::Configuration}};
use tokio::runtime::Runtime;

mod app;
mod windows;
mod backend;

#[derive(Debug)]
pub enum Command {
    Register {
        symbol: String,
        faction: Faction
    },
    Quit
}

#[derive(Debug, Default)]
pub struct CommandData {
    register_data: Option<Register201Response>,
}

fn main() -> Result<()> {
    color_eyre::install()?;
    tracing_subscriber::fmt::init();

    let msg_queue: Arc<Mutex<VecDeque<Command>>> = Arc::new(Mutex::new(VecDeque::new()));
    let response_data: Arc<Mutex<CommandData>> = Arc::default();

    let msg_queue_clone = Arc::clone(&msg_queue);
    let response_data_clone = Arc::clone(&response_data);

    let _ = std::thread::spawn(move || {
        let msg_queue = msg_queue_clone;
        let response_data = response_data_clone;
        let config = Configuration::new();
        let rt  = Runtime::new().unwrap();
        loop {
            std::thread::sleep(std::time::Duration::from_millis(100)); // Allow time for gui to lock
            let mut msg_queue_lock = msg_queue.lock().expect("FUGGG noooooo");
            if msg_queue_lock.is_empty() {
                drop(msg_queue_lock);
                continue;
            }
            // Check above garanties element
            let latest_cmd = msg_queue_lock.pop_back().unwrap();
            match latest_cmd {
                Command::Quit => break,
                Command::Register { symbol, faction } => {
                    let mut response_data_lock = response_data.lock().expect("OH SHIT, it's going down");
                    rt.block_on(async {
                        response_data_lock.register_data = register(&config, Some(RegisterRequest::new(faction, symbol))).await.ok();
                    })
                }
            }
        }
    });

    app::gui_main(msg_queue, response_data).unwrap();
    Ok(())
}

/*let waypoints = get_system_waypoints(&config, "X1-DF55", None, None)
    .await
    .unwrap()
    .data;
let waypoints: Vec<&spacedust::models::Waypoint> = waypoints
    .iter()
    .filter(|x| {
        x.traits
            .iter()
            .map(|x| x.symbol == waypoint_trait::Symbol::Marketplace)
            .collect::<Vec<bool>>()
            .contains(&true)
    })
    .collect();

let ship = get_my_ships(&config, Some(1), Some(10))
    .await.unwrap()
    .data
    .first()
    .unwrap()
    .to_owned();

let mut system_market_data = std::collections::HashMap::new();

loop {
    for waypoint in waypoints.iter() {
        let nav_details = navigate_ship(
            &config,
            &ship.symbol,
            Some(spacedust::models::NavigateShipRequest::new(
                waypoint.symbol.clone(),
            )),
        )
        .await;
        dbg!(&nav_details);
        skip!(nav_details);
        let nav_details = nav_details.unwrap();

        println!(
            "Traveling to: {}",
            nav_details.data.nav.route.destination.symbol
        );
        println!(
            "Fuel use: {}",
            nav_details.data.fuel.consumed.unwrap_or_default().amount
        );
        println!(
            "Fuel left: {} / {}",
            nav_details.data.fuel.current, nav_details.data.fuel.capacity
        );
        let now: DateTime<Utc> = Utc::now();
        let arrival = DateTime::parse_from_rfc3339(&nav_details.data.nav.route.arrival).unwrap();
        let wait_time = arrival.signed_duration_since(now);
        println!("Arrival in: {} sec", wait_time.num_seconds());
        println!("");

        std::thread::sleep(Duration::from_millis(wait_time.num_milliseconds() as u64));

        skip!(dock_ship(&config, &ship.symbol, 0.0).await);
        skip!(refuel_ship(&config, &ship.symbol, 0).await);
        let market_data = get_market(&config, &waypoint.system_symbol, &waypoint.symbol)
            .await
            .unwrap_or_default()
            .data
            .trade_goods;
        if market_data.is_none() {
            continue;
        }
        let market_data = market_data.unwrap();

        system_market_data.insert(waypoint.symbol.clone(), market_data);
    }

    let mut best_price_diffrence = 0;
    let mut best_swap_good: String = String::new();
    let mut best_buy_planet: String = String::new();
    let mut best_sell_planet: String = String::new();
    for (planet_x, goods_list_x) in system_market_data.iter() {
        for (planet_y, goods_list_y) in system_market_data.iter() {
            for item in goods_list_x {
                if let Some(v) = goods_list_y.iter().find(|x| x.symbol == item.symbol) {
                    let price_diffrence = v.sell_price - item.purchase_price;
                    if price_diffrence <= 0 {
                        continue;
                    }
                    println!(
                        "{:0>4} | {:>12} | Buy price @ {} = {} | Sell price @ {} = {}",
                        price_diffrence, item.symbol, planet_x, item.purchase_price, planet_y, v.sell_price
                    );
                    if price_diffrence > best_price_diffrence {
                        best_price_diffrence = price_diffrence;
                        best_swap_good = item.symbol.to_owned();
                        best_buy_planet = planet_x.to_owned();
                        best_sell_planet = planet_y.to_owned();
                    }
                }
            }
        }
    }
    println!("");
    println!(
        "Best Deal is Buying {} from {} and selling it to {} for {}$ profit!",
        best_swap_good, best_buy_planet, best_sell_planet, best_price_diffrence
    );
    println!("");

    let deal_reps = 10;
    for _ in 0..deal_reps {
        let nav_data =
            navigate_ship(
                &config,
                &ship.symbol,
                Some(spacedust::models::NavigateShipRequest::new(best_buy_planet.clone()))
            )
            .await;
        skip!(nav_data);
        let nav_data = nav_data.unwrap();
        let wait_time = DateTime::parse_from_rfc3339(&nav_data.data.nav.route.arrival).unwrap().signed_duration_since(Utc::now());
        std::thread::sleep(Duration::from_millis(wait_time.num_milliseconds() as u64));

        skip!(dock_ship(&config, &ship.symbol, 0.0).await);
        skip!(refuel_ship(&config, &ship.symbol, 0).await);

        let units_to_buy = ship.cargo.capacity - ship.cargo.units;
        dbg!(purchase_cargo(&config, &ship.symbol, Some(PurchaseCargoRequest::new(
            best_swap_good.clone(), units_to_buy))).await);

        let nav_data =
            navigate_ship(
                &config,
                &ship.symbol,
                Some(spacedust::models::NavigateShipRequest::new(best_sell_planet.clone()))
            )
            .await;
        skip!(nav_data);
        let nav_data = nav_data.unwrap();
        let wait_time = DateTime::parse_from_rfc3339(&nav_data.data.nav.route.arrival).unwrap().signed_duration_since(Utc::now());
        std::thread::sleep(Duration::from_millis(wait_time.num_milliseconds() as u64));

        skip!(dock_ship(&config, &ship.symbol, 0.0).await);
        skip!(refuel_ship(&config, &ship.symbol, 0).await);

        dbg!(sell_cargo(&config, &ship.symbol, Some(SellCargoRequest::new(best_swap_good.clone(), units_to_buy))).await);
    }
}*/

/*for ship in &get_my_ships(&config, Some(1), Some(10)).await?.data {
    println!("{} @ {} is {:?}", ship.symbol.green(), ship.nav.waypoint_symbol.blue(), ship.nav.status.yellow());
    dbg!(ship);
}*/

/*for (i, contract) in get_contracts(&config, Some(1), Some(10)).await?.data.iter().enumerate() {
    println!("--- Contract {} ---", contract.id);
    dbg!(contract);
    /*if let Some(v) = &contract.terms.deliver {
        println!("{:?}", v);
    }*/
}*/
/*for waypoint in get_system_waypoints(&config, "X1-DF55", None, None).await.unwrap().data {
    dbg!(waypoint);
}*/

//dbg!(navigate_ship(&config, "TESTINGWATERS-1", Some(spacedust::models::NavigateShipRequest::new("X1-DF55-17335A".into()))).await.unwrap());

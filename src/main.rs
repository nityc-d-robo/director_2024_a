use controllers::{p9n_interface, ps4_dualshock4::BUTTONS_DUALSHOCK4};
use safe_drive::topic::subscriber::Subscriber;
use safe_drive::{
    context::Context,
    error::DynError,
    logger::Logger,
    msg::common_interfaces::{geometry_msgs::msg, sensor_msgs},
    topic::publisher,
};

fn main() -> Result<(), DynError> {
    let _logger = Logger::new("director_2024_a");
    let ctx = Context::new()?;
    let mut selector = ctx.create_selector()?;
    let node = ctx.create_node("director_2024_a", None, Default::default())?;

    let s_joy0 = node.create_subscriber::<sensor_msgs::msg::Joy>("joy0", None)?;
    let s_joy1 = node.create_subscriber::<sensor_msgs::msg::Joy>("joy1", None)?;
    let s_joy2 = node.create_subscriber::<sensor_msgs::msg::Joy>("joy2", None)?;

    let p_r_joy1 = node.create_subscriber::<sensor_msgs::msg::Joy>("joy1", None)?;
    let p_r_joy2_1 = node.create_subscriber::<sensor_msgs::msg::Joy>("joy2_1", None)?;
    let p_r_joy2_2_1 = node.create_subscriber::<sensor_msgs::msg::Joy>("joy2_2_1", None)?;
    let p_r_joy2_2_2 = node.create_subscriber::<sensor_msgs::msg::Joy>("joy2_2_1", None)?;
    let p_r_joy2_3 = node.create_subscriber::<sensor_msgs::msg::Joy>("joy2_3", None)?;

    selector.add_subscriber(s_joy0, Box::new(move |msg| {}));

    selector.add_subscriber(s_joy1, Box::new(move |msg| {}));

    selector.add_subscriber(s_joy2, Box::new(move |msg| {}));

    loop {
        selector.wait()?;
    }
}

fn joy0() {}

fn joy1() {}

fn joy2() {}

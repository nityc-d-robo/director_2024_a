use controllers::p9n_interface;

#[allow(unused_imports)]
use safe_drive::{
    context::Context,
    error::DynError,
    logger::Logger,
    msg::common_interfaces::{sensor_msgs, sensor_msgs::msg::Joy},
    pr_info,
    topic::publisher::Publisher,
    topic::subscriber::TakenMsg,
};

use core::cell::RefCell;

fn main() -> Result<(), DynError> {
    let _logger = Logger::new("director_2024_a");
    let ctx = Context::new()?;
    let mut selector = ctx.create_selector()?;
    let node = ctx.create_node("director_2024_a", None, Default::default())?;

    let s_joy0 = node.create_subscriber::<sensor_msgs::msg::Joy>("joy0", None)?;
    let s_joy1 = node.create_subscriber::<sensor_msgs::msg::Joy>("joy1", None)?;
    let s_joy2 = node.create_subscriber::<sensor_msgs::msg::Joy>("joy2", None)?;

    let mut robocons_joy0 = RefCell::new((
        [
            node.create_publisher::<Joy>("rjoy1", None)?,
            node.create_publisher::<Joy>("rjoy2_1", None)?,
        ],
        0,
    ));
    let mut robocons_joy1 = RefCell::new((
        [
            node.create_publisher::<Joy>("rjoy2_2_1", None)?,
            node.create_publisher::<Joy>("rjoy2_2_2", None)?,
        ],
        0,
    ));
    let robocons_joy2 = node.create_publisher::<Joy>("rjoy2_3", None)?;

    selector.add_subscriber(
        s_joy0,
        Box::new(move |msg| {
            joy0_a_1(msg, &mut robocons_joy0);
        }),
    );

    selector.add_subscriber(
        s_joy1,
        Box::new(move |msg| {
            joy0_a_1(msg, &mut robocons_joy1);
        }),
    );

    selector.add_subscriber(s_joy2, Box::new(move |msg| joy2(msg, &robocons_joy2)));

    loop {
        selector.wait()?;
    }
}

fn joy0_a_1(joy_msg: TakenMsg<Joy>, _robocons: &mut RefCell<([Publisher<Joy>; 2], usize)>) {
    let binding = sensor_msgs::msg::Joy::new().unwrap();
    let mut joy_c = p9n_interface::DualShock4Interface::new(&binding);
    joy_c.set_joy_msg(&joy_msg);

    if joy_c.pressed_r2() {
        let robocons = _robocons.get_mut();
        robocons.1 = 0;
    }

    if joy_c.pressed_l2() {
        let robocons = _robocons.get_mut();
        robocons.1 = 1;
    }

    let pointer = _robocons.borrow().1;
    let unpointer = (pointer + 1) % _robocons.borrow().0.len();
    let robocons = _robocons.get_mut();

    robocons.0[pointer].send(&joy_msg).unwrap();
    robocons.0[unpointer].send(&Joy::new().unwrap()).unwrap()
}

fn joy2(joy1_msg: TakenMsg<Joy>, _robocons: &Publisher<Joy>) {
    _robocons.send(&joy1_msg).unwrap()
}

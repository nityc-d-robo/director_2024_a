use controllers::p9n_interface;
use safe_drive::{
    context::Context,
    error::DynError,
    logger::Logger,
    msg::common_interfaces::{sensor_msgs, sensor_msgs::msg::Joy},
    pr_info,
    topic::publisher::Publisher,
    topic::{subscriber::Subscriber, subscriber::TakenMsg},
};
use std::rc::Rc;

use core::cell::RefCell;

struct RoboCon {
    p_r_joy: Publisher<Joy>,
    s_img: Subscriber<Joy>,
    img_mode: bool,
    img_joy: Joy,
}
impl RoboCon {
    fn new(p_r_joy: Publisher<Joy>, s_img: Subscriber<Joy>, img_mode: bool, img_joy: Joy) -> Self {
        Self {
            p_r_joy,
            s_img,
            img_mode,
            img_joy,
        }
    }
}

fn main() -> Result<(), DynError> {
    let _logger = Logger::new("director_2024_a");
    let ctx = Context::new()?;
    let mut selector = ctx.create_selector()?;
    let node = ctx.create_node("director_2024_a", None, Default::default())?;

    let s_joy0 = node.create_subscriber::<sensor_msgs::msg::Joy>("joy0", None)?;
    let s_joy1 = node.create_subscriber::<sensor_msgs::msg::Joy>("joy1", None)?;
    let s_joy2 = node.create_subscriber::<sensor_msgs::msg::Joy>("joy2", None)?;

    let p_r_joy1 = RoboCon::new(
        node.create_publisher::<Joy>("r_joy1", None)?,
        node.create_subscriber::<Joy>("ijoy1", None)?,
        false,
        Joy::new().unwrap(),
    );
    let p_r_joy2_1 = RoboCon::new(
        node.create_publisher::<Joy>("r_joy2_1", None)?,
        node.create_subscriber::<Joy>("ijoy1", None)?,
        false,
        Joy::new().unwrap(),
    );
    let p_r_joy2_2_1 = RoboCon::new(
        node.create_publisher::<Joy>("r_joy2_2_1", None)?,
        node.create_subscriber::<Joy>("ijoy1", None)?,
        false,
        Joy::new().unwrap(),
    );
    let p_r_joy2_2_2 = RoboCon::new(
        node.create_publisher::<Joy>("r_joy2_2_2", None)?,
        node.create_subscriber::<Joy>("ijoy1", None)?,
        false,
        Joy::new().unwrap(),
    );
    let p_r_joy2_3 = RoboCon::new(
        node.create_publisher::<Joy>("r_joy2_3", None)?,
        node.create_subscriber::<Joy>("ijoy1", None)?,
        false,
        Joy::new().unwrap(),
    );

    let mut robocons_joy0 = RefCell::new([p_r_joy1, p_r_joy2_1]);
    let mut robocons_joy1 = RefCell::new([p_r_joy2_2_1, p_r_joy2_2_2]);
    let robocons_joy2 = p_r_joy2_3;

    selector.add_subscriber(
        s_joy0,
        Box::new(move |msg| {
            joy0(msg, &mut robocons_joy0);
        }),
    );

    selector.add_subscriber(
        s_joy1,
        Box::new(move |msg| {
            joy1(msg, &mut robocons_joy1);
        }),
    );

    selector.add_subscriber(s_joy2, Box::new(move |msg| joy2(msg, &robocons_joy2)));

    loop {
        selector.wait()?;
    }
}

fn joy0(joy0_msg: TakenMsg<Joy>, _robocons: &mut RefCell<[RoboCon; 2]>) {
    let binding = sensor_msgs::msg::Joy::new().unwrap();
    let mut joy0_c = p9n_interface::DualShock4Interface::new(&binding);
    joy0_c.set_joy_msg(&joy0_msg);

    let robocons = _robocons.get_mut();
    if joy0_c.pressed_r1() {
        robocons.swap(0, 1);

        let logger = Rc::new(Logger::new("director_2024_a"));
        pr_info!(logger, "joy0 r1",);
    }

    if joy0_c.pressed_l1() {
        robocons[0].img_mode = !robocons[0].img_mode;

        let logger = Rc::new(Logger::new("director_2024_a"));
        pr_info!(logger, "joy0 l1",);
    }

    if robocons[0].img_mode {
        robocons[0].p_r_joy.send(&robocons[0].img_joy).unwrap()
    } else {
        robocons[0].p_r_joy.send(&joy0_msg).unwrap();
    }

    if robocons[1].img_mode {
        robocons[1].p_r_joy.send(&robocons[1].img_joy).unwrap()
    } else {
        robocons[0].p_r_joy.send(&Joy::new().unwrap()).unwrap();
    }
}

fn joy1(joy1_msg: TakenMsg<Joy>, _robocons: &mut RefCell<[RoboCon; 2]>) {
    let binding = sensor_msgs::msg::Joy::new().unwrap();
    let mut joy1_c = p9n_interface::DualShock4Interface::new(&binding);
    joy1_c.set_joy_msg(&joy1_msg);

    let robocons = _robocons.get_mut();
    if joy1_c.pressed_r1() {
        robocons.swap(0, 1);

        let logger = Rc::new(Logger::new("director_2024_a"));
        pr_info!(logger, "joy1 r1",);
    }

    if joy1_c.pressed_l1() {
        robocons[0].img_mode = !robocons[0].img_mode;
        let logger = Rc::new(Logger::new("director_2024_a"));
        pr_info!(logger, "joy1 l1",);
    }

    if robocons[0].img_mode {
        robocons[0].p_r_joy.send(&robocons[0].img_joy).unwrap()
    } else {
        robocons[0].p_r_joy.send(&joy1_msg).unwrap();
    }

    if robocons[1].img_mode {
        robocons[1].p_r_joy.send(&robocons[1].img_joy).unwrap()
    } else {
        robocons[0].p_r_joy.send(&Joy::new().unwrap()).unwrap();
    }
}

fn joy2(joy1_msg: TakenMsg<Joy>, _robocons: &RoboCon) {
    _robocons.p_r_joy.send(&joy1_msg).unwrap()
}

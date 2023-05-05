use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread::sleep,
    time::Duration,
};

use aoko::{
    cli::CliArgs,
    gamepads::{Gamepads, Vec2},
    net::{
        connection::Connection,
        controller::{Controller, ControllerType, Keys},
    },
};
use gilrs::{Axis, Event, EventType, GamepadId, Gilrs};

fn clamp_circle(mut vec: Vec2) -> Vec2 {
    let line_length = (vec.x.powi(2) + vec.y.powi(2)).sqrt();
    if line_length > 1.0 {
        let sin = vec.y / line_length;
        let cos = vec.x / line_length;

        vec.x = cos;
        vec.y = sin;
    }

    vec
}

fn handle_event(
    event: EventType,
    gamepads: &mut Gamepads,
    net_gamepads: &mut [Option<Controller>; 4],
    id: GamepadId,
    left_multiplier: (f32, f32),
    right_multiplier: (f32, f32),
) {
    match event {
        EventType::AxisChanged(axis, change, ..) => {
            let gs = gamepads.index_of(id);
            let m_ref = net_gamepads[gs.index].as_mut().unwrap();

            *match axis {
                Axis::LeftStickX => &mut gs.axis.left.x,

                Axis::LeftStickY => &mut gs.axis.left.y,

                Axis::RightStickX => &mut gs.axis.right.x,

                Axis::RightStickY => &mut gs.axis.right.y,

                _ => return,
            } = change;

            match axis {
                Axis::LeftStickX | Axis::LeftStickY => {
                    m_ref.joy_left = clamp_circle(gs.axis.left)
                        .into_i32_multiplied(left_multiplier.0, left_multiplier.1);
                }

                Axis::RightStickX | Axis::RightStickY => {
                    m_ref.joy_right = clamp_circle(gs.axis.right)
                        .into_i32_multiplied(right_multiplier.0, right_multiplier.1);
                }

                _ => {}
            }
        }

        e @ (EventType::ButtonReleased(button, ..) | EventType::ButtonPressed(button, ..)) => {
            let released = matches!(e, EventType::ButtonReleased(..));
            let net_id = gamepads.index_of(id).index;
            let m_ref = net_gamepads[net_id].as_mut().unwrap();

            let flags = Keys::from(button);

            if released {
                m_ref.keys &= !flags;
            } else {
                m_ref.keys |= flags;
            }
        }

        EventType::Connected => {
            let numeric_id = gamepads.insert(id).index;
            println!(">> Connected {id} ({numeric_id})");
            net_gamepads[numeric_id] = Some(Controller {
                type_: ControllerType::ProController,
                keys: Keys::empty(),
                joy_left: Vec2::default(),
                joy_right: Vec2::default(),
            });
        }

        EventType::Disconnected => {
            let net_id = gamepads.index_of(id).index;
            net_gamepads[net_id] = None;
            gamepads.remove(id);

            println!(">> Disconnected {id} ({net_id})");
        }

        _ => {}
    }
}

fn main() -> eyre::Result<()> {
    let args = CliArgs::parse();
    let mut gilrs = Gilrs::new().unwrap();
    let mut connection = Connection::new(args.ip, args.port)?;
    let mut net_gamepads: [Option<Controller>; 4] = [None, None, None, None];
    let mut gamepads = Gamepads::new();

    let ticks = args.ticks;
    let running = Arc::new(AtomicBool::new(true));

    let ctrlc_flag = Arc::clone(&running);
    ctrlc::set_handler(move || {
        ctrlc_flag.store(false, Ordering::Release);
    })?;

    let common = args.axis_multiplier;

    let mut left_multiplier = (common, common);
    let mut right_multiplier = left_multiplier;

    if args.invert_left_x {
        left_multiplier.0 *= -1.0;
    }
    if args.invert_left_y {
        left_multiplier.1 *= -1.0;
    }

    if args.invert_right_x {
        right_multiplier.0 *= -1.0;
    }
    if args.invert_right_y {
        right_multiplier.1 *= -1.0;
    }

    while running.load(Ordering::Acquire) {
        while let Some(Event { id, event, .. }) = gilrs.next_event() {
            handle_event(
                event,
                &mut gamepads,
                &mut net_gamepads,
                id,
                left_multiplier,
                right_multiplier,
            );
        }

        connection.update(&net_gamepads);

        loop {
            sleep(Duration::from_secs_f64(1.0 / ticks));
            if let Err(e) = connection.send() {
                eprintln!("Error during send: {e}, reconnecting...");
                connection.reconnect()?;
            } else {
                break;
            }
        }
    }

    Ok(())
}

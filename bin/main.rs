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
    gamepads::Gamepads,
    net::{
        connection::Connection,
        controller::{Controller, ControllerType, Keys},
    },
};
use gilrs::{Axis, Event, EventType, GamepadId, Gilrs};

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
            let net_id = gamepads.index_of(id);
            let m_ref = net_gamepads[net_id].as_mut().unwrap();

            match axis {
                Axis::LeftStickX => {
                    m_ref.joy_left.0 = (change * left_multiplier.0) as i32;
                }

                Axis::LeftStickY => {
                    m_ref.joy_left.1 = (change * left_multiplier.1) as i32;
                }

                Axis::RightStickX => {
                    m_ref.joy_right.0 = (change * right_multiplier.0) as i32;
                }

                Axis::RightStickY => {
                    m_ref.joy_right.1 = (change * right_multiplier.1) as i32;
                }

                _ => {}
            }
        }

        e @ (EventType::ButtonReleased(button, ..) | EventType::ButtonPressed(button, ..)) => {
            let released = matches!(e, EventType::ButtonReleased(..));
            let net_id = gamepads.index_of(id);
            let m_ref = net_gamepads[net_id].as_mut().unwrap();

            let flags = Keys::from(button);

            if released {
                m_ref.keys &= !flags;
            } else {
                m_ref.keys |= flags;
            }
        }

        EventType::Connected => {
            let numeric_id = gamepads.insert(id);
            println!(">> Connected {id} ({numeric_id})");
            net_gamepads[numeric_id] = Some(Controller {
                type_: ControllerType::ProController,
                keys: Keys::empty(),
                joy_left: (0, 0),
                joy_right: (0, 0),
            });
        }

        EventType::Disconnected => {
            let net_id = gamepads.index_of(id);
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

    let mut left_multiplier = (args.axis_multiplier, args.axis_multiplier);
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

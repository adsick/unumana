use crate::*;

// this shit has to be minimal in fact, we will fix that in the future
pub fn input_system(
    time: Res<Time>,
    mut input: EventReader<KeyboardInput>,
    mut controller: Query<&mut Controller>,
    mut evwc: EventWriter<Command>,
    mut keymap: ResMut<KeymapList>,
) {
    let mut controller = controller.single_mut();
    let time = time.seconds_since_startup();

    for ki in input.iter() {
        let sc = ki.scan_code;
        let kc = ki.key_code;
        let state = ki.state;

        // in this loop the only thing we want to do is to update the keys in controller
        if state == bevy::input::ButtonState::Pressed {
            let mut space = false;
            if let Some(duration) = controller.get_pressed_duration(sc, time) {
                if sc == 57 {
                    if controller.mode != Mode::Normal && duration > 0.5 {
                        controller.mode = Mode::Normal;
                        println!("57 triggered Normal mode, duration: {:.3}", duration);
                        continue;
                    }
                    space = true;
                }
            } else {
                let last = controller.press(sc, time);
                println!("{} just pressed", sc);
                println!("r({})p({}) gap is {:.3}", last.0 .0, sc, time - last.0 .1);
                println!("p({})p({}) gap is {:.3}\n", last.1 .0, sc, time - last.1 .1);
            }

            if controller.mode == Mode::Normal {
                if sc == 34 {
                    controller.mode = Mode::Insert;
                } else if sc == 30 {
                    evwc.send(Command::MoveCursorRightward);
                    controller.mode = Mode::Insert;
                }

                let extra = space || sc == 57; //controller.is_pressed(32);

                if sc == 36 {
                    if controller.is_pressed(33) {
                        //U serves as mod key here
                        if extra {
                            evwc.send(Command::MoveCursorToTheFirstChar)
                        } else {
                            evwc.send(Command::MoveCursorLeftward);
                        }
                    } else {
                        evwc.send(Command::MoveCursorDownward);
                        //todo extra for moving to the end of file
                    }
                } else if sc == 37 {
                    if controller.is_pressed(33) {
                        if extra {
                            evwc.send(Command::MoveCursorToTheEndOfTheLine)
                        } else {
                            evwc.send(Command::MoveCursorRightward)
                        }
                    } else {
                        evwc.send(Command::MoveCursorUpward)
                        //todo extra for moving to the beginning of the file
                    }
                }
            } else if controller.mode == Mode::Insert {
                if sc == 57 {
                    return;
                }
                if sc == 58 {
                    //Caps
                    keymap.next();
                } else if sc == 14 {
                    //Backspace
                    evwc.send(Command::RemoveCharBeforeCursor)
                } else if sc == 28 {
                    evwc.send(Command::NewLineAfter)
                } else {
                    if let Some(mut ch) = keymap.convert(sc) {
                        controller
                            .is_pressed(42)
                            .then(|| ch = ch.to_uppercase().next().unwrap());

                        if ch.is_whitespace() {
                            println!("'{:?}' is a whitespace", ch);
                            return;
                        }
                        if ch.is_control() {
                            println!("'{:?}' is control", ch);
                            return;
                        }

                        evwc.send(Command::PutCharAfterCursor(ch));
                    }
                }
            }
        } else {
            if let Some((last_pressed, last_released, t)) = controller.release(sc, time) {
                let duration = time - t;
                println!("{} just released, duration: {:.3}", sc, duration);
                println!(
                    "p({})r({}) gap is {:.3}",
                    last_pressed.0,
                    sc,
                    time - last_pressed.1
                );
                println!(
                    "r({})r({}) gap is {:.3}\n",
                    last_released.0,
                    sc,
                    time - last_released.1
                );
                if controller.mode == Mode::Insert && sc == 57 && duration < 0.2 {
                    evwc.send(Command::PutCharAfterCursor(' '));
                }
            }
        }
    }
}

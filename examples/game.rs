const MIN_X: f32 = 200.0;
const MAX_X: f32 = 800.0;
const MIN_Y: f32 = 200.0;
const MAX_Y: f32 = 800.0;
const PADDING: f32 = 2.0;
const CUBE_WIDTH: f32 = 50.0;
const CUBE_HEIGHT: f32 = 50.0;
const SPEED: f32 = 2.0;
const BACKGROUND_COLOR: (f32, f32, f32, f32) = (0.0, 0.0, 0.2, 1.0);
const STAGE_BACKGROUND: (f32, f32, f32, f32) = (0.0, 0.0, 0.5, 1.0);

struct Player {
    x_position: f32,
    y_position: f32,
    children: Vec<Rectangle>
}

struct Rectangle {
    x_offset: f32,
    y_offset: f32,
    width: f32,
    height: f32,
    color: (f32, f32, f32, f32)
}

fn main() {
    // let mut x_position = 200.0;
    // let mut y_position = 30.0;
    let mut player = Player {
        x_position: 200.0,
        y_position: 30.0,
        children: vec![Rectangle {
            x_offset: CUBE_WIDTH / 2.0,
            y_offset: CUBE_HEIGHT / 2.0,
            width: 10.0,
            height: CUBE_HEIGHT * 2.0,
            color: (0.0, 0.5, 0.0, 1.0)
        }]
    };

    let mut x_direction = 0.0;
    let mut y_direction = 0.0;

    let mut speed_x = 0.0;
    let mut speed_y = 0.0;

    let mut left_pressed = false;
    let mut right_pressed = false;
    let mut up_pressed = false;
    let mut down_pressed = false;

    rustyengine::set_event_handler(move |context, event| match event {
        rustyengine::Event::Draw => {
            // let x_out_of_bounds = x_position + (x_direction * speed_x) > 500.0;
            
            player.x_position += x_direction * SPEED;
            player.y_position += y_direction * SPEED;

            if player.x_position < MIN_X {
                player.x_position = MIN_X;
            }

            if player.x_position + CUBE_WIDTH > MAX_X - PADDING {
                player.x_position = MAX_X - CUBE_WIDTH - PADDING;
            }

            if player.y_position < MIN_Y {
                player.y_position = MIN_Y;
            } 

            if player.y_position  + CUBE_HEIGHT> MAX_Y - PADDING{
                player.y_position = MAX_Y - CUBE_HEIGHT - PADDING;
            }
            let width = MAX_X - MIN_X;
            let height = MAX_Y - MIN_Y;

            // main app background
            context.clear_screen_to_color( BACKGROUND_COLOR.0, BACKGROUND_COLOR.1, BACKGROUND_COLOR.2, BACKGROUND_COLOR.3);
            
            // border for stage
            context.draw_rectangle(
                MIN_X - PADDING,
                MIN_Y - PADDING,
                width + PADDING,
                height + PADDING,
                1.0,
                1.0,
                1.0,
                1.0,
                None
            );

            // Playing area or "stage"
            context.draw_rectangle(
                MIN_X, 
                MIN_Y , 
                width - PADDING, 
                height - PADDING, 
                STAGE_BACKGROUND.0, 
                STAGE_BACKGROUND.1, 
                STAGE_BACKGROUND.2, 
                STAGE_BACKGROUND.3,
                None
            );

            // Moveable square
            context.draw_rectangle(
                player.x_position,
                player.y_position,
                CUBE_WIDTH,
                CUBE_HEIGHT,
                1.0,
                0.0,
                0.0,
                1.0,
                None
            );

            for rectangle in &player.children {
                context.draw_rectangle(
                    player.x_position + rectangle.x_offset - rectangle.width / 2.0, 
                    player.y_position + rectangle.y_offset, 
                    rectangle.width, 
                    rectangle.height, 
                    rectangle.color.0,
                    rectangle.color.1, 
                    rectangle.color.2, 
                    rectangle.color.3,
                    Some(1.0)
                );
            }
            
        },
        rustyengine::Event::KeyDown(key) => {
            match key {
                rustyengine::Key::Left => {
                    if right_pressed {
                        x_direction = 0.0;
                        // speed_x = 0.0;
                    } else {
                        x_direction = -1.0;
                        // speed_x = 1.0;
                    }
                    left_pressed = true;
                }
                rustyengine::Key::Right => {
                    if left_pressed {
                        x_direction = 0.0;
                        // speed_x = 0.0;
                    } else {
                        x_direction = 1.0;
                        // speed_x = 1.0;
                    }
                    right_pressed = true;
                }
                rustyengine::Key::Up => {
                    if down_pressed {
                        y_direction = 0.0;
                        // speed_y = 0.0;
                    } else {
                        y_direction = 1.0;
                        // speed_y = 1.0;
                    }
                    up_pressed = true;
                }
                rustyengine::Key::Down => {
                    if up_pressed {
                        y_direction = 0.0;
                        // speed_y = 0.0;
                    } else {
                        y_direction = -1.0;
                        // speed_y = 1.0;
                    }
                    down_pressed = true;
                }
                _ => {}
            }
        },
        rustyengine::Event::KeyUp(key) => {
            match key {
                rustyengine::Key::Left => {
                    if right_pressed {
                        x_direction = 1.0;
                        // speed_x = 1.0
                    } else {
                        x_direction = 0.0;
                        // speed_x = 0.0
                    }
                    left_pressed = false;
                }
                rustyengine::Key::Right => {
                    if left_pressed {
                        x_direction = -1.0;
                        // speed_x = 1.0
                    } else {
                        x_direction = 0.0;
                        // speed_x = 0.0
                    }
                    right_pressed = false;
                }
                rustyengine::Key::Up => {
                    if down_pressed {
                        y_direction = -1.0;
                        // speed_y = 1.0;
                    } else {
                        y_direction = 0.0;
                        // speed_y = 0.0;
                    }
                    up_pressed = false;
                }
                rustyengine::Key::Down => {
                    if up_pressed {
                        y_direction = 1.0;
                        // speed_y = 1.0;
                    } else {
                        y_direction = 0.0;
                        // speed_y = 0.0;
                    }
                    down_pressed = false;
                }
                _ => {}
            }
        }
        _ => {}
    })

    
}
use mazewar::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "MAZE".to_owned(),
        window_height: WINDOW_HEIGHT,
        window_width: WINDOW_WIDTH,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() -> std::io::Result<()> {
    let input_ip = read_input("Enter IP address: ".to_string(), InputType::Ip);
    let server_addr = to_ip(input_ip);
    let user_name = read_input("Enter Name:  ".to_string(), InputType::Name);
    
    let client = Client::new(server_addr);
    let sender_client = Arc::new(client);
    let receiver_client = sender_client.clone();
    let heartbeat_client = sender_client.clone();
    let (tx, rx) = channel::<Data>();

    thread::spawn(move || {
        receiver_client.send_data("connect", json!(user_name));
        loop {
            let received_data = receiver_client.read_message();
            tx.send(received_data).unwrap()
        }
    });
    
    thread::spawn( move || {
        loop {
            heartbeat_client.send_heartbeat();
        }
    });
    
    let mut game_state:GameState = GameState::Game;
    let mut data = Data::default();
    let mut is_shot = false;
    let mut shooting_timer = 0.0;
    let mut kill_timer = 0.0;
    let mut new_level_timer = 0.0;

    loop {
        if let Ok(received_data) = rx.try_recv() {
            data = received_data;
            if data.game_state == GameState::NewLevel {
                game_state = GameState::NewLevel
            }
        }

        clear_background(Color::new(0.0, 0.0, 0.0, 0.8));
        match game_state {
            GameState::Killed =>  {
                if kill_timer > 1.0 {
                    game_state = GameState::Game;
                    sender_client.send_action("revive");
                    kill_timer = 0.0;
                } else {
                    draw_text("YOU ARE KILLED AND STARTING OVER IN NEW POSITION:", 100.0, 200.0, 50.0, WHITE);
                    kill_timer += get_frame_time();
                }
            },
            GameState::NewLevel => {
                if new_level_timer > 2.0 {
                    game_state = GameState::Game;
                    new_level_timer = 0.0;
                } else {
                    draw_text("NEW LEVEL IS READY", 100.0, 200.0, 50.0, WHITE);
                    new_level_timer += get_frame_time();
                }
            },
            GameState::Game => {
                let game_window: GameWindow = data.map.draw(&data.players);
                let mut me = Player::new(Point::zero());
                let mut enemy_positions: Vec<Point> = vec![];

                //FIRST FOUND ME IN THE LIST to settle the position and draw "me" and the "camera"
                for (src, player) in &data.players {
                    if src.to_string() == sender_client.get_address().to_string() {
                        me = player.clone();
                        me.draw(&game_window, &data.map, is_shot);
                    }
                }

                // THEN DRAW ONLY THE ENEMIES
                for (src, player) in &data.players {
                    if src.to_string() != sender_client.get_address().to_string() {
                        let visible: bool = player.alive && data.map.check_visibility(&me, &player); 
                        me.draw_enemy(player.clone(), &game_window, visible);
                        enemy_positions.push(player.location);
                    }
                }
                
                if !me.alive {
                    game_state = GameState::Killed;
                }

                shooting_timer -= 1.0;
                if shooting_timer <= 0.0 {
                    is_shot = false;
                } 

                if is_key_pressed(KeyCode::Space) {
                    is_shot = true;
                    shooting_timer = 20.0;
                    for (_, player) in &data.players {
                        if data.map.check_visibility(&me, player) {
                            sender_client.send_data("shoot", json!(me));
                        }
                    }
                }

                listen_move_events(&sender_client, me, &data.map, &enemy_positions);
                if is_key_pressed(KeyCode::Escape) {
                    sender_client.send_action("QUIT");
                    exit(1)
                }

                draw_text(&format!("FPS: {}", get_fps()), screen_width() - 200.0, 30.0, 25.0, BLACK);
            }
        }
        next_frame().await;
    }
}

pub fn listen_move_events(client: &Client, mut me: Player, map: &Map, enemy_positions: &Vec<Point>) {
    let me_before_move = me.clone();
    if is_key_pressed(KeyCode::A) || is_key_pressed(KeyCode::Left) {
        me.turn_left();
    }
    if is_key_pressed(KeyCode::D) || is_key_pressed(KeyCode::Right) {
        me.turn_right();
    }
    if is_key_pressed(KeyCode::W) || is_key_pressed(KeyCode::Up) {
        let step =  me.step_difference();
        me.make_move(step, map, enemy_positions);
    }
    if is_key_pressed(KeyCode::S) || is_key_pressed(KeyCode::Down) {
        let step = reverse_difference(me.step_difference());
        me.make_move(step, map, enemy_positions);
    }
    if me_before_move != me {
        client.send_data("movement", json!(me))
    }
}

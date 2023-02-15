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
    let mut game_state:GameState = GameState::Game;
    //option for prod
    //add user input for server ip and user name

    let input_ip = read_input("Enter IP address: ".to_string(), InputType::Ip);
    // println!("A {}", input_ip.to_string());
    let server_addr = to_ip(input_ip);
    let user_name = read_input("Enter Name:  ".to_string(), InputType::Name);
    
    
    //option for tests
    //to test this has to be changed to local ip address
    // let server_addr: SocketAddr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(192,168, 1, 174)), 4242);

    // let my_local_ip = local_ip().unwrap();
    // let server_addr: SocketAddr = SocketAddr::new(my_local_ip, 4242);
    // let user_name = String::from("SILVER");

    let client = Client::new(server_addr);
    let sender_clone = Arc::new(client);
    let receiver_clone = sender_clone.clone();
    let (tx, rx) = channel::<Data>();

    thread::spawn(move || {
        receiver_clone.send_message("connect", json!(user_name));
        loop {
            let received_data = receiver_clone.read_message();
            tx.send(received_data).unwrap()
        }
    });
    
    let zero_point = Point::zero();
    let mut data = Data::default();
    let mut is_shot = false;
    let mut shooting_timer = 0;
    let mut kill_timer = 0.0;
    let mut new_level_timer = 0.0;
    // Current display updates based on events, should be from back
    loop {
        if let Ok(received_data) = rx.try_recv() {
            data = received_data;
            if data.game_state == GameState::NewLevel{
                game_state = GameState::NewLevel
            }
        }
        clear_background(Color::new(0.0, 0.0, 0.0, 0.8));
        match game_state {
            GameState::Killed =>  {
                if kill_timer > 1.0 {
                    game_state = GameState::Game;
                    sender_clone.send_message("revive", json!(""));
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
                let mut me = Player::new(zero_point);
                let mut enemy_positions: Vec<Point> = vec![];

                //FIRST FOUND ME IN THE LIST to settle the position and draw "me" and the "camera"
                for (src, player) in &data.players {
                    if src.to_string() == sender_clone.get_address().to_string() {
                        me = player.clone();
                        me.draw(&game_window, &data.map, is_shot);
                    }
                }

                if !me.alive{
                    game_state = GameState::Killed;
                }
                // THEN DRAW ONLY THE ENEMIES
                for (src, player) in &data.players {
                    if src.to_string() != sender_clone.get_address().to_string() {
                        let visible: bool = player.alive && data.map.check_visibility(&me, &player); 
                        me.draw_enemy(player.clone(), &game_window, visible);
                        enemy_positions.push(player.location);
                    }
                }

                //  IT IS UGLY FIX :D 
                if shooting_timer <= 0 {
                    is_shot = false;
                } else{
                    // me.shoot(&data.map.0);
                    shooting_timer -= 1;
                }
                if is_key_pressed(KeyCode::Space) {
                    is_shot = true;
                    shooting_timer = 20;
                    for (_, player) in &data.players {
                        if data.map.check_visibility(&me, player) {
                            sender_clone.send_message("shoot", json!(me));
                        }
                    }
                }
                listen_move_events(&sender_clone, me, &data.map, &enemy_positions);
                if is_key_pressed(KeyCode::Escape) {
                    sender_clone.send_message("I QUIT", json!(""));
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
        client.send_message("movement", json!(me))
    }
}

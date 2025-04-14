// main.rs - Основен файл за опростена имитация на Wolfenstein 3D
use std::f32::consts::PI;
use std::time::Instant;
use macroquad::prelude::*;

mod world;
use world::{World, Player};

// Основни настройки на прозореца и играта
const WINDOW_WIDTH: i32 = 800;
const WINDOW_HEIGHT: i32 = 600;
// const FOV: f32 = PI / 3.0; // Зрително поле (60 градуса)

fn window_conf() -> Conf {
    Conf {
        window_title: "Волфенщайн 3D Клонинг на Ръст".to_owned(),
        window_width: WINDOW_WIDTH,
        window_height: WINDOW_HEIGHT,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    // Създаваме нов свят и играч
    let world = World::new();
    let mut player = Player::new(2.5, 2.5, 0.0);
    
    let mut last_update = Instant::now();

    // Основен игрови цикъл
    loop {
        // Изчисляваме делта време (времето между кадрите)
        let now = Instant::now();
        let dt = now.duration_since(last_update).as_secs_f32();
        last_update = now;

        // Обработка на входа от потребителя
        handle_input(&mut player, &world, dt);
        
        // Изчистваме екрана и започваме ново рисуване
        clear_background(BLACK);
        
        // Рендиране на 3D изглед
        render_3d_view(&player, &world);
        
        // Рендиране на мини-карта
        render_minimap(&player, &world);
        
        // Край на кадъра - обновяваме екрана
        next_frame().await;
    }
}

// Функция за обработка на потребителския вход
fn handle_input(player: &mut Player, world: &World, dt: f32) {
    let move_speed = 2.0 * dt; // Скорост на движение (метри в секунда)
    let rot_speed = 2.0 * dt;  // Скорост на въртене (радиани в секунда)
    
    // Въртене наляво/надясно
    if is_key_down(KeyCode::Left) {
        player.angle -= rot_speed;
    }
    if is_key_down(KeyCode::Right) {
        player.angle += rot_speed;
    }
    
    // Нормализиране на ъгъла между 0 и 2*PI
    player.angle = player.angle % (2.0 * PI);
    if player.angle < 0.0 {
        player.angle += 2.0 * PI;
    }
    
    // Движение напред/назад
    let mut new_x = player.x;
    let mut new_y = player.y;
    
    if is_key_down(KeyCode::Up) || is_key_down(KeyCode::W) {
        new_x += player.angle.cos() * move_speed;
        new_y += player.angle.sin() * move_speed;
    }
    if is_key_down(KeyCode::Down) || is_key_down(KeyCode::S) {
        new_x -= player.angle.cos() * move_speed;
        new_y -= player.angle.sin() * move_speed;
    }
    
    // Странично движение (стрейфинг)
    if is_key_down(KeyCode::A) {
        new_x += (player.angle - PI/2.0).cos() * move_speed;
        new_y += (player.angle - PI/2.0).sin() * move_speed;
    }
    if is_key_down(KeyCode::D) {
        new_x += (player.angle + PI/2.0).cos() * move_speed;
        new_y += (player.angle + PI/2.0).sin() * move_speed;
    }
    
    // Проверка за колизии със стени
    if !world.is_wall(new_x, player.y) {
        player.x = new_x;
    }
    if !world.is_wall(player.x, new_y) {
        player.y = new_y;
    }
}

// Функция за рендиране на 3D изглед от първо лице
fn render_3d_view(player: &Player, world: &World) {
    let width = screen_width();
    let height = screen_height();
    
    // Рисуваме небе и под (таван и под)
    let ceiling_color = DARKBLUE;
    let floor_color = DARKGRAY;
    
    // Рисуване на таван
    draw_rectangle(0.0, 0.0, width, height / 2.0, ceiling_color);
    // Рисуване на под
    draw_rectangle(0.0, height / 2.0, width, height / 2.0, floor_color);
    
    // Рисуваме стени чрез raycast
    for x in 0..WINDOW_WIDTH {
        // Изчисляваме посоката на лъча
        let camera_x = 2.0 * x as f32 / WINDOW_WIDTH as f32 - 1.0;
        let ray_dir_x = player.angle.cos() + player.plane_x() * camera_x;
        let ray_dir_y = player.angle.sin() + player.plane_y() * camera_x;
        
        // Позиция на играча в координатите на картата
        let mut map_x = player.x as i32;
        let mut map_y = player.y as i32;
        
        // Дължина на лъча от текущата позиция до следващата страна x или y
        let mut side_dist_x;
        let mut side_dist_y;
        
        // Дължина на лъча от една x или y страна до следващата x или y страна
        let delta_dist_x = if ray_dir_x == 0.0 { f32::MAX } else { (1.0 / ray_dir_x).abs() };
        let delta_dist_y = if ray_dir_y == 0.0 { f32::MAX } else { (1.0 / ray_dir_y).abs() };
        
        // Посока за стъпване в координатите на картата (или -1 или 1)
        let step_x = if ray_dir_x < 0.0 { -1 } else { 1 };
        let step_y = if ray_dir_y < 0.0 { -1 } else { 1 };
        
        if ray_dir_x < 0.0 {
            side_dist_x = (player.x - map_x as f32) * delta_dist_x;
        } else {
            side_dist_x = (map_x as f32 + 1.0 - player.x) * delta_dist_x;
        }
        
        if ray_dir_y < 0.0 {
            side_dist_y = (player.y - map_y as f32) * delta_dist_y;
        } else {
            side_dist_y = (map_y as f32 + 1.0 - player.y) * delta_dist_y;
        }
        
        // DDA алгоритъм за намиране на стени
        let mut hit = false;
        let mut side = 0; // 0 за x страна, 1 за y страна
        
        while !hit {
            // Скачаме до следващия квадрат на картата
            if side_dist_x < side_dist_y {
                side_dist_x += delta_dist_x;
                map_x += step_x;
                side = 0;
            } else {
                side_dist_y += delta_dist_y;
                map_y += step_y;
                side = 1;
            }
            
            // Проверяваме дали сме ударили стена
            if world.is_wall(map_x as f32, map_y as f32) {
                hit = true;
            }
        }
        
        // Изчисляваме дистанцията до стената
        let perp_wall_dist = if side == 0 {
            (map_x as f32 - player.x + (1.0 - step_x as f32) / 2.0) / ray_dir_x
        } else {
            (map_y as f32 - player.y + (1.0 - step_y as f32) / 2.0) / ray_dir_y
        };
        
        // Изчисляваме височината на линията за рисуване
        let line_height = if perp_wall_dist > 0.0 { height / perp_wall_dist } else { height };
        
        let start_y = (-line_height / 2.0 + height / 2.0).max(0.0);
        let end_y = (line_height / 2.0 + height / 2.0).min(height);
        
        // Избираме цвета на стената в зависимост от вида и.
        // За по-голям реализъм правим x-стените по-тъмни
        let mut wall_color = RED;
        if side == 1 {
            wall_color = MAROON; // По-тъмен червен за y-стените
        }
        
        // Рисуваме вертикална линия
        draw_line(
            x as f32,
            start_y,
            x as f32,
            end_y,
            1.0,
            wall_color,
        );
        
        // Рисуваме обектите ако са в този лъч
        for obj in &world.objects {
            let obj_rel_x = obj.x - player.x;
            let obj_rel_y = obj.y - player.y;
            
            // Трансформираме позицията на обекта в координатната система на камерата
            let inv_det = 1.0 / (player.plane_x() * player.angle.sin() - player.angle.cos() * player.plane_y());
            let transform_x = inv_det * (player.angle.sin() * obj_rel_x - player.angle.cos() * obj_rel_y);
            let transform_y = inv_det * (-player.plane_y() * obj_rel_x + player.plane_x() * obj_rel_y);
            
            // Проверяваме дали обектът е видим и в правилния лъч
            if transform_y > 0.0 {
                let obj_screen_x = ((width / 2.0) * (1.0 + transform_x / transform_y)) as i32;
                
                // Проверяваме дали е близо до текущия лъч
                if (obj_screen_x - x).abs() < 5 {
                    // Изчисляваме височината на обекта
                    let obj_height = height / transform_y;
                    
                    // Рисуваме обекта като вертикална линия
                    let obj_start_y = (-obj_height / 2.0 + height / 2.0).max(0.0);
                    let obj_end_y = (obj_height / 2.0 + height / 2.0).min(height);
                    
                    draw_rectangle(
                        x as f32 - 2.0, 
                        obj_start_y, 
                        4.0, 
                        obj_end_y - obj_start_y, 
                        obj.color
                    );
                }
            }
        }
    }
}

// Функция за рендиране на мини-карта
fn render_minimap(player: &Player, world: &World) {
    const MINIMAP_SIZE: f32 = 150.0;
    const CELL_SIZE: f32 = 15.0;
    
    // Рисуваме фон на мини-картата
    draw_rectangle(10.0, 10.0, MINIMAP_SIZE, MINIMAP_SIZE, Color::new(0.0, 0.0, 0.0, 0.5));
    
    // Рисуваме стените
    for y in 0..world.height {
        for x in 0..world.width {
            if world.map[y as usize][x as usize] == 1 {
                draw_rectangle(
                    10.0 + x as f32 * CELL_SIZE,
                    10.0 + y as f32 * CELL_SIZE,
                    CELL_SIZE,
                    CELL_SIZE,
                    GRAY,
                );
            }
        }
    }
    
    // Рисуваме обектите
    for obj in &world.objects {
        draw_circle(
            10.0 + obj.x * CELL_SIZE,
            10.0 + obj.y * CELL_SIZE,
            2.0,
            obj.color,
        );
    }
    
    // Рисуваме играча
    draw_circle(
        10.0 + player.x * CELL_SIZE,
        10.0 + player.y * CELL_SIZE,
        3.0,
        YELLOW,
    );
    
    // Рисуваме посоката на гледане
    draw_line(
        10.0 + player.x * CELL_SIZE,
        10.0 + player.y * CELL_SIZE,
        10.0 + (player.x + player.angle.cos() * 0.5) * CELL_SIZE,
        10.0 + (player.y + player.angle.sin() * 0.5) * CELL_SIZE,
        1.0,
        YELLOW,
    );
}

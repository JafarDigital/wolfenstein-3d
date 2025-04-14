// world.rs - Модул за дефиниране на света, играча и обектите
use std::f32::consts::PI;
use macroquad::prelude::*;

// Структура за играча
pub struct Player {
    pub x: f32,     // X позиция на играча в света
    pub y: f32,     // Y позиция на играча в света
    pub angle: f32, // Ъгъл на гледане в радиани (0 сочи на изток)
}

impl Player {
    pub fn new(x: f32, y: f32, angle: f32) -> Self {
        Self { x, y, angle }
    }
    
    // Функции за изчисляване на равнината на камерата
    pub fn plane_x(&self) -> f32 {
        (self.angle + PI/2.0).cos() * 0.66
    }
    
    pub fn plane_y(&self) -> f32 {
        (self.angle + PI/2.0).sin() * 0.66
    }
}

// Структура за обект в света (декорация)
pub struct WorldObject {
    pub x: f32,
    pub y: f32,
    pub color: Color,
}

// Структура за света
pub struct World {
    pub width: i32,
    pub height: i32,
    pub map: Vec<Vec<i32>>,  // 0 = проходим под, 1 = стена
    pub objects: Vec<WorldObject>,
}

impl World {
    pub fn new() -> Self {
        // Създаваме карта за една голяма стая с няколко стени отвътре
        // 1 = стена, 0 = проходимо пространство
        let map = vec![
            vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
            vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 1],
            vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 1],
            vec![1, 0, 0, 1, 0, 0, 1, 0, 0, 1],
            vec![1, 0, 0, 1, 0, 0, 1, 0, 0, 1],
            vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 1],
            vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 1],
            vec![1, 0, 0, 1, 1, 1, 1, 0, 0, 1],
            vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 1],
            vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
        ];
        
        // Добавяме няколко декоративни обекта в стаята
        let objects = vec![
            WorldObject { x: 2.5, y: 6.5, color: GREEN },      // Зелена колона
            WorldObject { x: 7.5, y: 6.5, color: BLUE },       // Синя колона
            WorldObject { x: 5.0, y: 2.5, color: GOLD },       // Златен предмет
            WorldObject { x: 6.5, y: 3.7, color: PURPLE },     // Лилав предмет
        ];
        
        Self {
            width: map[0].len() as i32,
            height: map.len() as i32,
            map,
            objects,
        }
    }
    
    // Проверява дали дадените координати са стена
    pub fn is_wall(&self, x: f32, y: f32) -> bool {
        // Проверка дали координатите са в рамките на картата
        if x < 0.0 || y < 0.0 || x >= self.width as f32 || y >= self.height as f32 {
            return true;
        }
        
        // Проверка дали координатите съответстват на стена
        self.map[y as usize][x as usize] == 1
    }
}

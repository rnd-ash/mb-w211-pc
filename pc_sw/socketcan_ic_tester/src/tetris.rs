use std::{time::Duration, fmt::format, borrow::BorrowMut, process::exit, cmp::min};

use agw_lib::custom_display_format::{CDMIsoTp, YaxisSetting, ToneType, ToneRepeatType};
use termion::{raw::IntoRawMode, input::TermRead, event::Key};

use crate::blocks::Shape;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Rotation {
    North,
    East,
    South,
    West
}

impl Rotation {
    pub fn is_horizontal(&self) -> bool {
        *self == Rotation::East || *self == Rotation::West
    }
}

pub struct Block {
    ty: Shape,
    rotation: Rotation,
    pos: (u32, u32),
    prev_draw_command: String
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct CollisionData {
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool
}

impl Block {
    // game_pos - Position of top left of the block
    pub fn new(ty: Shape, game_pos: (u32, u32)) -> Self {
        Self {
            ty,
            rotation: Rotation::North,
            pos: game_pos,
            prev_draw_command: String::new()
        }
    }

    pub fn gen_draw_command(&mut self) -> String {
        let mut s = String::new();
        

        let grid = self.ty.get_draw_grid(self.rotation);
        let mut draw_commands: Vec<((u32, u32), (u32, u32))> = Vec::new();
        // Optimize square
        if self.ty == Shape::Square {
            draw_commands.push(((0, 0),(1, 1)))
        } else {
            if self.rotation.is_horizontal() {
                // Scan Left -> Right
                'yiter: for y in 0..4u32 {
                    let mut start_pos: Option<u32> = None;
                    for x in 0..4u32 {
                        if grid[y as usize][x as usize] == 1 {
                            if start_pos.is_none() {
                                start_pos = Some(x)
                            }
                        } else {
                            if let Some(start_x) = start_pos.take() {
                                draw_commands.push(((start_x, y), (x-1, y)));
                                continue 'yiter;
                            }
                        }
                    }
                    if let Some(start_x) = start_pos {
                        draw_commands.push(((start_x, y), (3, y)));
                    }
                }
            } else {
                // Scan Top -> Down
                'xiter: for x in 0..4u32 {
                    let mut start_pos: Option<u32> = None;
                    for y in 0..4u32 {
                        if grid[y as usize][x as usize] == 1 {
                            if start_pos.is_none() {
                                start_pos = Some(y)
                            }
                        } else {
                            if let Some(start_y) = start_pos.take() {
                                draw_commands.push(((x, start_y), (x, y-1)));
                                continue 'xiter;
                            }
                        }
                    }
                    if let Some(start_y) = start_pos {
                        draw_commands.push(((x, start_y), (x, 3)));
                    }
                }
            }
        }

        for (top_left, bottom_right) in draw_commands {

            let translated_tl = (top_left.0 + self.pos.0, top_left.1 + self.pos.1);
            let translated_br = (bottom_right.0 + self.pos.0, bottom_right.1 + self.pos.1);

            let convert_tl = convert_game_coords_to_lcd(translated_tl, false);
            let convert_br = convert_game_coords_to_lcd(translated_br, true);

            s.push_str(&format!(
                "~P{:02X?}{:02X?}~Q{:02X?}{:02X?}",
                convert_tl.0,
                convert_tl.1,
                convert_br.0,
                convert_br.1
            ));
        }
        s
    }

    // Use rotation to test where the block is colliding
    pub fn test_collision(&self, g: &GAME_GRID) -> CollisionData {
        let mut ret = CollisionData::default();
        let block_grid = self.ty.get_draw_grid(self.rotation);

        let mut start_pos = self.pos;

        fn find_grid_value(grid: &GAME_GRID, coords: (usize, usize)) -> bool {
            if let Some(row) = grid.get(coords.1) {
                if let Some(x_value) = row.get(coords.0) {
                    *x_value == 1
                } else {
                    true
                }
            } else {
                true
            }
        }

        for x in 0..4 {
            for y in 0..4 {
                let grid_loc_x = start_pos.0 as usize + x;
                let grid_loc_y = start_pos.1 as usize + y;
                if block_grid[y][x] == 1 {
                    // Block is rendered in this location
                    ret.left |= find_grid_value(g, (grid_loc_x.wrapping_sub(1), grid_loc_y));
                    ret.right |= find_grid_value(g, (grid_loc_x.wrapping_add(1), grid_loc_y));

                    ret.up |= find_grid_value(g, (grid_loc_x, grid_loc_y.wrapping_sub(1)));
                    ret.down |= find_grid_value(g, (grid_loc_x, grid_loc_y.wrapping_add(1)));
                }
            }
        }
        ret
    }

}

pub const PIXEL_SCALE: u32 = 5; // MUST BE ODD
pub const LCD_WIDTH: u32 = 120;
pub const LCD_HEIGHT: u32 = 90;

pub const GAME_W: u32 = 10;
pub const GAME_H: u32 = 15;

pub const CLEAR_REGION_TOP_LEFT: (u32, u32) = ((LCD_WIDTH/2)-((PIXEL_SCALE*(GAME_W))/2), (LCD_HEIGHT/2)-((PIXEL_SCALE*(GAME_H))/2));
pub const CLEAR_REGION_BOTTOM_RIGHT: (u32, u32) = ((LCD_WIDTH/2)+((PIXEL_SCALE*(GAME_W))/2)-1, ((LCD_HEIGHT/2)+((PIXEL_SCALE*(GAME_H))/2)));

pub const GAME_REGION_TOP_LEFT: (u32, u32) = (CLEAR_REGION_TOP_LEFT.0+PIXEL_SCALE/2, CLEAR_REGION_TOP_LEFT.1+PIXEL_SCALE/2);
pub const GAME_REGION_BOTTOM_RIGHT: (u32, u32) = (CLEAR_REGION_BOTTOM_RIGHT.0 - PIXEL_SCALE, CLEAR_REGION_BOTTOM_RIGHT.1 - PIXEL_SCALE);

pub const BORDER_TOP_LEFT: (u32, u32) = (CLEAR_REGION_TOP_LEFT.0-2, CLEAR_REGION_TOP_LEFT.1-2);
pub const BORDER_BOTTOM_RIGHT: (u32, u32) = (CLEAR_REGION_BOTTOM_RIGHT.0+2, CLEAR_REGION_BOTTOM_RIGHT.1+2);

pub fn convert_game_coords_to_lcd(input: (u32, u32), bottom_right: bool) -> (u32, u32) {
    let (mut lcd_tlx, mut lcd_tly) = GAME_REGION_TOP_LEFT;

    // Input is the middle of the cell
    //
    // TR: (bottom_right is false)
    //
    // *----
    // | x |
    // -----

    // BL: (bottom_right is true)
    //
    // -----
    // | x |
    // ----*

    let mut ret_x = lcd_tlx + (input.0*PIXEL_SCALE);
    let mut ret_y = lcd_tly + (input.1*PIXEL_SCALE);

    if bottom_right {
        ret_x += PIXEL_SCALE/2;
        ret_y += PIXEL_SCALE/2;
    } else {
        ret_x -= PIXEL_SCALE/2;
        ret_y -= PIXEL_SCALE/2;
    }

    (ret_x, ret_y)
}

pub type GAME_GRID = [[u8; GAME_W as usize]; GAME_H as usize];

pub fn write_block_to_grid(g: &mut GAME_GRID) {
    for r in g.iter_mut() {
        for c in r.iter_mut() {
            if *c == 2 {
                *c = 1;
            }
        }
    }
}

fn get_clearing_rows(g: &GAME_GRID) -> Vec<usize> {
    let mut ret = vec![];
    for (idx, row) in g.iter().enumerate() {
        if row == &[1; GAME_W as usize] {
            ret.push(idx);
        }
    }
    ret
}

pub struct Tetris {
    handler: CDMIsoTp,
    status: GAME_GRID
}

impl Tetris {
    pub fn new(isotp: CDMIsoTp) -> Self {
        Self {
            handler: isotp,
            status: [[0; GAME_W as usize]; GAME_H as usize]
        }
    }

    pub fn setup_game_ui(&self) -> String {
        // Simplet form. Draw rectangle, then hollow the center

        format!("~P1C64~G0W211 TETRIS~P{:02X?}{:02X?}~Q{:02X?}{:02X?}", 
            BORDER_TOP_LEFT.0, 
            BORDER_TOP_LEFT.1, 
            BORDER_BOTTOM_RIGHT.0, 
            BORDER_BOTTOM_RIGHT.1, 
            //NBLOCK_BORDER_TOP_LEFT.0, 
            //NBLOCK_BORDER_TOP_LEFT.1, 
            //NBLOCK_BORDER_BOTTOM_RIGHT.0, 
            //NBLOCK_BORDER_BOTTOM_RIGHT.1, 
        )
    }

    pub fn draw_game_area(&self) -> String {
        let s  = format!("~F44~P{:02X?}{:02X?}~Q{:02X?}{:02X?}~F00", 
            CLEAR_REGION_TOP_LEFT.0, 
            CLEAR_REGION_TOP_LEFT.1, 
            CLEAR_REGION_BOTTOM_RIGHT.0, 
            CLEAR_REGION_BOTTOM_RIGHT.1, 
            //NBLOCK_REGION_TOP_LEFT.0, 
            //NBLOCK_REGION_TOP_LEFT.1, 
            //NBLOCK_REGION_BOTTOM_RIGHT.0, 
            //NBLOCK_REGION_BOTTOM_RIGHT.1, 
        );
        s
    }

    pub fn draw_clear_step(&self, rows: &[usize], from: usize, to: usize) -> String {
        let mut ret = String::new();
        for row in rows {
            let tl = convert_game_coords_to_lcd((from as u32, *row as u32), false);
            let br = convert_game_coords_to_lcd((to as u32, *row as u32), true);
            ret.push_str(&format!("~P{:02X?}{:02X?}~D{:02X?}{:02X?}", 
                tl.0,
                tl.1,
                br.0,
                br.1
            ));
        }
        ret
    }

    fn create_shapes(coords: &[(usize, usize)], solve_for_y: bool) -> Vec<[(usize, usize); 2]> {
        let mut ret = Vec::new();
        println!("{coords:?}");
        let mut i = 0;
        loop {
            if let Some(sc) = coords.get(i) {
                // Start of shape
                let start_coords = sc;
                let mut end_coords = sc;
                let mut test_idx = 1;
                
                while let Some(next) = coords.get(i+test_idx) {
                    if solve_for_y && next.0 == sc.0 && next.1 == sc.1 + test_idx {
                        // We can expand the shape to this coord!
                        i += test_idx;
                        end_coords = next;
                        test_idx += 1;
                    } else if !solve_for_y && next.0 == sc.0 + test_idx && next.1 == sc.1 {
                        println!("OK {next:?} {sc:?} {test_idx}");
                        // We can expand the shape to this coord!
                        i += test_idx;
                        end_coords = next;
                        test_idx += 1;
                    } else {
                        println!("FA {next:?} {sc:?} {test_idx}");
                        break;
                    }
                }
                // Generate shape
                ret.push([*start_coords, *end_coords]);
                i += 1;
            } else {
                break;
            }
        }
        println!("{ret:?}");
        ret
    }

    fn gen_render_delta_command(prev_grid: &GAME_GRID, now: &GAME_GRID) -> String {
        let mut s = String::new();
        // For drawing PD command
        let mut different_pixels: Vec<(usize, usize)> = Vec::new();
        // For drawing PQ command
        let mut on_pixels_now: Vec<(usize, usize)> = Vec::new();
        for x in 0..GAME_W as usize {
            for y in 0..GAME_H as usize {
                if (prev_grid[y][x] != 0) != (now[y][x] != 0) {
                    different_pixels.push((x, y));
                    if now[y][x] != 0 {
                        on_pixels_now.push((x, y));
                    }
                }
            }
        }

        let draw_x = Self::create_shapes(&on_pixels_now, false);
        let draw_y = Self::create_shapes(&on_pixels_now, true);

        let (clear_commands, draw_commands) = if draw_y.len() > draw_x.len() {
            (
                Self::create_shapes(&different_pixels, true),
                draw_x
            )
        } else {
            (
                Self::create_shapes(&different_pixels, false),
                draw_y
            )
        };



        for [tl, br] in clear_commands {

            let tl_convert = convert_game_coords_to_lcd((tl.0 as u32, tl.1 as u32), false);
            let br_convert = convert_game_coords_to_lcd((br.0 as u32, br.1 as u32), true);

            s.push_str(&format!(
                "~P{:02X}{:02X}~D{:02X}{:02X}",
                tl_convert.0,
                tl_convert.1,
                br_convert.0,
                br_convert.1
            ));
        }

        for [tl, br] in draw_commands {

            let tl_convert = convert_game_coords_to_lcd((tl.0 as u32, tl.1 as u32), false);
            let br_convert = convert_game_coords_to_lcd((br.0 as u32, br.1 as u32), true);

            s.push_str(&format!(
                "~P{:02X}{:02X}~Q{:02X}{:02X}",
                tl_convert.0,
                tl_convert.1,
                br_convert.0,
                br_convert.1
            ));
        }

        s
    }

    pub fn run(&mut self) {
        self.handler.stop_display();
        std::thread::sleep(Duration::from_millis(500));
        self.handler.show_display(u32::MAX);
        // Setup
        self.handler.update_buffer_live(&format!("~IF~C0{}{}", self.setup_game_ui(), self.draw_game_area()));

        //let mut coords = (0, 0);
        
        let all_blocks = vec![Shape::Square, Shape::Straight, Shape::Z, Shape::S, Shape::T, Shape::L, Shape::LRev];
        let rotations = vec![Rotation::North, Rotation::West, Rotation::South, Rotation::East];
        let mut rotation_id = 0;
        let mut block = Block::new(all_blocks[rand::random::<usize>() % 7], ((GAME_W/2)-2, 0));
        let mut beep = false;
        let mut clearing_animation = false;
        let mut clearing_rows: Vec<usize> = Vec::new();

        //let mut stdout = std::io::stdout().into_raw_mode().unwrap();
        //let mut stdin = termion::async_stdin().keys();

        self.status[6] = [1; GAME_W as usize];
        let mut grid_prev_step = self.status;
        loop {
            if clearing_animation {
                // 4-5
                self.handler.update_buffer_live(&self.draw_clear_step(&clearing_rows, 4, 5));
                // 3-6
                self.handler.update_buffer_live(&self.draw_clear_step(&clearing_rows, 3, 6));
                // 2-7
                self.handler.update_buffer_live(&self.draw_clear_step(&clearing_rows, 2, 7));
                // 1-8
                self.handler.update_buffer_live(&self.draw_clear_step(&clearing_rows, 1, 8));
                // 0-9
                self.handler.update_buffer_live(&self.draw_clear_step(&clearing_rows, 0, 9));
                // Drop
                for row in &clearing_rows {
                    // Drop above rows
                    for i in (1..=*row).rev() {
                        self.status[i] = self.status[i-1];
                    }
                    self.status[0] = [0; GAME_W as usize];
                }
                clearing_animation = false;
            } else {
                let draw_cmd = Self::gen_render_delta_command(&grid_prev_step, &self.status);
                println!("{}\x00", &draw_cmd);
                self.handler.update_buffer_live(&format!("{}\x00", &draw_cmd));
                grid_prev_step = self.status;

                clearing_rows = get_clearing_rows(&self.status);
                if !clearing_rows.is_empty() {
                    clearing_animation = true;
                    continue;
                }

                if beep {
                    self.handler.sound_buzzer(ToneType::ShortBeep, ToneRepeatType::None);
                    beep = false;
                }
                block.pos.1 += 1;
                let collision_now = block.test_collision(&self.status);
                let mut c = None;
                //while let Some(Ok(Key::Char(in_char))) = stdin.next() {
                //    c = Some(in_char);
                //}
                if collision_now.down {
                    if collision_now.up {
                        // GAME OVER
                        self.handler.sound_buzzer(ToneType::Chime, ToneRepeatType::None);
                        loop {
                            self.handler.update_buffer_live("~IF~C2{}~P0010~G2~Z~J2GAME OVER~L~L~G0Score: %04d~P7890");
                            std::thread::sleep(Duration::from_millis(1000));
                            println!("Game end");
                            exit(0);
                        }
                    }
                    write_block_to_grid(&mut self.status);
                    rotation_id = 0;
                    block = Block::new(all_blocks[rand::random::<usize>() % 7], ((GAME_W/2)-2, 0));
                    beep = true;
                }
                if let Some(in_char) = c {
                    match in_char {
                        'a' => {
                            if !collision_now.left {
                                block.pos.0 -= 1;
                            }
                        },
                        'd' => {
                            if !collision_now.right {
                                block.pos.0 += 1;
                            }
                        },
                        'r' => {
                            rotation_id = (rotation_id + 1) % 4;
                            block.rotation = rotations[rotation_id];
                        },
                        _ => {}
                    }
                }
                //while let Some(Ok(Key::Char(in_char))) = stdin.next() {
                //}
            }
        }
    }
}
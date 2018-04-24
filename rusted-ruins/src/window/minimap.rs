
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use sdl2::pixels::Color;
use array2d::*;
use common::gobj;
use game::Game;
use sdlvalues::SdlValues;
use game::{Animation, InfoGetter};
use window::Window;
use config::SCREEN_CFG;

pub struct MiniMapWindow {
    rect: Rect,
}

impl MiniMapWindow {
    pub fn new() -> MiniMapWindow {
        MiniMapWindow {
            rect: SCREEN_CFG.minimap_window.into(),
        }
    }
}

impl Window for MiniMapWindow {
    fn draw(
        &mut self, canvas: &mut WindowCanvas, game: &Game, sv: &mut SdlValues,
        _anim: Option<(&Animation, u32)>) {

        canvas.set_viewport(self.rect);
        draw_minimap(canvas, self.rect, game, sv);
    }
}

const RECT_SIZE: u32 = 3;
const RECT_SIZE_I: i32 = RECT_SIZE as i32;

fn draw_minimap(canvas: &mut WindowCanvas, rect: Rect, game: &Game, _sv: &mut SdlValues) {
    use std::cmp::{max, min};
    let map = game.gd.get_current_map();
    let map_size = map.size();
    let n_width = (rect.width() / RECT_SIZE) as i32;
    let n_height = (rect.height() / RECT_SIZE) as i32;
    let center_p = game.gd.player_pos();
    let top_left = (center_p.0 - n_width / 2, center_p.1 - n_height / 2);
    let bottom_right = (min(map_size.0 as i32 - 1, center_p.0 + n_width / 2),
                             min(map_size.1 as i32 - 1, center_p.1 + n_height / 2));
    let (dx, dy) = (top_left.0 * RECT_SIZE_I, top_left.1 * RECT_SIZE_I);
    let top_left = (max(0, top_left.0), max(0, top_left.1));
                             
    for p in RectIter::new(top_left, bottom_right) {
        let color = if p == center_p {
            (255, 255, 0)
        } else if let Some(wall_idx) = map.observed_tile[p].wall.idx() {
            gobj::get_obj(wall_idx).symbol_color
        } else if let Some(tile) = map.observed_tile[p].tile {
            gobj::get_obj(tile[0].idx).symbol_color
        } else {
            continue;
        };
        let color = Color::RGB(color.0, color.1, color.2);
        
        let draw_rect = Rect::new(p.0 * RECT_SIZE_I - dx, p.1 * RECT_SIZE_I - dy, RECT_SIZE, RECT_SIZE);
        canvas.set_draw_color(color);
        check_draw!(canvas.fill_rect(draw_rect));
    }
}


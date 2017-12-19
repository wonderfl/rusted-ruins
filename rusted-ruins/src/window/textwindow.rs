
use super::commonuse::*;
use super::widget::*;
use sdlvalues::FontKind;

pub struct TextWindow {
    rect: Rect,
    label: LabelWidget,
}

impl TextWindow {
    pub fn new(rect: Rect, s: &str) -> TextWindow {
        TextWindow {
            rect,
            label: LabelWidget::wrapped(
                (0, 0, rect.w as u32, 0), s, FontKind::M, rect.w as u32),
        }
    }

    pub fn get_rect(&self) -> Rect {
        self.rect
    }
}

impl Window for TextWindow {
    
    fn redraw(
        &mut self, canvas: &mut WindowCanvas, _game: &Game, sv: &mut SdlValues,
        _anim: Option<(&Animation, u32)>) {

        let window_size = self.label.adjust_widget_size(sv);
        self.rect.w = window_size.0 as i32;
        self.rect.h = window_size.1 as i32;

        draw_rect_border(canvas, self.rect);
        self.label.draw(canvas, sv);
    }
}


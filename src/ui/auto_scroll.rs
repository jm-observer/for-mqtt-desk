
use crate::ui::ids::SELECTOR_AUTO_SCROLL;


use druid::widget::{Controller, Scroll};
use druid::{
    Env, Event, EventCtx, Rect, Widget,
};


pub struct AutoScrollController;

impl<T: druid::Data, W: Widget<T>> Controller<T, Scroll<T, W>> for AutoScrollController {
    fn event(
        &mut self,
        child: &mut Scroll<T, W>,
        _ctx: &mut EventCtx,
        event: &Event,
        data: &mut T,
        _env: &Env,
    ) {
        match event {
            Event::Command(cmd) => {
                if let Some(_) = cmd.get(SELECTOR_AUTO_SCROLL) {
                    let size = child.child_size();
                    let end_region =
                        Rect::new(size.width - 1., size.height - 1., size.width, size.height);
                    child.scroll_to(_ctx, end_region);
                }
            }
            _ => child.event(_ctx, event, data, _env),
        }
    }
}

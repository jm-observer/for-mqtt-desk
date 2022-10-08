use crate::ForError;
use druid::text::ValidationError;
use druid::widget::{Either, Label, SizedBox, TextBoxEvent, ValidationDelegate};
use druid::{
    BoxConstraints, Color, Data, Env, Event, EventCtx, LayoutCtx, LifeCycle, LifeCycleCtx,
    PaintCtx, Point, Selector, Size, UpdateCtx, Widget, WidgetExt, WidgetId, WidgetPod,
};
use log::debug;

pub const ID_CLIENT_ID: WidgetId = WidgetId::reserved(2);
pub const ID_ADDR: WidgetId = WidgetId::reserved(3);
pub const ID_PORT: WidgetId = WidgetId::reserved(4);
pub const ID_PARAMS: WidgetId = WidgetId::reserved(5);

pub const ID_PUBLISH_TOPIC: WidgetId = WidgetId::reserved(6);
pub const ID_PUBLISH_QOS: WidgetId = WidgetId::reserved(7);
pub const ID_PUBLISH_MSG: WidgetId = WidgetId::reserved(8);
pub const ID_SUBSCRIBE_TOPIC: WidgetId = WidgetId::reserved(9);
pub const ID_SUBSCRIBE_QOS: WidgetId = WidgetId::reserved(10);
pub const ID_BUTTON_CONNECT: WidgetId = WidgetId::reserved(11);
pub const ID_BUTTON_RECONNECT: WidgetId = WidgetId::reserved(12);

pub const ERROR_TEXT_COLOR: Color = Color::rgb8(0xB6, 0x00, 0x04);

/// Sent by the [`TextBoxErrorDelegate`] when an error should be displayed.
pub const SHOW_ERROR: Selector<ValidationError> = Selector::new("druid-example.show-error");
/// Sent by the [`TextBoxErrorDelegate`] when an error should be cleared.
pub const CLEAR_ERROR: Selector = Selector::new("druid-example.clear-error");
/// Sent by the [`TextBoxErrorDelegate`] when editing began.
///
/// This is used to set the contents of the help text.
pub const EDIT_BEGAN: Selector<WidgetId> = Selector::new("druid-example.edit-began");
/// Sent by the [`TextBoxErrorDelegate`] when editing finishes.
///
/// This is used to set the contents of the help text.
pub const EDIT_FINISHED: Selector<WidgetId> = Selector::new("druid-example.edit-finished");

pub struct TextBoxErrorDelegate {
    target: WidgetId,
    check_fn: fn(&str) -> bool,
    sends_partial_errors: bool,
}

impl TextBoxErrorDelegate {
    pub fn new(target: WidgetId, check_fn: fn(&str) -> bool) -> TextBoxErrorDelegate {
        TextBoxErrorDelegate {
            target,
            check_fn,
            sends_partial_errors: false,
        }
    }

    pub fn sends_partial_errors(mut self, flag: bool) -> Self {
        self.sends_partial_errors = flag;
        self
    }
}

impl ValidationDelegate for TextBoxErrorDelegate {
    fn event(&mut self, ctx: &mut EventCtx, event: TextBoxEvent, _current_text: &str) {
        match event {
            TextBoxEvent::Began => {
                debug!("Began");
                ctx.submit_command(CLEAR_ERROR.to(self.target));
                ctx.submit_command(EDIT_BEGAN.with(self.target));
            }
            TextBoxEvent::Changed if self.sends_partial_errors => {
                debug!("Changed");
                ctx.submit_command(CLEAR_ERROR.to(self.target));
            }
            TextBoxEvent::PartiallyInvalid(err) if self.sends_partial_errors => {
                debug!("PartiallyInvalid");
                ctx.submit_command(SHOW_ERROR.with(err).to(self.target));
            }
            TextBoxEvent::Invalid(err) => {
                debug!("Invalid");
                ctx.submit_command(SHOW_ERROR.with(err).to(self.target));
            }
            TextBoxEvent::Cancel | TextBoxEvent::Complete => {
                debug!("Cancel | Complete: {}", _current_text);
                if (self.check_fn)(_current_text) {
                    ctx.submit_command(CLEAR_ERROR.to(self.target));
                }
            }
            _ => (),
        }
    }
}

pub struct ErrorController<W> {
    child: WidgetPod<Option<ValidationError>, W>,
    error: Option<ValidationError>,
}
impl<W: Widget<Option<ValidationError>>> ErrorController<W> {
    pub fn new(child: W) -> ErrorController<W> {
        ErrorController {
            child: WidgetPod::new(child),
            error: None,
        }
    }
}

impl<T, W: Widget<Option<ValidationError>>> Widget<T> for ErrorController<W> {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, _data: &mut T, env: &Env) {
        match event {
            Event::Command(cmd) if cmd.is(SHOW_ERROR) => {
                self.error = Some(cmd.get_unchecked(SHOW_ERROR).to_owned());
                ctx.request_update();
            }
            Event::Command(cmd) if cmd.is(CLEAR_ERROR) => {
                self.error = None;
                ctx.request_update();
            }
            _ => self.child.event(ctx, event, &mut self.error, env),
        }
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, _: &T, env: &Env) {
        self.child.lifecycle(ctx, event, &self.error, env);
    }

    fn update(&mut self, ctx: &mut UpdateCtx, _: &T, _: &T, env: &Env) {
        self.child.update(ctx, &self.error, env)
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, _: &T, env: &Env) -> Size {
        let size = self.child.layout(ctx, bc, &self.error, env);
        self.child.set_origin(ctx, &self.error, env, Point::ZERO);
        size
    }

    fn paint(&mut self, ctx: &mut PaintCtx, _: &T, env: &Env) {
        self.child.paint(ctx, &self.error, env);
    }

    fn id(&self) -> Option<WidgetId> {
        Some(self.child.id())
    }
}

use druid::{BoxConstraints, Data, Env, Event, EventCtx, ImageBuf, LayoutCtx, LifeCycle, LifeCycleCtx, PaintCtx, Size, UpdateCtx, Widget, WidgetExt, WidgetId};
use druid::widget::{Flex, Image, Scroll, SizedBox};
use piet_common::InterpolationMode;

use crate::app_state::AppState;

pub struct Gallery {
    inner: Box<dyn Widget<AppState>>,
}

impl Gallery {
    pub fn new() -> Gallery {
        Gallery {
            inner: SizedBox::empty().boxed(),
        }
    }

    fn rebuild_inner(&mut self, data: &AppState) {
        self.inner = build_widget(&data);
    }
}

impl Widget<AppState> for Gallery {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut AppState, env: &Env) {
        self.inner.event(ctx, event, data, env)
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &AppState, env: &Env) {
        if let LifeCycle::WidgetAdded = event {
            self.rebuild_inner(data);
        }
        self.inner.lifecycle(ctx, event, data, env)
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &AppState, data: &AppState, _env: &Env) {
        if !old_data.same(&data) {
            self.rebuild_inner(data);
            ctx.children_changed();
        }
    }

    fn layout(
        &mut self,
        ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        data: &AppState,
        env: &Env,
    ) -> Size {
        self.inner.layout(ctx, bc, data, env)
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &AppState, env: &Env) {
        self.inner.paint(ctx, data, env)
    }

    fn id(&self) -> Option<WidgetId> {
        self.inner.id()
    }
}

fn build_widget(state: &AppState) -> Box<dyn Widget<AppState>> {
    let mut col = Flex::row();

    for file in &state.files {
        let png_data = ImageBuf::from_file(file).unwrap();

        let mut img = Image::new(png_data).fill_mode(state.fill_strat);
        img.set_interpolation_mode(InterpolationMode::Bilinear);

        let mut sized: SizedBox<AppState> = SizedBox::new(img);
        sized = sized.fix_width(307.2);
        sized = sized.fix_height(192.0);

        col.add_child(sized);
        col.add_default_spacer();
    }

    Scroll::new(col).boxed()
}
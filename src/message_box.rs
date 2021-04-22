use crate::AppState;
use druid::{Widget, WidgetExt, LifeCycle, EventCtx, PaintCtx, BoxConstraints, LifeCycleCtx, Size, LayoutCtx, Event, Env, UpdateCtx, WidgetId, Data, Color};
use druid::widget::{SizedBox, Label, Flex};

pub struct MessageBox {
    inner: Box<dyn Widget<AppState>>,
}

impl MessageBox {
    pub fn new() -> MessageBox {
        MessageBox {
            inner: SizedBox::empty().boxed(),
        }
    }

    fn rebuild_inner(&mut self, data: &AppState) {
        self.inner = build_widget(&data);
    }
}

impl Widget<AppState> for MessageBox {
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
    let mut col = Flex::column();

    for file in &state.messages {
        let label = Label::new(file.as_str()).with_text_color(Color::BLACK);
        col.add_child(label);
    }

    col.boxed()
}
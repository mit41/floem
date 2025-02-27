use floem::{
    animate::Animation,
    event::EventListener as EL,
    peniko::color::palette,
    reactive::{RwSignal, SignalGet, Trigger},
    unit::DurationUnitExt,
    views::{empty, h_stack, Decorators},
    IntoView,
};

pub fn animation_view() -> impl IntoView {
    let animation = RwSignal::new(
        Animation::new()
            .duration(5.seconds())
            .keyframe(0, |f| f.computed_style())
            .keyframe(50, |f| {
                f.style(|s| s.background(palette::css::BLACK).size(30, 30))
                    .ease_in()
            })
            .keyframe(100, |f| {
                f.style(|s| s.background(palette::css::AQUAMARINE).size(10, 300))
                    .ease_out()
            })
            .repeat(true)
            .auto_reverse(true),
    );

    let pause = Trigger::new();
    let resume = Trigger::new();

    h_stack((
        empty()
            .style(|s| s.background(palette::css::RED).size(500, 100))
            .animation(move |_| animation.get().duration(10.seconds())),
        empty()
            .style(|s| {
                s.background(palette::css::BLUE)
                    .size(50, 100)
                    .border(5.)
                    .border_color(palette::css::GREEN)
            })
            .animation(move |_| animation.get())
            .animation(move |a| {
                a.keyframe(0, |f| f.computed_style())
                    .keyframe(100, |f| {
                        f.style(|s| s.border(5.).border_color(palette::css::PURPLE))
                    })
                    .duration(5.seconds())
                    .repeat(true)
                    .auto_reverse(true)
            }),
        empty()
            .style(|s| s.background(palette::css::GREEN).size(100, 300))
            .animation(move |_| {
                animation
                    .get()
                    .pause(move || pause.track())
                    .resume(move || resume.track())
                    .delay(3.seconds())
            })
            .on_event_stop(EL::PointerEnter, move |_| {
                pause.notify();
            })
            .on_event_stop(EL::PointerLeave, move |_| {
                resume.notify();
            }),
    ))
    .style(|s| s.size_full().gap(10).items_center().justify_center())
}

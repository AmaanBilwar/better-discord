use gpui::*;
use gpui_component::{button::*, *};

#[derive(Clone, Copy, PartialEq, Eq)]
enum Screen {
    Welcome,
    Main,
}

struct AppState {
    current_screen: Screen,
}

pub struct Welcome {
    app_state: Entity<AppState>,
}

struct Main;

struct AppRoot {
    app_state: Entity<AppState>,
    welcome_view: Entity<Welcome>,
    main_view: Entity<Main>,
}

impl Render for AppRoot {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl Element {
        let screen = self.app_state.read(cx).current_screen;
        div().size_full().child(match screen {
            Screen::Welcome => self.welcome_view.clone().into_any_element(),
            Screen::Main => self.main_view.clone().into_any_element(),
        })
    }
}

impl Render for Welcome {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        let app_state = self.app_state.clone();
        div()
            .v_flex()
            .gap_2()
            .size_full()
            .items_center()
            .justify_center()
            .child("Welcome to Better Discord!")
            .child(
                Button::new("ok")
                    .primary()
                    .label("Let's Jump In!")
                    .on_click(move |_, _, cx| {
                        // logic to render new view
                        app_state.update(cx, |state, cx| {
                            state.current_screen = Screen::Main;
                            cx.notify();
                        })
                    }),
            )
    }
}

impl Render for Main {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        div()
            .v_flex()
            .size_full()
            .items_baseline()
            .justify_center()
            .child("You're in the main app!")
    }
}

fn main() {
    let app = Application::new();

    app.run(move |cx| {
        gpui_component::init(cx);

        cx.spawn(async move |cx| {
            cx.open_window(WindowOptions::default(), |window, cx| {
                let app_state = cx.new(|_| AppState {
                    current_screen: Screen::Welcome,
                });

                cx.observe(&app_state, |_, _| {}).detach();
                let welcome_view = cx.new(|_| Welcome {
                    app_state: app_state.clone(),
                });
                let main_view = cx.new(|_| Main);
                let root_view = cx.new(|cx| {
                    cx.observe(&app_state, |_, _, _| {}).detach();
                    AppRoot {
                        app_state: app_state.clone(),
                        welcome_view,
                        main_view,
                    }
                });
                cx.new(|cx| Root::new(root_view, window, cx))
            })?;

            Ok::<_, anyhow::Error>(())
        })
        .detach();
    });
}

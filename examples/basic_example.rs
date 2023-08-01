use conrod_core::{UiBuilder, Ui, widget_ids, widget, Positionable, Sizeable, Labelable, Widget};
use conrod_glium::{Renderer, glium_events_conversion::{WasEventHandled, handle_glium_event}};
use glium::{glutin::{event_loop::{EventLoop, ControlFlow}, window::WindowBuilder, ContextBuilder, event::WindowEvent}, Display, backend::glutin, Surface};

widget_ids!(struct Ids { text });

fn main() {
    // creating glium stuff
    let event_loop = EventLoop::new();
    let wb = WindowBuilder::new(); 
    let cb = ContextBuilder::new().with_srgb(false);
    let display = Display::new(wb, cb, &event_loop).expect("failed to create glium display");

    // creating conrod stuff
    let mut ui = UiBuilder::new([2560 as f64, 1080 as f64]).build();
    let image_map = conrod_core::image::Map::<glium::texture::Texture2d>::new();
    let mut renderer = Renderer::new(&display).unwrap();

    // adding default font (change this path with whatever you want)
    ui.fonts.insert_from_file("fonts/JetBrainsMono-Regular.ttf").expect("failed to add fonts");

    let button_id = Ids::new(ui.widget_id_generator());
    let text_id = Ids::new(ui.widget_id_generator());

    event_loop.run(move |ev, _, control_flow| {
        // handling glium events
        match ev {
            // doing rendering here
            glium::glutin::event::Event::MainEventsCleared => {
                // drawing stuff 
                let mut target = display.draw();
                // making background gray
                target.clear_color_srgb(0.5, 0.5, 0.5, 1.0);

                // drawing ui
                let primitives = ui.draw();
                renderer.fill(&display, primitives, &image_map);
                renderer.draw(&display, &mut target, &image_map).unwrap();

                target.finish().unwrap();
            },
            // if some event happened(mouse movement, click, keyboard etc)
            glutin::glutin::event::Event::WindowEvent  { event, .. } => {
                // handling glium event
                let event_handled = handle_glium_event(&mut ui, &event, display.gl_window().window());
                match event_handled {
                    WasEventHandled::Yes => set_ui(&mut ui, &button_id, &text_id), // updating ui
                    WasEventHandled::No => (),
                }

                match event {
                    WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                    _ => ()
                }
            }
            _ => (),
        }
    });
}

// function for updating ui
fn set_ui(ui: &mut Ui, button_id: &Ids, text_id: &Ids) {
    let mut ui_cell = ui.set_widgets();

    // adding ui 
    widget::Button::new()
        .middle_of(ui_cell.window)
        .y_dimension(conrod_core::position::Dimension::Absolute(100.0))
        .x_dimension(conrod_core::position::Dimension::Absolute(300.0))
        .label("press me!")
        .label_font_size(32)
        .press_color(conrod_core::Color::Rgba(1.0, 1.0, 1.0, 1.0))
        .set(button_id.text, &mut ui_cell);

    // adding some text
    widget::Text::new("check event handling by pressing the button above")
        .y_relative(-100.0)
        .set(text_id.text, &mut ui_cell);
}

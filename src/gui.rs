
use iui::prelude::*;
use iui::controls::{Label, Button, VerticalBox, Group};

use osc::*;
use ugens::*;

pub fn run_gui() {
       // Initialize the UI library
    let ui = UI::init().expect("Couldn't initialize UI library");
    // Create a window into which controls can be placed
    let mut win = Window::new(&ui, "Test App", 200, 200, WindowType::NoMenubar);
    
    // Create a vertical layout to hold the controls
    let mut vbox = VerticalBox::new(&ui);
    vbox.set_padded(&ui, true);

    let mut group_vbox = VerticalBox::new(&ui);
    let mut group = Group::new(&ui, "Group");

    // Create two buttons to place in the window
    let mut button = Button::new(&ui, "Start");
    button.on_clicked(&ui, {
        let ui = ui.clone();
        move |btn| {
            btn.set_text(&ui, "Started!");
            sc_start();
        }
    });
    let mut button1 = Button::new(&ui, "Play");
    button1.on_clicked(&ui, {
        let ui = ui.clone();
        move |btn| {
            btn.set_text(&ui, "Playing");
            sc_play(&sin_osc(440.0, 0.0));
        }
    });
    let mut button11 = Button::new(&ui, "Play Stereo(?)");
    button11.on_clicked(&ui, {
        let ui = ui.clone();
        move |btn| {
            btn.set_text(&ui, "Playing");
            sc_play_vec(vec![sin_osc(440.0, 0.0), sin_osc(100.0, 0.0)]);
        }
    });

    let mut button2 = Button::new(&ui, "Stop");
    button2.on_clicked(&ui, {
        let ui = ui.clone();
        move |btn| {
            btn.set_text(&ui, "Stopped!");
            sc_stop();
        }
    });

    let mut quit_button = Button::new(&ui, "Quit");
    quit_button.on_clicked(&ui, {
        let ui = ui.clone();
        move |_| {
            ui.quit();
        }
    });

    group_vbox.append(&ui, button, LayoutStrategy::Compact);
    group_vbox.append(&ui, button1, LayoutStrategy::Compact);
    group_vbox.append(&ui, button11, LayoutStrategy::Compact);
    group_vbox.append(&ui, button2, LayoutStrategy::Compact);
    group_vbox.append(&ui, quit_button, LayoutStrategy::Compact);
    group.set_child(&ui, group_vbox);
    vbox.append(&ui, group, LayoutStrategy::Compact);


     // Actually put the button in the window
    win.set_child(&ui, vbox);
    // Show the window
    win.show(&ui);
    // Run the application
    ui.main();

}
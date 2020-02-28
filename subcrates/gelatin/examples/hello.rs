use gelatin::{application::*, button::*, line_layout_container::*, misc::*, window::*};

use std::cell::Cell;
use std::f32;
use std::rc::Rc;

fn main() {
    let mut application = Application::new();
    // A window
    let window = Window::new(&mut application);
    let container = Rc::new(HorizontalLayoutContainer::new());
    container.set_margin_top(5.0);
    container.set_margin_bottom(5.0);
    container.set_height(Length::Stretch { min: 0.0, max: f32::INFINITY });
    container.set_width(Length::Stretch { min: 0.0, max: f32::INFINITY });
    
    let button = Rc::new(Button::new());
    button.set_margin_top(5.0);
    //button.set_pos(LogicalVector::new(5.0, 5.0));
    //button.set_fixed_size(LogicalVector::new(24.0, 24.0));
    button.set_height(Length::Fixed(24.0));
    button.set_width(Length::Fixed(24.0));
    button.set_horizontal_align(Alignment::Center);
    //button.set_width(Length::Stretch { min: 0.0, max: f32::INFINITY });
    
    let button2 = Rc::new(Button::new());
    button2.set_margin_top(5.0);
    //button.set_pos(LogicalVector::new(5.0, 5.0));
    //button.set_fixed_size(LogicalVector::new(24.0, 24.0));
    button2.set_height(Length::Fixed(24.0));
    button2.set_width(Length::Fixed(24.0));
    button2.set_horizontal_align(Alignment::Center);
    
    let button3 = Rc::new(Button::new());
    button3.set_margin_top(5.0);
    button3.set_height(Length::Fixed(24.0));
    button3.set_width(Length::Stretch { min: 0.0, max: 200.0 });
    button3.set_horizontal_align(Alignment::Start);
    
    container.add_child(button.clone());
    container.add_child(button2.clone());
    container.add_child(button3.clone());
    
    container.set_margin_left(0.0);
    container.set_margin_right(0.0);
    button.set_margin_left(5.0);
    button.set_margin_right(5.0);
    button2.set_margin_left(5.0);
    button2.set_margin_right(5.0);
    button3.set_margin_left(5.0);
    button3.set_margin_right(5.0);
    {
        let button_clone = button.clone();
        let pos = Cell::new(5.0);
        button.set_on_click(move || {
            let new_pos = pos.get() + 5.0;
            pos.set(new_pos);

            button_clone.set_margin_left(new_pos);
            button_clone.set_margin_top(new_pos);
        });
    }
    window.set_root(Some(container));
    application.start_event_loop();
}
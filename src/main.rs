
use gtk;
use gtk::{WidgetExt, ContainerExt, LabelExt, GtkApplicationExt, Inhibit, DrawingArea, FlowBox, StackExt};
use gtk::prelude::BuilderExtManual;
use gtk::GtkWindowExt;
use gio::{ApplicationFlags, ApplicationExt};
use gio::prelude::ApplicationExtManual;
use cairo::{Format, Content, Context, Pattern};
use chipcalc_native_rust::matrix::{Matrix, MatrixRotationCache, MatrixRotation};
use chipcalc_native_rust::shape::Shape;
use chipcalc_native_rust::chip::Color;
use enum_iterator::IntoEnumIterator;
use lazy_static::lazy_static;
use std::sync::{Arc, Mutex};
use std::f64::consts::PI;

lazy_static! {
	static ref CURRENT_ROTATION: Arc<Mutex<u8>> = Arc::new(Mutex::new(0));
}

const FLOW_BOXES: [([&str; 2], MatrixRotation); 4] = [
	([
		 "all_chips_orange_cw0_flow_box",
		 "all_chips_blue_cw0_flow_box"
	 ], MatrixRotation::Cw0),
	([
		 "all_chips_orange_cw90_flow_box",
		 "all_chips_blue_cw90_flow_box"
	 ], MatrixRotation::Cw90),
	([
		 "all_chips_orange_cw180_flow_box",
		 "all_chips_blue_cw180_flow_box"
	 ], MatrixRotation::Cw180),
	([
		 "all_chips_orange_cw270_flow_box",
		 "all_chips_blue_cw270_flow_box"
	 ], MatrixRotation::Cw270),
];

fn main() {
    //gtk init
    gtk::init();

	let builder = gtk::Builder::from_string(include_str!("chipcalc.glade"));
    let mut application = gtk::Application::new(None, ApplicationFlags::default()).unwrap();
    let mut window: gtk::ApplicationWindow = builder.get_object("main_application_window").unwrap();
    window.set_title("칩셋조합기");
    application.connect_activate(move | app | {
        app.add_window(&window);
        window.show_all();
    });


    //all_chips init
    all_chips_init(&builder);

    //all_chips_rotate button init
    let button_left: gtk::Button = builder.get_object("chip_rotate_left_button").unwrap();
	let stack_orange: gtk::Stack = builder.get_object("all_chips_orange_rotation_stack").unwrap();
	let stack_blue: gtk::Stack = builder.get_object("all_chips_blue_rotation_stack").unwrap();
    button_left.connect_button_release_event(move | button, event_button | {
	    let mut lock = CURRENT_ROTATION.lock().unwrap();
	    let mut current_rotation = lock.clone();
	    *lock += 3;
	    *lock %= 4;
	    let child = stack_orange.get_children();
	    stack_orange.set_visible_child(&child[*lock as usize]);
	    let child = stack_blue.get_children();
	    stack_blue.set_visible_child(&child[*lock as usize]);
        Inhibit(false)
    });



    application.run(&[]);
}

fn draw_matrix(drawing_area: &DrawingArea, context: &Context, matrix: &Matrix, color: Color) {
    match color {
        Color::Orange => context.set_source_rgb(1.0, 0.31, 0.0),
        Color::Blue => context.set_source_rgb(0.0, 0.0, 1.0)
    }
    drawing_area.set_size_request(60, 60);

    let x_offset = 30 - (matrix.x_size * 5);
    let y_offset = 30 - (matrix.raw_map.len() * 5);
    let mut y_pos = y_offset;
    for line in &matrix.raw_map {
        for i in 0..8 {
            if (*line & (0b10000000 >> i)) >> (7 - i) == 0b00000001 {
                let x_pos = x_offset + (i * 10);
                context.rectangle(x_pos as f64, y_pos as f64, 10.0, 10.0);
                context.fill();
            }
        }
        y_pos += 10;
    }
    context.stroke();
}

fn all_chips_init(builder: &gtk::Builder) {

	for (flow_boxes, rot) in &FLOW_BOXES {
		let mut orange = true;
		for flow_box in flow_boxes {
			let flow_box: FlowBox = builder.get_object(flow_box).unwrap();
			for x in Shape::into_enum_iter() {
				if x != Shape::NONE {
					let drawing_area = gtk::DrawingArea::new();
					let matrix = x.get_rotation_cache().get(&rot);
					drawing_area.connect_draw(move | drawing_area, context | {
						draw_matrix(drawing_area, context, matrix, if orange { Color::Orange } else { Color::Blue });
						Inhibit(false)
					});
					flow_box.add(&drawing_area);
				}
			}
			flow_box.show_all();
			orange = false;
		}
	}
}


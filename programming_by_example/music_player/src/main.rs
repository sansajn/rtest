use std::env;
use gio::{ApplicationExt, ApplicationExtManual, ApplicationFlags};
use gtk::{
	Application, 
	ApplicationWindow, 
	WidgetExt, 
	GtkWindowExt,
	ContainerExt,
	Adjustment,
	Image,
	ImageExt,
	Scale,
	ScaleExt
};
use gtk::Orientation::{Horizontal, Vertical};

mod playlist;
mod toolbar;

use crate::playlist::Playlist;
use crate::toolbar::MusicToolbar;

struct App {
	adjustment: Adjustment,
	cover: Image,
	playlist: Playlist,
	toolbar: MusicToolbar,
	window: ApplicationWindow
}

impl App {
	fn new(application: Application) -> Self {
		let window = ApplicationWindow::new(&application);
		window.set_title("Rusic");

		let vbox = gtk::Box::new(Vertical, 0);
		window.add(&vbox);

		let toolbar = MusicToolbar::new();
		vbox.add(toolbar.toolbar());

		let playlist = Playlist::new();
		vbox.add(playlist.view());

		let cover = Image::new();
		cover.set_from_file("cover.jpg");
		vbox.add(&cover);
		
		let adjustment = Adjustment::new(0.0, 0.0, 10.0, 0.0, 0.0, 0.0);
		let scale = Scale::new(Horizontal, &adjustment);
		scale.set_draw_value(false);
		vbox.add(&scale);

		window.show_all();

		let app = App {
			adjustment,
			cover,
			playlist,
			toolbar, 
			window
		};

		app.connect_events();
		app.connect_toolbar_events();
		app
	}

	fn connect_events(&self) {}
}


fn main() {
	let application = Application::new("com.rust-by-example", ApplicationFlags::empty())
		.expect("Application initialization failed");

	application.connect_startup(|application| {
		let _app = App::new(application.clone());		
	});

	application.connect_activate(|_| {});

	application.run(&env::args().collect::<Vec<_>>());
}

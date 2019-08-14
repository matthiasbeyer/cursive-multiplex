extern crate cursive;

use cursive::Cursive;
use cursive::views::TextView;
use cursive_multiplex::Mux;

fn main() {
    let mut siv = Cursive::default();
    let (mut mux, root) = Mux::new(TextView::new("Center".to_string()));
    let _id = mux.add_vertical_id(TextView::new("Up"), root).expect("Left failed");
    let _id = mux.add_vertical_id(TextView::new("Down"), root).expect("Right failed");
    mux.remove_id(root).expect("remove failed");

    siv.add_fullscreen_layer(mux);
    siv.run();
}
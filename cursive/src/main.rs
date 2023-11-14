extern crate cursive;
use cursive::traits::Identifiable; // for .with_id() and .call_on_id()

use cursive::views::{Dialog, EditView, TextView, ListView, Checkbox}; // for Dialog, EditView, TextView
use cursive::Cursive; // for Cursive::default()

struct CatsayOption<'a> {
    message: &'a str,
    dead: bool,
}

fn input_step(siv: &mut Cursive){
    siv.add_layer(
        Dialog::new()
            .title("Please fill out the form for the cat")
            .content(
                ListView::new()
                    .child("Message", EditView::new().with_id("message"))
                    .child("Dead", Checkbox::new().with_id("dead"))
            )
            .button("Ok", |s| {
                let message = s
                    .call_on_id("message", |t: &mut EditView| t.get_content())
                    .unwrap();
                let dead = s
                    .call_on_id("dead", |t: &mut Checkbox| t.is_checked())
                    .unwrap();
                let cat_option = CatsayOption {
                    message: &message,
                    dead,
                };
                result_step(s, &cat_option);
            })
    )
}

fn result_step(siv: &mut Cursive, options: &CatsayOption) {
    let eye = if options.dead { "x" } else { "o" };
    let cat_text = format!(
        "{msg}
\\
\\
/\\_/\\
( {eye} {eye} )
=( I )=",
        msg = options.message,
        eye = eye
    );

    siv.pop_layer(); 
    siv.add_layer( 
        Dialog::around(TextView::new(cat_text))
            .title("The cat says...")
            .button("OK", |s| s.quit()),
    );
}

fn main() {
    let mut siv = Cursive::default();
    input_step(&mut siv);
    siv.run();
}
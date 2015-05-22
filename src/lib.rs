#![feature(plugin)]
#![plugin(interpolate_idents)]

#[macro_use]
extern crate apache2;

#[macro_use]
mod code;

use apache2::{Request, Status, HookOrder};

use code::{STYLESHEET, SCRIPT};


apache2_module!(parallax, b"mod_parallax\0", handlers {
   (parallax_handler, handler, HookOrder::MIDDLE),
   (js_handler, handler, HookOrder::MIDDLE)
});


fn parallax_handler(r: &mut Request) -> Result<Status, ()> {
   if get!(r.handler()) != "parallax" {
      return Ok(Status::DECLINED)
   }

   r.set_content_type("text/html");

   try!(r.write(format!(
      html_template!(),
      STYLESHEET,
      SCRIPT
   )));

   Ok(Status::OK)
}


fn js_handler(r: &mut Request) -> Result<Status, ()> {
   if get!(r.handler()) != "js" {
      return Ok(Status::DECLINED)
   }

   r.set_content_type("text/plain");

   try!(r.write("JS"));

   Ok(Status::OK)
}

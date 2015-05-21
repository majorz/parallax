#![feature(plugin)]
#![plugin(interpolate_idents)]

#[macro_use]
extern crate apache2;

use apache2::{Request, Status};

apache2_module!(parallax, b"mod_parallax\0");


macro_rules! html_template {() => ("<!doctype html>
<html lang='en'>
   <head>
      <meta charset='utf-8'>
      <meta http-equiv='X-UA-Compatible' content='IE=edge,chrome=1'>
      <title>PolyDraw / Parallax</title>
      <meta name='viewport' content='width=device-width, initial-scale=1.0'>

      <link rel='stylesheet' href='https://cdnjs.cloudflare.com/ajax/libs/normalize/3.0.3/normalize.css'>
      <link rel='stylesheet' href='https://fonts.googleapis.com/css?family=Khand:700'>

      <script src='https://cdnjs.cloudflare.com/ajax/libs/jquery/2.1.3/jquery.js'></script>
      <script src='https://cdnjs.cloudflare.com/ajax/libs/jquery-mousewheel/3.1.12/jquery.mousewheel.js'></script>
      <script src='https://cdnjs.cloudflare.com/ajax/libs/velocity/1.2.2/velocity.js'></script>
   </head>

   <style>
      {}
   </style>

   <body>

      <div id='page-wrapper'>
         <section id='S001'>
            <div class='container'>
               S001
            </div>
         </section>

         <section id='S002'>
            <div class='container'>
               S002
            </div>
         </section>

         <section id='S003'>
            <div class='container'>
               S003
            </div>
         </section>

         <section id='S004'>
            <div class='container'>
               S004
            </div>
         </section>

         <section id='S005'>
            <div class='container'>
               S005
            </div>
         </section>
      </div>

      <script>
         {}
      </script>

   </body>
</html>
")}


fn parallax_handler(r: &mut Request) -> Result<Status, ()> {
   if try!(r.handler()) != "parallax" {
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


const STYLESHEET: &'static str = "
   body {
      overflow: hidden;

      background-color: #000000;

      font-family: 'Khand', sans-serif;
      font-size: 180px;
      font-weight: 700;
      text-align: center;
      color: white;
   }

   section {
      display: block;
      position: relative;
   }

   .container {
      position: relative;
   }

   #S001 {
      background-color: #ed8c2b;
   }

   #S002 {
      background-color: #cf4a30;
   }

   #S003 {
      background-color: #911146;
   }

   #S004 {
      background-color: #35203b;
   }

   #S005 {
      background-color: #88a825;
   }
";


const SCRIPT: &'static str = "
   var CURRENT_SECTION = 0;
   var SECTION_COUNT = 5;

   var ACCUMULATED_SCROLL = 0;
   var TIMER_ID = null;

   var HEIGHT_RATIO = 4 / 7;

   var TOUCHSTART_Y = null;
   var TOUCHSTART_SECTION = null;

   var easingFunction = jQuery.fn.velocity.Easings.easeInQuad;

   $(document).keydown(function(e) {
      if (e.which == 40) { // Down arrow
         if (CURRENT_SECTION == SECTION_COUNT - 1) return;

         CURRENT_SECTION += 1;

         navigateToCurrentSection();
      }

      if (e.which == 38) { // Up arrow
         if (CURRENT_SECTION == 0) return;

         CURRENT_SECTION -= 1;
         navigateToCurrentSection();
      }
   });

   $(window).resize(function(){
      adjustContainerHeight();
   });

   $(window).mousewheel(function(event) {
      ACCUMULATED_SCROLL += -(event.deltaY * event.deltaFactor);

      processAccumulatedMove();
   });

   $(window).on('touchstart', function(event) {
      event.preventDefault();

      var e = event.originalEvent;
      TOUCHSTART_Y = e.touches[0].pageY;
      TOUCHSTART_SECTION = CURRENT_SECTION;

      console.log(e);
   });

   $(window).on('touchmove', function(event) {
      event.preventDefault();

      var e = event.originalEvent;
      var y = e.touches[0].pageY;

      ACCUMULATED_SCROLL = TOUCHSTART_Y - y;

      processAccumulatedMove();

      if (CURRENT_SECTION != TOUCHSTART_SECTION) {
         TOUCHSTART_Y = y;
         TOUCHSTART_SECTION = CURRENT_SECTION;
      }
   });

   var processAccumulatedMove = function() {
      if ((ACCUMULATED_SCROLL < 0 && CURRENT_SECTION == 0) || (ACCUMULATED_SCROLL > 0 && CURRENT_SECTION == SECTION_COUNT-1)) {
         ACCUMULATED_SCROLL = 0;
         return;
      }

      var switchDistance = $(window).height() * HEIGHT_RATIO;

      if (ACCUMULATED_SCROLL >= switchDistance) {
         ACCUMULATED_SCROLL = 0;
         CURRENT_SECTION += 1;
         navigateToCurrentSection();
      } else if (ACCUMULATED_SCROLL <= -switchDistance) {
         ACCUMULATED_SCROLL = 0;
         CURRENT_SECTION -= 1;
         navigateToCurrentSection();
      } else {
         transitionToAccumulated();

         if (TIMER_ID != null) {
            clearTimeout(TIMER_ID);
            TIMER_ID = null;
         };

         TIMER_ID = setTimeout(function() {
            TIMER_ID = null;
            navigateToCurrentSection(200);
         }, 600);
      }
   }

   var transitionToAccumulated = function() {
      var height = $(window).height();

      var switchDistance = height * HEIGHT_RATIO;

      var normalized = Math.abs(ACCUMULATED_SCROLL / switchDistance);
      var easing_ratio = easingFunction(normalized);

      $('#page-wrapper').velocity('stop').velocity({
         translateY: -(CURRENT_SECTION * height + ACCUMULATED_SCROLL * easing_ratio / 3)
      }, {
         easing: 'linear',
         duration: 100
      });
   }

   var navigateToCurrentSection = function(duration) {
      if (duration === undefined) {
         duration = 300;
      }
      var offset = -CURRENT_SECTION * $(window).height();
      $('#page-wrapper').velocity('stop').velocity({
         translateY: offset
      }, duration);
   }

   var adjustContainerHeight = function() {
      var height = $(window).height();

      $('.container')
         .height(height)
         .css({
            lineHeight: height + 'px'
         });

      navigateToCurrentSection();
   }

   adjustContainerHeight();
";

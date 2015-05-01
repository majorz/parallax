CURRENT_SECTION = 0
SECTION_COUNT = 5
ACCUMULATED_SCROLL = 0
TIMER_ID = null
HEIGHT_RATIO = 4 / 7
TOUCHSTART_Y = null
TOUCHSTART_SECTION = null

easingFunction = jQuery.fn.velocity.Easings.easeInQuad

$(document).keydown (e) ->
   if e.which == 40
      # Down arrow
      if CURRENT_SECTION == SECTION_COUNT - 1
         return
      CURRENT_SECTION += 1
      navigateToCurrentSection()
   if e.which == 38
      # Up arrow
      if CURRENT_SECTION == 0
         return
      CURRENT_SECTION -= 1
      navigateToCurrentSection()
   return

$(window).resize ->
   adjustContainerHeight()
   return

$(window).mousewheel (event) ->
   ACCUMULATED_SCROLL += -(event.deltaY * event.deltaFactor)
   processAccumulatedMove()
   return

$(window).on 'touchstart', (event) ->
   event.preventDefault()
   e = event.originalEvent
   TOUCHSTART_Y = e.touches[0].pageY
   TOUCHSTART_SECTION = CURRENT_SECTION
   console.log e
   return

$(window).on 'touchmove', (event) ->
   event.preventDefault()
   e = event.originalEvent
   y = e.touches[0].pageY
   ACCUMULATED_SCROLL = TOUCHSTART_Y - y
   processAccumulatedMove()
   if CURRENT_SECTION != TOUCHSTART_SECTION
      TOUCHSTART_Y = y
      TOUCHSTART_SECTION = CURRENT_SECTION
   return

processAccumulatedMove = ->
   if ACCUMULATED_SCROLL < 0 and CURRENT_SECTION == 0 or ACCUMULATED_SCROLL > 0 and CURRENT_SECTION == SECTION_COUNT - 1
      ACCUMULATED_SCROLL = 0
      return
   switchDistance = $(window).height() * HEIGHT_RATIO
   if ACCUMULATED_SCROLL >= switchDistance
      ACCUMULATED_SCROLL = 0
      CURRENT_SECTION += 1
      navigateToCurrentSection()
   else if ACCUMULATED_SCROLL <= -switchDistance
      ACCUMULATED_SCROLL = 0
      CURRENT_SECTION -= 1
      navigateToCurrentSection()
   else
      transitionToAccumulated()
      if TIMER_ID != null
         clearTimeout TIMER_ID
         TIMER_ID = null
      TIMER_ID = setTimeout((->
         TIMER_ID = null
         navigateToCurrentSection 200
         return
      ), 600)
   return

transitionToAccumulated = ->
   height = $(window).height()
   switchDistance = height * HEIGHT_RATIO
   normalized = Math.abs(ACCUMULATED_SCROLL / switchDistance)
   easing_ratio = easingFunction(normalized)
   $('#page-wrapper').velocity('stop').velocity { translateY: -(CURRENT_SECTION * height + ACCUMULATED_SCROLL * easing_ratio / 3) },
      easing: 'linear'
     duration: 100
   return

navigateToCurrentSection = (duration) ->
   if duration == undefined
      duration = 300
   offset = -CURRENT_SECTION * $(window).height()
   $('#page-wrapper').velocity('stop').velocity { translateY: offset }, duration
   return

adjustContainerHeight = ->
   height = $(window).height()
   $('.container').height(height).css lineHeight: height + 'px'
   navigateToCurrentSection()
   return

adjustContainerHeight()

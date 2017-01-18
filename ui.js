function UI() {
  this.ui = null

  this.mousemove = function(crabs, e) {
    var pt = new Vec(e.clientX, e.clientY)
    var physicsPos = crabs.renderer.s2p(crabs, pt)
  }

  this.mousedown = function(crabs, e) {
  }

  this.mouseup = function(crabs, e) {
  }
}

function TestUI() {
  this.mousemove = function(c, e) {
    // log(log.DEBUG, 'mouse move')
    // console.log(e)
  }
  this.click = function(c, e) {
    log(log.DEBUG, 'click')
    console.log(e)
  }
  this.keypress = function(c, e) {
    log(log.DEBUG, 'keypress')
    console.log(e)
  }
}



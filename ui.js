function UI() {
  this.ui = null

  this.selected = null
  this.itemSelected = false
  this.itemHovered = false
  this.mouseDown = false

  this.lastMousePos = null

  this.run = function(crabs) {
    if (this.itemSelected) {
      if (crabs.physics.masses[this.selected]) {
        var physicsPos = crabs.renderer.s2p(crabs, this.lastMousePos)
        crabs.physics.masses[this.selected].pos = physicsPos
      }
    }
  }

  this.resize = function(c) {
  }

  this.mousemove = function(crabs, e) {
    var pt = new Vec(e.clientX, e.clientY)
    this.lastMousePos = pt
    var physicsPos = crabs.renderer.s2p(crabs, pt)

    // if one isn't already selected, find the closest object
    if (!this.itemSelected) {
      var me = this
      var physics = crabs.physics
      var distance = 50.0 // should be fine until people start having 100K screens
      var foundId = ''
      crabs.ids.forEach(function(id) {
        if (physics.masses[id]) {
          var dist = Vec.mag(Vec.sub(physicsPos, physics.masses[id].pos))
          if (dist < distance) {
            distance = dist
            foundId = id
          }
        } else if (physics.springs[id]) {
          var aPos = physics.masses[physics.springs[id].ida].pos
          var bPos = physics.masses[physics.springs[id].idb].pos
          var center = Vec.scale(0.5, Vec.add(aPos, bPos))
          var dist = Vec.mag(Vec.sub(physicsPos, center))
          if (dist < distance) {
            distance = dist
            foundId = id
          }
        }
      })

      this.selected = foundId
    } else {
      if (crabs.physics.masses[this.selected]) {
        crabs.physics.masses[this.selected].pos = crabs.renderer.s2p(crabs, pt)
        crabs.physics.masses[this.selected].vel = Vec.zero()
      }
    }
    if (crabs.paused) {
      crabs.renderer.run(crabs)
    }
  }

  this.mousedown = function(crabs, e) {
    this.mouseDown = true 

    if (this.selected != '') {
      this.itemSelected = true
    }
  }

  this.mouseup = function(crabs, e) {
    this.mouseDown = false
    this.itemSelected = false
  }

  this.keypress = function(crabs, e) {
    if (e.key == 'p' || e.key == 'P') {
      crabs.paused = !crabs.paused
      if (!crabs.paused) {
        crabs.go()
      }
    }
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



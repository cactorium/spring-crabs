'use strict'

var log = function(level, msg) {
  var levelNames = ['DEBUG', 'INFO', 'WARN', 'ERR']
  if (levelNames[level]) {
    console.log(levelNames[level] + ': ' + msg)
  } else {
    console.log('UNKNOWN: ' + msg)
  }
}
log.DEBUG = 0
log.INFO  = 1
log.WARN  = 2
log.ERR   = 3

var time = function(t, f) {
  var start = performance.now()
  f()
  var end = performance.now()
  t = end - start
}

function Crabs(canvas, ui) {
  this.canvas = canvas
  this.uiCanvas = ui
  this.ids = []
  this.lastId = -1
  this.physics = new Physics()
  this.renderer = new Renderer()
  this.ui = new UI()
  this.components = [this.physics, this.ui, this.renderer]
  this.hooks = [this.ui, new TestUI()]
  this.paused = false
  this.width = canvas.width
  this.height = canvas.height

  this.init = function() {
    var me = this

    var events = ['mousemove', 'mousedown', 'mouseup', 'click', 'keypress']
    events.forEach(function(name) {
      var listeners = me.hooks.filter(function(l) {
        return typeof(l[name]) != 'undefined'
      })
      log(log.INFO, 'adding hooks for ' + name + ': ' + listeners)
      ui.addEventListener(name, function(e) {
        listeners.forEach(function(l) {
          l[name](me, e)
        })
      })
    })

    this.resize()
  }
  this.go = function() {
    if (!this.paused) {
      this.components.forEach(function(c) {
        c.run(crabs)
      })

      var me = this
      window.requestAnimationFrame(function() {
        me.go()
      })
    }
    // log(log.INFO, 'frame')
  }
  this.resize = function() {
    var w = window.innerWidth, h = window.innerHeight
    log(log.DEBUG, 'resize ' + w + ', ' + h)

    this.width = w
    this.height = h

    canvas.width = w
    canvas.height = h
    ui.width = w
    ui.height = h

    this.components.forEach(function(c) {
      c.resize(this)
    })

    this.renderer.run(this)
  }

  this.addMass = function(pos, vel) {
    var newIdNum = this.lastId + 1
    var newId = newIdNum.toString()
    this.ids.push(newId)
    var ret = this.physics.addMass(newId, pos, vel, this.physics.DEFAULT_MASS)
    this.lastId = newIdNum
    return ret
  }

  this.addSpring = function(m0, m1, r, k) {
    var newIdNum = this.lastId + 1
    var newId = newIdNum.toString()
    this.ids.push(newId)
    var ret = this.physics.addSpring(newId, m0, m1, r, k)
    this.lastId = newIdNum
    return ret
  }

  this.addMuscle = function(s, r, a, p) {
    return this.physics.addMuscle(s, r, a, p)
  }
}

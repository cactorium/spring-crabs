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

function Vec(x, y) {
  this.x = x
  this.y = y
}
Vec.add = function(a, b) {
  return new Vec(a.x + b.x, a.y + b.y)
}
Vec.sub = function(a, b) {
  return new Vec(a.x - b.x, a.y - b.y)
}
Vec.dot = function(a, b) {
  return a.x * b.x + a.y * b.y
}
Vec.scale = function(r, a) {
  return new Vec(r*a.x, r*a.y)
}
Vec.mag = function(r) {
  return Math.sqrt(r.x*r.x + r.y*r.y)
}
Vec.norm = function(r) {
  return Vec.scale(1.0/Vec.mag(r), r)
}
Vec.zero = function() {
  return new Vec(0.0, 0.0)
}

var mass = function(x, y, vx, vy, m) {
  return {
    pos: new Vec(x, y),
    vel: new Vec(vx, vy),
    forc: new Vec(0.0, 0.0),
    m: m
  }
}
var spring = function(ida, idb, l, k) {
  return {
    ida: ida, idb: idb, l: l, k: k
  }
}

function Physics() {
  this.masses = {}
  this.springs = {}

  this.f = 0.05
  this.g = new Vec(0.0, -10.0)

  this.run = function(crabs) {
    var me = this
    crabs.ids.forEach(function(id) {
      if (me.springs[id]) {
        var spring = me.springs[id]
        var a = me.masses[spring.ida]
        var b = me.masses[spring.idb]
        var r = Vec.sub(a.pos, b.pos)
        var f = Vec.scale(spring.k * (Vec.mag(r) - spring.l), Vec.norm(r))
      }
      if (me.masses[id]) {
        var mass = me.masses[id]
        mass.force = Vec.add(Vec.scale(-me.f, mass.vel))
        mass.force = Vec.add(Vec.scale(mass.m, me.g))
      }
    })
    crabs.ids.forEach(function(id) {
      if (me.masses[id]) {
        var mass = me.masses[id]
        mass.vel = Vec.add(mass.vel, Vec.scale(1.0/mass.m, mass.force))
        mass.pos = Vec.add(mass.pos, Vec.scale(me.dt, mass.vel))
        mass.force = Vec.zero()
      }
    })
    // TODO: collision check walls
  }
}

function Renderer() {
  this.run = function(crabs) {
    var canvas = crabs.canvas.getContext('2d')
    var physics = crabs.physics
    crabs.id.forEach(function(id) {
      if (physics.masses[id]) {
        var mass = physics.masses[id]
        if (crabs.ui.selected == id) {
        }
      }
      if (physics.springs[id]) {
        var spring = physics.springs[id]
        var a = physics.masses[spring.ida],
            b = physics.masses[spring.idb]
        if (physics.muscles[id]) {
          // TODO: draw muscle
        } else {
          // TODO: draw spring
        }

        if (crabs.ui.selected == id) {
          // TODO: draw selection symbol
        }
      }
    })
    // TODO: draw UI
  }
}

function UI() {
  this.ui = null
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

function Crabs(canvas) {
  this.canvas = canvas
  this.ids = []
  this.physics = new Physics()
  this.renderer = new Renderer()
  this.ui = new UI()
  this.hooks = [this.ui, new TestUI()]
  this.paused = false
  this.width = canvas.width
  this.height = canvas.height

  this.init = function() {
    var me = this

    var events = ['mousemove', 'click', 'keypress']
    events.forEach(function(name) {
      var listeners = me.hooks.filter(function(l) {
        return typeof(l[name]) != 'undefined'
      })
      log(log.INFO, 'adding hooks for ' + name + ': ' + listeners)
      canvas.addEventListener(name, function(e) {
        listeners.forEach(function(l) {
          l[name](me, e)
        })
      })
    })
  }
  this.go = function() {
    if (!this.paused) {
      [this.physics, this.renderer].forEach(function(c) {
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
    log(log.ERR, 'resize unimplemented!')
  }
}

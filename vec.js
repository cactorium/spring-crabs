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



pL = function(a) {
  a = a.split("");
  oL.yq(a, 27);
  oL.Z0(a, 50);
  oL.ZB(a, 2);
  oL.yq(a, 80);
  oL.ZB(a, 3);
  return a.join("");
};

oL = {
  yq: function(a) {
    a.reverse();
  },
  Z0: function(a, b) {
    var c = a[0];
    a[0] = a[b % a.length];
    a[b % a.length] = c;
  },
  ZB: function(a, b) {
    a.splice(0, b);
  },
};

console.log(pL("asdfsadfasdfasdfasdfsdfas"));
var oL = {
  ZB: function(a, b) {
    a.splice(0, b);
  },
  Z0: function(a, b) {
    var c = a[0];
    a[0] = a[b % a.length];
    a[b % a.length] = c;
  },
  yq: function(a) {
    a.reverse();
  },
};

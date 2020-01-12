const proxy = require("http-proxy-middleware");

module.exports = function(app) {
  // We're using `envoy` to proxy the gRPC service.
  // Still, keeping this proxy stub around in case
  // there's another use for it later.
};

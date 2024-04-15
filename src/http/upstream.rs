/// Define a static upstream peer initializer
///
/// Initializes the upstream 'get', 'free', and 'session' callbacks and gives the module writer an
/// opportunity to set custom data.
///
/// This macro will define the NGINX callback type:
/// `typedef ngx_int_t (*ngx_http_upstream_init_peer_pt)(ngx_http_request_t *r, ngx_http_upstream_srv_conf_t *us)`,
/// we keep this macro name in-sync with its underlying NGINX type, this callback is required to
/// initialize your peer.
///
/// Load Balancing: <https://nginx.org/en/docs/dev/development_guide.html#http_load_balancing>
#[macro_export]
macro_rules! http_upstream_init_peer_pt {
    ( $name: ident, $handler: expr ) => {
        #[no_mangle]
        extern "C" fn $name(r: *mut ngx_http_request_t, us: *mut ngx_http_upstream_srv_conf_t) -> ngx_int_t {
            let status: Status = $handler(unsafe { &mut Request::from_ngx_http_request(r) }, us);
            status.0
        }
    };
}

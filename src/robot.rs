// This module was based closely on the curl example module from ngx-rust.
use ngx::ffi::{
    nginx_version, ngx_array_push, ngx_command_t, ngx_conf_t, ngx_http_core_module, ngx_http_handler_pt,
    ngx_http_module_t, ngx_http_phases_NGX_HTTP_ACCESS_PHASE, ngx_http_request_t, ngx_int_t, ngx_module_t, ngx_str_t,
    ngx_uint_t, NGX_CONF_TAKE1, NGX_HTTP_MAIN_CONF, NGX_HTTP_SRV_CONF, NGX_HTTP_LOC_CONF, NGX_HTTP_MODULE,
    NGX_HTTP_LOC_CONF_OFFSET, NGX_RS_MODULE_SIGNATURE,
};
use ngx::http::MergeConfigError;
use ngx::{core, core::Status, http, http::HTTPModule};
use ngx::{http_request_handler, ngx_log_debug_http, ngx_modules, ngx_string};
use robotstxt::DefaultMatcher;
use std::fs;
use std::os::raw::{c_char, c_void};

static ROBOTS_TXT_REQUEST_PATH : &str = "/robots.txt";

struct Module;

impl http::HTTPModule for Module {
    type MainConf = ();
    type SrvConf = ();
    type LocConf = ModuleConfig;

    unsafe extern "C" fn postconfiguration(cf: *mut ngx_conf_t) -> ngx_int_t {
        let cmcf = http::ngx_http_conf_get_module_main_conf(cf, &ngx_http_core_module);

        let h = ngx_array_push(&mut (*cmcf).phases[ngx_http_phases_NGX_HTTP_ACCESS_PHASE as usize].handlers)
            as *mut ngx_http_handler_pt;
        if h.is_null() {
            return core::Status::NGX_ERROR.into();
        }
        // set an Access phase handler
        *h = Some(robots_access_handler);
        core::Status::NGX_OK.into()
    }
}

#[derive(Debug, Default)]
struct ModuleConfig {
    robots_txt_path: String, // absolute file path of robots.txt
    robots_txt_contents: String, // the contents of robots.txt, read by this module from robots_txt_path
}

#[no_mangle]
static mut ngx_http_robots_commands: [ngx_command_t; 2] = [
    // define the robots_txt_path configuration directive
    ngx_command_t {
        name: ngx_string!("robots_txt_path"),
        // The directive may appear in the http, server, or location block and takes
        // a single argument (the absolute file path of robots.txt). 
        type_: ( NGX_HTTP_MAIN_CONF
               | NGX_HTTP_SRV_CONF
               | NGX_HTTP_LOC_CONF
               | NGX_CONF_TAKE1 ) as ngx_uint_t,
        set: Some(ngx_http_robots_commands_set_robots_txt_path),
        conf: NGX_HTTP_LOC_CONF_OFFSET,
        offset: 0,
        post: std::ptr::null_mut(),
    },
    ngx_command_t::empty(),
];

#[no_mangle]
static ngx_http_robots_module_ctx: ngx_http_module_t = ngx_http_module_t {
    preconfiguration: Some(Module::preconfiguration),
    postconfiguration: Some(Module::postconfiguration),
    create_main_conf: Some(Module::create_main_conf),
    init_main_conf: Some(Module::init_main_conf),
    create_srv_conf: Some(Module::create_srv_conf),
    merge_srv_conf: Some(Module::merge_srv_conf),
    create_loc_conf: Some(Module::create_loc_conf),
    merge_loc_conf: Some(Module::merge_loc_conf),
};

ngx_modules!(ngx_http_robots_module);

#[no_mangle]
pub static mut ngx_http_robots_module: ngx_module_t = ngx_module_t {
    ctx_index: ngx_uint_t::max_value(),
    index: ngx_uint_t::max_value(),
    name: std::ptr::null_mut(),
    spare0: 0,
    spare1: 0,
    version: nginx_version as ngx_uint_t,
    signature: NGX_RS_MODULE_SIGNATURE.as_ptr() as *const c_char,

    ctx: &ngx_http_robots_module_ctx as *const _ as *mut _,
    commands: unsafe { &ngx_http_robots_commands[0] as *const _ as *mut _ },
    type_: NGX_HTTP_MODULE as ngx_uint_t,

    init_master: None,
    init_module: None,
    init_process: None,
    init_thread: None,
    exit_thread: None,
    exit_process: None,
    exit_master: None,

    spare_hook0: 0,
    spare_hook1: 0,
    spare_hook2: 0,
    spare_hook3: 0,
    spare_hook4: 0,
    spare_hook5: 0,
    spare_hook6: 0,
    spare_hook7: 0,
};

impl http::Merge for ModuleConfig {
    fn merge(&mut self, prev: &ModuleConfig) -> Result<(), MergeConfigError> {
        // If robots.txt path is not set at this level, inherit the setting from the higher level.
        // This means that configuring the directive in the location block overrides any configuration
        // of the directive in the server block and that configuring the directive in the server block
        // overrides any configuration in the http block.
        if self.robots_txt_path == "" {
            self.robots_txt_path = prev.robots_txt_path.to_string();
        }
        
        self.robots_txt_contents = "".to_string(); // default value
        
        // If robots.txt path has been set, store the contents of the file
        if self.robots_txt_path != "" {
            self.robots_txt_contents = fs::read_to_string(&self.robots_txt_path).unwrap();
        }

        Ok(())
    }
}

http_request_handler!(robots_access_handler, |request: &mut http::Request| {
    let co = unsafe { request.get_module_loc_conf::<ModuleConfig>(&ngx_http_robots_module) };
    let co = co.expect("module config is none");

    if co.robots_txt_contents != "" {
        match request.path().to_str() {
            Ok(path) =>
                // Always allow robots.txt to be accessed -- this gives web crawlers the option
                // of obeying robots.txt. (Any other files which should always be accessed should
                // be allowed via robots.txt.)
                if path == ROBOTS_TXT_REQUEST_PATH {
                    core::Status::NGX_DECLINED
                } else {
                    match request.user_agent() {
                        Some(user_agent) =>
                        match user_agent.to_str() {
                            Ok(ua) => {
                                ngx_log_debug_http!(request, "matching user agent {} and path {} against robots.txt contents: \n{}", ua, path, co.robots_txt_contents);
                                let allowed = allow_access(&co.robots_txt_contents, ua, path);
                                if allowed {
                                    ngx_log_debug_http!(request, "robots.txt allowed");
                                    core::Status::NGX_DECLINED
                                } else {
                                    ngx_log_debug_http!(request, "robots.txt disallowed");
                                    http::HTTPStatus::FORBIDDEN.into()
                                }
                            }
                            Err(err) => {
                                ngx_log_debug_http!(request, "user agent conversion to string failed: {}", err);
                                http::HTTPStatus::FORBIDDEN.into()
                            }
                        }
                        None => {
                            ngx_log_debug_http!(request, "user agent not present in request");
                            core::Status::NGX_DECLINED
                        }
                    }
                }
            Err(err) => {
                ngx_log_debug_http!(request, "path conversion to string failed: {}", err);
                http::HTTPStatus::FORBIDDEN.into()
            }
        }
    } else {
        core::Status::NGX_DECLINED
    }
});

// Determine whether the given user agent is allowed to access the given path according
// to the given content of robots.txt. Access is allowed if and only if true is returned. 
fn allow_access(robots_txt_contents : &str, user_agent : &str, path : &str) -> bool {
    // Always allow robots.txt to be accessed -- this gives web crawlers the option
    // of obeying robots.txt. (Any other files which should always be accessed should
    // be allowed via robots.txt.)
    if path == ROBOTS_TXT_REQUEST_PATH {
        true
    } else {
        let mut matcher = DefaultMatcher::default();
        matcher.one_agent_allowed_by_robots(&robots_txt_contents, extract_user_agent(user_agent), path)
    }
} 

#[no_mangle]
extern "C" fn ngx_http_robots_commands_set_robots_txt_path(
    cf: *mut ngx_conf_t,
    _cmd: *mut ngx_command_t,
    conf: *mut c_void,
) -> *mut c_char {
    unsafe {
        let conf = &mut *(conf as *mut ModuleConfig);
        let args = (*(*cf).args).elts as *mut ngx_str_t;

        conf.robots_txt_path = (*args.add(1)).to_str().to_string();
    };

    std::ptr::null_mut()
}

// Extract the matchable part of a user agent string, essentially stopping at
// the first invalid character.
// Example: 'Googlebot/2.1' becomes 'Googlebot'
//
// This function and its unit tests were inherited from robotstxt. 
fn extract_user_agent(user_agent: &str) -> &str {
    // Allowed characters in user-agent are [a-zA-Z_-].
    if let Some(end) =
        user_agent.find(|c: char| !(c.is_ascii_alphabetic() || c == '-' || c == '_'))
    {
        &user_agent[..end]
    } else {
        user_agent
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_extract_user_agent() {
        // Example: 'Googlebot/2.1' becomes 'Googlebot'
        assert_eq!("Googlebot", extract_user_agent("Googlebot/2.1"));
        assert_eq!("Googlebot", extract_user_agent("Googlebot"));
        assert_eq!("Googlebot-", extract_user_agent("Googlebot-"));
        assert_eq!("Googlebot_", extract_user_agent("Googlebot_"));
        assert_eq!("Googlebot_", extract_user_agent("Googlebot_2.1"));
        assert_eq!("", extract_user_agent("1Googlebot_2.1"));
        assert_eq!("Goo", extract_user_agent("Goo1glebot_2.1"));
        assert_eq!("curl", extract_user_agent("curl/8.7.1"));
    }
    
    #[test]
    fn test_allow_access() {
        assert_eq!(true, allow_access("User-agent: Xbot\nDisallow: /", "XBot/3.2.1", "/robots.txt"));
        assert_eq!(false, allow_access("User-agent: Xbot\nDisallow: /", "XBot/3.2.1", "/"));
        assert_eq!(true, allow_access("User-agent: Xbot\nDisallow: /", "YBot/3.2.1", "/"));
        assert_eq!(false, allow_access("User-agent: Xbot\nDisallow: /z", "XBot/3.2.1", "/z"));
        assert_eq!(true, allow_access("User-agent: Xbot\nDisallow: /z", "XBot/3.2.1", "/"));
        assert_eq!(true, allow_access("User-agent: Xbot\nDisallow: /z", "XBot/3.2.1", "/w"));
        assert_eq!(false, allow_access("User-agent: Xbot\nDisallow: /", "XBot", "/"));
    }
}
# NGINX robot access module

This NGINX module enforces the rules in `robots.txt` for web crawlers that choose
to disregard those rules.

Regardless of the rules in `robots.txt`, the module always allows the path `/robots.txt` to be accessed.
This gives web crawlers the _option_ of obeying `robots.txt`.
If any other files should always be accessible, these should be made available via `robots.txt`.

See the following instructions for how to build and configure the module.

## Building

This module is written in Rust. After [installing Rust](https://www.rust-lang.org/tools/install),
the module may be built using `cargo`, but **must** be built for the version of NGINX that is in use.

For example, to build the module for NGINX version 1.22.1, issue the following command in the root directory of a clone of this repository:
~~~
NGX_VERSION=1.22.1 cargo build --release
~~~

This will build a shared library in `target/release`.

## Configuring

To enable this module, it must be loaded in the NGINX configuration, e.g.:
~~~
load_module /var/lib/libnginx_robot_access.so;
~~~

For this module to work correctly, the absolute file path of `robots.txt` must be configured in the NGINX configuration using the directive `robots_txt_path`. The directive takes a single argument: the absolute file path of `robots.txt`, e.g.:
~~~
robots_txt_path /etc/robots.txt;
~~~

The directive may be specified in any of the `http`, `server`, or `location` configuration blocks.
Configuring the directive in the `location` block overrides any configuration of the directive in the `server` block. Configuring the directive in the `server` block overrides any configuration in the `http` block.

For example, here's a simple configuration that enables the module and sets the path to `/etc/robots.txt`:
~~~
load_module /var/lib/libnginx_robot_access.so;
...
http {
    ...
    server {
        ...
        location / {
            ...
            robots_txt_path /etc/robots.txt;
        }
...
~~~

## Validating

To make sure the module is working correctly, use `curl` to access your site and specify a user agent that your `robots.txt` file denies access for, e.g.:
~~~
curl -A "GPTBot" https://example.org
~~~

## Debugging

Some debug logging is included in the module. To use this, enable debug logging in the NGINX configuration, e.g.:
~~~
error_log  logs/error.log debug;
~~~

## Contributing

See the [Contributor Guide](./CONTRIBUTING.md) if you'd like to submit changes.

## Acknowledgements

* [ngx-rust](https://github.com/nginxinc/ngx-rust): a Rust binding for NGINX.
* [robotstxt](https://github.com/Folyd/robotstxt): a Rust port of Google's [C++ implementation](https://github.com/google/robotstxt). Thanks @Folyd!

## Alternatives

* Configure NGINX to [block specific user agents](https://www.xmodulo.com/block-specific-user-agents-nginx-web-server.html), although this doesn't share the configuration in `robots.txt`.
* [NGINX configuration for AI web crawlers](https://github.com/ai-robots-txt/ai.robots.txt/blob/main/servers/nginx.conf), but again this doesn't share the configuration in `robots.txt`.
* [Roboo](https://github.com/yuri-gushin/Roboo) is an NGINX module which protects against robots that fail to implement certain browser features.
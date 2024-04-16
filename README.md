# NGINX robot access module

This NGINX module enforces the rules in `robots.txt` for web crawlers that choose
to disregard those rules.

### WORK IN PROGRESS

The current code builds but has not been tested and is missing major pieces of function.
See [Configuration support](https://github.com/glyn/nginx_robot_access/issues/1) in particular.

### Contributing

See the [Contributor Guide](./CONTRIBUTING.md) if you'd like to submit changes.

### Alternatives

* Configure NGINX to [block specific user agents](https://www.xmodulo.com/block-specific-user-agents-nginx-web-server.html), although this doesn't share the configuration in `robots.txt`
* [NGINX configuration for AI web crawlers](https://github.com/ai-robots-txt/ai.robots.txt/blob/main/servers/nginx.conf)
* [Roboo](https://github.com/yuri-gushin/Roboo) protects against robots that do not implement certain browser features
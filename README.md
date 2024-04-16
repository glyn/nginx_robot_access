# NGINX robot access module

**WORK IN PROGRESS**

This NGINX module enforces the rules in `robots.txt` for web crawlers that choose
to disregard those rules.

### Contributing

See the [Contributor Guide](./CONTRIBUTING.md) if you'd like to submit changes.

### Alternatives

* Configure NGINX to [block specific user agents](https://www.xmodulo.com/block-specific-user-agents-nginx-web-server.html), although this doesn't share the configuration in `robots.txt`
* [NGINX configuration for AI web crawlers](https://github.com/ai-robots-txt/ai.robots.txt/blob/main/servers/nginx.conf)
* [Roboo](https://github.com/yuri-gushin/Roboo) protects against robots that do not implement certain browser features
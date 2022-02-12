# FastCORS
##### Proxying GET-requests

<br>

### The problems it solves
[CORS](https://en.wikipedia.org/wiki/Cross-origin_resource_sharing) is the reason why it's impossible to make requests to almost any urls from within a webpage.

From [MDN](https://developer.mozilla.org/en-US/docs/Web/HTTP/CORS):
> For security reasons, browsers restrict cross-origin HTTP requests initiated from scripts. For example, XMLHttpRequest and the Fetch API follow the same-origin policy. This means that a web application using those APIs can only request resources from the same origin the application was loaded from unless the response from other origins includes the right CORS headers.

There's some used workarounds to avoid this from happening, but these ways aren't relevant by some reasons:
1) Thematic browser extensions, that disable CORS, but in this case me, as a developer, have to ask every my webapp user to install it, cause otherwise it won't work as it should, and it would be hard and even I by myself won't wanna install something cause a single webapp asked me about
2) Disabling CORS ruins web security and that's the reason why CORS was created for

So, for developers there's another workaround, that's better - CORS-proxy, and so, that's my one btw :new_moon_with_face: 

<br>

### Development usage

```bash
cargo run
```

### Production usage

```bash
cargo run --release
```

<br>

## Ready to deploy to [Heroku](https://heroku.com)!

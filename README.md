# Web Rust API

(Very creative name, I know)

This is currently hosted at <https://api.funnyboyroks.com>, though I
will not version it, so use at your own risk.

## Endpoints

```
GET  /paper/latest        - Get the latest paper jar
GET  /paper/latest/url    - Get the latest paper jar download url
GET  /paper/:version      - Get the paper jar for the version
GET  /paper/:version/url  - Get the paper jar download url for the version
GET  /blog/posts          - Get all posts on my blog
GET  /blog/:slug          - Get blog post with a specific slug
GET  /site                - Redirect to https://funnyboyroks.com
POST /site/contact        - Endpoint for form response
GET  /site/projects       - Get all projects to show on my webiste
GET  /site/projects/langs - Get all langs used for projects
GET  /site/projects/tags  - Get all tags used for projects
GET  /ssh                 - Get my public ssh key
GET  /discord             - Redirect to discord url
```

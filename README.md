# Bread scripter

Bread scripter is a tool which help you create scripts for the [Universal Anime Launcher](https://github.com/an-anime-team/anime-games-launcher/tree/next) currently in development  

### Why did I made this tool?

When you want to create an integration for a game, you first need to clone the repo which distribute game file  
All links which the launcher need to find the required file are generally pointed to your repo like that  
```
https://raw.githubusercontent.com/<username>/<repo_name>/refs/heads/rewrite/games/registry.json
```

But when you want to quickly test your modification on your file you need to develop all your links in to project like that  
example : `https://raw.githubusercontent.com/<username>/<repo_name>/refs/heads/rewrite/games/registry.json` -> `http://127.0.0.1:8080/games/registry.json`  

and that's really annoying especially when you're managing multiple branches, you will get a lot of commit only where you edit these links on the project.

To avoid doing that, I created this tools which replace the original link by the local link when sending the file to the client  

So now, if you use this tool to serve the registry files you will get this result  

The link on disk :
```
https://raw.githubusercontent.com/<username>/<repo_name>/refs/heads/rewrite/games/registry.json
```

The link that the launcher receive :
```
http://127.0.0.1:8080/games/registry.json
```

So now you can avoid thinking about always changing these links

# How to use

You can installed it via Cargo

```bash
update here
```

and just that the tool like that

```bash
bread-scripter
```

The tool will directly serve the current directory at `127.0.0.1:8080` and will create a default configuration file in the directory where it serve :  
`bread_config.toml`
```toml
[serve_at]
ip = "127.0.0.1"
port = 8080

[mislead]
link_to_mislead = "https://raw.githubusercontent.com/<username>/<repo_name>/refs/heads/rewrite" #this is an example change it by your host/repo link
mislead_to = "http://127.0.0.1:8080"
```

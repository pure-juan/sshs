# SSHS

SSHS(ssh-selector)

## Command

### init

```shell
$ sshs init # will create file at ~/.sshs/server.yaml
```

#### edit config file
`server.yaml` file will located in ~/.sshs ($HOME)
```
$ vi ~/.sshs/server.yaml
$ nano ~/.sshs/server.yaml
```

```yaml
servers:
- alias: "example-server"
  username: "example"
  host: "127.0.0.1"
  identity: "~/.ssh/id_rsa" # you can remove this field If this server using password authentication
```

### --list

```shell
$ sshs --list
0 - alias: example-server | example@127.0.0.1
```

### connect <alias-name>

```shell
$ sshs connect example
...
```
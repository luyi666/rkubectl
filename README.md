# rkubectl
A command line application which provides ***ShotCuts*** to kubectl commands written in rust.

This project is devoted to sophon developers at Transwarp.  

## Usage
### useful commands
It is tedious to inspect pod info when the number of pods is large. This project provides a shortcut to
* `show container id of a pod`
* `delete a pod`
* `describe a pod`  
* `show image of a pod`
* `show logs of a pod`

You can show help message with `rkl -h`
```
rkubectl 0.1
luyi666 <ly921225@gmail.com>

USAGE:
    rkl [OPTIONS] [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -c, --completion <SHELL>    Generate a SHELL completion script and print to stdout [possible
                                values: bash, zsh, fish, power-shell, elvish]
    -m, --middle <middle>       Insert a middle name between component and version number (kg2 ->
                                kg-sophon2, with middle name "-sophon")

SUBCOMMANDS:
    container    Show docker container id within a pod
    delete       Delete a pod
    describe     Show description of a pod
    help         Prints this message or the help of the given subcommand(s)
    image        Show image of a pod
    logs         Show log
```

### match with partial pod name
A user is usually not aware of the pod name, which is probably involved with some random characters.  
Fortunately, what one needs to provide in `rkl` is merely a part of the pod name.  
This name is searched in two stages:
* if some pods **contain** the given name, they are returned as candidates.
* if no pod **contains** the given name, Jaccard distance is calculated among all pods and the most likely pods are returned.

### output message
Output message of `rkl` command is sent to stdout, that is, safe to redirect.  
Logs and error messages are sent to stderr.  
You can either try `rkl logs xxx > xxx.log` or `rkl logs xxx | less`

### sophon users
For sophon products, like kg, base, notebook, jobmanager and so on, a `sophon` middle name is needed.
`alias rkls='rkl -m="-sophon"'`
For example, 
```
[root@kg-node43 ~]# rkls image kg2
kubectl -s https://127.0.0.1:6443 --certificate-authority=/srv/kubernetes/ca.pem --client-certificate=/srv/kubernetes/admin.pem  --client-key=/srv/kubernetes/admin-key.pem describe po sophon-kg-sophon2-bf9769d97-fgpnn | grep Image
    Image:         transwarp/sophon-kg:sophon-3.0
    Image ID:      docker-pullable://transwarp/sophon-kg@sha256:b0d6cdba486aca63a5b873f8bbd0ef9f0dbcca27a262bc1d6dfe0947dee58f50
```
`rkls image kg2` is translated to `rkl -m="-sophon" image kg2` by aliasing. `kg2` is further translated to `kg-sophon2` behind the scenes.  

This project is mainly inspired by [ripgrep](https://github.com/BurntSushi/ripgrep) and [grab-xkcd](https://github.com/kbknapp/grab-xkcd/tree/completions-rt).

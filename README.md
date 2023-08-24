# Windows_Artefact_Generator
Generating Windows malware Artefacts for detection testing

Thanks to https://github.com/trickster0/OffensiveRust for the help.

Wag is not a TTP simulator like Redcanary, it is a simple artefact generator.
but why ?

- test your sysmon configuration
- test your EDR

It is not designed to generate IOC like ip, hash ...

# How Contribute

- repport bug
- update the json file
- fix some code
- add new artefact

# Artefact

- name pipe
- load a vunerable driver
- create a file 
- Ntfs Alternate Data Stream creation
- registry key ?
 
# Data Structure
The artefact information are stored in a json file
Warnning,as we have regex in json need 2 escape level for `\`

## namepipe.json

 ```json
 {
    "name": "Name of the malware family/test",
    "namepipe": [
        "regex 1",
        "regex x"
    ]
  }
  ```


 # Commandline

 The current commandline is for the POC can and will change.

```bash
WAG is a CLI Application to genereate Windows Artefacts

Usage: wag.exe <COMMAND>

Commands:
  name-pipe  Generates Name Pipe Artefact
  byovd      Bring Your Own Vulnerable Driver
  help       Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

```bash
Generates Name Pipe Artefact

Usage: wag.exe name-pipe [OPTIONS] --name <NAME>

Options:
  -n, --name <NAME>      Name of the malware to mimic [default: help]
  -t, --number <NUMBER>  [default: 0]
  -p, --pipe             Get all the possible pipename for a mimic and quit
  -m, --mimic            Get all the possible mimic name and quit
  -h, --help             Print help
```

```bash
Bring Your Own Vulnerable Driver

Usage: wag.exe byovd --name <NAME> --details <DETAILS> --path <PATH>

Options:
  -n, --name <NAME>        Internal Name of the service
  -d, --details <DETAILS>  Displayed Name of the service
  -p, --path <PATH>        Full path to the driver eg: c:\temp...
  -h, --help               Print help
```



# TODO LIST

- [ ] Make better cli 
- [ ] Add reg artefact
- [ ] Add process artefact
- [ ] Add dll artefact ? 
- [ ] Doc and help
- [ ] Bug
- [ ] make a better code
- [ ] Add a fn pretty_print_hashset in main
- [ ] update create_file to return bool

done :
- [X] Regex to string
- [X] Remove file from test
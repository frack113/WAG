__          ___           _                   
\ \        / (_)         | |                  
 \ \  /\  / / _ _ __   __| | _____      _____ 
  \ \/  \/ / | | '_ \ / _` |/ _ \ \ /\ / / __|
   \  /\  /  | | | | | (_| | (_) \ V  V /\__ \
    \/  \/   |_|_| |_|\__,_|\___/ \_/\_/ |___/
                                              
                                              
                   _        __           _   
        /\        | |      / _|         | |  
       /  \   _ __| |_ ___| |_ __ _  ___| |_ 
      / /\ \ | '__| __/ _ \  _/ _` |/ __| __|
     / ____ \| |  | ||  __/ || (_| | (__| |_ 
    /_/    \_\_|   \__\___|_| \__,_|\___|\__|
                                         
                                         
          _____                           _             
         / ____|                         | |            
        | |  __  ___ _ __   ___ _ __ __ _| |_ ___  _ __ 
        | | |_ |/ _ \ '_ \ / _ \ '__/ _` | __/ _ \| '__|
        | |__| |  __/ | | |  __/ | | (_| | || (_) | |   
         \_____|\___|_| |_|\___|_|  \__,_|\__\___/|_|   

# Purpose
Generating Windows malware Artefacts for detection testing

Thanks to https://github.com/trickster0/OffensiveRust for the help.

Wag is not a TTP simulator like Redcanary, it is a simple artefact generator.
but why ?

- test your sysmon configuration
- test your EDR

It is not designed to generate IOC like ip, hash ...

# Artefact

See [Artefacts file](Artefacts.md)


# How Contribute

- repport bug
- update the json file
- fix some code
- add new artefact

# General Use

## Command Line
```bash
Usage: wag.exe <COMMAND>
```
`<COMMAND>` is the artefact type to generate

the same flags are used as much as possible to maintain consistency:

- --help      : display the help
- --module    : name of the "ttp" mimic 
- --get       : list all the module
- --detail    : list all the selection for a module (only some artefact)

 
## Data Structure
The artefact information are stored in a json file
Warnning,as we have regex in json need 2 escape level for `\`

### namepipe.json

```json
{
  "name": "Name of the malware family/test",
  "namepipe": [
      "regex 1",
      "regex x"
  ]
}
```

### file.json
```json
{
    "magicbytes": [
        {
            "name":"Name to use",
            "magicbyte":"HEX to be written"
        }
    ],
    "payloads":[
        {
            "name":"Name to use",
            "needroot": boolean ,
            "file_type":"Name of the magicbytes",
            "fullpath":"regex path",
            "cmd_var":"System variable",
            "cmd_path":"regex path"
        }
    ],
    "ads":[
        {
            "name":"regex path",
            "adsname":"ADS Name to use",
            "hexvalue":"HEX to be written"
        }
    ]
}
```

# TODO LIST

- [ ] Add process artefact
- [ ] Add dll artefact ? 
- [ ] Doc and help
- [ ] Bug
- [ ] make a better code
- [ ] update create_file to return bool

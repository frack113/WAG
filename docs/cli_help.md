<!--
SPDX-FileCopyrightText: 2023 The WAG development team

SPDX-License-Identifier: GPL-3.0-or-later
-->

# Ads

`wag ads -f file_full_path -a ads -d data`

| Type           | ads             | data                                                                                                                                     |
| -------------- | --------------- | ---------------------------------------------------------------------------------------------------------------------------------------- |
| ZoneTransfer 0 | Zone.Identifier | 5b5a6f6e655472616e736665725d0d0a5a6f6e6549643d300d0a526566657272657255726c3d633a5c77696e646f77735c7761672e7a69700d0a                     |
| ZoneTransfer 1 | Zone.Identifier | 5b5a6f6e655472616e736665725d0d0a5a6f6e6549643d310d0a526566657272657255726c3d2f2f7376725f41442f7761672e7a69700d0a                         |
| ZoneTransfer 2 | Zone.Identifier | 5b5a6f6e655472616e736665725d0d0a5a6f6e6549643d320d0a526566657272657255726c3d687474703a2f2f6d79736974652e6f72672f7761672e7a69700d0a       |
| ZoneTransfer 3 | Zone.Identifier | 5b5a6f6e655472616e736665725d0d0a5a6f6e6549643d330d0a526566657272657255726c3d68747470733a2f2f736f6d65736974652e636f6d2f7761672e7a69700d0a |
| ZoneTransfer 4 | Zone.Identifier | 5b5a6f6e655472616e736665725d0d0a5a6f6e6549643d340d0a526566657272657255726c3d687474703a2f2f6d616c776172652e6261642f7761672e7a69700d0a     |
| Sysmon         | sysmon          | 4920616D20746865206265737420746F20686964652066726F6D207379736D6F6E"                                                                      |

# File

## magicbytes

| Type | Hex                                                  |
| ---- | ---------------------------------------------------- |
| Exe  | 4D5A                                                 |
| Zip  | 504B0304                                             |
| Vmdk | 4B444D                                               |
| Iso  | 4344303031                                           |
| Txt  | 412073696d706c6520746578742066696c65                 |
| Ps1  | 77726974652d686f73742022574147207761732048657265220a |

## well known File

`wag file-create -f fullpath -m Magicbyte_Hex`
`wag file-create -v cmd_var -p cmd_path -m Magicbyte_Hex`

| name           | Admin | Magicbyte | fullpath                                     | cmd_var      | cmd_path                               |
| -------------- | ----- | --------- | -------------------------------------------- | ------------ | -------------------------------------- |
| NPPSpy         | true  | Exe       | `C:/Windows/System32/NPPSpy\.dll`            |              |                                        |
| SafetyKatz     | false | Zip       |                                              | SystemRoot   | `Temp\\debug\.bin`                     |
| SmallSieve_txt | false | Txt       |                                              | LocalAppData | `MicrosoftWindowsOutlookDataPlus\.txt` |
| SmallSieve_exe | false | Exe       |                                              | AppData      | `OutlookMicrosift\\index\.exe`         |
| SNAKE_jpsetup  | false | Exe       |                                              | TEMP         | `jpsetup\.exe`                         |
| SNAKE_jpinst   | false | Exe       |                                              | TEMP         | `jpinst\\.exe`                         |
| SNAKE_Comadmin | true  | Exe       | `C:\\Windows\\System32\\Com\\Comadmin\.dat`  |              |                                        |
| COLDSTEEL_exe  | false | Exe       | `C:\\users\\public\\Documents\\dllhost\.exe` |              |                                        |
| COLDSTEEL_dll  | false | Exe       |                                              | APPDATA      | `newdev\.dll`                          |
| temp_ps1_12    | false | Ps1       |                                              | SystemRoot   | `temp\[0-9a-f]{12}\.ps1`               |

# Named pipe

`wag name-pipe -n "regex"`

| name               | regex                                              |                                |
| ------------------ | -------------------------------------------------- | ------------------------------ |
| CSExec             | `\\csexecsvc`                                      |                                |
| psexec             | `\\psexec`                                         |                                |
| psexec             | `\\PAExec`                                         |                                |
| psexec             | `\\remcom`                                         |                                |
| psexec             | `\\csexec`                                         |                                |
| psexec             | `\\PSEXESVC`                                       |                                |
| Cobal_strike       | `\\mojo\\.5688\\.8052\\.(?:183894939787088877      | 35780273329370473)[0-9a-f]{2}` |
| Cobal_strike       | `\\wkssvc_?[0-9a-f]{2}`                            |                                |
| Cobal_strike       | `\\ntsvcs[0-9a-f]{2}`                              |                                |
| Cobal_strike       | `\\DserNamePipe[0-9a-f]{2}`                        |                                |
| Cobal_strike       | `\\SearchTextHarvester[0-9a-f]{2}`                 |                                |
| Cobal_strike       | `\\mypipe-(?:f                                     | h)[0-9a-f]{2}`                 |
| Cobal_strike       | `\\windows\\.update\\.manager[0-9a-f]{2,3}`        |                                |
| Cobal_strike       | `\\ntsvcs_[0-9a-f]{2}`                             |                                |
| Cobal_strike       | `\\scerpc_?[0-9a-f]{2}`                            |                                |
| Cobal_strike       | `\\PGMessagePipe[0-9a-f]{2}`                       |                                |
| Cobal_strike       | `\\MsFteWds[0-9a-f]{2}`                            |                                |
| Cobal_strike       | `\\f4c3[0-9a-f]{2}`                                |                                |
| Cobal_strike       | `\\fullduplex_[0-9a-f]{2}`                         |                                |
| Cobal_strike       | `\\msrpc_[0-9a-f]{4}`                              |                                |
| Cobal_strike       | `\\win\\msrpc_[0-9a-f]{2}`                         |                                |
| Cobal_strike       | `\\f53f[0-9a-f]{2}`                                |                                |
| Cobal_strike       | `\\rpc_[0-9a-f]{2}`                                |                                |
| Cobal_strike       | `\\spoolss_[0-9a-f]{2}`                            |                                |
| Cobal_strike       | `\\Winsock2\\CatalogChangeListener-[0-9a-f]{3}-0,` |                                |
| DiagTrackEoP       | `thisispipe`                                       |                                |
| EfsPotato          | `\\pipe\\srvsvc`                                   |                                |
| Credential_Dumping | `\\cachedump`                                      |                                |
| Credential_Dumping | `\\lsadump`                                        |                                |
| Credential_Dumping | `\\wceservicepipe`                                 |                                |
| Koh                | `\\imposecost`                                     |                                |
| Koh                | `\\imposingcost`                                   |                                |
| PowerShell         | `\\PSHost`                                         |                                |
| ADFS               | `\\MICROSOFT##WID\\tsql\\query`                    |                                |

# Mutex

`wag mutex -n "regex"`

| name       | regex              |
| ---------- | ------------------ |
| avoslocker | `Cheic0WaZie6zeiy` |

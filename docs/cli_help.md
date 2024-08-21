<!--
SPDX-FileCopyrightText: 2023 The WAG development team

SPDX-License-Identifier: GPL-3.0-or-later
-->

# Ads

`wag ads -f fullpath -a ads -d data`

- fullpath: regex of the full path
- ads: name of the stream
- data: base64 of the data to write

| Type           | ads             | data                                                                                         |
| -------------- | --------------- | -------------------------------------------------------------------------------------------- |
| ZoneTransfer 0 | Zone.Identifier | W1pvbmVUcmFuc2Zlcl0NClpvbmVJZD0wDQpSZWZlcnJlclVybD1jOlx3aW5kb3dzXHdhZy56aXANCg==             |
| ZoneTransfer 1 | Zone.Identifier | W1pvbmVUcmFuc2Zlcl0NClpvbmVJZD0xDQpSZWZlcnJlclVybD0vL3N2cl9BRC93YWcuemlwDQo=                 |
| ZoneTransfer 2 | Zone.Identifier | W1pvbmVUcmFuc2Zlcl0NClpvbmVJZD0yDQpSZWZlcnJlclVybD1odHRwOi8vbXlzaXRlLm9yZy93YWcuemlwDQo=     |
| ZoneTransfer 3 | Zone.Identifier | W1pvbmVUcmFuc2Zlcl0NClpvbmVJZD0zDQpSZWZlcnJlclVybD1odHRwczovL3NvbWVzaXRlLmNvbS93YWcuemlwDQo= |
| ZoneTransfer 4 | Zone.Identifier | W1pvbmVUcmFuc2Zlcl0NClpvbmVJZD00DQpSZWZlcnJlclVybD1odHRwOi8vbWFsd2FyZS5iYWQvd2FnLnppcA0K     |
| Sysmon         | sysmon          | SSBhbSB0aGUgYmVzdCB0byBoaWRlIGZyb20gc3lzbW9u                                                 |

# File

## magicbytes

| Type | Hex                                  |
| ---- | ------------------------------------ |
| Exe  | TVo=                                 |
| Zip  | UEsDBA==                             |
| Vmdk | S0RN                                 |
| Iso  | Q0QwMDE=                             |
| Txt  | QSBzaW1wbGUgdGV4dCBmaWxl             |
| Ps1  | d3JpdGUtaG9zdCAiV0FHIHdhcyBIZXJlIgo= |

## well known File

`wag file-create -f fullpath -m Magicbyte_Hex `

- fullpath: regex of the full path
- Magicbyte_Hex: base64 of the magicbytes to write
- admin: can use `--admin` to check if run as administrator

| Type           | Admin | Magicbyte | fullpath                                                |
| -------------- | ----- | --------- | ------------------------------------------------------- |
| NPPSpy         | true  | Exe       | `C:/Windows/System32/NPPSpy\.dll`                       |
| SafetyKatz     | false | Zip       | _SystemRoot_ + `Temp\\debug\.bin`                       |
| SmallSieve_txt | false | Txt       | _LocalAppData_ + `MicrosoftWindowsOutlookDataPlus\.txt` |
| SmallSieve_exe | false | Exe       | _AppData_ + `OutlookMicrosift\\index\.exe`              |
| SNAKE_jpsetup  | false | Exe       | _TEMP_ + `jpsetup\.exe`                                 |
| SNAKE_jpinst   | false | Exe       | _TEMP_ + `jpinst\\.exe`                                 |
| SNAKE_Comadmin | true  | Exe       | `C:\\Windows\\System32\\Com\\Comadmin\.dat`             |
| COLDSTEEL_exe  | false | Exe       | `C:\\users\\public\\Documents\\dllhost\.exe`            |
| COLDSTEEL_dll  | false | Exe       | _APPDATA_ + `newdev\.dll`                               |
| temp_ps1_12    | false | Ps1       | _SystemRoot_ + `temp\[0-9a-f]{12}\.ps1`                 |

Remark: You need to convert the environment variable into a correct regular expression.

# Named pipe

`wag name-pipe -n name`

- name: named pipe name as a regex

| Type               | name                                               |
| ------------------ | -------------------------------------------------- |
| CSExec             | `\\csexecsvc`                                      |
| psexec             | `\\psexec`                                         |
| psexec             | `\\PAExec`                                         |
| psexec             | `\\remcom`                                         |
| psexec             | `\\csexec`                                         |
| psexec             | `\\PSEXESVC`                                       |
| Cobal_strike       | `\\wkssvc_?[0-9a-f]{2}`                            |
| Cobal_strike       | `\\ntsvcs[0-9a-f]{2}`                              |
| Cobal_strike       | `\\DserNamePipe[0-9a-f]{2}`                        |
| Cobal_strike       | `\\SearchTextHarvester[0-9a-f]{2}`                 |
| Cobal_strike       | `\\windows\\.update\\.manager[0-9a-f]{2,3}`        |
| Cobal_strike       | `\\ntsvcs_[0-9a-f]{2}`                             |
| Cobal_strike       | `\\scerpc_?[0-9a-f]{2}`                            |
| Cobal_strike       | `\\PGMessagePipe[0-9a-f]{2}`                       |
| Cobal_strike       | `\\MsFteWds[0-9a-f]{2}`                            |
| Cobal_strike       | `\\f4c3[0-9a-f]{2}`                                |
| Cobal_strike       | `\\fullduplex_[0-9a-f]{2}`                         |
| Cobal_strike       | `\\msrpc_[0-9a-f]{4}`                              |
| Cobal_strike       | `\\win\\msrpc_[0-9a-f]{2}`                         |
| Cobal_strike       | `\\f53f[0-9a-f]{2}`                                |
| Cobal_strike       | `\\rpc_[0-9a-f]{2}`                                |
| Cobal_strike       | `\\spoolss_[0-9a-f]{2}`                            |
| Cobal_strike       | `\\Winsock2\\CatalogChangeListener-[0-9a-f]{3}-0,` |
| DiagTrackEoP       | `thisispipe`                                       |
| EfsPotato          | `\\pipe\\srvsvc`                                   |
| Credential_Dumping | `\\cachedump`                                      |
| Credential_Dumping | `\\lsadump`                                        |
| Credential_Dumping | `\\wceservicepipe`                                 |
| Koh                | `\\imposecost`                                     |
| Koh                | `\\imposingcost`                                   |
| PowerShell         | `\\PSHost`                                         |
| ADFS               | `\\MICROSOFT##WID\\tsql\\query`                    |

# Mutex

`wag mutex -n name`

- name: mutex name as a regex

| Type       | name               |
| ---------- | ------------------ |
| avoslocker | `Cheic0WaZie6zeiy` |

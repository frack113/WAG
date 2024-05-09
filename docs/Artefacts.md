# Artefact list <!-- omit in toc -->

- [Sysmon V15 Artefact](#sysmon-v15-artefact)
  - [Process creation (1)](#process-creation-1)
  - [process changed a file creation time (2)](#process-changed-a-file-creation-time-2)
  - [Network connection (3)](#network-connection-3)
  - [Sysmon service state changed (4)](#sysmon-service-state-changed-4)
  - [Process terminated (5)](#process-terminated-5)
  - [Driver loaded (6)](#driver-loaded-6)
  - [Image loaded (7)](#image-loaded-7)
  - [CreateRemoteThread (8)](#createremotethread-8)
  - [RawAccessRead (9)](#rawaccessread-9)
  - [ProcessAccess (10)](#processaccess-10)
  - [FileCreate (11)](#filecreate-11)
  - [RegistryEvent (12,13,14)](#registryevent-121314)
  - [FileCreateStreamHash (15)](#filecreatestreamhash-15)
  - [ServiceConfigurationChange (16)](#serviceconfigurationchange-16)
  - [PipeEvent (17,18)](#pipeevent-1718)
  - [WmiEvent (19,20,21)](#wmievent-192021)
  - [DNSEvent (22)](#dnsevent-22)
  - [FileDelete (23)](#filedelete-23)
  - [ClipboardChange (24)](#clipboardchange-24)
  - [ProcessTampering (25)](#processtampering-25)
  - [FileDeleteDetected (26)](#filedeletedetected-26)
  - [FileBlockExecutable (27)](#fileblockexecutable-27)
  - [FileBlockShredding (28)](#fileblockshredding-28)
  - [FileExecutableDetected (29)](#fileexecutabledetected-29)
  - [Error (255)](#error-255)
- [Windows builtin Channel](#windows-builtin-channel)

# Sysmon V15 Artefact

- ✔ Wag can create artefact
- ✖ Wag will not create artefact
  -❓ Need to be check

| EventID | Description                                           | Cover by wag |
| ------- | ----------------------------------------------------- | ------------ |
| 1       | Process creation                                      | ✖           |
| 2       | process changed a file creation time                  | ❓           |
| 3       | Network connection                                    | ✖           |
| 4       | Sysmon service state changed                          | ✖           |
| 5       | Process terminated                                    | ✖           |
| 6       | Driver loaded                                         | ✔           |
| 7       | Image loaded                                          | ❓           |
| 8       | CreateRemoteThread                                    | ❓           |
| 9       | RawAccessRead                                         | ❓           |
| 10      | ProcessAccess                                         | ❓           |
| 11      | FileCreate                                            | ✔           |
| 12      | RegistryEvent (Object create and delete)              | ✖           |
| 13      | RegistryEvent (Value Set)                             | ✖           |
| 14      | RegistryEvent (Key and Value Rename)                  | ✖           |
| 15      | FileCreateStreamHash                                  | ✔           |
| 16      | ServiceConfigurationChange                            | ✖           |
| 17      | PipeEvent (Pipe Created)                              | ✔           |
| 18      | PipeEvent (Pipe Connected)                            | ❓           |
| 19      | WmiEvent (WmiEventFilter activity detected)           | ❓           |
| 20      | WmiEvent (WmiEventConsumer activity detected)         | ❓           |
| 21      | WmiEvent (WmiEventConsumerToFilter activity detected) | ❓           |
| 22      | DNSEvent (DNS query)                                  | ✖           |
| 23      | FileDelete (File Delete archived)                     | ❓           |
| 24      | ClipboardChange (New content in the clipboard)        | ❓           |
| 25      | ProcessTampering (Process image change)               | ❓           |
| 26      | FileDeleteDetected (File Delete logged)               | ❓           |
| 27      | FileBlockExecutable                                   | ❓           |
| 28      | FileBlockShredding                                    | ❓           |
| 29      | FileExecutableDetected                                | ❓           |
| 255     | Error                                                 | ✖           |

## Process creation (1)

Cover by other tools like Atomic RedTeam

## process changed a file creation time (2)

Need to see its usefulness

## Network connection (3)

Cover by other tools like Atomic RedTeam

## Sysmon service state changed (4)

Need to see its usefulness

## Process terminated (5)

Cover by other tools like Atomic RedTeam

## Driver loaded (6)

Done by the option X

## Image loaded (7)

Need to see its usefulness

## CreateRemoteThread (8)

Need to see its usefulness

## RawAccessRead (9)

Need to see its usefulness

## ProcessAccess (10)

Need to see its usefulness

## FileCreate (11)

Done by the option X

## RegistryEvent (12,13,14)

Cover by other tools like Atomic RedTeam

## FileCreateStreamHash (15)

Done but get a bug when in Sysmon to validate

## ServiceConfigurationChange (16)

Need to see its usefulness

## PipeEvent (17,18)

Only Pipe Created , no Pipe Connected

## WmiEvent (19,20,21)

Need to see its usefulness

## DNSEvent (22)

Cover by other tools like Atomic RedTeam

## FileDelete (23)

Need to see its usefulness

## ClipboardChange (24)

Need to see its usefulness

## ProcessTampering (25)

Need to see its usefulness

## FileDeleteDetected (26)

Need to see its usefulness

## FileBlockExecutable (27)

Need to see its usefulness

## FileBlockShredding (28)

Need to see its usefulness

## FileExecutableDetected (29)

Need to see its usefulness

## Error (255)

Need to see its usefulness

# Windows builtin Channel

- code_integrity when use driver option

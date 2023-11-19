* MISC
```
 .\build\_deps\service_fabric_cpp-src\scripts\echomain_ctl.ps1 -Action Add  
```

* Linux
Use vscode dev container. It has all deps installed.

* Notes
  * gcc ld linker has problems with SF .so so we use lld from clang which is configured in .cargo config.
  * fabric_pal.so is needed to be able to provide windows C functions needed by windows-rs. Code is checked-in in /bintemp folder.


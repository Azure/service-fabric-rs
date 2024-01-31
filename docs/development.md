* MISC
```
 .\build\_deps\service_fabric_cpp-src\scripts\echomain_ctl.ps1 -Action Add  
```

* Linux
Use vscode dev container. It has all deps installed.

* Notes
  * gcc ld linker has problems with SF .so so we use lld from clang which is configured in .cargo config.
  * fabric_pal.so is needed to be able to provide windows C functions needed by windows-rs. Code is checked-in in /bintemp folder.

* WSL
Install SF in WSL maybe slow due to windows paths in /mnt/c are searched.
Change the following to remove /mnt/c etc paths
```sh
$ sudo nano /etc/wsl.conf
```
add section
```conf
# This removes windows path
[interop]
appendWindowsPath = false
# This do not mount windows drive
[automount]
enabled = false
```
Mount is needed to use vscode wsl. So after install sf the automount section needs to be removed to use vscode wsl.

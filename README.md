## iPod Execution Method Swapper (iPEMS)

# About 
iPEMS - A tool that allows you to quickly change the old method (disk swap) to a new one (unencrypted as encrypted (in short, UAE)) for iPod nano 7 2012 and 2015

# What does the utility do?
It performs many operations, in particular:
•Checking that the MSE is from the nano 6(Nano 6 is unfortunately not supported and is unlikely to be supported, after numerous tests with this method, this method did not work on the Nano 6)
•Checks swap partitions(which is used for partition swapping vulnerability)
•Checks the status of rsrc partition(through a simple check for the presence of regular bytes for rsrc)
•Performs reverse partition swap
•Sets the value of the rsrc partition, from "unencrypted and signed" to "encrypted to encrypted and signed"(The whole point of the UAE exploit)

# Build:

``` sh
cargo build --release
```
# Launch and use:
0. Download(or clone repo via git) and build the utility
1. Place the `Firmware.MSE` file from the ipsw file as the root (without renaming it)
2. Run the utility with the command:
./target/release/iPEMS
3. Follow the instructions on the screen
4. The output will be `Firmware_modified.MSE`
5. Save the original file somewhere
6. Rename `Firmware_modified.MSE` to `Firmware.MSE`
7. Replace it in ipsw
8. Flash it via iTunes(by pressing update, while holding down the shift key)

# Fast Mode
Adding the `--fast` parameter disables all delays in the code.
This is useful when you are an experienced user and want to skip the wait before warnings.

# Let's talk a little about exploits
•disk swap
In fact, it changes the partition assignments for disk mode and RetailOS, that is, RetailOS is disk mode, disk mode is RetailOS
And in this situation, although you have to hold down the volume keys, this allows you to run unsigned code. I'll explain why.
In short, in disk mode, the iPod does not check the rsrc partition for a valid signature, which opens a security hole, and this vulnerability works with both nano 6 and nano 7(2012 and 2015)(This might work on older nano models, but it's essentially pointless since they have full-fledged exploits)

•UAE
UAE or Unencrypted As Encrypted - A new exploit that, although only works on the nano 7, allows you to use the exploit without unnecessary keystrokes.
This method is more clever, it changes the state of the resource section or rsrc from "unencrypted and signed" to "encrypted and signed", for some reason, this breaks the signature verification on both revisions of the iPod nano 7, and creates a security hole in the device that only requires modifying the firmware and changing one byte in it.

# License
iPod Execution Method Swapper is licensed under the Creative Commons Attribution-NonCommercial-ShareAlike 4.0 International License.
See full license text in LICENSE file or at: https://creativecommons.org/licenses/by-nc-sa/4.0/legalcode

# Authors
TIS - code for implementing the exploit
Fong(_fong, from iPod hacking server) - discovery and first exploitation of this vulnerability
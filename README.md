required:
- win x64
- cl includes: /Zi /Gm- /hotpatch /Gy /Gw
- link includes: /functionpadmin /opt:noref /opt:noicf /debug:full

looks like exe.pdb has command lines

if only a string is changed, the binary of the function might not be any
different, the relocations are identical too. could be only .rdata change.

does it makes sense to just slam the obj in fully instead of trying to patch
only the pieces that may or may not have changed between runs? i think all
sections of the obj need to be reloc'd anyway

maybe it's as simple as:
- relink as a dll with a stub DllMain (so pdb gets connected)
- LoadLibrary that blob into the process (with lots of dead code)
- for all functions in objs that were changed during recompilation, hotpatch
  original location to point to new one
- but how do you identify relocations in those to know they should point back at
  the old ones rather than the ones it built in? i guess .rdata is updated,
  .data isn't?

in order to only have the changed obj in the dll:
- replace relocations (/references?) to
  - UNDEF External
  - .bss External
- but only the ones that are ours? we still want to link against msvcrt.lib or
  whatever, to remove relocations that are references to things in our project
  set, but not other ones (so need a cache of those as they change)
  
throw away .bss entirely

maybe need to find some memory 'near' the old one and use /fixed base to load it
at the right place so that jumps can be short? i think /functionpadmin would
let us double jump for functions, but need to be able to relocate for nearby
addresses.

global variables in update code aren't changed, so nothing to do there

"just" run a linker over the new obj to point it at all the 'internal' externals
(i.e. to symbols in our translation units, rather than libc etc), then hotpatch
the old one to ours

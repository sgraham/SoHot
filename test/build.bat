@echo off
del *.exe *.pdb *.obj *.ilk *.dump
cl /nologo /Zi /Gm- /Gy /Gw main.cc /c /Fomain.obj /Fdmain.obj.pdb || exit /b
copy stuff.orig.cc stuff.cc
cl /nologo /Zi /Gm- /Gy /Gw stuff.cc /c /Fostuff.obj /Fdstuff.obj.pdb || exit /b
link /nologo /functionpadmin /opt:noref /opt:noicf /debug:full /pdb:test.exe.pdb main.obj stuff.obj /out:test.exe || exit /b


sed -i -e "s/in test/modified/g" stuff.cc
cl /nologo /Zi /Gm- /Gy /Gw stuff.cc /c /Fostuff.sohot_1.obj /Fdstuff.sohot_1.obj.pdb || exit /b
::..\target\debug\patchgen main.obj stuff.obj -i stuff.sohot_1.obj -o stuff.sohot_1.patch.obj || exit /b
link /force /dll /incremental:no /noexp /nologo /functionpadmin /opt:noref /opt:noicf /debug:full /pdb:test.sohot_1.exe.pdb stuff.sohot_1.patch.obj /out:test.sohot_1.exe || exit /b

::cl /nologo /Zi /Gm- /Gy /Gw stuff.cc /c /Fostuff.obj /Fdstuff.obj.pdb || exit /b
::dumpbin /all /disasm stuff_orig.obj >orig.dump || exit /b
::dumpbin /all /disasm stuff.obj >mod.dump || exit /b
::
::sed -i -e "s/in test/modified/g" stuff.cc

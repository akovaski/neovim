#!bash
if [ "$#" -ne 1 ]; then
    echo "Accepts one file as argument. Goodbye."
    exit 1
fi

echo "remove multiline comments"; perl -0777 -i -pe  's{/\*.*?\*/}{}gs' $1

rustfmt --edition 2018 --config max_width=600 $1

echo "remove mods"; sed -i '/^pub mod .* {/,/^\}/d' $1
echo "remove private fn"; sed -i '/^unsafe .* fn /,/^}/d' $1
echo "remove open brackets"; sed -i 's/{.*$/;/g' $1
echo "remove close brackets"; sed -i '/^}/d' $1
echo "remove indented lines"; sed -i '/^ \+/d' $1
echo "remove single comments"; sed -i '/^ *\/\//d' $1
echo "remove hashthings"; sed -i '/#\[/d' $1
echo "remove use's"; sed -i '/^use /d' $1
echo "remove blank lines"; sed -i '/^$/d' $1
echo "remove static assignments"; sed -i 's/\(static.*\) =.*/\1;/g' $1

echo "move constants to top of file";
CONST_FILE=`mktemp`
grep    '^.* const .*$' $1 > $CONST_FILE
grep -v '^.* const .*$' $1 >> $CONST_FILE
cat $CONST_FILE > $1
rm $CONST_FILE


echo "insert use crate::\*"; sed -i '1iuse crate::*;' $1
echo "insert newline"; sed -i '1G' $1

echo "remove 'extern \"C\"'"; sed -i 's/ extern "C"//g' $1
echo "remove start 'extern \"C\"' block"; sed -i '0,/^\(pub unsafe\|static mut\)/ s/^\(pub unsafe\|static mut\)/extern "C" {\n\1/' $1
echo "Add brace to end of file"; echo '}' >> $1

echo "remove mut"; sed -i 's/\(, *\|(\)mut /\1/g' $1

rustfmt --edition 2018 --config max_width=600 $1

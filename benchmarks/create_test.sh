mkdir $1
cd $1
for i in {1..10}
do
  ip="$i  ())"
  mkdir "$ip"
  for j in {1..10}
  do
    jp="$ip/$j asd()()wcwc  ())"
    mkdir "$jp"
    for k in {1..1000}
    do
      touch "$jp/$k asd()()wcwc  ()sdasfefe).txt"
    done
  done
done

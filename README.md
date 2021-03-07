[BUILD]
rm find_em*;cargo build;date +%s | xargs -i cp target/debug/find_empty_block ./find_empty_block_{}
[USEAGE]
nohup ./find_empty_block_1615114992 --data-path=/data/nfs/37/sealed/s-t059813-150134 --offset=0 --size=34359738368  --parallel=32 >> nohup.out &

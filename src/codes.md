codes 3 True

**static** > AAA:f32 = 3.45;
**const**  > BBB:bool = true;
**var**    > let (mut) (_)x:u8 = 11; ^(mut = mutable, _ => not used)^
**Types**  > u.. i.. f.. char bool \"\"
**Array**  > let an_array:[u8,3]=[1,2,4];
**Vector** > let vec = vec![]
**Tuple**  > let tup = ('a',2,3.24)
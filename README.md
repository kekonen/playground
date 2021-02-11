
## Rules

### 1. Overlapping reservations
Just dont use the rooms that are used in further overlapping reservations

A, B and C mutually intersect eachother, and it doesnt matter if they are similar or not:

       4
    2  3
 2  1  2
 1  0  1
[N][N][N]
 A  B  C

NNN
1NN
2NN
-X
N0N
10N
20N
-X
N1N - skip 1
21N
-X
N2N
12N
--X
NN1
2N1 - skip 1
-X
N01 - skip 1
201
-X
N21 - skip 1
--X
...



Challenge: check if both rules work together



### 2. Similar reservations cannot be equal or lower than the further ones
Lets pretend that we have 2 reservations that are identical, in a sense that have same dates and other properties, hence same potential units - there is no need to test interchangingly - only unique combinations

 2  2  2
 1  1  1
 0  0  0
[N][N][N]
NNN
0NN
1NN
2NN
-X
10N <-- important that 0th is already higher than the 1st, since N0N is same as 0NN in this case
20N
-X
21N
--X
210

Total: 8 combintions





### 3. Similar Units
If units are absolutely the same (like both same floor), there is no difference in which put people.
It is possible to skip them till the next non-similar, if it is not occupied by higher reservations, else then just skip.



## Parallelisation
The first part (generation of combinations) easier to perform on one unit consequently.
The second part (ealuation of combinations) can be done separately by batches in different threads with noo headache.
Finally, sort operation can be also done in parallel, using rayon sort function  
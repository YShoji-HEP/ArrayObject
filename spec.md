Specification of the ArrayObject encording
==========================================
A footer is added to the data in reversed bytes. It has the following structure.
||Data type|Compression|Dimension|Short Data|Shape|
|----|----|----|----|----|----|
|Short|3 bit|0 bit| 0 bit| 5 bit|0bit
|Long|3 bit|1 bit| 4 bit|0 bit|8 bit x required|

Data type
---------
The first three bits of the last byte indicate the data type.
|Number|Data type|
|-|-|
|0| SHORT_UNSIGNED_INTEGER|
|1| SHORT_SIGNED_INTEGER|
|2| UNSIGNED_INTEGER|
|3| SIGNED_INTEGER|
|4| REAL|
|5| COMPLEX|
|6| STRING|
|7| (Not assigned)|

Compression
-----------
The forth bit indicates the compression type for number / string. The short integers do not have this bit. 
|Number|Compression (number)|Compression (string)|
|-|-|-|
|0| FIXED_LENGTH|JOINED|
|1| VARIABLE_LENGTH|DICTIONARY|

Dimension
---------------
The last four bits are used to store the dimension of the array.

Short Data
----------
For SHORT_UNSIGNED_INTEGER and SHORT_SIGNED_INTEGER, the last five bits are used to store the data.

Shape
-----
The shape of the array is stored with the variable integer in row major order. If it is empty, it indicates a scalar variable.

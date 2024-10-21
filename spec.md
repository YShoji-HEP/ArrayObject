Specification of the ArrayObject encording
==========================================
A footer is added to the data in reversed bytes. It has the following structure.
|Data type|Compression|Dimension/Data|Shape|
|-----------|----|-----|----|
|3 bit|1 bit or 0 bit| 4 bit or 5 bit|8 bit x required|


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

Additional Data
---------------
The last four or five bits are used to store the dimension of the array or the data (short integers).

Shape
-----
For array data, the shape is stored with the variable integer in row major order.

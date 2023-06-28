# math_lib
Math library I'm coding as a way to get familiar with rust


This is for educational purposes do not use as it is not efficient nor safe.

I've implemented a Z/pZ struct and Matrix and Poly struct over any type that implements the Ring supertrait, as matrices and poly implement the Ring trait you can nest them.

Also added Field extension struct over a Field trait type, the first field type is Z/pZ but as field extensions
implement the Field trait you can make extensions of extensions.
I added Elliptic curves defined over any type that implements the field trait so Z/pZ or a field extension

Later on I'd like to implement several other things like FFT, a prime checking algorithm and I'd like to have enough to build a few cryptograhpic protocols without using an external library (although i'll need to get familiar with arkworks at some point)

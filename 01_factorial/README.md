This is a simple factorial function. It takes a number n and multiplies all numbers from 1 up to n.

In this specific case I wanted to run a little benchmark against Python using timeit. 

The first thing I noticed is that u128 is not as big as you would like for this kind of projects, panicking at 35!.

After changing from u128 to BigUInt the time was almost x100 for the small numbers. 
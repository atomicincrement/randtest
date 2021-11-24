# randtest


R runif and rnorm:

```R
> system.time(runif(1000000))
   user  system elapsed 
  0.035   0.000   0.035 
> system.time(rnorm(1000000))
   user  system elapsed 
  0.059   0.007   0.065 
```

Python numpy.random.uniform and numpy.random.normal

```python
>>> import numpy
>>> import time
>>> t0 = time.time(); r = numpy.random.uniform(size=1000000); print(time.time()-t0)
0.035430908203125
>>> t0 = time.time(); r = numpy.random.normal(size=1000000); print(time.time()-t0)
0.060460567474365234
```

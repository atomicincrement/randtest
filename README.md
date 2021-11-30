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

C rand()
```
#include <stdlib.h>

__attribute__((noinline))
void f(double *d, int len) {
    while (len--) {
        *d++ = rand() * (1.0/RAND_MAX);
    }
}

double d[1000000];

int main() {
    for (int i = 0; i != 1000; ++i) {
        f(d, 1000000);
    }
}

$ time ./a.out 

real    0m6.057s
user    0m6.048s
sys     0m0.008s
```


C++

```
#include <random>

using namespace std;

__attribute__((noinline))
void f(double *d, int len) {
    random_device rd; 
    mt19937 gen(rd()); 
    uniform_real_distribution<> dis(0,1.0);
    while (len--) {
        *d++ = dis(gen);
    }
}

double d[1000000];

int main() {
    for (int i = 0; i != 1000; ++i) {
        f(d, 1000000);
    }
}

$ time ./a.out 

real    0m13.564s
user    0m13.563s
sys     0m0.000s
```

C++ rnorm

```

#include <random>

using namespace std;

__attribute__((noinline))
void f(double *d, int len) {
    random_device rd; 
    mt19937 gen(rd()); 
    normal_distribution<double> dis(0.0,1.0);
    while (len--) {
        *d++ = dis(gen);
    }
}

double d[1000000];

int main() {
    for (int i = 0; i != 1000; ++i) {
        f(d, 1000000);
    }
}

$ time ./a.out 

real    0m31.030s
user    0m31.019s
sys     0m0.008s
```

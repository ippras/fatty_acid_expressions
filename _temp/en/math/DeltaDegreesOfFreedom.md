# Delta Degrees of Freedom (DDOF)

$$\sigma = \sqrt{\frac{\sum_{i=1}^N (x_i - \mu)^2}{N - ddof}}$$

---

The divisor used in calculations is $N - ddof$, where $N$ represents the number of elements. By default ddof is 1.

Technically, the `DDOF` parameter can be set to any number (even a fractional one), but in practice, only two values are used: **0** or **1**.

## DDOF=0

Calculates the standard deviation for the **population**. You use this if your data represents *absolutely all* possible values that exist in nature, and you do not need to make predictions outside of this dataset.

## DDOF=1

Calculates the standard deviation for a **sample**. This is called *Bessel's correction*. You use this if your replicates are just a fraction (a sample) of an infinite number of possible experiments. Dividing by $N-1$ makes the variance estimate *unbiased*, artificially increasing the standard deviation slightly to compensate for the fact that a small sample typically underestimates the true spread.

## Links

* https://numpy.org/devdocs/reference/generated/numpy.std.html

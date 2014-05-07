pub fn

bool
miller_rabin(long long n,int t)
{
    if (n <  2) return false;
    if (n == 2) return true;
    if (!(n&1)) return false;

    long long q = n-1;
    int k = 0;
    while ((q&1) == 0) {
        k++;
        q >>= 1;
    }
    for (int i=0; i < t; ++i) {
        long long a = rand() % (n-1) + 1;
        long long x = mod_exp(a, q, n);
        if (x==1) {
            continue;
        }
        bool found = false;
        for (int j=0; j<k; ++j) {
            if ( x == n-1) {
                found = true;
                break;
            }
            x = mod_mult(x, x, n);
        }
        if (found) {
            continue;
        }
        return false;
    }
    return true;
}
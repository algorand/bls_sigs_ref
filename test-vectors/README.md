The file in this directory comprises test inputs, one per line.

Each line is a space-separated tuple (msg, key).

msg and key should be interpreted as hex-encoded octet strings, wherein each
pair of hex characters represents one byte.

Equivalently, msg and key can be interpreted as hexadecimal integers and then
converted to octet strings using I2OSP (RFC 8017).

In Python, the following code can be used to load the file:

~~~
import binascii
with open("fips_186_3", "r") as vec_file:
  test_inputs = [ tuple( binascii.unhexlify(val)       \
                  for val in line.strip().split(' ') ) \
                  for line in vec_file.readlines()     \
                ]
~~~

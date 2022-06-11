## Data Generator
a simple domain specific language which allows you to generate data fast and easy
## Usage

Here is an example of how to generate data of a+b problem using data generator

create a new text file named `example.txt`

open the editor and write

```
{i32} {i32}
```
execute using data generator

```bash
## linux
./data_generator -f example.txt -c 10
## replace `./data_generator` to `data_generator.exe` in windows
```

waiting for some time and it is done,how easy it is!

## Tokens

### single tokens

here a some single tokens which are allow to use in data_generator

```rust
i16 //short
i32 //int
i64 //long long
i128 //int128_t

/*tokens start with u are all the tokens above but unsigned.So no marks here*/
u16
u32
u64
u128

char // A UTF-8 charaters
string //ASCII. range from 20 to 126 (that means no control charaters.Will add some later) (the maxium len are u16::MAX)
bigint //maxium len are u16::MAX
```

### multiple tokens

TODO...



## TODOs

add multiple tokens (allows user to control the random data more fine)

add documentations for the source code
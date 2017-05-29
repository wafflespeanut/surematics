## ukpost

Find the distance between two UK regions represented by postal codes.

### Build and usage

 - Install `rustc` and `cargo` from https://rustup.rs/
 - `cd` into the directory and run `cargo run -- <postal_code_1> <postal_code_2>`

For example,

``` bash
$ cargo run -- M320JG OX495NU
Distance: 216.99865394505971 km
```

### Note

This program fetches postal data from https://api.postcodes.io/ (which provides UK location data legally for free). There are [other sources][1] [which provide][2] the same data, but they're either outdated or illegal.

The result from this program could be verified using [this website][3].

[1]: https://wikileaks.org/wiki/UK_government_database_of_all_1,841,177_post_codes_together_with_precise_geographic_coordinates_and_other_information,_8_Jul_2009
[2]: https://github.com/academe/UK-Postcodes/
[3]: https://www.freemaptools.com/distance-between-uk-postcodes.htm

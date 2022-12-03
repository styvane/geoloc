Requirements
------------
The only requirement for this is Rust. See the documentation for how to install Rust.

Supported Rust Versions
--------------------------
Geoloc is built against the latest stable release. The minimum supported version is 1.65.
There is no guarantee to build on Rust versions earlier than the minimum supported version.

Build
-----
To build the application, use the `cargo` command.

```bash
$ cargo b --release
```

Preprocess the data
--------------------

You need to preprocess the data using the python script in the root level directory. `db.py`

```bash
$ python db.py database.csv
```
You can edit the script to change the output file destination. By default it write the output to `geoloc.db`


Run
---
Run the following command in the termial.

```bash
$ ./geoloc geoloc.db # Change geoloc.db to your database's name if it was change in the previous step.
```

Performance data
----------------

*****
Database loaded Memory usage: 3.66mb Load time: 1ms
- OK    1.0.0.0 US Los Angeles Memory usage: 3.9mb Lookup time: 2ms
- OK    71.6.28.0 US San Jose Memory usage: 4.08mb Lookup time: 1ms
- OK    71.6.28.255 US San Jose Memory usage: 4.31mb Lookup time: 639μs
- OK    71.6.29.0 US Concord Memory usage: 4.54mb Lookup time: 591μs
- OK    53.103.144.0 DE Stuttgart Memory usage: 4.95mb Lookup time: 1ms
- OK    53.255.255.255 DE Stuttgart Memory usage: 5.32mb Lookup time: 770μs
- OK    54.0.0.0 US Rahway Memory usage: 5.79mb Lookup time: 904μs
- OK    223.255.255.255 AU Brisbane Memory usage: 6.39mb Lookup time: 1ms
- OK    5.44.16.0 GB Hastings Memory usage: 6.59mb Lookup time: 530μs
- OK    8.24.99.0 US Hastings Memory usage: 7.25mb Lookup time: 1ms

Final points for 10 measurements:  1172.9039375


*****
Database loaded Memory usage: 3.6mb Load time: 1ms
- OK    1.0.0.0 US Los Angeles Memory usage: 3.89mb Lookup time: 352μs
- OK    71.6.28.0 US San Jose Memory usage: 4.29mb Lookup time: 655μs
- OK    71.6.28.255 US San Jose Memory usage: 4.75mb Lookup time: 665μs
- OK    71.6.29.0 US Concord Memory usage: 5.25mb Lookup time: 676μs
- OK    53.103.144.0 DE Stuttgart Memory usage: 5.74mb Lookup time: 642μs
- OK    53.255.255.255 DE Stuttgart Memory usage: 6.32mb Lookup time: 779μs
- OK    54.0.0.0 US Rahway Memory usage: 6.94mb Lookup time: 871μs
- OK    223.255.255.255 AU Brisbane Memory usage: 7.64mb Lookup time: 1ms
- OK    5.44.16.0 GB Hastings Memory usage: 7.86mb Lookup time: 272μs
- OK    8.24.99.0 US Hastings Memory usage: 8.41mb Lookup time: 810μs

Final points for 10 measurements:  763.8235625
****
Database loaded Memory usage: 3.72mb Load time: 912μs
- OK    1.0.0.0 US Los Angeles Memory usage: 4.02mb Lookup time: 382μs
- OK    71.6.28.0 US San Jose Memory usage: 4.43mb Lookup time: 560μs
- OK    71.6.28.255 US San Jose Memory usage: 4.91mb Lookup time: 562μs
- OK    71.6.29.0 US Concord Memory usage: 5.49mb Lookup time: 674μs
- OK    53.103.144.0 DE Stuttgart Memory usage: 6.14mb Lookup time: 798μs
- OK    53.255.255.255 DE Stuttgart Memory usage: 6.69mb Lookup time: 599μs
- OK    54.0.0.0 US Rahway Memory usage: 7.31mb Lookup time: 839μs
- OK    223.255.255.255 AU Brisbane Memory usage: 8.02mb Lookup time: 1ms
- OK    5.44.16.0 GB Hastings Memory usage: 8.35mb Lookup time: 495μs
- OK    8.24.99.0 US Hastings Memory usage: 9.11mb Lookup time: 1ms

Final points for 10 measurements:  802.1666875000001

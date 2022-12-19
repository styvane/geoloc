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

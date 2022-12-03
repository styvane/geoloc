
#!/usr/local/bin/env python

import sqlite3

import csv
import sys

def insert_rows(conn, rows):
    """
    Insert each row in into the table associated with the cursor.
    """
    for row in rows:
        try:
            row = (int(row[0]), int(row[1]), row[2].encode("utf-*8"), row[5].encode("utf-8"))
        except:
            continue
        conn.execute("INSERT INTO iptable (start, end, country_code, city) VALUES(?, ?, ?, ?)", row)


def create_table(conn):
    """
    Create database table.
    """
    try:
        conn.execute(
            """
            CREATE TABLE iptable (
              start INTEGER NOT NULL UNIQUE,
              end INTEGER NOT NULL UNIQUE,
              country_code BLOB NOT NULL,
              city BLOB NOT NULL
            );
           """
        )
        return True
    except:
        return False


if __name__ == "__main__":
    if len(sys.argv) != 2:
        print("Usage:\n\tpython db.py database.csv")
        exit(1)
        
    with open(sys.argv[1]) as f:
        conn = sqlite3.connect("test.db")
        reader = csv.reader(f)
        if create_table(conn):
            with conn:
                insert_rows(conn, reader)
        conn.close()
        
        


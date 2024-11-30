<div align="center">
  <picture>
    <source media="(prefers-color-scheme: light)" srcset="logo_horizontal.svg">
    <source media="(prefers-color-scheme: dark)" srcset="logo_horizontal_dark.svg">
    <img alt="AlexDB logo" src="logo_horizontal.svg" height="125">
  </picture>
</div>

##  AlexDB Album Analysis Example

I've compiled a list of some albums I have listened to over the years, and included some basic information about each album as well as a subjective score out of ten I would give each album.

### Dataset

[albums.csv](../datasets/albums.csv)

### Using the Dataset

Before loading a CSV file, you must create a schema to load the data into; let's do that now.

```
CREATE TABLE albums (name str, artist str, year num, runtime num, num_songs num, score num)
```

Now we can load the data from the given CSV file.

```
IMPORT CSV 'datasets/albums.csv' INTO albums
```

Let's see what my top three favorite albums are:

```
SELECT name, artist FROM albums ORDER BY score DESC LIMIT 3
```

```
name            artist
'OK Computer'   'Radiohead'
'In Rainbows'   'Radiohead'
'Jen'           'Plums'
```

What about the score I gave BRAT by Charli XCX?

```
SELECT score FROM albums WHERE artist == 'Charli XCX' && name == 'BRAT' 
```

```
score
8
```

Now, let's rank the five longest albums by runtime and export that query as a CSV for safekeeping.

```
SELECT name, artist, runtime FROM albums ORDER BY runtime DESC LIMIT 5 EXPORT CSV 'longest_songs.csv'
```

```
name                                               artist                          runtime
'Nervous Young Man'                                'Car Seat Headrest'             128
'To Be Kind'                                       'Swans'                         121
'Lift Your Skinny Fists Like Antennas to Heaven'   'Godspeed You! Black Emperor'   87
'The Suburbs'                                      'Arcade Fire'                   74
'Twin Fantasy'                                     'Car Seat Headrest'             71
```

Say somebody wants to listen to listen to some new music, and they're interested in albums that I consider to be close to perfect. Rather than explaining my rating system to them, it might be easier to create a new column to mark whether or not I think an album is close to perfect, and give that data to them. We can start by creating an `is_perfect` column that flags albums with a high rating.

```
CREATE COLUMN (bool) is_perfect = score >= 10 INTO albums
```

If we select a couple of rows from the albums table, we can see this new column.

```
SELECT * FROM albums LIMIT 2
```

```
name                artist            year   runtime   num_songs   score   is_perfect
'Imaginal Disk'     'Magdalena Bay'   2024   54        15          9       false
'Mercurial World'   'Magdalena Bay'   2021   46        14          8       false
```

We can query this new column to easily find all perfect albums.

```
SELECT name, artist FROM albums WHERE is_perfect
```

```
'OK Computer'                                      'Radiohead'
'In Rainbows'                                      'Radiohead'
'Jen'                                              'Plums'
'The Money Store'                                  'Death Grips'
'Lift Your Skinny Fists Like Antennas to Heaven'   'Godspeed You! Black Emperor'
```

Finally, let's export this table so we can give it to our music-interested acquaintance.

```
EXPORT CSV 'albums_with_perfect.csv' FROM albums
```

As our final demonstration in this example, let's see if my album ratings are fair, or if I'm just a hater; we can do this by looking at my average score. To compute the average album score, let's create some aggregates to count the number of rows in the database and the sum of scores I've given every album.

```
CREATE AGGREGATE num_rows = current + 1 INIT 1 INTO albums
```

```
CREATE AGGREGATE sum_scores = current + score INIT score INTO albums
```

Next, let's create a computation to figure out the average by combining our aggregates.

```
CREATE COMP avg_score = sum_scores / num_rows INTO albums
```

Finally, we can look at this computation to see if I'm a hater or not.

```
SELECT COMP avg_score FROM albums
```

```
8.30188679245283
```

My average score is fairly high, which proves that I'm not a hater.

### Conclusion

You can easily import and export CSV data to/from AlexDB, and I some *very* bad music opinions.
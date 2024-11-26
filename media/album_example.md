<div align="center">
  <picture>
    <source media="(prefers-color-scheme: light)" srcset="logo_horizontal.svg">
    <source media="(prefers-color-scheme: dark)" srcset="logo_horizontal_dark.svg">
    <img alt="AlexDB logo" src="logo_horizontal.svg" height="125">
  </picture>
</div>

##  AlexDB Album Analysis Example

I've compiled a list of albums I have listened to over the years, and included some basic information about each album as well as a score out of ten I would give each album.

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

Let's see what my top three favorite albums of all time are:

```
SELECT name, artist FROM albums ORDER BY score DESC LIMIT 3
```

```
name            artist
'OK Computer'   'Radiohead'
'In Rainbows'   'Radiohead'
'Jen'           'Plums'
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

### Conclusion

You can import and export CSV data to/from AlexDB, and Alex has a *very* good taste in music.
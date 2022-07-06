<div align="center">

# Wikibase merge

Batch merge wikibase items

</div>

## Usage

```sh
wikibase-merge --ids=Q1,Q2,Q3 --ids=Q4,Q5,Q6 --url=https://www.wikidata.org/w/api.php --workers=4

# or 

cat data.txt | wikibase-merge --url=https://www.wikidata.org/w/api.php --workers=4
```

With this example Q1,Q2 will be merged into Q3 and Q4,Q5 will be merged into Q6. Ids are merged with the last id in the list. Ids must be a comma separated list of Wikibase item ids.

## Limitations

This script don't use authentication yet. All modifications are performed anonymously.

If you encouter DBConnectionError or any other Wikibase related errors, try to reduce the number of workers.

## License

This software is licensed under the MIT license.
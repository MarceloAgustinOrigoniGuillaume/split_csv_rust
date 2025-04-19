-------------

- Descargar algun dataset/csv por ejemplo https://www.kaggle.com/datasets/najzeko/steam-reviews-2021 
- Guardar y descomprimir en un path conocido, dentro de una carpeta con solo ese csv preferiblemente.

Ejecución
---------

```
cargo run <src-dir> <max records per file> <output-dir> <base-name>
```

por ejemplo

```
cargo run ../big_csv_folder 100000 ../splitted splitted_csv
```

crearia en la carpeta splitted , splitted_csv_1.csv..., splitted_csv_n.csv cada uno con max 100 000 registros.

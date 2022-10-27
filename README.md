## Более лёгкая версия [l_approx](https://gitlab.zenit-kmz.ru/dep570/tpkk/l_approx)

Реализованы аргументы `-p`, `-v`, `-s`.

## Примеры:
Расчёт термоуводов по "сырым данным":
```
r-approx -p [CSV file]
```

Валидация термоуводов с исправленными данными:
```
r-approx -v [CSV file]
```

Валидация термоуводов с исправленными данными и запись в `ct` файл:
```
r-approx -v [CSV file] -s [SERIAL NUMBER]
```

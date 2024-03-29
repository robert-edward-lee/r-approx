## Облегчённый [l_approx](https://gitlab.zenit-kmz.ru/dep570/tpkk/l_approx)

Реализованы аргументы `-p`, `-v`, `-s`.

### Установка
Перед сборкой проекта установить [`rustup`](https://win.rustup.rs/x86_64), при установке выбрав конфигурацию
линковщика `x86_64-pc-windows-gnu` в зависимости от имеющегося в системе.
Для установки набрать команду:
```
cargo install --path .
```
Установленная программа будет находится в папке `C:\Users\[user name]\.cargo\bin`

Для сборки 32 битной версии:
```bash
rustup install stable-i686-pc-windows-gnu
rustup default stable-i686-pc-windows-gnu
cargo build --release
rustup default stable-x86_64-pc-windows-gnu
```

### Примеры:
Расчёт термоуводов по "сырым данным":
```
r-approx -p [CSV file]
```

Валидация термоуводов с вычисленными данными в файле по умолчанию:
```
r-approx -v [CSV file]
```

Валидация термоуводов с вычисленными данными в файле по умолчанию:
```
r-approx -v [CSV file] [VALID CSV file]
```

Валидация термоуводов с вычисленными данными в файле по умолчанию и запись в `ct` файл:
```
r-approx -v [CSV file] -s [SERIAL NUMBER]
```

Если в ключ `-s` не передать серийный номер, то приложение попытается взять его имя из имени папки:
```
r-approx -v [CSV file] -s
```

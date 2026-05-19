# Translate

## Generating the PO Template

```sh
MDBOOK_OUTPUT='{"xgettext": {}}' \
  mdbook build -d po
```

```sh
MDBOOK_OUTPUT='{"xgettext": {"depth": 3}}' \
  mdbook build -d po
```

## Initialize a New Translation

```sh
msginit -i po/messages.pot -l ru -o po/ru.po --no-translator
```

```sh
msginit -i po/messages.pot -l ru -o po/ru.po
```

kgv@users.noreply.github.com

## Updating an Existing Translation

```
msgmerge --update po/ru.po po/messages.pot
```

```
msgcat --no-wrap po/ru.po -o po/ru.po
```

```
msgcat po/ru.po -o po/ru.po
```

```
msgfmt --statistics po/ru.po
```

## Build and postprocess all translations

```sh
for language in en ru; do
    echo "Start building $language translation"
    MDBOOK_BOOK__LANGUAGE=$language \
    mdbook build -d book/$language
    echo "End building"
done
```

```sh
for language in en ru; do
    echo "Start postprocess $language translation"
    
    # 1. Создание целевой папки
    mkdir -p "book/markdown/$language"

    # 2. Копирование файлов
    cp -r "book/$language/markdown/"* "book/markdown/$language/"

    # 3. Замена в файлах
    if [ "$language" != "en" ]; then
        # find "book/markdown/$language" -type f -name "*.md" -print0 | xargs -0 perl -pi -e 's|\\\[|[|g; s|\\\]|]|g; s|\\\\|\\|g;'
        find "book/markdown/$language" -type f -name "*.md" -print0 | xargs -0 perl -pi -e 's|\\\[|[|g; s|\\\]|]|g;'
    fi
    find "book/markdown/$language" -type f -name "*.md" -print0 | xargs -0 perl -pi -e 's|([{}*])|{"$1"}|g; s|\n|\n |g;'

    echo "End postprocess translation"
done
```

## Other

`s/<!-- i18n:skip -->\s*\n//g;`

`msgid "\[.*\]\(.*\)"`

```
MDBOOK_BOOK__LANGUAGE=ru mdbook build
```

```
MDBOOK_BOOK__LANGUAGE=ru mdbook serve
```

## References

- [Chen et al., 2020](https://doi.org/10.3390/ijms21165695 "Nutritional Indices for Assessing Fatty Acids: A Mini-Review")
- [Dal Bosco et al., 2022](https://doi.org/10.3390/nu14153110 "Indexing of Fatty Acids in Poultry Meat for Its Characterization in Healthy Human Nutrition: A Comprehensive Application of the Scientific Literature and New Proposals")
- [Kralik et al., 2025](https://doi.org/10.18047/poljo.31.1.8 "A Fatty acid profile and THE Health Lipid Indices in Table Eggs")

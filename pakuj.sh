#!/bin/bash
NUMER_WERSJI=$(cargo pkgid -p app | cut -d# -f2 | cut -d: -f2)

NOWA_WERSJA=$(echo "$NUMER_WERSJI" | awk -F. '{$NF = $NF + 1;} 1' OFS=.)
WERSJA_Z_PODKRESLNIKAMI=${NOWA_WERSJA//./_}
NAZWA_PLIKU="asset_bundler_2_ver_${WERSJA_Z_PODKRESLNIKAMI}"
sed -i "s/^version = \"$NUMER_WERSJI\"/version = \"$NOWA_WERSJA\"/" "src/app/Cargo.toml"
cargo clean
cargo build -r
printf "\033c"
echo "Zbudowano wersję release"
echo "wersja $STARA_WERSJA -> $NOWA_WERSJA"
mkdir -p export/temp
mv target/release/Asset-bundler-2 export/temp
cargo clean
cargo build
printf "\033c"
cp export/temp/Asset-bundler-2 export/temp/Asset-bundler-2_kompresja
echo "przeniesiono app do export/temp"
upx --best --lzma export/temp/Asset-bundler-2_kompresja
#echo "skompresowano"
mv export/temp/Asset-bundler-2_kompresja export/release/${NAZWA_PLIKU}

echo "przeniesione do export/release"

rm -rf AppDir

# 1. Pobierz linuxdeploy (jeśli nie ma)
if [ ! -f linuxdeploy-x86_64.AppImage ]; then
    wget https://github.com/linuxdeploy/linuxdeploy/releases/download/continuous/linuxdeploy-x86_64.AppImage
    chmod +x linuxdeploy-x86_64.AppImage
fi

export VERSION=$NOWA_WERSJA

# 2. Stwórz strukturę AppDir i zbuduj AppImage
# --executable: ścieżka do Twojej binarki
# --i: ścieżka do ikony (np. app-icon.png)
# --d: ścieżka do pliku .desktop
./linuxdeploy-x86_64.AppImage --appdir AppDir \
    --executable "export/temp/Asset-bundler-2" \
    --icon-file "export/resources/align-justify.png" \
    --icon-filename "app-icon" \
    --desktop-file "app.desktop" \
    --output appimage

# 3. Przenieś gotowy AppImage do export/

mv *.AppImage export/release

rm -r export/temp

#shutdown + 15

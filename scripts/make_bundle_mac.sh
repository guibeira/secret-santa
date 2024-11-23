
#!/bin/bash

APP_NAME="Secret Santa"
ICON_PATH="icons/santa.icns"
EXECUTABLE="target/release/secret-santa"

# create bundler folder
mkdir -p "$APP_NAME.app/Contents/MacOS"
mkdir -p "$APP_NAME.app/Contents/Resources"

# Add the icon and executable
cp "$ICON_PATH" "$APP_NAME.app/Contents/Resources/app.icns"
cp "$EXECUTABLE" "$APP_NAME.app/Contents/MacOS/$APP_NAME"

# Create the Info.plist file
cat > "$APP_NAME.app/Contents/Info.plist" <<EOL
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>CFBundleName</key>
    <string>$APP_NAME</string>
    <key>CFBundleDisplayName</key>
    <string>$APP_NAME</string>
    <key>CFBundleExecutable</key>
    <string>$APP_NAME</string>
    <key>CFBundleIconFile</key>
    <string>app.icns</string>
    <key>CFBundleIdentifier</key>
    <string>com.example.$APP_NAME</string>
    <key>CFBundleVersion</key>
    <string>1.0</string>
</dict>
</plist>
EOL

#!/bin/bash

# Build the release binary
cargo build --release

# Define app name and paths
APP_NAME="PepeKeyboard"
APP_DIR="${APP_NAME}.app"
CONTENTS_DIR="${APP_DIR}/Contents"
MACOS_DIR="${CONTENTS_DIR}/MacOS"
RESOURCES_DIR="${CONTENTS_DIR}/Resources"

# Create directory structure
mkdir -p "${MACOS_DIR}"
mkdir -p "${RESOURCES_DIR}"

# Copy the binary
cp target/release/PepeKeyboard "${MACOS_DIR}/${APP_NAME}"

# Copy the sound files
cp click1.wav "${RESOURCES_DIR}/click1.wav"
cp click2.wav "${RESOURCES_DIR}/click2.wav"
cp click3.wav "${RESOURCES_DIR}/click3.wav"
cp click4.wav "${RESOURCES_DIR}/click4.wav"
cp enter.wav "${RESOURCES_DIR}/enter.wav"
cp cmd.wav "${RESOURCES_DIR}/cmd.wav"

# Create Info.plist
cat > "${CONTENTS_DIR}/Info.plist" << EOF
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>CFBundleDevelopmentRegion</key>
    <string>en</string>
    <key>CFBundleExecutable</key>
    <string>${APP_NAME}</string>
    <key>CFBundleIdentifier</key>
    <string>com.rtexty.pepekeyboard</string>
    <key>CFBundleInfoDictionaryVersion</key>
    <string>6.0</string>
    <key>CFBundleName</key>
    <string>${APP_NAME}</string>
    <key>CFBundlePackageType</key>
    <string>APPL</string>
    <key>CFBundleShortVersionString</key>
    <string>1.0</string>
    <key>CFBundleVersion</key>
    <string>1</string>
    <key>LSMinimumSystemVersion</key>
    <string>10.11</string>
    <key>LSUIElement</key>
    <true/>
    <key>NSHighResolutionCapable</key>
    <true/>
    <key>NSAppleEventsUsageDescription</key>
    <string>PepeKeyboard needs to monitor keystrokes to play sounds.</string>
</dict>
</plist>
EOF

echo "Successfully created ${APP_DIR}"
echo "You can now run it by double-clicking ${APP_DIR} in Finder."

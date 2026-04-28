mkdir gen
actool ./birdpro-icon-mac.icon --compile ./gen --output-format human-readable-text --notices --warnings --errors \
  --output-partial-info-plist ./gen/birdpro.plist \
  --app-icon birdpro-icon-mac --include-all-app-icons \
  --enable-on-demand-resources NO \
  --target-device mac \
  --minimum-deployment-target 26.0 \
  --platform macosx

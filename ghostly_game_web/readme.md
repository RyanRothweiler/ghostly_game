# Desc
The web platform implementation for the Ghostly game

# Build
"wasm-pack build --target web"

# Run server
"python3 -m http.server"

# Deploy to firebase
- Copy index file and pkg folder into firebase_hosting/public
- Run "firebase deploy"
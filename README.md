A simple keyboard typing visualiser that comprises of a web-app that receives
key-event from a small rust app over websocket and uses SVG to animate the
keystroke events. 
```sh
cargo run --bin keyserver
npx serve

# Open a browser in headless mode to show the app
open -na "Brave Browser" --args  --incognito --app="http://localhost:3000/app/index.html"
```

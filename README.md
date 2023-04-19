# PomoTime

PomoTime is a simple pomodoro timer that helps you focus on your work.

It is made with leptos in rust.

It should get hosted on github pages soon.

## File structure
 - `src/` contains the rust source code
 - `assets/` contains the static assets used in the app
 - `styles/` contains the css/sass styles used in the app
 - `svg-src` contains the unminified svg files used in the app. Should be minified before use. Also loads fonts from google fonts instead of locally so that it works without our server. These are currently minified using a web tool manually. The minimized files are in `assets/`

const { app, BrowserWindow } = require('electron')

function createWindow() {
  const win = new BrowserWindow({
    width: 1200,
    height: 800,
    webPreferences: {
      nodeIntegration: true
    },
    frame: false,
    titleBarStyle: 'hidden'
  })
  
  win.loadFile('index.html') // Your HTML file
}

app.whenReady().then(createWindow)

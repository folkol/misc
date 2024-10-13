const express = require('express')
const cors = require('cors')
const path = require('node:path');
const app = express()
const port = 3000;


app.disable('etag');

app.get('/junk', (req, res) => {
    res.json({msg: "No Junk for you!"})
});

app.use(cors());

app.get('/stuff', (req, res, next) => {
    setTimeout(() => {
        res.sendFile(path.join(__dirname, 'stuff.json'));
    }, 2500);
})

app.get('/items', (req, res) => {
    res.setHeader('content-type', 'text/xml');
    res.send(
        `<?xml version="1.0" encoding="UTF-8"?>
<text>
  <para>Hello, I am JSON! (Maybe?)</para>
</text>`
    )
});

app.get('/misc', (req, res) => {
    res.status(400).json({'error': 'Missing required query parameter: `version`'})
});

app.listen(port, () => {
    console.log(`Listening on port ${port}`)
});
